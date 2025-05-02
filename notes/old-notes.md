<!-- markdownlint-disable-file MD031 -->

# Ahn

Some potential ideas of names:

## git wip

There's a similar tool that deals with wip branches rather than wip tags,
[Development has stalled](https://github.com/bartman/git-wip/issues/35) even
though there are outstanding issues.

* [git wip - help track git Work In Progress branches](https://github.com/bartman/git-wip)

* [using WIP branches to save every edit](http://www.jukie.net/~bart/blog/save-everything-with-git-wip)

I think I will call my tool `git wip-tag`, just to not conflict with that.

## git snapshot

Or. Maybe it should be called `git snapshot`?

[What is a git "Snapshot"?](https://stackoverflow.com/questions/4964099/what-is-a-git-snapshot>)

> It is the replacement term for "Revision". In other version control systems, changes to individual files are tracked and refered to as revisions, but with git you are tracking the entire workspace, so they use the term snapshot to denote the difference.
> From <http://gitref.org/index.html>
> > Instead of writing a tool that versions each file individually, like
> > Subversion, we would probably write one that makes it easier to store
> > snapshots of our project without having to copy the whole directory each
> > time. This is essentially what Git is. You tell Git you want to save a
> > snapshot of your project with the git commit command and it basically
> > records a manifest of what all of the files in your project look like at
> > that point. Then most of the commands work with those manifests to see how
> > they differ or pull content out of them, etc. If you think about Git as a
> > tool for storing and comparing and merging snapshots of your project, it may
> > be easier to understand what is going on and how to do things properly.

But actually.. there's is a
[git-snapshot](https://github.com/mob-sakai/git-snapshot) already

> git-snapshot is a command-line tool to take a snapshot of the directory and
> creates/updates another branch, like git subtree split --squash.

But it makes disjoint histories it seems..

well

## git save

I think I will call my thing git save

And the other thing, cargo save

.. but it exists as well.. [kinda](https://github.com/search?q=%22git-save%22&type=repositories)

* [git-save](https://github.com/knirch/git-save): this one is the most similar,
  it is an alternative to git stash / has to do with stashes (tracking which
  branch a stashes belongs to). well.. my tool kinda is too..

  Anyway this was inspired by the [save alias
  here](https://codingkilledthecat.wordpress.com/2012/04/27/git-stash-pop-considered-harmful/)

  But.. this is a single commit with a 30-line shell script and I think I'm okay
  with this name clash. The other two projects (git-snapshot and git-wip) are
  way more meaningful.

* [git-save](https://github.com/dbrock/git-save) Execute a command and commit
  the results in one step, not related

## Git workflow tools

Ok so while the above tools do literally the same as mine (or close to it), this other tool

* [git-gamble](https://crates.io/crates/git-gamble)

* [tcrdd](https://github.com/FaustXVI/tcrdd)

## Cargo stuff

* [Post-build script execution](https://github.com/rust-lang/cargo/issues/545), rejeitado (issue de 2014)

* [Add a new argument "--no-build" to "cargo run"](https://github.com/rust-lang/cargo/issues/3773), rejeitado (issue de 2017)

* [Is there an scriptable way to get the final binary which cargo run uses?](https://github.com/rust-lang/cargo/issues/7895)

```console
cargo build --message-format=json
```

Also

```console
cargo build -Z unstable-options --build-plan
```

And

```console
cargo metadata --format-version=1
```

To get all files from a crate there is [cargo
files](https://crates.io/crates/cargo-files), but this misses things like
`Cargo.lock`..

It's simpler to check what is in [include and
exclude](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields),
or better yet, have Cargo report this to us.

Maybe [cargo package
--list](https://doc.rust-lang.org/cargo/commands/cargo-package.html#package-options)?

<!-- markdownlint-disable MD010 -->
## Git CLI

* [What is the internal format of a Git tree object?](https://stackoverflow.com/questions/14790681/what-is-the-internal-format-of-a-git-tree-object)

* [How to get the tree hash of the index in git?](https://stackoverflow.com/questions/58668952/how-to-get-the-tree-hash-of-the-index-in-git#comment103673057_58674006)

```console
$ git ls-tree HEAD
040000 tree 8f7540105c67d16965959c47fad4305e57ef5269	.cargo
100644 blob 4d954c6c82671376b9342c9dfbfb2ff95979a329	.gitignore
100644 blob dbd35113f45812dea411002c508f0d6ed9e2beca	Cargo.lock
100644 blob f8a86cd7d351ad44326559614811906c9bd2100f	Cargo.toml
040000 tree 3fbf03e92f5894c138202a4fc937a43390e7c034	schema
040000 tree a72f19837a2fafd3dfb21462e78d748a687a1e35	types
040000 tree f2f80971e3f5e4b052c41b89ea29ecfd5746d089	xtask
```

```console
$ git ls-tree HEAD:types
100644 blob 4fffb2f89cbd8f2169ce9914bd16bd43785bb368	.gitignore
100644 blob dd4b6f92cb9e2624347307e0ee27a695e2054514	Cargo.toml
040000 tree 05c7154e1513ef40590601d301e5fdda9eb1db37	src
```

I can build the tree myself

```console
$ git ls-tree HEAD:types | git mktree
a72f19837a2fafd3dfb21462e78d748a687a1e35
```

* [How to find the commit(s) that point to a git tree object?](https://stackoverflow.com/questions/41088069/how-to-find-the-commits-that-point-to-a-git-tree-object)

This doesn't work for top-level trees

```console
$ git rev-list --all | git diff-tree --stdin --find-object=a72f19837a2fafd3dfb21462e78d748a687a1e35
7eb9d824bdf12a13daf2dcf95b5b575fd1e2bbc8
:040000 040000 6e2552c84d42e2065fede47eb4ecb7e62183f470 a72f19837a2fafd3dfb21462e78d748a687a1e35 M      types
```

For top level trees:

```console
$ git write-tree HEAD
9be47d9be4b00cde5ffed36034576a7e244dab36
```

```console
$ git ls-tree HEAD | git mktree
9be47d9be4b00cde5ffed36034576a7e244dab36
```

[Given the hash of a tree in git, how can I see what commits have that tree?](https://stackoverflow.com/questions/21092468/given-the-hash-of-a-tree-in-git-how-can-i-see-what-commits-have-that-tree)

```console
$ git log --pretty=format:"%H %T" --all
7eb9d824bdf12a13daf2dcf95b5b575fd1e2bbc8 9be47d9be4b00cde5ffed36034576a7e244dab36
b57de65f3c61d337671d5b3ce9c7b1e11ca9f526 000944201e7e943efe0e31a6d65cdf9d35d1fd61
a9f912d274960e7c2c365cd52c0cd0fa26cb8e0e d39c1bb61657ae0fe0cce93b714b19e51fa59fdc
$ git log --pretty=format:"%H %T" --all | awk -v sha=9be47d9be4b00cde5ffed36034576a7e244dab36 '$2 == sha { print $1 }'
7eb9d824bdf12a13daf2dcf95b5b575fd1e2bbc8
```

To find the top level trees themselves the command is

```console
$ git rev-list --objects --all --filter=combine:object:type=tree+tree:1 --filter-provided-objects
9be47d9be4b00cde5ffed36034576a7e244dab36
000944201e7e943efe0e31a6d65cdf9d35d1fd61
d39c1bb61657ae0fe0cce93b714b19e51fa59fdc
```

But it doesn't show the commit, so you can't filter by it.

But, note,

> git rev-list takes the same arguments as git log. In fact, they're basically
> the same command! They are built from one source file that just changes the
> default settings when run as git log vs git rev-list. Rev-list is intended for
> use in scripts, though, while log is intended for use by humans. In any case
> A..B "means" B ^A so origin/master..master and master ^origin/master are
> exactly the same thing here. In this case you can use git rev-list --branches
> ^origin/master (or maybe --branches --tags)

And hmmm. I think that you can use a command that finds the top level tree
without doing a grep.

Also

> To check top-level trees you would git cat-file -p $commithash as well and see
> if it has the hash in it.

And finally.. `git-mktree` receives a flat ls-tree, it doesn't work with recursive listings:

```console
$ git ls-tree -rt HEAD
040000 tree 8f7540105c67d16965959c47fad4305e57ef5269	.cargo
100644 blob 3a46360b62170ce12a1696a7e26df36262d92c39	.cargo/config.toml
100644 blob 4d954c6c82671376b9342c9dfbfb2ff95979a329	.gitignore
100644 blob dbd35113f45812dea411002c508f0d6ed9e2beca	Cargo.lock
100644 blob f8a86cd7d351ad44326559614811906c9bd2100f	Cargo.toml
040000 tree 3fbf03e92f5894c138202a4fc937a43390e7c034	schema
100644 blob 57c342bc4787079f0fc968c80a0446320f3c0898	schema/attempt1_dynamic_jsonschema.sql
100644 blob 57599ff62ffe0ad218c7af27d57981666c7f66d3	schema/clients.sql
100644 blob 2d9261eb6a1f51e2a1e94e659a221e7eac6a0ed1	schema/events.sql
100644 blob bf930401761ed11da69faa1553dc2165fbdca29c	schema/extensions.sql
040000 tree d6c5ed3f6e053ab34b17da64546215fe4651d8e8	schema/generated
100644 blob 88b424faad05c32cdbced5d1a3ff6dd93caf0136	schema/generated/task.sql
100644 blob 7e3fb5605764e8f1554fc355841322148926a0eb	schema/types.sql
040000 tree a72f19837a2fafd3dfb21462e78d748a687a1e35	types
100644 blob 4fffb2f89cbd8f2169ce9914bd16bd43785bb368	types/.gitignore
100644 blob dd4b6f92cb9e2624347307e0ee27a695e2054514	types/Cargo.toml
040000 tree 05c7154e1513ef40590601d301e5fdda9eb1db37	types/src
100644 blob deba74d5ffc1bc8d6bccc1e222af30b2cb63c141	types/src/lib.rs
040000 tree f2f80971e3f5e4b052c41b89ea29ecfd5746d089	xtask
100644 blob 40f1580e89f7191ee6d9ff5a34762deaf8119311	xtask/Cargo.toml
040000 tree eb7c06790ba860511055047ea117e0458713c842	xtask/src
100644 blob aa6987a3462eadc2a76564dd1d3cb3cd776acb4c	xtask/src/main.rs
```

```console
$ git ls-tree -r HEAD | git mktree
fatal: path .cargo/config.toml contains slash
$ git ls-tree -rt HEAD | git mktree
fatal: path .cargo/config.toml contains slash
```

<!-- markdownlint-enable MD010 -->

---

[How to get the tree hash of the working copy in git?](https://stackoverflow.com/questions/58668967/how-to-get-the-tree-hash-of-the-working-copy-in-git)

```bash
#! /bin/sh -e
export GIT_INDEX_FILE=$(mktemp)
trap "rm -f $GIT_INDEX_FILE" 0 1 2 3 15
cp $(git rev-parse --git-dir)/index $GIT_INDEX_FILE
git add -A && git write-tree
```

Ohhh, `GIT_INDEX_FILE` is a nice variable for temporarily working with another index

---

* [Git: can I directly look up the tree hash of a tree-ish, such as a path in a specific commit?](https://stackoverflow.com/questions/61545946/git-can-i-directly-look-up-the-tree-hash-of-a-tree-ish-such-as-a-path-in-a-spe)

    > The `git rev-parse` program can do this easily.  One of its primary jobs is to
    > turn any *name* into a Git *object ID* in accordance with the rules set out in
    > [the gitrevisions documentation](https://git-scm.com/docs/gitrevisions):
    > ```console
    > git rev-parse v2.12:animals/ant
    > ```
    > As the documentation says, if you want to verify that the resulting object is
    > specifically a *tree* object (and not one of the other kinds of objects), you
    > can add a suffix:
    > ```console
    > git rev-parse v2.12:animals/ant^{tree}
    > ```

---

[What does "git ls-files" do exactly and how do we remove a file from it?](https://stackoverflow.com/questions/56235287/what-does-git-ls-files-do-exactly-and-how-do-we-remove-a-file-from-it)

[Building git commit objects with git hash-object?](https://stackoverflow.com/questions/16064968/building-git-commit-objects-with-git-hash-object)

[How to read the mode field of git-ls-tree's output](https://stackoverflow.com/questions/737673/how-to-read-the-mode-field-of-git-ls-trees-output)

[What are the possible modes for entries in a Git tree object? \[duplicate\]](https://stackoverflow.com/questions/54596206/what-are-the-possible-modes-for-entries-in-a-git-tree-object)

[In cache.h](https://github.com/git/git/blob/950264636c68591989456e3ba0a5442f93152c1a/cache.h#L268-L277C2)

```c
static inline unsigned int canon_mode(unsigned int mode)
{
    if (S_ISREG(mode))
        return S_IFREG | ce_permissions(mode);
    if (S_ISLNK(mode))
        return S_IFLNK;
    if (S_ISDIR(mode))
        return S_IFDIR;
    return S_IFGITLINK;
}
```

Esses macros são do unix: [Linux Kernel  3.7.1](https://docs.huihoo.com/doxygen/linux/kernel/3.7/include_2uapi_2linux_2stat_8h.html)

```c
#define  S_IFREG   0100000
#define  S_IFLNK   0120000
#define  S_IFDIR   0040000
```

menos S_IFGITLINK, [definido no cache.h mesmo](#define S_IFGITLINK 0160000)

```c
#define S_IFGITLINK 0160000
```

## git2

Building git tree objects with git2, without touching anything in `.git`:

* [TreeBuidler](https://docs.rs/git2/latest/git2/struct.TreeBuilder.html)
  creates a tree in memory

* [Oid::hash_file](https://docs.rs/git2/latest/git2/struct.TreeBuilder.html)
  creates a hash of a file.

## Build-time crates

* [scratch](https://crates.io/crates/scratch): This crate exposes a compile-time
  temporary directory sharable by multiple crates in a build graph and erased by
  cargo clean.

* [build](https://docs.rs/build-rs/latest/build/),
  [build_script](https://docs.rs/build_script/latest/build_script/),
  [cargo-lib](https://crates.io/crates/cargo-lib)

* To embed things in the binary: [shadow-rs](https://crates.io/crates/shadow-rs)
  and other stuff
