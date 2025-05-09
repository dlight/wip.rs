# `wip-amend-commit`

- [x] Get git root directory into `WipTomlBase`
- [x] Check whether the working tree is dirty by running `git status
      --porcelain` and checking if stdout is empty
- [x] Pretty-print commit messages, verifying they round-trip
- [ ] Check whether the tag `target-version` exists. If yes, and we are in a
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
