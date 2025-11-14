#!/bin/bash

# Automated GitHub Setup Script
# This script helps you push your Rust integration examples to GitHub

set -e  # Exit on error

echo "========================================="
echo "  GitHub Setup for Rust Integration"
echo "========================================="
echo ""

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo "❌ Error: git is not installed"
    echo "Please install git first: https://git-scm.com/downloads"
    exit 1
fi

echo "✅ Git is installed ($(git --version))"
echo ""

# Check if already in a git repository
if [ -d ".git" ]; then
    echo "📁 Already in a git repository"
    echo ""
else
    echo "📁 Initializing git repository..."
    git init
    echo "✅ Git repository initialized"
    echo ""
fi

# Add all files
echo "📦 Adding all files..."
git add .
echo "✅ Files added"
echo ""

# Check if there are changes to commit
if git diff --cached --quiet; then
    echo "ℹ️  No changes to commit (everything already committed)"
else
    # Create commit
    echo "💾 Creating initial commit..."
    git commit -m "Initial commit: Rust integration examples for Python and TypeScript

- Python + Rust integration with PyO3 (20-50x speedup)
- TypeScript + Rust integration with napi-rs and WASM (15-40x speedup)
- Comprehensive documentation and examples
- Production-ready patterns and benchmarks"
    echo "✅ Commit created"
fi
echo ""

# Check for GitHub CLI
if command -v gh &> /dev/null; then
    echo "✅ GitHub CLI detected"
    echo ""
    echo "Would you like to create a new GitHub repository using GitHub CLI?"
    echo "This will create the repository and push automatically."
    echo ""
    read -p "Create GitHub repository? (y/n): " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo ""
        read -p "Repository name [rust-integration-examples]: " repo_name
        repo_name=${repo_name:-rust-integration-examples}

        echo ""
        read -p "Make repository public? (y/n) [y]: " -n 1 -r
        echo ""
        visibility=${REPLY:-y}

        if [[ $visibility =~ ^[Yy]$ ]]; then
            vis_flag="--public"
        else
            vis_flag="--private"
        fi

        echo ""
        echo "Creating GitHub repository..."
        gh repo create "$repo_name" $vis_flag --source=. --remote=origin

        echo ""
        echo "✅ Repository created and remote added!"
        echo ""
        echo "Pushing to GitHub..."
        git branch -M main
        git push -u origin main

        echo ""
        echo -e "${GREEN}=========================================${NC}"
        echo -e "${GREEN}  ✅ Success!${NC}"
        echo -e "${GREEN}=========================================${NC}"
        echo ""
        echo "Your repository is now on GitHub!"
        echo ""
        echo "View it at: https://github.com/$(gh api user -q .login)/$repo_name"
        echo ""
        exit 0
    fi
fi

# Manual setup instructions
echo "========================================="
echo "  Manual GitHub Setup"
echo "========================================="
echo ""
echo "To push to GitHub, follow these steps:"
echo ""
echo -e "${BLUE}1. Create a repository on GitHub:${NC}"
echo "   Go to: https://github.com/new"
echo "   Repository name: rust-integration-examples"
echo "   Don't initialize with README, .gitignore, or license"
echo ""
echo -e "${BLUE}2. Add the remote and push:${NC}"
echo ""
echo -e "${YELLOW}git remote add origin https://github.com/YOUR_USERNAME/rust-integration-examples.git${NC}"
echo -e "${YELLOW}git branch -M main${NC}"
echo -e "${YELLOW}git push -u origin main${NC}"
echo ""
echo "Replace YOUR_USERNAME with your GitHub username."
echo ""
echo "========================================="
echo ""
echo "For more detailed instructions, see:"
echo "  GITHUB_SETUP_GUIDE.md"
echo ""
echo "Quick reference:"
echo "  QUICK_GITHUB_REFERENCE.md"
echo ""
