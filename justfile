default:
    @just --list

build:
    cargo build

test:
    cargo test
    bash -n bin/ct-session bin/ct-run bin/ct-render bin/ct-mark

doctor:
    cargo run -q -- doctor

# symlink the ct-* commands into ~/.local/bin (already on PATH on most systems)
install:
    mkdir -p ~/.local/bin
    for f in bin/ct-*; do \
      b=$(basename $f); \
      [ "$b" = "ct-lib.sh" ] || ln -sfn "$PWD/$f" ~/.local/bin/$b; \
    done
    @echo "installed. check with: ct-audio devices"

# install the review prompts into the current project's .claude/commands
install-prompts:
    mkdir -p .claude/commands
    cp prompts/*.md .claude/commands/

fmt:
    cargo fmt
