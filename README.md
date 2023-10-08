# Docker Prefetch Image

Configuration-driven downloading/fetching of Docker images.

This can be used to ensure images are present on a host before a container needs to run.

## Usage

```
Usage: docker-prefetch-image [OPTIONS]

Options:
  -d, --docker-socket <DOCKER_SOCKET>  [default: unix:///var/run/docker.sock]
  -c, --config-file <CONFIG_FILE>      [default: docker-prefetch-image.toml]
  -l, --log-level <LOG_LEVEL>          [default: DEBUG]
  -h, --help                           Print help
```

### Example

```
docker-prefetch-image -c docker-prefect-image.toml.example
```


## Still Todo

* Daemon mode to sleep between attempts to pull
* "Alternative" location to download an image.
  * Pull from a remote repository, tag it as the local repository. This would allow a fallback to a remote repository but using the image as if it were the local repository.
