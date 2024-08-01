# SSH Tunnel Manager

SSH Tunnel Manager is a simple, user-friendly GUI application for managing SSH tunnels. It allows you to easily start and stop SOCKS proxy tunnels defined in a configuration file.

## Features

- GTK-based graphical user interface
- System tray icon for easy access
- Ability to hide the main window without quitting the application
- Configuration via YAML file
- Start and stop SSH tunnels with a simple switch

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming environment
- GTK3 development libraries
- libappindicator3 development libraries

On Ubuntu or Debian-based systems, you can install the required libraries with:

```sh
sudo apt-get install libgtk-3-dev libappindicator3-dev
```

## Installation

1. Clone this repository:
   ```sh
   git clone https://github.com/yourusername/ssh-tunnel-manager.git
   cd ssh-tunnel-manager
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. The binary will be available at `target/release/ssh_tunnel_manager`

## Configuration

Create a configuration file at `~/.config/ssh_tunnel_manager/config.yaml` with the following structure:

```yaml
tunnels:
  - name: Example Tunnel
    socks_port: 9000
    ssh_host: user@example.com
  - name: Another Tunnel
    socks_port: 9001
    ssh_host: user@another-example.com
```

## Usage

1. Run the application:
   ```sh
   ./target/release/ssh_tunnel_manager
   ```

2. The main window will appear, showing a list of configured tunnels with switches to start/stop them.

3. You can close the main window to hide it. The application will continue running in the system tray.

4. Click the tray icon to show the menu, where you can:
   - Show the main window again
   - Quit the application

## Contributing

Contributions to the SSH Tunnel Manager are welcome. Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [GTK-rs](https://gtk-rs.org/) for the Rust GTK bindings
- [libappindicator](https://github.com/libappindicator/libappindicator) for the system tray functionality
