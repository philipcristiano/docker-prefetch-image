# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/philipcristiano/docker-prefetch-image/compare/v0.1.4...v0.2.0) (2024-04-02)


### Features

* Fetch alternatives and tag as the desired image ([2a81ca0](https://github.com/philipcristiano/docker-prefetch-image/commit/2a81ca08627af942b1ec7d08ec756fa2fb070d98))


### Bug Fixes

* Avoid cargo caching tmp main.rs ([4bf0d8b](https://github.com/philipcristiano/docker-prefetch-image/commit/4bf0d8b9ed0017062b9d047eace3491d7061e870))
* **deps:** update rust crate anyhow to 1.0.81 ([d734433](https://github.com/philipcristiano/docker-prefetch-image/commit/d734433c67a98941f78c6b1fd83059318bf274c3))
* **deps:** update rust crate clap to 4.5.2 ([5fb46a4](https://github.com/philipcristiano/docker-prefetch-image/commit/5fb46a4f52f31e92e77348c8e834d8d058e88209))
* **deps:** update rust crate clap to 4.5.3 ([51de76d](https://github.com/philipcristiano/docker-prefetch-image/commit/51de76d24889f6d09be37a697bfff003213d2c9b))
* **deps:** update rust crate clap to 4.5.4 ([2eb6f57](https://github.com/philipcristiano/docker-prefetch-image/commit/2eb6f579fde84bad0aac4fe49d762b64532b9f88))
* **deps:** update rust crate service_conventions to 0.0.10 ([1064808](https://github.com/philipcristiano/docker-prefetch-image/commit/1064808fc5759b8032459992fe58ac42f9546af8))
* **deps:** update rust crate service_conventions to 0.0.12 ([21f5821](https://github.com/philipcristiano/docker-prefetch-image/commit/21f5821b82e969bdb5c32809b46334560ea59194))
* **deps:** update rust crate service_conventions to 0.0.3 ([2a7d298](https://github.com/philipcristiano/docker-prefetch-image/commit/2a7d2982c444bcc4bf7b02bc8fb9c21845269b03))
* **deps:** update rust crate service_conventions to 0.0.4 ([10a4d7e](https://github.com/philipcristiano/docker-prefetch-image/commit/10a4d7e7731da8dd73e5977cc5f90cb7d3d57f9d))
* **deps:** update rust crate service_conventions to 0.0.5 ([93829b3](https://github.com/philipcristiano/docker-prefetch-image/commit/93829b37dfb71d75ae16ae7155b36155ec0d2671))
* **deps:** update rust crate service_conventions to 0.0.6 ([6fe1ee3](https://github.com/philipcristiano/docker-prefetch-image/commit/6fe1ee39f52682b151921c413d4e663d2b202ca6))
* **deps:** update rust crate service_conventions to 0.0.7 ([785aba3](https://github.com/philipcristiano/docker-prefetch-image/commit/785aba3c49da4f7193d38b5cc67977d976084de0))
* **deps:** update rust crate service_conventions to 0.0.8 ([de11045](https://github.com/philipcristiano/docker-prefetch-image/commit/de1104551a24db03928d683f6e82131f244df3d4))
* **deps:** update rust crate tokio to 1.37.0 ([3081157](https://github.com/philipcristiano/docker-prefetch-image/commit/3081157e23a88e4272520c03dcc520d6257a8404))
* **deps:** update rust crate toml to 0.8.11 ([32f57cb](https://github.com/philipcristiano/docker-prefetch-image/commit/32f57cb243183b27167ffa3004d078f6a61efb30))
* **deps:** update rust crate toml to 0.8.12 ([22f2b83](https://github.com/philipcristiano/docker-prefetch-image/commit/22f2b835db3393de3db717f7f4ccc1b47f1c6241))

## [Unreleased]

## [0.1.4](https://github.com/philipcristiano/docker-prefetch-image/compare/v0.1.3...v0.1.4) - 2023-12-05

### Other
- Docker release

## [0.1.3](https://github.com/philipcristiano/docker-prefetch-image/compare/v0.1.2...v0.1.3) - 2023-12-05

### Other
- Attempt to push semver tagged docker images

## [0.1.2](https://github.com/philipcristiano/docker-prefetch-image/compare/v0.1.1...v0.1.2) - 2023-12-05

### Other
- Use Github PAT

## [0.1.1](https://github.com/philipcristiano/docker-prefetch-image/compare/v0.1.0...v0.1.1) - 2023-12-05

### Other
- Use library for tracing setup
- Use variable instead of repo name
- Add permissions needed for github Token
- Avoid reserved token conflict
- Release packages if tests pass
- Try reusable
- release

## [0.1.0](https://github.com/philipcristiano/docker-prefetch-image/releases/tag/v0.1.0) - 2023-12-04

### Fixed
- Add doc and license for upload to crates.io

### Other
- Check conventional commits
- release-plz
- *(deps)* bump rust from 1.73-bookworm to 1.74-bookworm
- Remove unused dep: axum
- *(deps)* bump the patch-dependencies group with 1 update
- *(deps)* bump the patch-dependencies group with 1 update
- Group levels
- *(deps)* bump the dependencies group with 3 updates
- Adjust for otel renaming
- *(deps)* bump the opentelemetry group with 4 updates
- *(deps)* bump the dependencies group with 3 updates
- Update dependabot.yml
- *(deps)* bump clap from 4.4.7 to 4.4.8
- *(deps)* bump toml from 0.8.6 to 0.8.8
- Bump serde from 1.0.189 to 1.0.190
- Bump clap from 4.4.6 to 4.4.7
- Bump toml from 0.8.2 to 0.8.6
- Bump futures from 0.3.28 to 0.3.29
- *(deps)* bump serde_json from 1.0.107 to 1.0.108
- Allow automerge
- Add permissions for automated review
- Automerge dependabot PRs
- Bump tracing from 0.1.39 to 0.1.40
- Bump tracing from 0.1.37 to 0.1.39
- Bump serde from 1.0.188 to 1.0.189
- Split out functions
- Bump rust from 1.72-bookworm to 1.73-bookworm
- Include example job file
- Include structured data for image
- Bump tokio from 1.32.0 to 1.33.0
- Loop fetching
- Bump tracing-opentelemetry from 0.20.0 to 0.21.0
- Bump toml from 0.7.8 to 0.8.2
- Add README
- Exit non-zero on error
- Image from a config file
- List images and fetch
- Initial commit
