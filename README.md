# rbranchsearch

A CLI that helps you manage your git branches in a repository. This is a rewrite of the same idea
of [branchsearch](https://github.com/npazosmendez/branchsearch) in Rust.

This allows you to switch very easily and fast between branches or delete the branches you have
locally to keep things tidy.

## Usage

```
USAGE:
    rbranchsearch [FLAGS] [BRANCH]

FLAGS:
    -a, --all        Display all branches (remote included)
    -h, --help       Prints help information
    -u, --update     Update branches, removing those that were deleted
    -V, --version    Prints version information

ARGS:
    <BRANCH>    Branch name you want to quickly switch to
```

## Install

- You have to [install Rust](https://www.rust-lang.org/tools/install) in your system
- Clone the repository: `git clone git@github.com:danifujii/rbranchsearch.git`
- In the repository's directory, run `cargo install --path .`
