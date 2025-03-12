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
