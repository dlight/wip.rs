@@ -0,0 +1,115 @@ 
#!/bin/bash -e

json_status() {
    status="$1"
    commit="$2"

    set +e

    parent="\"$(git rev-parse "$commit"^ 2>/dev/null)\""

    if [[ $? -ne 0 ]]; then
        parent=null
    fi

    tags="$(git tag --points-at "$commit" | sed 's/^/"/; s/$/"/' | sed -z 's/\n/, /g; s/, $//')"

    printf '{\n  status: "%s",\n  commit: "%s",\n  parent: %s,\n  tags: [%s]\n}\n' \
        "$status" "$commit" "$parent" "$tags"
}

if [[ -z $(git status --porcelain) ]]; then
    echo it is clean, so use "$(git rev-parse HEAD)" "(tags: $(git tag --points-at HEAD))"
    echo
    json_status clean "$(git rev-parse HEAD)"
    exit 0
fi

children() {
    # https://stackoverflow.com/a/39565190

    set -f -- $(git rev-list --all --not HEAD^@ --children | grep "^$(git rev-parse HEAD)")
    shift
    echo "$@"
}

get_tree() {
    # https://initialcommit.com/blog/what-is-a-tree-in-git#git-rev-parse-head

    git rev-parse "$1^{tree}"
}

get_current_branch_or_commit() {
    # prints a commit if HEAD is detached

    git symbolic-ref -q --short HEAD || git rev-parse HEAD
}

head_file="$(git rev-parse --git-dir)"/HEAD
head_content="$(<"$head_file")"

write=false

trapfn() {
    rm -f "$GIT_INDEX_FILE"

    if [[ $write = "true" ]]; then
        echo "$head_content" >"$head_file"
    fi
}

# https://stackoverflow.com/a/58674064

export GIT_INDEX_FILE="$(mktemp)"

#trap "rm -f \"$GIT_INDEX_FILE\"" ERR EXIT INT HUP QUIT TERM  #0 1 2 3 15

trap trapfn ERR EXIT INT HUP QUIT TERM

cp "$(git rev-parse --git-dir)/index" "$GIT_INDEX_FILE"

git add -A

new_tree="$(git write-tree)"

for i in $(children); do
    i_tree="$(get_tree "$i")"
    if [[ $i_tree = $new_tree ]]; then
        # https://stackoverflow.com/a/15353441
        echo tree exists in $i "(tags: $(git tag --points-at "$i"))"
        echo

        json_status existing "$i"

        exit 1
    fi
done

short_tree="$(git rev-parse --short "$new_tree")"

# TODO: a trap to go back if there is an error
#current_branch="$(get_current_branch_or_commit)"
#git checkout --detach HEAD

current_commit="$(git rev-parse HEAD)"

write=true
echo "$current_commit" >"$head_file"

# Isn't there a way to make a detached commit, without moving the current branch nor moving HEAD?
git commit -q -m "wip: $1"

git tag "$1-$2-$short_tree"

echo tree did not exist, commit "$(git rev-parse HEAD)" created "(tags: $(git tag --points-at HEAD))"
echo

json_status created "$(git rev-parse HEAD)"

#git log

# git checkout would only work if I made it not change the working tree and also not change the index
# https://stackoverflow.com/questions/6070179/switching-branches-without-touching-the-working-tree
#git checkout "$current_branch"

exit 2