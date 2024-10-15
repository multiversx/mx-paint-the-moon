# Runs cargo check and clippy for specific targets
run_check_and_clippy() {
    BIN=$1
    TARGET=$2

    echo "Checking $BIN (target: $TARGET)..."
    if [ -n "$TARGET" ]; then
        cargo check --bin "$BIN" --target "$TARGET"
        if [ $? -ne 0 ]; then
            echo "$BIN check failed."
            exit 1
        fi

        echo "Running Clippy for $BIN (target: $TARGET)..."
        cargo clippy --bin "$BIN" --target "$TARGET" -- -D warnings
        if [ $? -ne 0 ]; then
            echo "Clippy failed for $BIN."
            exit 1
        fi
    else
        cargo check --bin "$BIN"
        if [ $? -ne 0 ]; then
            echo "$BIN check failed."
            exit 1
        fi

        echo "Running Clippy for $BIN..."
        cargo clippy --bin "$BIN" -- -D warnings
        if [ $? -ne 0 ]; then
            echo "Clippy failed for $BIN."
            exit 1
        fi
    fi
}

# Check moon-render-yew targeting wasm32-unknown-unknown
run_check_and_clippy "moon-render-yew" "wasm32-unknown-unknown"

# Check microservice (no specific target)
run_check_and_clippy "microservice" ""

# Check paint-harvest-sc (no specific target)
run_check_and_clippy "paint-harvest-sc-meta" ""

# Check paint-the-moon-sc (no specific target)
run_check_and_clippy "paint-the-moon-sc-meta" ""

echo "All checks passed successfully."