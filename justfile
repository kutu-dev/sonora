# Default recipe of the justfile
default: help

# Run the program in debug mode
run file-path:
  cargo run -- {{file-path}}

# Fix the formatting and fixable linting warnings
check:
  cargo fix
  # Run the flake formatting if the `nix` CLI is available
  addlicense -l mpl .

# Show this info message
help:
  just --list

# Update and relock the inputs of the flake
update-flake:
  nix flake update --commit-lock-file

# See all the things that need to be done
todo:
  rg TODO:
