# Q

I'm here:

```rust
impl Program {
    fn build_subtree(&self) {
        let mut builder = TreeUpdateBuilder::new();

        for relative_path in &self.my_metadata.subtree.include {
            let absolute_path = self.top_level_dir.join(relative_path);

            let oid = Oid::hash_file(ObjectType::Blob, absolute_path).unwrap();

            builder.upsert(relative_path, oid, FileMode::Blob);
        }
    }
}
```

That's the relevant docs:

* <https://docs.rs/git2/latest/git2/struct.Tree.html>
* <https://docs.rs/git2/latest/git2/struct.Oid.html>
* <https://docs.rs/git2/latest/git2/build/struct.TreeUpdateBuilder.html>
* <https://docs.rs/git2/latest/git2/struct.TreeBuilder.html>

Since I'm using `TreeUpdateBuilder`, this is what I should use to write down the tree

<https://docs.rs/git2/latest/git2/build/struct.TreeUpdateBuilder.html#method.create_updated>

(which calls this:
<https://libgit2.org/libgit2/#HEAD/group/tree/git_tree_create_updated> "This
function is optimized for common file/directory addition, removal and
replacement in trees. It is much more efficient than reading the tree into a
git_index and modifying that, but in exchange it is not as flexible.")

But what if I just wanted to know its `Oid`, without writing to disk?

It seems I should use something else, then.

But would it be `TreeBuilder`? Then I would need to use

<https://docs.rs/git2/latest/git2/struct.TreeBuilder.html#method.write>

It uses <https://libgit2.org/libgit2/#HEAD/group/treebuilder/git_treebuilder_insert>

Some links

* <https://stackoverflow.com/questions/69512715/how-to-build-represents-worktree-of-git-bare-repo-on-memory-with-libgit2>
* <https://github.com/rust-lang/git2-rs/issues/693>: Creating an in-memory Git repository

I tried to use `TreeUpdateBuilder` but it's meant to build tree updates and
hmm.. I *could* build as an update, but I'm not sure if it makes sense.

I uses <https://libgit2.org/libgit2/#HEAD/group/tree/git_tree_create_updated>

.. maybe if the baseline tree is an empty tree? Or if I just remove the files not present

---

What about using gitoxide?

<https://github.com/Byron/gitoxide>

It appears that git2 is much more used than gix <https://lib.rs/stats#vs> (the underlying library of gitoxide)

Gix also doesn't have support for many things, like creating commits: <https://github.com/Byron/gitoxide#feature-discovery>

<https://docs.rs/gix/0.55.2/gix/#libgit2-api-to-gix>

> This doc-aliases are used to help finding methods under a possibly changed name. Just search in the docs. Entering git2 into the search field will also surface all methods with such annotations.
>
> What follows is a list of methods you might be missing, along with workarounds if available.
>
> * git2::Repository::open_bare() ➡ ❌ - use open() and discard if it is not bare.
> * git2::build::CheckoutBuilder::disable_filters() ➡ ❌ (filters are always applied during checkouts)
> * git2::Repository::submodule_status() ➡ Submodule::state() - status provides more information and conveniences though, and an actual worktree status isn’t performed.
