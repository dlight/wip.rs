## wip.rs

I decided to prototype in shell script. You need
[`git-wip`](https://github.com/bartman/git-wip) installed.

## The shell script prototype

You are supposed to add `./bin` to your `$PATH`, then add `$(wip-prompt)` to your shell prompt.

Run `wipd` while in some directory (the idea is to set up so it runs automatically in direnv).
Then edit files, and check out `git log refs/wip/main`.


## About the Rust thing

It doesn't do much.

Ok so this is dumb but finally the first step works

To test it do

```sh
cd test
just wip
```

(Note: the `test` directory is on another git repository)

Then terminate either `just` or the shell (`sh`) that is spawned, killing the
displayed PIDs. The Rust program that is running on a busy loop (`xtask`) should
terminate as well.
