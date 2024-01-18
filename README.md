[![Rust](https://github.com/katekorsaro/iron-dice/actions/workflows/rust.yml/badge.svg?branch=next)](https://github.com/katekorsaro/iron-dice/actions/workflows/rust.yml)

# iron-dice

A CLI die roller for every RPG enthusiast.

## Usage

- `iron_dice` will yield the result of 3d6
- `iron_dice -d "d20"` will throw and yield 1d20
- `iron_dice -d "d100+40"` will throw and yield 1d100 and will add 40 to the result
- `iron_dice -d "5d6 sc6"` will throw 5d6 and for every 6 will count 1 success
- `iron_dice -d "4d6 max3"` will throw 4d6 keeping the largest 3
- `iron_dice -d "3d8 ex7"` will throw 3d8 and explode every die showing 7 or more
- `iron_dice -d "3d6" -t 5` will throw 3d6 5 times
