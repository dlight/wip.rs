# Ahm

Okay so those are things I saved that I might or might not use

First, I'm using right now:

* <https://docs.rs/cargo_toml/latest/cargo_toml/struct.Manifest.html>
* <https://docs.rs/pathdiff/latest/pathdiff/fn.diff_paths.html>
* <https://crates.io/crates/git2/>
* <https://crates.io/crates/cargo-post>

I'm not using this frontend of git2

* <https://lib.rs/crates/git-meta>

I was going to use this to listen to all files, but I coded my own (and better,
because it listens to src/ rather than individual src files, which means it
listens when creating a new file)

* <https://lib.rs/crates/rerun_in_except>

I'm not using those crates that embed all sorts of stuff (including git stuff) to the binary

* <https://lib.rs/crates/vergen>
* <https://lib.rs/crates/shadow-rs>

I'm also not using those simpler crates that embed git stuff to the binary

* <https://lib.rs/crates/git-version>
* <https://lib.rs/crates/build-info>
* <https://lib.rs/crates/build-data>
* <https://lib.rs/crates/git-testament>
* <https://lib.rs/crates/git_info>
* <https://lib.rs/crates/crate-git-revision>

Nor this:

* <https://lib.rs/crates/garden-tools> - <https://garden-rs.gitlab.io/>

  Garden streamlines development workflows that involve a loosely-coupled set of
  multiple, independent Git trees.

  Garden allows you to define dynamic relationships and workflows between these
  repositories using a declarative YAML config file that can be shared and used
  as a bootstrapping mechanism for getting a auditable, from-source project
  cloned, built, installed and running with minimal effort for consumers of a
  Garden file.

  Garden sits above any individual project's build scripts and conventions.
  Garden is all about making it easy to remix and reuse libraries maintained in
  seperate Git repositories.
