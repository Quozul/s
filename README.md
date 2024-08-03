# `s`

`s` is a command manager, configure a list of commands, run `s` and choose the command you want to run.  
It was firstly intended to use as an SSH connection manager.

## Installation

### Cargo

```shell
cargo install --git https://github.com/Quozul/s.git
```

## Configuration

The configuration directory is as follows:

- Linux: `$XDG_CONFIG_HOME/s` or `$HOME/.config/s`
- Windows: `{FOLDERID_RoamingAppData}/s/config`
- MacOS: `$HOME/Library/Application Support/s`

A single `config.toml` file is expected containing a list of commands. Here is an example:

```toml
Hello = "echo Hello, World!"
PingGoogle = "ping 8.8.8.8"
```
