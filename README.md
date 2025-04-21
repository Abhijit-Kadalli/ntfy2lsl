# NTFY to LSL Bridge

This application creates a bridge between [ntfy.sh](https://ntfy.sh) notifications and the [Lab Streaming Layer (LSL)](https://github.com/sccn/liblsl) protocol. It subscribes to a specified ntfy.sh channel and forwards received notifications as string data through an LSL stream.

## Overview

This bridge allows you to:
- Stream notifications from ntfy.sh to any LSL-compatible application
- Process notification alerts in real-time neuroscience/BCI experiments
- Monitor system alerts (like disk alerts) in LSL-compatible visualization tools

## Prerequisites

- Rust and Cargo
- An ntfy.sh API token
- LSL libraries installed on your system

## Installation

There are a few ways to build and run this application:

### 1. Using Cargo (Standard Rust Build)

Follow these steps if you have Rust and Cargo installed on your system.

1. Clone this repository:
   ```bash
   git clone https://github.com/Abhijit-Kadalli/ntfy-to-lsl.git
   cd ntfy-to-lsl
   ```

2. Create a `.env` file in the project root with your ntfy.sh API token:
   ```
   NTFY_API_TOKEN=your_token_here
   ```

3. Build the application:
   ```bash
   cargo build --release
   ```

### 2. Using the Build Script (Linux)

A convenience script `build.sh` is provided to automate dependency fetching (including the LSL library) and building the project on Linux systems.

1. Clone the repository (if you haven't already):
   ```bash
   git clone https://github.com/Abhijit-Kadalli/ntfy-to-lsl.git
   cd ntfy-to-lsl
   ```

2. Create the `.env` file as described above.

3. Make the script executable and run it:
   ```bash
   chmod +x build.sh
   ./build.sh
   ```
   This will build the release binary in the `target/release/` directory.

### 3. Using Docker

A `DOCKERFILE` is included for building a containerized version of the application. This is useful for ensuring a consistent build environment.

1. Clone the repository (if you haven't already):
   ```bash
   git clone https://github.com/Abhijit-Kadalli/ntfy-to-lsl.git
   cd ntfy-to-lsl
   ```

2. Create the `.env` file as described above.

3. Build the Docker image:
   ```bash
   docker build -t ntfy-to-lsl .
   ```

## Usage

### Running the Binary (Cargo or Build Script)

After building with Cargo or the build script, run the application from the project root:

```bash
./target/release/ntfy2lsl
```

Alternatively, if you built with `cargo build` (without `--release`), use:
```bash
cargo run
```

### Running with Docker

Run the application using the Docker image, making sure to pass the `.env` file:

```bash
docker run --rm --env-file .env ntfy-to-lsl
```

The application will:
1. Connect to the ntfy.sh "diskalerts" channel
2. Create an LSL stream named "NtfyAlerts" with stream type "Notifications"
3. Forward any received notifications to the LSL stream
4. Automatically attempt to reconnect to the NTFY stream if the connection is lost.

## Configuration

Currently, the application is configured to monitor the "diskalerts" channel. To modify this or other settings, you'll need to update the source code.

## LSL Stream Details

- Stream name: "NtfyAlerts"
- Stream type: "Notifications"
- Data format: String
- Channel count: 1
- Stream ID: "ntfy_alerts_001"

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 