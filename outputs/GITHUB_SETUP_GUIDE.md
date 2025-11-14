# GitHub Setup Guide

Three easy methods to push these Rust integration examples to GitHub.

## Prerequisites

- Git installed (`git --version`)
- GitHub account
- (Optional) GitHub CLI (`gh --version`)

---

## Method 1: Quick Setup with Script (Recommended)

We provide an automated bash script that handles everything:

```bash
cd outputs
chmod +x setup-github.sh
./setup-github.sh
```

The script will:
1. Check if you're in a git repository
2. Initialize git if needed
3. Add all files
4. Create initial commit
5. Guide you through GitHub setup
6. Push to GitHub

**That's it!** Your projects are now on GitHub.

---

## Method 2: Manual Setup (Step-by-Step)

### Step 1: Initialize Git Repository

```bash
cd outputs
git init
```

### Step 2: Add Files

```bash
git add .
```

### Step 3: Create Initial Commit

```bash
git commit -m "Initial commit: Rust integration examples for Python and TypeScript"
```

### Step 4: Create GitHub Repository

**Option A: Using GitHub CLI (easiest)**

```bash
gh repo create rust-integration-examples --public --source=. --remote=origin
```

**Option B: Using GitHub Website**

1. Go to https://github.com/new
2. Repository name: `rust-integration-examples`
3. Description: "Production-ready Rust integration examples for Python and TypeScript"
4. Public or Private (your choice)
5. **Do NOT initialize with README, .gitignore, or license** (we already have these)
6. Click "Create repository"

### Step 5: Add Remote and Push

```bash
# Replace YOUR_USERNAME with your GitHub username
git remote add origin https://github.com/YOUR_USERNAME/rust-integration-examples.git

# Push to GitHub
git branch -M main
git push -u origin main
```

---

## Method 3: Push to Existing Repository

If you already have a GitHub repository:

```bash
cd outputs

# Initialize git (if not already)
git init

# Add files
git add .

# Commit
git commit -m "Add Rust integration examples"

# Add your existing repo as remote
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO.git

# Push
git push -u origin main
```

---

## Verifying Your Push

After pushing, verify by visiting:
```
https://github.com/YOUR_USERNAME/rust-integration-examples
```

You should see:
- ✅ Both project directories
- ✅ Master README.md
- ✅ All documentation files
- ✅ Beautiful README with examples

---

## Customizing Your Repository

### Update Repository Description

**Via GitHub CLI:**
```bash
gh repo edit --description "15-50x performance improvements with Rust integration in Python & TypeScript"
```

**Via GitHub Website:**
1. Go to your repository
2. Click⚙️ Settings
3. Edit "Description"
4. Add topics: `rust`, `python`, `typescript`, `performance`, `pyo3`, `wasm`, `napi-rs`

### Add Topics (Tags)

```bash
gh repo edit --add-topic rust,python,typescript,performance,pyo3,wasm,webassembly,napi-rs
```

Or on GitHub website: Settings → Topics

### Enable GitHub Pages (for WASM demo)

1. Go to Settings → Pages
2. Source: Deploy from a branch
3. Branch: `main`, folder: `/typescript-rust-integration/packages/typescript-app/public`
4. Save

Your WASM demo will be live at:
```
https://YOUR_USERNAME.github.io/rust-integration-examples/
```

---

## Common Issues

### Issue: "Permission denied (publickey)"

**Solution:** Set up SSH keys or use HTTPS

```bash
# Use HTTPS instead
git remote set-url origin https://github.com/YOUR_USERNAME/rust-integration-examples.git
```

### Issue: "Repository not found"

**Solution:** Make sure the repository exists and you have access

1. Check repository exists on GitHub
2. Verify your username in the URL
3. Try using personal access token:

```bash
git remote set-url origin https://YOUR_TOKEN@github.com/YOUR_USERNAME/rust-integration-examples.git
```

### Issue: "refusing to merge unrelated histories"

**Solution:** Force push (careful!)

```bash
git push -u origin main --force
```

---

## Best Practices

### 1. Write Good Commit Messages

```bash
# Good
git commit -m "Add Python+Rust log processing example with 46x speedup"

# Bad
git commit -m "update"
```

### 2. Create .gitignore Early

We've provided `.gitignore` files in both projects. They exclude:
- Build artifacts (`target/`, `dist/`, `*.pyc`)
- Dependencies (`node_modules/`, `venv/`)
- IDE files (`.vscode/`, `.idea/`)

### 3. Use Branches for Development

```bash
# Create feature branch
git checkout -b feature/add-new-example

# Make changes, commit
git add .
git commit -m "Add new example"

# Push branch
git push -u origin feature/add-new-example

# Create pull request on GitHub
gh pr create
```

### 4. Tag Releases

```bash
# Create tag
git tag -a v1.0.0 -m "Release v1.0.0: Production-ready Rust integration examples"

# Push tags
git push --tags
```

---

## Repository Template

Want to use this as a template for your own projects?

### Make it a Template Repository

1. Go to Settings
2. Check "Template repository"
3. Save

Now others can click "Use this template" to create their own copy!

### Or, Clone for Your Own Use

```bash
git clone https://github.com/YOUR_USERNAME/rust-integration-examples.git my-rust-project
cd my-rust-project
rm -rf .git
git init
# Start fresh with your modifications
```

---

## Updating Your Repository

After making changes:

```bash
# Check status
git status

# Add changed files
git add .

# Commit with descriptive message
git commit -m "Improve documentation and add benchmarks"

# Push to GitHub
git push
```

---

## GitHub Actions (CI/CD)

Add continuous integration to automatically build and test:

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test-python:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build Python+Rust
        working-directory: ./rust-python-integration
        run: |
          pip install maturin
          cd rust_processor
          maturin develop
      - name: Run examples
        working-directory: ./rust-python-integration
        run: python examples/basic_usage.py

  test-typescript:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '20'
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build TypeScript+Rust
        working-directory: ./typescript-rust-integration/packages/node-binding
        run: |
          npm install
          npm run build
      - name: Run examples
        working-directory: ./typescript-rust-integration/packages/typescript-app
        run: |
          npm install
          npm run example
```

---

## Need Help?

### GitHub Documentation
- [Creating a repository](https://docs.github.com/en/get-started/quickstart/create-a-repo)
- [Pushing to GitHub](https://docs.github.com/en/get-started/using-git/pushing-commits-to-a-remote-repository)
- [GitHub CLI](https://cli.github.com/)

### Quick Reference
See [QUICK_GITHUB_REFERENCE.md](./QUICK_GITHUB_REFERENCE.md) for a one-page cheat sheet!

---

**Ready to share your projects with the world? Pick a method and get started!** 🚀
