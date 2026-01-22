# Builds the crate
build:
    cargo build

# Runs tests
test:
    cargo test

# Checks linting and formatting
check-lint:
    cargo clippy -- -D warnings
    cargo fmt -- --check

# Creates README.md
readme: _readme-plait _readme-plait-macros

[working-directory('plait')]
_readme-plait:
    cargo readme > README.md
    sed -i '' 's/\[\(`[^`]*`\)\]/\1/g' README.md

[working-directory('plait-macros')]
_readme-plait-macros:
    cargo readme > README.md
    sed -i '' 's/\[\(`[^`]*`\)\]/\1/g' README.md
