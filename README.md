# autostruct

`autostruct` is a command-line utility that generates Rust structs from SQL schema. It connects to a specified database, reads the schema, and generates Rust structs for each table found in the database schema. The generated structs are written to the specified output directory.

## Table of Contents

- [autostruct](#autostruct)
  - [Table of Contents](#table-of-contents)
  - [Description](#description)
  - [User Guide](#user-guide)
    - [Installation](#installation)
    - [Usage](#usage)
      - [Options](#options)
      - [Examples](#examples)
  - [Developer Guide](#developer-guide)
    - [Setup](#setup)
    - [Building and Running](#building-and-running)
    - [Testing](#testing)

## Description

AutoStruct simplifies the process of creating Rust structs from an existing SQL schema. It supports various databases, including PostgreSQL, MySQL, MSSQL, and SQLite. The utility provides options for specifying output directories, database connection strings, and whether to generate struct names in their singular form.

## User Guide

### Installation

To use AutoStruct, you need to have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

1. Clone the repository:

```sh
git clone https://github.com/yourusername/autostruct.git
cd autostruct
```

2. Build the project:

```sh
cargo build --release
```

3. You can now use the `autostruct` binary from the `target/release` directory.

### Usage

AutoStruct provides a `generate` subcommand to generate Rust structs from an SQL schema.

```sh
autostruct generate [OPTIONS]
```

#### Options

- `-o, --output <OUTPUT>`: Sets the directory in which the generated files should be written to. Default is `./output`.
- `-d, --database_url <DATABASE_URL>`: Sets the connection string to the database. Can also be set via the `DATABASE_URL` environment variable.
- `--singular`: Creates struct names in the singular variant of the table name. Default is `false`.
- `-D, --database <DATABASE>`: Specifies the database type (PostgreSQL, MySQL, MSSQL, SQLite). Default is `PostgreSQL`.

#### Examples

Generate Rust structs from a PostgreSQL database:

```sh
autostruct generate -o ./models -d "postgres://user:password@localhost/db" --database PostgreSQL
```

Generate Rust structs from a MySQL database, using the singular form for struct names:

```sh
autostruct generate -o ./models -d "mysql://user:password@localhost/db" --database MySQL --singular
```

## Developer Guide

### Setup

1. Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

2. Clone the repository:

```sh
git clone https://github.com/yourusername/autostruct.git
cd autostruct
```

3. Build the project:

```sh
cargo build
```

### Building and Running

To build the project, run:

```sh
cargo build
```

To run the project with the `generate` command, use:

```sh
cargo run -- generate -o ./output -d "your_database_connection_string" --database PostgreSQL
```

### Testing
The project uses integration tests with test containers to verify functionality against real databases. Currently, PostgreSQL integration tests are implemented.

Integration tests require Docker Desktop to be installed and running. For more information on how to install Docker Desktop, see the [official documentation](https://www.docker.com/products/docker-desktop/).

To run all tests (unit + integration) it is recommended to the commands provided via the Makefile:

```sh
make test.all
```

For more information on the Makefile commands, run:

```sh
make help
```
