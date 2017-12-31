# Pathfinder
Simple pathfinder library in Rust

## License
This project is licensed under "THE BEER-WARE LICENSE" (Revision 42).

<rchabowski@gmail.com> wrote this project. As long as you retain this notice you
can do whatever you want with this stuff. If we meet some day, and you think
this stuff is worth it, you can buy me a beer in return.

Yours,
mgr inż. Rafał

## Scope
This is a little pet-project that I use to build my Rust-skills.

So far the only aim of the library is to find shortest path to the destination on a 2d rectangular map.

## Usage

1. Prepare size of your map
```rust
    let width = 7;
    let height = 5;
```

2. Prepare start and destination coordinates as tuples
```rust
    let start = (1, 1);
    let destination = (4, 1);
```

3. Prepare your test playfield by creating vector of f64. Floating point value represents the penalty of crossing the field
```rust
    let test_level = vec![
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 0.1, 0.7, 0.1, 0.1, 0.1, 1.0,
        1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
        1.0, 0.1, 0.1, 0.1, 0.1, 0.1, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0
    ];
```

4. Call the library function
```rust
    let result = calculate_shortest_path(width, height, test_level, start, destination);
```

5. See the results formatted as JSON
```
    {"steps":[{"x":1,"y":1},{"x":1,"y":2},{"x":2,"y":2},{"x":3,"y":2},{"x":3,"y":1},{"x":4,"y":1}]}
```
Please note that the calculated path avoids the field at 2,1 which has a high penalty defined (0.7).
