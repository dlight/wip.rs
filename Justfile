set unstable

wip := '../scripts/wip-commit.sh test v1'

[script('cargo', 'run', '--manifest-path', '../xtask/Cargo.toml', '--', 'wip')]
wip:
    q

wip-q:
    cargo run --manifest-path ../xtask/Cargo.toml -- wip
