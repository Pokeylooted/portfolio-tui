# Portfolio TUI Viewer

A Terminal User Interface (TUI) application built in Rust that displays portfolio information fetched from a GitHub _config.yml file.

## Description

This application provides a developer-friendly way to view portfolio information directly from the terminal. It fetches data from a GitHub repository's _config.yml file and displays it in a well-organized terminal interface.

By default, it uses the _config.yml file from https://github.com/Pokeylooted/Pokeylooted.github.io, but it can be configured to use any compatible _config.yml file.

## Features

- Terminal-based portfolio viewer
- Fetches data from GitHub or local files
- Displays personal information, projects, skills, and social links
- ASCII art logo on the Home view
- Dynamic content section navigation based on YAML file
- Keyboard navigation with arrow keys and numeric shortcuts
- Support for local _config.yaml file
- Dark mode support

## Installation

### Prerequisites

- Rust and Cargo (https://rustup.rs/)

### Building from Source

1. Clone the repository:
   ```
   git clone <repository-url>
   cd portfolio-tui
   ```

2. Build the application:
   ```
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/portfolio-tui`

## Usage

### Running the Application

```
cargo run
```

Or, if you've built the release version:

```
./target/release/portfolio-tui
```

### Navigation

- Press `h` to return to Home view
- Press `←/→` arrow keys to navigate between content sections
- Press number keys (0-9) for direct access to specific content sections
- Press `q` to quit the application

## Configuration

The application can be configured to use a different _config.yml file in two ways:

1. By placing a local `_config.yaml` file in the same directory as the executable
2. By modifying the URL in the `app.rs` file

Future versions will support command-line arguments for specifying the source file.

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── config/                 # Configuration handling
│   ├── mod.rs
│   ├── args.rs             # Command-line arguments
│   └── settings.rs         # Application settings
├── data/                   # Data handling
│   ├── mod.rs
│   ├── fetcher.rs          # Data fetching logic
│   ├── parser.rs           # YAML parsing
│   └── models.rs           # Data structures
├── processor/              # Data processing
│   ├── mod.rs
│   └── formatter.rs        # Data formatting for display
└── ui/                     # User interface
    ├── mod.rs
    ├── app.rs              # Main application state
    ├── events.rs           # Event handling
    ├── ascii_art.rs        # ASCII art for the application
    └── views/              # Different UI views
        ├── mod.rs
        ├── home.rs         # Home view
        ├── content.rs      # Dynamic content section view
        ├── projects.rs     # Projects view (legacy)
        ├── skills.rs       # Skills view (legacy)
        └── about.rs        # About view (legacy)
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework for Rust
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library
- [serde](https://serde.rs/) - Serialization/deserialization framework
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client

## Recent Updates

- Added dynamic content section navigation based on YAML file structure
- Implemented Home view with portfolio owner's name, title, ASCII art, and contact info
- Added support for reading from a local _config.yaml file
- Improved navigation with arrow keys and numeric shortcuts
- Fixed compiler warnings and improved code organization