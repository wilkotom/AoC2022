# Advent Of Code Helpers

Various structs and associated methods which may come in useful when solving Advent of Code challenges:

- `Direction` - Compass Direction
- `Coordinate` - Standard 2D Cartesian Coordinate
- `Particle` - Location with compass direction
- `Rectangle` - Pair of Coordinates describing a rectangle
- `Coordinate3d` - Standard 3D Cartesian Coordinate
- `Cuboid` - Pair of 3D Coordinates describing a cuboid
- `ScoredItem` - Used with std::collections::BinaryHeap to implement A* or Djikstra's algorithms
- `get_daily_input()` - fetches and caches the input for a given day's puzzle
- `parse_number_grid<T>()` - converts a grid of 0-9 digits to a HashMap<Coordinate<usize>, T>
