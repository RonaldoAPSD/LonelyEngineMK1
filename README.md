Lonely Engine MK1

Lonely Engine MK1 is a lightweight and minimalistic game engine designed to provide an event-driven architecture for game development. The engine is built with simplicity in mind, allowing developers to focus on game logic while handling core functionalities like rendering, input, and audio.

Features

Event-Driven System: Efficiently manage game updates and interactions.

Minimal Dependencies: Designed to be lightweight with only essential external libraries.

Rendering System: Handles ASCII-based rendering efficiently.

Input System: Provides an easy way to capture user inputs.

Audio System: Basic audio playback support.

Custom Update Loop: Allows integration with the user’s game logic.

Installation

Prerequisites

Rust (latest stable version recommended)

Cargo package manager

Cloning the Repository

git clone https://github.com/RonaldoAPSD/LonelyEngineMK1.git
cd LonelyEngineMK1

Building the Engine

cargo build --release

Running the Example (if available)

cargo run --example demo

Usage

To use Lonely Engine MK1 in your project, add it as a dependency in your Cargo.toml:

[dependencies]
lonely_engine = { path = "../LonelyEngineMK1" }

Basic Setup

use lonely_engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.run();
}

Module Overview

engine.rs

Manages the game loop and event-driven updates.

render.rs

Handles ASCII-based rendering and output.

input.rs

Captures and processes keyboard input.

audio.rs

Provides basic audio playback functionality.

Contributing

Contributions are welcome! Feel free to fork the repository and submit a pull request.

License

This project is licensed under the MIT License. See the LICENSE file for more details.

Contact

For questions or suggestions, feel free to open an issue or reach out to the author.
