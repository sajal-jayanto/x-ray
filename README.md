# X-Ray CLI

A simple, interactive CLI tool built with Rust to create, initialize, build, and run projects using Docker.

X-Ray helps you quickly bootstrap projects with preconfigured runtime environments without manually creating Dockerfiles.

## Features

- **Create Project** – Create a new project in a separate directory.
- **Initialize Project** – Initialize a project in the current directory.
- **Runtime Selection** – Choose your preferred runtime interactively.
- **Automatic Dockerfile Generation** – Generate a Dockerfile based on the selected runtime.
- **Automatic Docker Build** – Build a Docker image when running a project for the first time.
- **Image Reuse** – Skip building if the Docker image already exists.
- **Container Execution** – Run your project inside a Docker container.

## Supported Runtimes

| Runtime | Version                      | Main File   |
| ------- | ---------------------------- | ----------- |
| Node.js | 24.x                         | `index.js`  |
| Java    | 25                           | `Main.java` |
| Go      | Latest supported by template | `main.go`   |

## Prerequisites

Make sure you have installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Docker](https://docs.docker.com/get-docker/)

Docker must be running before executing the `run` command.

## Installation

Clone the repository:

```
git clone https://github.com/your-username/x-ray.git
```

Navigate to the project:

```
cd x-ray
```

Build the CLI:

```
cargo build --release
```

Run the CLI:

```
cargo run --release
```

## Usage

### 1. Create a New Project

```
x-ray create my-app
```

Choose a runtime from the interactive menu.

Example output:

```
Choose a runtime:
> Node.js 24.x
  Java 25
  Go
```

Generated structure:

```
my-app/
├── index.js
└── Dockerfile
```

The generated files depend on your selected runtime.

### 2. Initialize an Existing Project

Navigate to your project directory:

```
cd my-app
```

Run:

```
x-ray init
```

This generates the runtime-specific main file and Dockerfile in the current directory.

### 3. Run a Project

Navigate to the project directory:

```
cd my-app
```

Execute:

```
x-ray run
```

X-Ray will:

1. Check whether a Docker image with the current directory name exists.
2. Build the Docker image if it does not exist.
3. Reuse the existing image if available.
4. Start a Docker container.
5. Display the container output in your terminal.

## Generated Dockerfile Examples

### Node.js

```
FROM node:24-alpine
WORKDIR /app
COPY . .
CMD ["node", "index.js"]
```

### Java

```
FROM eclipse-temurin:25-jdk-alpine
WORKDIR /app
COPY . .
RUN javac Main.java
CMD ["java", "Main"]
```

### Go

```
FROM golang:1.23-alpine
WORKDIR /app
COPY . .
RUN go build -o app main.go
CMD ["./app"]
```

## Tech Stack

- **Rust** – CLI application development.
- **Clap** – Command-line argument parsing.
- **Dialoguer** – Interactive runtime selection.
- **Indoc** – Clean multiline string formatting.
- **Docker** – Project containerization and execution.

## Project Structure

```
x-ray/
├── src/
│   ├── main.rs
│   └── runtime.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Roadmap

- Add Python runtime support.
- Add Rust runtime support.
- Add custom runtime configuration.
- Add environment variable support.
- Add Docker volume mounting.
- Add port mapping.
- Add container stop and remove commands.
- Add project templates.
- Add interactive container management.
- Add container logs and status commands.

## Contributing

Contributions, feature requests, and bug reports are welcome.

Feel free to fork the repository and submit a pull request.

## License

MIT License.

One important improvement: Your current `run` command uses the directory name as the Docker image name. Consider adding `--rm` to `docker run` so finished containers are automatically removed. Also, add `-p` and `-v` options later for port mapping and volume mounting.