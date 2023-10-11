variable "image_id" {
  type        = string
  description = "The docker image used for task."
  default     = "philipcristiano/docker-prefetch-image:0.0.1"
}

job "docker-prefetch-image" {
  datacenters = ["dc1"]
  type        = "system"

  group "app" {

    restart {
      attempts = 2
      interval = "1m"
      delay    = "10s"
      mode     = "delay"
    }

    service {
      name = "docker-prefetch-image"

      check {
        name     = "version"
        type     = "script"
        task     = "app"

        command   = "/usr/local/bin/docker-prefetch-image"
        args      = ["-h"]

        interval = "30s"
        timeout  = "2s"
      }
    }

    task "app" {
      driver = "docker"

      config {
        image = var.image_id

        args = [
          "-c", "local/config.toml"
        ]

        volumes = [
          "/var/run/docker.sock:/var/run/docker.sock"
        ]

      }

      template {
          destination = "local/config.toml"
          data = <<EOF

[[image]]
image = "busybox:latest"

EOF
      }
      resources {
        cpu    = 10
        memory = 16
        memory_max = 64
      }

    }
  }
}

