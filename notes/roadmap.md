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

### `wip`

A library crate (not a binary) containing some common code used by all Rust
binaries. Currently only the code for reading a target `wip.toml`. It's only one
file, `src/lib.rs`.

Currently all tools are in the same crate, in different binaries), but I may
change this later.

* [x] Move common code from `wip-read-toml` to `lib`
* [x] Read version in `wip.toml` either directly (`version = 1.0.0`) or copied
      from `Cargo.toml` (`version = { from = "Cargo.toml" }`).
* [x] Make the `WipTomlBase` struct also store the root git directory using
      either gix or shelling out to git.
  * [ ] Ensure paths in `wip.toml` can't point outside the Git repository.

### `wip-influences-build`

A tool written in Rust that parses `wip.toml` from a target inside a repo, reads
the part that says which directories influences the build of the target, and
prints all files in them. (Note: the repo-wide `wip.toml` is going to get
ignored for now)

* [ ] Allow globs rather than only exact paths
* [x] Make it able to exclude files too (like readme etc)
* [ ] Respect `.gitignore`
* [x] Rename from `wip-read-toml` to `wip-influences-build`

### `wip-subset-tree`

A shell script that calls `wip-read-toml` and builds a git tree containing only
files that influences the build of a given target, and prints its sha1.

### `wip-stop`

A shell script to stop all wipd instances (if they are running
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

A shell script that prints the commit hash of the current repository if it is
clean, or prints the commit hash of the wip commit that corresponds to the
working tree (calling `git wip` first to ensure it exists)

## Planned tools

### `wip-amend-commit`

* [*] A tool that amends the commit message of the last commit - which is either
    `HEAD` if the working dir is clean, or the wip commit if there are changes
    (creating the wip commit if it doesn't exist), to be called after every
    successful build of a given target - it inserts some build info (such as the
    version, the subset tree of all files influencing the build, and any build
    warnings).\
    \
    It must also creates two refs: something like
    `refs/build/mytarget-v<semver>` pointing to the commit it was built, and
    `refs/influences-build/mytargett-v<semver>` pointing to the subset tree of
    all files influencing the build. The generated semver is very particular,
    with specific pre-release and build metadata, it's something like this:
    `1.2.3-wip.2+2025-03-01` for wip commits, and `1.2.3-wip.2+2025-03-01` for
    commits in a branch (made when the working tree was clean). (Or maybe just
    `mytarget-v1.2.3-wip.2`, I can't decide myself).\
    \
    Since the only two languages I am using here is Rust and shell script, I
    think this tool needs to be written in Rust: correctly adding metadata to
    the commit message may be tricky if there is already previous metadata (for
    example, if I build one target, then build another target without changing
    any file)

  * [x] Commit metadata in toml format
  * [ ] Check whether working tree is dirty, to decide whether amend `HEAD` or
        the wip commit
    * [ ] Check how many builds were recorded for the given target and version
          (they will be in `refs/build` and `ref/influences-build`), to create a
          prerelease commit. It's only when someone manually creates a tag
  * [ ] Add a flag to decide whether also add build metadata to `HEAD` commits,
        or if I should only add it to wip commits. The default should be to not
        add metadata to `HEAD` commits (to not pollute the git history)

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

  * [ ] Decide whether I want to store the hash of the tree object of the subset
        tree (like I am doing today), or if I want to make a commit, child of
        the amended commit, and use this commit hash instead. (This is relevant
        when writing `refs/influences-build/something` - is it a tree hash or is
        it a commit hash?)

  * [ ] Decide whether the commit metadata will be versioned (I am tending to
        no, but maintain backwards compatibility)

### `wip-verify-target`

* [ ] Some tool that, for a given target, verifies if HEAD has the same partial
      tree as some wip build (but not the same partial tree as some build in a
      branch), and if yes, create a `refs/build/..` ref to it (but don't perform
      the build). This will be called opportunistically at every build. The goal
      is that if I make some wip build and just commit the code as is (maybe
      making some inconsequential edit like in a readme),

### Naming and cross-cutting concerns

* [ ] Decide whether to call *subset trees* something else (see
      `notes/subset-trees.md`)
* [x] Decide whether to call *project* something else - maybe *target*? (see the
      footnote on `notes/subset-trees.md`). (Decided to use *target*)
    * [ ] Note: maybe it should be called `package`? (As in: `package-version`)
* [ ] Decide on the format `target-1.0.0.wip.1` or `target-v1.0.0.wip.1` (do I
      need a `v` there?) About this, [Must the version tags include a
      v?](https://github.com/semver/semver.org/issues/1)

    > When tagging releases in a version control system, the tag for a version
    > MUST be "vX.Y.Z" e.g. "v3.1.0".

    But then this was removed from semver:

    > So the reason why the SemVerTag was removed from the SemVer spec is, that
    > it is out of scope how tags are setup for the different version control
    > systems.\
    > And: Each developer team can use any kind of tag scheme for their version
    > control system, e.g. "v1.2.3" or "release-1.2.3" or "v1.2.3-foo" as long as
    > the embedded SemVer tuple conforms to the SemVer spec.\
    > Using the bare SemVer Tuple is just an easy to parse and also the most
    > simple option. But using a prefix like 'v' can help to "just disambiguates
    > (it) a little more than the straight version number by itself."\
    > For some years now I use the scheme 'v1.2.3' for my projects in Git. For
    > most other revision systems this should work, too. It allows filtering the
    > output of 'git tag' with grep to just geht a list of release tags, while
    > ignoring all other tags for other purposes. Filtering the bare SemVer number
    > requires more effort and more complex regex expressions.

    (Note, it was very hard to render those blockquotes without botching the
    markdown. [How to add a block quote inside a github-flavoured markdown list
    item?](https://stackoverflow.com/questions/27762125/how-to-add-a-block-quote-inside-a-github-flavoured-markdown-list-item))
