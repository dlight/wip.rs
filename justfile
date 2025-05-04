test:
    #!/bin/bash
    output="$(cargo run --bin wip-read-toml test-repo/frontend/wip.toml | \
        xargs realpath --relative-to .)"
    sorted_output="$(echo "$output" | sort -)"

    expected="$(< test-repo/frontend.depends_on)"
    sorted_expected="$(echo "$expected" | sort -)"

    if [[ $sorted_output == $sorted_expected ]]; then
        echo "Test passed"
    else
        echo "Test failed"
        echo
        echo Output:
        echo "$sorted_output"
        echo
        echo Expected:
        echo "$sorted_expected"
    fi
