use clap::Parser;
use futures::executor::block_on;
use serde::Deserialize;
use std::str;

use futures::StreamExt;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "unix:///var/run/docker.sock")]
    docker_socket: String,
    #[arg(short, long, value_enum, default_value = "INFO")]
    log_level: tracing::Level,
    #[arg(long, action)]
    log_json: bool,
}

#[derive(Clone, Debug, Deserialize)]
struct AppConfig {}

mod app_init;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    app_init::tracing(args.log_level);

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

    let url = "docker-registry.home.cristiano.cloud/busybox:latest";
    let pull_opts = docker_api::opts::PullOptsBuilder::default().image(url).build();
    let dimages = docker.images();
    let mut pull = dimages.pull(&pull_opts);
    while let Some(v) = pull.next().await {
        println!("{:?}", v)

    };

}
