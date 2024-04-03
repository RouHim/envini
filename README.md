# ENVINI

Allows to declarative define environment variables that are mapped to ini file values.

## Motivation

In the past I developed a lot of container images for game server.
During this, I was facing the need to have a way to define essential server configuration in a declarative way.
Most of the time you have to fiddle with ini files, which is not very user-friendly in a containerized environment.
The idea is to have transition layer between the declarative environment variables and the ini files located in the
container.

## Build

```shell
cargo build
```

## Install

Use the build from the [latest release](https://github.com/RouHim/envini/releases) or build the binary yourself.

## Usage

### How it works

Follow this steps to use envini in an container:

1) Declare the mapping between the environment variables and the ini file properties in a mapping file.
2) Add booth, the envini binary and the mapping file to the container.
3) Declare the environment variables you want to map in the container file.
4) Make sure to execute _envini_ before the actual server starts, to apply the mapping every time the container
   starts. (E.g. in an entrypoint script)
5) The environment variables are evaluated and the values are written to the ini file.
6) Start the actual server.

### Mapping configuration

First you have to create a mapping configuration file,
which defines the mapping between the environment variables and the ini file properties.
Example `envini_mapping.ini`:

```ini
[KF2_SERVER_NAME]
ini_file = server-config.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerName

[KF2_SERVER_PORT]
ini_file = server-config.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerPort
```

### Parameter

Each section in the mapping file represents a mapping between an environment variable and an ini property.
The following parameters per section are available:

| Name                       | Description                                                                                   | Example                      |
|----------------------------|-----------------------------------------------------------------------------------------------|------------------------------|
| `[<ENVIRONMENT_VARIABLE>]` | The name of the environment variable that will contain the value to write to the ini property | `[KF2_SERVER_NAME]`          |
| `ini_file`                 | The path to the ini file to be modified                                                       | `server-config.ini`          |
| `ini_section`              | The section in the ini file, can be empty                                                     | `Engine.GameReplicationInfo` |
| `ini_key`                  | The ini property key in the section, to this property will the env variable value be applied  | `ServerName`                 |

To apply the mapping run the following command:

```shell
envini <path/to/config.ini>
```

This will evaluate the configured environment variables,
and writes the values to the ini file as configured in the config file.

## Example implementation

Here we have an example implementation of a container using envini.

### envini_mapping.ini

```ini
[SERVER_NAME]
ini_file = server-config.ini
ini_section = GameSettings
ini_key = ServerName
```

### Dockerfile

```dockerfile
FROM ubuntu:latest
ENV SERVER_NAME=""

# Install envini
COPY envini /envini
COPY envini_mapping.ini /envini_mapping.ini

# Install the game server
COPY game-server /server/game-server
COPY server-config.ini /server/server-config.ini

# Start the server
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
```

### entrypoint.sh

```shell
#!/bin/bash

# Apply the mapping
/envini /envini_mapping.ini

# Start the server
/server/game-server
```

### server-config.ini

```ini
[GameSettings]
ServerName = Default server name
```

### docker-compose.yml

```yaml
services:
  game-server:
    image: my-game-server
    environment:
      SERVER_NAME: "My fancy server"
```

When starting the server with the docker-compose file, the environment variable `SERVER_NAME` will be applied to the
`ServerName` property in the `server-config.ini` file.

