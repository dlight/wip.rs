# wip.rs

This is a daemon that watches file changes and creates wip commits (by calling
`git wip`). It is either run as a systemd daemon in the background with `wipd
--systemd` or in the foreground with `wipd`.

I decided to prototype in shell script. You need
[`git-wip`](https://github.com/bartman/git-wip) installed.

## The shell script prototype

You are supposed to add the `./bin` directory from this repo to your `$PATH`,
and then add `$(wip-prompt)` to your shell prompt.

Run `wipd` to run wipd in the foreground.

Or, run `wipd --systemd` to run in background as a systemd user service (It will
add a service file to `~/.config/systemd/user` if none exists. To stop it, run
`systemctl --user stop wipd@<current_directory>`).

Or put `wip --systemd` in your `.envrc` to run wipd automatically in
[direnv](https://direnv.net/), whenever your cd into the directory.

Then edit files, and check out `git log refs/wip/main`.

To list all wipd instances run `wip-list`, and to stop them, run `wip-stop`.

## Current progress

There's a rough roadmap tracked in `todo.md`, and some ideas in the `notes`
directory.

But anyway I am already using the prototype: [Zed can't run commands on
save](https://github.com/zed-industries/zed/issues/18523) (in my case, `git
wip`), and on the issue they actually suggested a daemon just like this (I
indeed am currently just running `inotifywait` under the hood).

## About the .rs thing

There will eventually be a pure Rust reimplementation of the whole thing,
replacing the shell script prototype and the `git wip` dependency (which is also
written in shell script).

Or at least that's the plan.

## Testing

To test it do

```sh
just test
```
