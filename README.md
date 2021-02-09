# mass_http

mass_http is a simple async program to send many HEAD requests specified in a file

## Installation 

```sh
git clone https://github.com/EnamelEmu/mass_http
cd mass_http
cargo build
```

## Usage

```sh
USAGE:
    mass_http [OPTIONS] <address-file>
OPTIONS:
    -t, --timeout <time-out>    How long until the connection drops [default: 10]
ARGS:
    <address-file>    The path to the file to read
```
## Screenshot

![Image](https://github.com/EnamelEmu/mass_http/raw/master/img.png)
