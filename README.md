<div align="center">
  <h1><code>rust-maze-gen</code></h1>

  <span>Commandline maze generator, written in Rust</span>
</div>

![](maze.png)

## Installation

```sh
cargo build --release
```

## Usage example

You can build the project like any rust project and run it with

```sh
# Build optimised release
cargo build --release

# Run the tool with width and height args
./target/release/maze-gen 24 12
```

Or you can run it directly with `cargo run [width] [height]`.

```sh
cargo run        # Generates 16 x 16 maze
cargo run 24     # Generates 24 x 24 maze
cargo run 24 12  # Generates 24 wide x 12 tall maze
```

## Release History

* 0.1.0
  * Initial release

## License

Distributed under the MIT license. See [LICENSE](LICENSE.md) for more information.
