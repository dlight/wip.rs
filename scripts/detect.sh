#!/bin/bash

# Set up a pipe
pipe=$(mktemp -u)
mkfifo "$pipe"

# Function to clean up the pipe
cleanup() {
    echo "Parent died, cleaning up..."
    rm -f "$pipe"
    exit 0
}

# Set trap for SIGHUP and other signals
trap cleanup SIGHUP SIGTERM SIGINT

# The trick: Read from pipe in background, when parent dies,
# read will fail and trigger cleanup
(
    # Parent PID to monitor
    ppid=$$

    # Read from pipe in a subshell that will exit when parent dies
    # Note: When parent dies, the shell running this background process
    # will receive SIGHUP, causing read to terminate
    read < "$pipe"
) &

# Main script content here
echo "Script running with PID: $$"
echo "Monitoring for parent death ($PPID)..."

# Simulate work in the script
while true; do
    echo "Working..."
    sleep 1
done
