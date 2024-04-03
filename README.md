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

First you have to create a mapping configuration file,
which defines the mapping between the environment variables and the ini file properties.
Example `envini_mapping.ini`:

```ini
[KF2_SERVER_NAME]
ini_file = server-config.ini
ini_section = Engine.GameReplicationInfo
ini_key = ServerName
```

### Parameter

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