default:
    @just --list

build:
    cargo build

test:
    cargo test
    bash -n bin/ct-session bin/ct-run bin/ct-render bin/ct-mark

doctor:
    cargo run -q -- doctor

# install the review prompts into the current project's .claude/commands
install-prompts:
    mkdir -p .claude/commands
    cp prompts/*.md .claude/commands/

fmt:
    cargo fmt
