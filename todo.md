* [x] `wipd --systemd`: run as a systemd instanced service in ~/.config/systemd
    * [ ] have some way to stop the systemd service of current dir (note: that
          is kinda hard if you have wipd running in direnv)
    * [ ] have a way to stop all wipd instances (if they are running on systemd,
          run systemd stop, if not, just kill). Right now what can be done is
          `killall -9 wip-fd41eca378` (note, `kilalll -9 wipd` doesn't work)

## Some new tools

* [x] A tool `wip-read-toml` written in Rust that parses `wip.toml` from a
      project inside a repo, reads the part that says which directories
      influences the build of the project, and prints all files in them. (Note:
      the project-wide `wip.toml` is going to get ignored for now)
    * [ ] Make it able to exclude files too (like readme etc)
    * [ ] Allow globs rather than only exact paths
    * [ ] Respect `.gitignore`
    * [ ] Make `Config` also store the root directory using either gix or shelling out to
          git.

* [ ] A shell script `wip-partial-tree` that calls `wip-read-toml` and builds a
      git tree containing only files that influences the build of a given
      project, and prints its sha1.

* [ ] A tool that amends the commit message of the last commit - which is either
    `HEAD` if the working dir is clean, or the wip commit if there are changes
    (creating the wip commit if it doesn't exist), to be called after every
    successful build of a given project - it inserts some build info (such as
    the version, the partial tree of all files influencing the build, and any
    warnings). It also creates two refs: something like
    `refs/build/myproject-v<semver>` pointing to the commit it was built, and
    `refs/partial-tree/myproject-v<semver>` pointing to the partial tree of all
    files influencing the build. The generated semver is very particular, it's
    something like this: `myproject-v1.2.3-wip.2+2025-03-01` for wip commits,
    and `myproject-v1.2.3-wip.2+2025-03-01` for commits in a branch (made when
    the working tree was clean).

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

* [ ] Some tool that, for a given project, verifies if HEAD has the same partial
      tree as some wip build (but not the same partial tree as some build in a
      branch), and if yes, create a `refs/build/..` ref to it (but don't perform
      the build). This will be called opportunistically at every build. The goal
      is that if I make some wip build and just commit the code as is (maybe
      making some inconsequential edit like in a readme),
