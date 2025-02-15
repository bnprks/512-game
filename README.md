# 512
A clone of [128](https://ashervollmer.github.io/2048/128.html) by Asher Vollmer, which is itself a variant of the famous 2048 game.

This has a few notable improvements over 2048:
- A smaller grid results in shorter games
- The new tiles are always 2, resulting in more predictable strategic gameplay

These two changes make it feasible to max out the board, filling it with one tile each from 512 down to 2. This is quite challenging to do, but possible with a little luck and persistence.

I've added hints at the bottom which use a pre-calculated database of perfect play. The cool things about this are:
- It turns out that perfect play allows a 100% win rate, leaving nothing up to chance (!)
- Using rust + webassembly, we can use a single code base to read and write the perfect
  play database

Getting the database down to 10MB was a fun challenge! We can reduce the 56 million reachable boards down to 7 million positions after removing reduntant rotations and reflections. Then, because we know all possible input positions, we can use the [BBHash](https://github.com/rizkg/BBHash) algorithm to get a datastructure that maps each board to a unique index with only ~3 bits per board, then store the perfect-play win rate for each position in a plain array of 8-bit numbers.

## Setup
To get the build dependencies, first [install rust](https://www.rust-lang.org/tools/install), then run `cargo install wasm-pack`.


To build the perfect play database:
```shell
cargo run --release strategy.bin
```

To build the webassembly package:
```shell
wasm-pack build --target no-modules --out-dir js/strategy --release --no-pack --no-typescript
rm js/strategy/.gitignore
```


## License
2048 is licensed under the [MIT license.](https://github.com/gabrielecirulli/2048/blob/master/LICENSE.txt), as is this fork

