# Contributing
Since Anevicon is a free (in sense of freedom) kind of software, you are always welcome to contribute! Please look through our [code of conduct](https://github.com/Gymmasssorla/anevicon/blob/master/CODE_OF_CONDUCT.md) and the liberal [GPLv3 license](https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE), under which the product is distributed.

## Environment setup
To setup your development environment for contribution, you only need to [install the Rust toolchain](https://www.rust-lang.org/tools/install) using one convenient command below:

```bash
# Setup all the rust programming language toolchain
curl https://sh.rustup.rs -sSf | sh
```

## Building and testing
As it should be in correct projects, all the building and testing procedures are performed by [Travis CI](https://travis-ci.com/Gymmasssorla/anevicon). But, of course, you can do it by yourself using the following commands:

```bash
$ cargo build --verbose --all
$ cargo test --verbose --all
```

## Debugging
You can receive addditional debugging information and some traces by specifying the `--verbosity 5` option before running the compiled program. It's possible to print your own messages using the [`trace!()`](https://docs.rs/log/0.4.6/log/macro.trace.html) and the [`debug!()`](https://docs.rs/log/0.4.6/log/macro.debug.html) macros.

## Formatting
To make the code readable and maintainable, we use the great tool from the original Rust team called [rustfmt](https://github.com/rust-lang/rustfmt). You need to format your code before pushing any changes just by typing the `cargo fmt` command in your terminal.

## Where to go?
 - **[Issues](https://github.com/Gymmasssorla/anevicon/issues)** are meant for reporting found bugs and new functionality suggestions. Discussions are welcome too, and I will try to answer you in near future.
 
 - **[Pulls](https://github.com/Gymmasssorla/anevicon/pulls)** are meant for implementing new functionality and fixing bugs. Note that other people can criticize your code, and you should answer them.
