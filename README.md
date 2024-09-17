# Project Name: Game of Life

![Game of Life Screenshot](./src/assets/image.png)

## Description
This project is an implementation of Conway's Game of Life in Rust. The Game of Life is a cellular automaton that simulates the evolution of a population of cells based on a set of rules. It is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input.

## Features
- Interactive grid where users can toggle cells on and off
- Customizable grid size and cell colors
- Generation counter to keep track of the evolution
- Play, pause, and reset functionality

## Installation
1. Clone the repository: `git clone https://github.com/your-username/game-of-life.git`
2. Navigate to the project directory: `cd game-of-life`
3. Install the SDL2 library:
    - On macOS, you can install the SDL2 library using Homebrew. Open a terminal and run the following command:
      ```
      brew install sdl2
      ```
    - On Windows, you can install the SDL2 library by following these steps:
      1. Visit the SDL2 website at https://www.libsdl.org/download-2.0.php.
      2. Scroll down to the Development Libraries section and download the appropriate version of the SDL2 development library for your system (32-bit or 64-bit).
      3. Extract the downloaded archive to a location of your choice.
      4. If the SDL2 library is not found, you may need to update your bash or zsh environment variables to include the path to the SDL2 library. You can do this by adding the following line to your `~/.bashrc`, `~/.bash_profile`, or `~/.zshrc` file:
        ```
        export PATH="/path/to/sdl2:$PATH"
        ```
        Replace `/path/to/sdl2` with the actual path to the SDL2 library directory.

4. Build the project: `cargo build`
5. Run the project: `cargo run`

## Usage
2. Use the mouse to toggle cells on and off.
3. Press the spacebar to start or pause the simulation.
4. Press the R key to reset the grid.

## Contributing
Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
