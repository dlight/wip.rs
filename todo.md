# Integrate with rust-analyzer

- [ ] Add an option to listen to rust-analyzer's build (when configured to run
      `cargo build` rather than `cargo run`) rather than running on its own.
      This also means adding a delay: if the build doesn't happen, run it
      anyway. This should be the default, too.

      See the comment on /home/x/code/weber/todo.md for details.

# `wip-amend-commit`

- [x] Get git root directory into `WipTomlBase`
- [x] Check whether the working tree is dirty by running `git status
      --porcelain` and checking if stdout is empty
- [x] Pretty-print commit messages, verifying they round-trip
- [-] Check whether the tag `target-version` exists. If yes, and we are in a
      clean working tree, and the tag is in the current commit.. our version is
      just the version in `wip.toml`

# Miscellaneous and bug fixes

- [x] Moved far-reaching todo to `notes/roadmap.md`, `todo.md` now is for short
      term, focused things
- [x] Remove unused flags from `wip-list`
- [x] Add `--directory` flag to `git-working-tree-commit` (I *could* run `git -C
      dir working-tree-commit`, but then I would have to rely on `./bin` being
      on `PATH`)
- [x] Fix a `wip-subset-tree` bug where it wouldn't actually run git add in the
      right repo (note: when implemented in Rust, hopefully we won't need to get
      back and forth with cd-ing directories)
