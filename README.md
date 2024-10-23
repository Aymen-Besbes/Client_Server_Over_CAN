# Client Server Communication Using CAN

## Overview
This repository is presenting a simple Client Server communication for CAN Protocol Using Rust Programming Language.
The client sends a data frame within a frameid via an interface configured into the configuration file.toml
## Project Structure

* Src Directory :
    - client.rs: Implementation of the client side to handle creation of the frame and sending it to the server.
    - server.rs: Implementation of the server side to create a virtual CAN interface and receiving the data from the client.
    - config.rs: Implementation of the configuration structure.

* Config Directory :

Contains the default configuration and other configurations using toml files.

* Cargo.Toml :

    Contains the project configuration including the dependencies, the binaries, etc...


## Usage:
1. clone the project via : git clone https://gitlab.com/Aymen-Besbes/client_server_over_can.git
2. Run $cargo build
2. Enter to the working directory and in different terminals run :
    - $cargo run --bin client config_file
    - $cargo run --bin server config_file