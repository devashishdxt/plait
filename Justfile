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
    perl -i.bak -pe 's/\[(`[^`]+`)\]\([A-Za-z_][A-Za-z0-9_:]*\)/$1/g; s/\[(`[^`]+`)\](?![\[(])/$1/g unless /^ {0,3}\[`[^`]+`\]:/' README.md
    rm README.md.bak

[working-directory('plait-macros')]
_readme-plait-macros:
    cargo readme > README.md
    perl -i.bak -pe 's/\[(`[^`]+`)\]\([A-Za-z_][A-Za-z0-9_:]*\)/$1/g; s/\[(`[^`]+`)\](?![\[(])/$1/g unless /^ {0,3}\[`[^`]+`\]:/' README.md
    rm README.md.bak

# Builds documentation
doc:
    cargo +nightly doc --open --no-deps --all-features
