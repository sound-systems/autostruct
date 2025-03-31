# autostruct

`autostruct` is a command-line utility that generates Rust structs from PostgreSQL database schemas. It connects to a PostgreSQL database, reads the schema, and generates Rust structs for each table found in the database schema. The generated structs are written to the specified output directory.

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

AutoStruct simplifies the process of creating Rust structs from an existing PostgreSQL schema. It supports various PostgreSQL data types including:

- Basic types (integer, numeric, boolean, etc.)
- Character types (varchar, text, citext)
- Date/Time types (timestamp, date, time, interval)
- Network types (inet, cidr, macaddr)
- JSON types (json, jsonb)
- Geometric types (point, line, polygon)
- Range types (int4range, numrange, daterange)
- Arrays
- Composite types
- Enumerated types
- And more

The generated structs include proper type mappings and can be used with popular Rust ORMs like SQLx.

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

AutoStruct provides a `generate` subcommand to generate Rust structs from a PostgreSQL schema.

```sh
autostruct generate [OPTIONS]
```

#### Options

- `-o, --output <OUTPUT>`: Sets the directory in which the generated files should be written to. Default is `./output`.
- `-d, --database_url <DATABASE_URL>`: Sets the connection string to the PostgreSQL database. Can also be set via the `DATABASE_URL` environment variable.
- `--singular`: Creates struct names in the singular variant of the table name. Default is `false`.
- `--framework <FRAMEWORK>`: Specifies the framework to use for generated code. Options are:
  - `none`: Basic struct generation with Debug and Clone derives
  - `sqlx`: Adds SQLx-specific derives and types
- `--exclude <EXCLUDE>`: Tables to exclude from generation (can be specified multiple times)
- `-t, --timeout`: Sets the timeout to be used when establishing a database connection

#### Examples

Generate Rust structs from a PostgreSQL database:

```sh
autostruct generate -o ./models -d "postgres://user:password@localhost/db"
```

Generate Rust structs with SQLx support and singular table names:

```sh
autostruct generate -o ./models -d "postgres://user:password@localhost/db" --framework sqlx --singular
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
cargo run -- generate -o ./output -d "your_database_connection_string"
```

### Testing

The project uses integration tests with test containers to verify functionality against a real PostgreSQL database. Integration tests require Docker Desktop to be installed and running. For more information on how to install Docker Desktop, see the [official documentation](https://www.docker.com/products/docker-desktop/).

To run all tests (unit + integration) it is recommended to use the commands provided via the Makefile:

```sh
make test.all
```

For more information on the Makefile commands, run:

```sh
make help
```

## Contributing

While the project currently focuses on PostgreSQL support, we welcome contributions to add support for other databases. If you're interested in adding support for another database, please open an issue to discuss the implementation approach.
