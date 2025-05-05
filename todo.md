# My todo

## Existing tools

### `wipd`

The daemon that watches file changes and creates wip commits (by calling `git
wip`). It is either run as a systemd daemon with `wipd --systemd` or in the
foreground with `wipd`.

* [x] `wipd --systemd`: run as a systemd instanced service in ~/.config/systemd

### `wip-list`

Lists all instances of `wipd` running, alongside the repositories they are
watching.

* [ ] `wip-list --current-repo`: list all wipd instances that are running in the
      current repo. (This would just copy over most code from `wip-prompt`)\
      \
      Besides having a bug, more than one instance will run in the same repo
      only if there was a repo (maybe a git submodule) that previously existed
      and was removed (maybe by removing `.git`). Because of that, the final
      `wipd` (fully written in Rust - not this shell script poc) must watch
      `.git` files and quit when they are deleted.

### `wip-prompt`

Put $(wip-prompt) in the prompt to show whether the repo whose the current dir
belongs to is being watched by `wipd`.

### `wip-read-toml`

A tool `wip-read-toml` written in Rust that parses `wip.toml` from a
project/target inside a repo, reads the part that says which directories
influences the build of the project, and prints all files in them. (Note: the
repo-wide `wip.toml` is going to get ignored for now)

* [ ] Allow globs rather than only exact paths
* [x] Make it able to exclude files too (like readme etc)
* [ ] Respect `.gitignore`
* [ ] Make the `Config` struct also store the root directory using either
      gix or shelling out to git.

### `wip-subset-tree`

A shell script `wip-subset-tree` that calls `wip-read-toml` and builds a git
tree containing only files that influences the build of a given project, and
prints its sha1.

### `wip-stop`

A shell script `wip-stop` to stop all wipd instances (if they are running
on systemd, run systemd stop, if not, just kill).

* [ ] Make it print which directory each stopped instance was watching. (It
      already kind of does when stopping systemd instances - through the
      debug messages - but I need to call `systemd-escape -u -- ..` on the
      escaped path)
* [ ] `wip-stop --current-repo` to stop the `wipd` instance in the current
      repo (note: that is kinda hard to do reliably if you have wipd running
      in direnv, because direnv will keep restarting it if you enter a new
      directory)

### `git-working-tree`

A shell script `git-working-tree` that prints the commit hash of the current
repository if it is clean, or prints the commit hash of the wip commit (calling
`git wip` first to ensure it exists)

## Planned tools

### `wip-amend-commit`

* [ ] A tool that amends the commit message of the last commit - which is either
    `HEAD` if the working dir is clean, or the wip commit if there are changes
    (creating the wip commit if it doesn't exist), to be called after every
    successful build of a given project/target - it inserts some build info
    (such as the version, the subset tree of all files influencing the build,
    and any build warnings).\
    \
    It must also creates two refs: something like
    `refs/build/myproject-v<semver>` pointing to the commit it was built, and
    `refs/influences-build/myproject-v<semver>` pointing to the subset tree of
    all files influencing the build. The generated semver is very particular,
    with specific pre-release and build metadata, it's something like this:
    `1.2.3-wip.2+2025-03-01` for wip commits, and `1.2.3-wip.2+2025-03-01` for
    commits in a branch (made when the working tree was clean). (Or maybe just
    `myproject-v1.2.3-wip.2`, I can't decide myself).\
    \
    Since the only two languages I am using here is Rust and shell script, I
    think this tool needs to be written in Rust: correctly adding metadata to
    the commit message may be tricky if there is already previous metadata (for
    example, if I build one project, then build another project without changing
    any file)


  * [ ] I also needed some way to differentiate between wip builds and true
        releases. One idea is a flag in `wip.toml` that says that every commit
        in a branch (non-wip) should bump the version, that would be perfect for
        my personal projects. Or maybe every build should be a wip build (even
        in a branch), until I decide to release by creating a tag (a `refs/tags`
        not a `refs/builds`).

  * [ ] Whenever amending a commit, we need to ensure the build refs that point
        to it are updated (there can be more than one; if I have multiple
        builds, all will be reflected into the commit message). There should be
        a tool to check for integrity, and then we run it in a postcommit hook,
        using [cargo-husky](https://github.com/rhysd/cargo-husky) or (most
        likely) having our own tool.

### `wip-verify-project` (or: `wip-verify-target`)

* [ ] Some tool that, for a given project/target, verifies if HEAD has the same
      partial tree as some wip build (but not the same partial tree as some
      build in a branch), and if yes, create a `refs/build/..` ref to it (but
      don't perform the build). This will be called opportunistically at every
      build. The goal is that if I make some wip build and just commit the code
      as is (maybe making some inconsequential edit like in a readme),

### Naming

* [ ] Decide whether to call *subset trees* something else (see
      `notes/subset-trees.md`)
* [ ] Decide whether to call *project* something else - maybe *target*? (see the
      footnote on `notes/subset-trees.md`). Right now, I'm writing
      *project/target* sometimes.
