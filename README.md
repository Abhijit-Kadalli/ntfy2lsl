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

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/ntfy-to-lsl.git
   cd ntfy-to-lsl
   ```

2. Create a `.env` file in the project root with your ntfy.sh API token:
   ```
   NTFY_API_TOKEN=your_token_here
   ```

3. Build the application:
   ```
   cargo build --release
   ```

## Usage

Run the application:

```
cargo run --release
```

The application will:
1. Connect to the ntfy.sh "diskalerts" channel
2. Create an LSL stream named "NtfyAlerts" with stream type "Notifications"
3. Forward any received notifications to the LSL stream

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