Subset trees are a git tree object that includes just some files of a
repository. I currently actually store the tree object into the git repository
whenever I produce them, but this isn't really required: I just need the *hash*
of a subset tree, in order to refer unambigously to the set of files a given
target[^targetnaming] depends.


To see an example of which files are included, run

```console
cargo run --bin wip-read-toml test-repo/frontend/wip.toml | xargs realpath --relative-to .
```

(Note, the file `test-repo/frontend.depends_on` contains the expected output;
you can run a test case comparing them with `just test`)

And compare with the contents of the `test-repo` itself

```console
fd test-repo -tf
```

I initially had the idea to name them *partial trees* or maybe *mask trees*, but
settled on *subset trees*.

One naming issue here is that a subset tree is *not* just the subtree where the
target lives, because a target may depend on a library outside its directory
(which may be a common library used in many other parts of the git repo), and
because some files within the target doesn't influences the build (such as
readmes, documentation, etc). It's a *mask* in the sense that it's the original
repo, but with some files masked (removed). And it's *partial* because it's not
the full repo. But maybe subset trees evoke the right meaning.

[^targetnaming]: Note, I was previously been calling *project* a buildable
project inside a git monorepo, but I am currently calling it a *target* (just
like build systems like make, CMake, etc calls them). I could maybe call it a
*binary*, but binary is usually the produced artifact, not the target itself. Or
a *subproject* (this one is actually is viable, unless there is only one target
in the reppo) or something else. The established word for it actually is
*target*, but unfortunately this name is too overloaded in the Rust world: there
is the `target` directory with build outputs for the entire workspace, and there
is the concept of compilation targets (things like `x86_64-unknown-linux-musl`).
So, anyway, I think *project* is the wrong word for it (normally the whole repo
is a project), I don't know any better word, so *target* it is.
