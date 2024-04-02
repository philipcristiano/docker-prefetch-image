use clap::Parser;
use futures::executor::block_on;
use serde::Deserialize;
use std::fs;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use futures::StreamExt;
use thiserror::Error;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "unix:///var/run/docker.sock")]
    docker_socket: String,
    #[arg(short, long, default_value = "docker-prefetch-image.toml")]
    config_file: String,
    #[arg(short, long, value_enum, default_value = "DEBUG")]
    log_level: tracing::Level,
    #[arg(long, action)]
    log_json: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct AppConfig {
    period_seconds: Option<u64>,
    image: Vec<ImageConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct ImageConfig {
    image: String,
    #[serde(default)]
    alternative_images: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    service_conventions::tracing::setup(args.log_level);

    let app_config = read_app_config(args.config_file);
    // Signal handling
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();

    // Start connections
    tracing::info!("connecting to {}", args.docker_socket);
    let docker = docker_api::Docker::new(args.docker_socket).unwrap();

    list_images(&docker);

    let sleep_duration = Duration::from_secs(app_config.period_seconds.unwrap_or(1500));

    while !term.load(Ordering::Relaxed) {
        for image in app_config.image.clone() {
            if pull_image_or_alternatves(&docker, &image).is_err() {
                tracing::error!("Failed to pull image or alternatives");
            };
        }

        tracing::info!("Sleeping for {:?}", sleep_duration);
        sleep(sleep_duration).await;
    }
}

fn read_app_config(path: String) -> AppConfig {
    let config_file_error_msg = format!("Could not read config file {}", path);
    let config_file_contents = fs::read_to_string(path).expect(&config_file_error_msg);
    let app_config: AppConfig =
        toml::from_str(&config_file_contents).expect("Problems parsing config file");

    app_config
}

fn list_images(docker: &docker_api::Docker) {
    let imagesf = block_on(docker.images().list(&Default::default()));
    match imagesf {
        Ok(images) => {
            for image in images {
                println!("{0:?}", image.repo_tags);
            }
        }
        Err(e) => eprintln!("Something bad happened! {e}"),
    }
}

#[derive(Error, Debug)]
pub enum PullError {
    #[error(transparent)]
    PullError(#[from] docker_api::Error),
}

fn pull_image_or_alternatves(
    docker: &docker_api::Docker,
    image_config: &ImageConfig,
) -> Result<(), PullError> {
    let image_url = image_config.image.clone();
    let pull_result = pull_image(&docker, image_url.clone());
    if pull_result.is_ok() {
        return Ok(());
    };
    tracing::info!("Could not pull image {}", image_config.image);
    for alternative in &image_config.alternative_images {
        tracing::info!("Trying alternative image {}", alternative);
        if pull_image(&docker, alternative.clone()).is_ok() {
            tracing::info!("Tagging alternative {}", alternative);
            if tag_image_as(&docker, &alternative, &image_url).is_ok() {
                return Ok(());
            }
        }
    }
    Ok(())
}

fn pull_image(docker: &docker_api::Docker, url: String) -> Result<(), PullError> {
    tracing::info!("Pulling image {:?}", url);
    // let pull_opts = docker_api::opts::PullOptsBuilder::default().image(url).build();
    let pull_opts = docker_api::opts::PullOpts::builder()
        .image(url.clone())
        .tag("")
        .build();
    tracing::info!("Opts {:?}", pull_opts);
    let dimages = docker.images();
    let mut pull = dimages.pull(&pull_opts);
    while let Some(v) = block_on(pull.next()) {
        match v {
            Ok(m) => {
                tracing::debug!(image = url, "{:?}", m)
            }
            Err(err) => {
                tracing::error!(image = url, "{:?}", err);
                Err(err)?
            }
        }
    }
    return Ok(());
}

fn tag_image_as(
    docker: &docker_api::Docker,
    image: &String,
    new_tag: &String,
) -> Result<(), docker_api::Error> {
    let docker_image = docker_api::Image::new(docker.clone(), image.clone());
    let tag_opts = docker_api::opts::TagOpts::builder().repo(new_tag).build();
    block_on(docker_image.tag(&tag_opts))?;
    return Ok(());
}
