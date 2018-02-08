# contrail

Takes a series of commands, executes them asynchronously, then
concatenates their output in their original calling order.

For example:

```bash
contrail -c "echo Hello, world!" \
         -c "pwd" \
         -c "ls" \
         -c "git status"
```

```
Hello, world!
/home/z/development/contrail
#README.md#
Cargo.lock
Cargo.toml
LICENSE
README.md
src
target
On branch 0.3-refactoring
Your branch is up to date with 'origin/0.3-refactoring'.
```

## Installation

You need the latest stable version of
[Rust](https://www.rust-lang.org) (install with
[rustup](http://doc.crates.io/index.html)).

Ensure your `$PATH` includes `$HOME/.cargo/bin`.

Clone the repository and install with `cargo`:

```bash
git clone https://github.com/ben01189998819991197253/contrail ~/contrail
cd ~/contrail
cargo test && cargo install
# If updating, you may need to do `cargo install --force`
```

## Usage

`contrail -h` and `contrail -V` will print the help information and
the version number, respectively.

## Contributing

...is welcomed! Please submit any issues and pull requests, although
do run your code through `rustfmt` first, please.
