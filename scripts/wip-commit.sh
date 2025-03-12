#!/bin/bash -e

if [[ $# -lt 2 || $1 = -h || $1 = --help ]]; then
    less "$0"
    exit
fi

service="$1"
shift

version="$1"
shift

flag="$1"

verbose() {
    if [[ $flag = -v || $flag = --verbose ]]; then
        echo "$@"
        echo
    fi
}

json_status() {
    local status="$1"
    local commit="$2"

    local parent="\"$(git rev-parse "$commit"^ 2>/dev/null)\""

    local tags="$(git tag --points-at "$commit" | sed 's/^/"/; s/$/"/' | sed -z 's/\n/, /g; s/, $//')"

    printf '{\n    status: "%s",\n    commit: "%s",\n    parent: "%s",\n    tags: [%s]\n}\n' \
        "$status" "$commit" "$parent" "$tags"
}

children() {
    # https://stackoverflow.com/a/39565190

    set -f -- $(git rev-list --all --not HEAD^@ --children | grep "^$(git rev-parse HEAD)")
    shift
    echo "$@"
}

if [[ -z $(git status --porcelain) ]]; then
    verbose working tree is clean, so use "$(git rev-parse HEAD)" "(tags: $(git tag --points-at HEAD))"

    json_status clean "$(git rev-parse HEAD)"
    exit
fi

head_file="$(git rev-parse --git-dir)"/HEAD
head_content="$(<"$head_file")"

trapfn() {
    rm -f "$GIT_INDEX_FILE"

}

# https://stackoverflow.com/a/58674064

export GIT_INDEX_FILE="$(mktemp)"

trap trapfn ERR EXIT INT HUP QUIT TERM

cp "$(git rev-parse --git-dir)/index" "$GIT_INDEX_FILE"

git add -A

new_tree="$(git write-tree)"

for i in $(children); do
    i_tree="$(git rev-parse "$i^{tree}")"
    if [[ $i_tree = $new_tree ]]; then
        # https://stackoverflow.com/a/15353441
        verbose tree exists in $i "(tags: $(git tag --points-at "$i"))"

        json_status existing-tree "$i"

        exit
    fi
done

short_tree="$(git rev-parse --short "$new_tree")"

tagname="$service-$version-$short_tree"

commit="$(git commit-tree "$new_tree" -p HEAD -m "wip: $tagname")"

git tag "$tagname" "$commit"

verbose tree did not exist, commit "$(git rev-parse "$commit")" created "(tags: $(git tag --points-at "$commit"))"

json_status wip-commit "$(git rev-parse "$commit")"
