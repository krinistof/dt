# Contributing to Democratic Tier

## Development Environment

This project uses **Nix Flakes** to manage the development environment and dependencies.
Ensure you have [Nix installed](https://nixos.org/download.html) and [Flakes enabled](https://nixos.wiki/wiki/Flakes).

To enter the development shell:
```bash
nix develop
```

## Git Hooks

We use git hooks to ensure code quality (linting, testing, formatting) before every commit.
This helps catch issues early and maintains a clean codebase.

### Setting up Hooks

To install the git hooks, run the following command in the root of the repository:

```bash
nix run .#install-hooks
```

This command will set up a `commit-msg` hook in your `.git/hooks` directory.

### Hook Behavior

The `commit-msg` hook analyzes your commit message and runs appropriate checks:

*   **Documentation Commits**: If your commit message starts with `docs:` or `docs`, the hook will run in **Lint Mode**. This runs faster checks (linting) and skips full build/test cycles.
*   **Standard Commits**: For all other commits, the hook runs in **All Mode**, executing the full suite of checks:
    *   **UQ**: Builds the dependency.
    *   **Frontend**: Lints, builds, and runs tests.
    *   **Backend**: Formats, lints (clippy), builds, and runs tests.

### Running Checks Manually

You can also run the checks manually at any time:

```bash
# Run all checks
nix run .#check

# Run only lint checks
nix run .#check -- lint
```
