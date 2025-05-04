# wip.rs

I decided to prototype in shell script. You need
[`git-wip`](https://github.com/bartman/git-wip) installed.

## The shell script prototype

You are supposed to add `./bin` to your `$PATH`, then add `$(wip-prompt)` to your shell prompt.

Run `wipd` while in some directory. Or, run `wipd --systemd` to run in
background (to stop it, do `systemctl --user stop wipd@<current_directory>`).

(Also: the idea is to set up so it runs automatically in direnv)

Then edit files, and check out `git log refs/wip/main`.

# Current progress

Some things are written to `todo.rs`. But anyway I am already using the
prototype in Zed: [Zed can't run commands on
save](https://github.com/zed-industries/zed/issues/18523), and on the issue they
actually suggested a daemon just like this.

## About the Rust thing

There are some old things in here. They will eventually become a pure Rust
implementation of the shell script prototype. Or at least that's the plan.

## Testing

To test it do

```sh
just test
```
