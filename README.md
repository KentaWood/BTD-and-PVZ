# Monkey Shooter and Plants vs Zombies Game Prototypes

## Project Idea

This project contains two different game prototypes developed using Rust. The first game, "Monkey Shooter," is inspired by the popular tower defense genre where players place monkeys to shoot darts at incoming balloons. The second game, "Plants vs Zombies," is a simplified version of the classic game where players place plants to shoot peas at approaching zombies. These prototypes demonstrate the use of game development concepts and Rust programming skills, showcasing the ability to create engaging and interactive games.

## Used Elements

### Libraries and Frameworks
- **engine_immediate**: A custom game engine module for handling game logic, rendering, and collision detection.
- **kira**: An audio management library for playing sound effects and background music.

### Game Assets
- **Spritesheets**: Various image assets for characters, background, and objects within the game.
- **Audio**: Background music and sound effects to enhance the gaming experience.

### Key Rust Modules
- `std::time::{Duration, Instant}`: For handling game timing and intervals.
- `rand::Rng`: For generating random numbers, used in zombie spawning logic.

### Core Components
- **Game Structures**: Structs for managing game state and entities (e.g., Monkey, Balloon, Dart, Circle, Plant, Zombie, Pea).
- **Collision Detection**: Custom collision detection logic to handle interactions between game objects.
- **Rendering**: Drawing sprites and text on the screen to create the game visuals.

## How to Play

### Monkey Shooter
1. **Start the Game**: Click anywhere on the screen to begin the game.
2. **Place Monkeys**: Click on the circles to place monkeys. Each monkey costs points.
3. **Shoot Balloons**: Monkeys automatically shoot darts at balloons. Destroy balloons to earn points.
4. **Pause the Game**: Press the 'P' key to pause or resume the game.
5. **Game Over**: If all balloons are destroyed, a win screen is displayed. If a balloon reaches the end of the path, the game ends.

### Plants vs Zombies
1. **Start the Game**: Click anywhere on the screen to begin the game.
2. **Place Plants**: Click on the grid to place plants. Each plant costs sunflower points.
3. **Shoot Zombies**: Plants automatically shoot peas at approaching zombies. Destroy zombies to prevent them from reaching the end of the path.
4. **Earn Sunflower Points**: Points are periodically earned to allow placing more plants.
5. **Game Over**: If a zombie reaches the end of the path, the game ends, displaying a game over screen.

## Running the Games

### Prerequisites
- Ensure you have Rust installed on your system.

### Steps to Run

1. **Clone the Repository**: 
   ```sh
   git clone https://github.com/yourusername/game-prototypes.git
   cd game-prototypes
   ```
2. **Add Assets**: Place the required assets (sprites and audio files) in the `assets` directory.
3. **Build and Run**: To run either game, ensure you are in the root directory of the repository. Then use the following commands:

   **For Monkey Shooter:**
   ```sh
   cargo run --bin btd
   ```

   **For Plants vs Zombies:**
   ```sh
   cargo run --bin pvz
   ```

### Controls
- **Mouse Click**: Place objects (monkeys/plants) or interact with the game.
- **P Key**: Pause or resume the game (Monkey Shooter).

These prototypes serve as a demonstration of game development capabilities and can be expanded further with more features, levels, and improved graphics.
