# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------

# Run tests
test:
    cargo test
    
    
# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt
    cargo fix --allow-dirty --allow-staged