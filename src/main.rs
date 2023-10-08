use clap::Parser;
use futures::executor::block_on;
use serde::Deserialize;
use std::fs;
use std::str;

use futures::StreamExt;

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
    image: Vec<ImageConfig>,
}

#[derive(Clone, Debug, Deserialize)]
struct ImageConfig {
    image: String,
}

mod app_init;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    app_init::tracing(args.log_level);

    let config_file_error_msg = format!("Could not read config file {}", args.config_file);
    let config_file_contents = fs::read_to_string(args.config_file).expect(&config_file_error_msg);

    let app_config: AppConfig = toml::from_str(&config_file_contents).expect("Problems parsing config file");

    tracing::info!("connecting to {}", args.docker_socket);
    let docker =
        docker_api::Docker::new(args.docker_socket).unwrap();

    let imagesf = block_on(docker.images().list(&Default::default()));
    match imagesf {
        Ok(images) => {
            for image in images {
                println!("{0:?}", image.repo_tags);
            }
        }
        Err(e) => eprintln!("Something bad happened! {e}"),
    }

    let mut exit_code = 0;
    for image in app_config.image {
        let url = image.image;
        tracing::info!("Pulling image {:?}", url);
        // let pull_opts = docker_api::opts::PullOptsBuilder::default().image(url).build();
        let pull_opts = docker_api::opts::PullOpts::builder().image(url).tag("").build();
        tracing::info!("Opts {:?}", pull_opts);
        let dimages = docker.images();
        let mut pull = dimages.pull(&pull_opts);
        while let Some(v) = pull.next().await {
            match v {
                Ok(m) => {
                   tracing::debug!("{:?}", m)

                }
                Err(err) => {
                   tracing::error!("{:?}", err);
                   exit_code = 1
                }
            }
        };

    }

    std::process::exit(exit_code)

}
