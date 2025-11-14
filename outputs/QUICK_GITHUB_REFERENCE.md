# Quick GitHub Reference

One-page cheat sheet for working with these Rust integration projects on GitHub.

## Initial Setup

```bash
# Navigate to project
cd outputs

# Initialize git
git init

# Add all files
git add .

# Initial commit
git commit -m "Initial commit: Rust integration examples"

# Create GitHub repo (using GitHub CLI)
gh repo create rust-integration-examples --public --source=. --remote=origin

# Or add remote manually
git remote add origin https://github.com/YOUR_USERNAME/rust-integration-examples.git

# Push
git branch -M main
git push -u origin main
```

## Daily Workflow

```bash
# Check status
git status

# Add changes
git add .

# Commit
git commit -m "Description of changes"

# Push
git push
```

## Branching

```bash
# Create and switch to new branch
git checkout -b feature/my-feature

# Push branch
git push -u origin feature/my-feature

# Create PR (GitHub CLI)
gh pr create

# Merge PR
gh pr merge

# Delete branch
git branch -d feature/my-feature
git push origin --delete feature/my-feature
```

## Useful Commands

```bash
# View commit history
git log --oneline

# Undo last commit (keep changes)
git reset --soft HEAD~1

# Discard local changes
git checkout -- .

# Update from remote
git pull

# View remotes
git remote -v

# Create tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push --tags
```

## GitHub CLI Quick Reference

```bash
# Create repo
gh repo create NAME --public --source=. --remote=origin

# View repo
gh repo view

# Create PR
gh pr create --title "Title" --body "Description"

# List PRs
gh pr list

# Merge PR
gh pr merge NUMBER

# Create issue
gh issue create --title "Title" --body "Description"

# List issues
gh issue list
```

## Common Issues

### "Permission denied (publickey)"
```bash
git remote set-url origin https://github.com/YOUR_USERNAME/rust-integration-examples.git
```

### "Repository not found"
Check repo exists and URL is correct:
```bash
git remote -v
```

### "refusing to merge unrelated histories"
```bash
git pull origin main --allow-unrelated-histories
```

### Reset to remote state
```bash
git fetch origin
git reset --hard origin/main
```

## Project-Specific Commands

### Python + Rust
```bash
cd rust-python-integration
make build              # Build Rust module
make example           # Run examples
make benchmark         # Run benchmarks
```

### TypeScript + Rust (Node.js)
```bash
cd typescript-rust-integration/packages/node-binding
npm install
npm run build
cd ../typescript-app
npm run example
npm run benchmark
```

### TypeScript + Rust (WASM)
```bash
cd typescript-rust-integration/packages/wasm-binding
wasm-pack build --target web
cd ../typescript-app
open public/index.html
```

## Repository Settings

### Enable GitHub Pages
Settings → Pages → Source: `main` → `/typescript-rust-integration/packages/typescript-app/public`

### Add Topics
```bash
gh repo edit --add-topic rust,python,typescript,performance,pyo3,wasm,napi-rs
```

### Set Description
```bash
gh repo edit --description "15-50x performance improvements with Rust integration"
```

---

**For detailed instructions, see:** [GITHUB_SETUP_GUIDE.md](./GITHUB_SETUP_GUIDE.md)
