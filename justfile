# WorkTimer build recipes

# Display all available recipes
default:
    @just --list

# Build the project in release mode
build:
    cargo build --release

# Run tests
test:
    cargo test

# Run clippy linting
lint:
    cargo clippy -- -D warnings

# Check code formatting
fmt-check:
    cargo fmt -- --check

# Format code
fmt:
    cargo fmt

# Create a full release: bump version, commit, tag, push, and publish to cargo
# Usage: just release v0.3.2
release version:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Validate version format (v followed by semver)
    if ! [[ "{{version}}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
        echo "Invalid version format: {{version}}"
        echo "✓ Expected format: v0.1.0 or v0.1.0-rc1"
        exit 1
    fi
    
    # Check if tag already exists
    if git rev-parse "{{version}}" >/dev/null 2>&1; then
        echo "Tag {{version}} already exists"
        exit 1
    fi
    
    # Check for uncommitted changes
    if ! git diff-index --quiet HEAD --; then
        echo "You have uncommitted changes. Please commit or stash them first."
        exit 1
    fi
    
    # Extract version without 'v' prefix
    VERSION="{{version}}"
    VERSION="${VERSION#v}"
    echo "📦 Preparing release {{version}} (version: $VERSION)..."
    
    # Bump version in Cargo.toml
    echo "📝 Bumping version in Cargo.toml to $VERSION..."
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.bak
    
    # Verify the change
    if ! grep -q "version = \"$VERSION\"" Cargo.toml; then
        echo "Failed to update version in Cargo.toml"
        exit 1
    fi
    echo "✓ Version updated in Cargo.toml"
    
    # Run tests to ensure everything works
    echo "Running tests..."
    cargo test --quiet
    echo "✓ Tests passed"
    
    # Commit version bump
    echo "Committing version bump..."
    git add Cargo.toml Cargo.lock
    git commit -m "Bump version to $VERSION"
    echo "✓ Version bump committed"
    
    # Create and push tag
    echo " Creating git tag {{version}}..."
    git tag "{{version}}"
    echo "✓ Tag created"
    
    echo "Pushing to remote..."
    git push origin main
    git push origin "{{version}}"
    echo "✓ Changes and tag pushed"
    
    # Publish to crates.io
    echo "Publishing to crates.io..."
    cargo publish
    echo "✓ Published to crates.io"
    
    echo ""
    echo "Release {{version}} complete!"
    echo "GitHub Actions will now build and publish pre-built binaries"
    echo "Watch progress at: https://github.com/$(git config --get remote.origin.url | sed 's/.*:\(.*\)\.git/\1/')/actions"
    echo "Crate available at: https://crates.io/crates/work-tuimer"

# Dry-run release: check what would happen without making changes
release-check version:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Validate version format
    if ! [[ "{{version}}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
        echo "❌ Invalid version format: {{version}}"
        exit 1
    fi
    
    VERSION="{{version}}"
    VERSION="${VERSION#v}"
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    
    echo "🔍 Release check for {{version}}"
    echo ""
    echo "Current version: $CURRENT_VERSION"
    echo "New version:     $VERSION"
    echo ""
    
    # Check git status
    if git diff-index --quiet HEAD --; then
        echo "✓ No uncommitted changes"
    else
        echo "❌ Uncommitted changes detected"
    fi
    
    # Check if tag exists
    if git rev-parse "{{version}}" >/dev/null 2>&1; then
        echo "❌ Tag {{version}} already exists"
    else
        echo "✓ Tag {{version}} is available"
    fi
    
    # Check cargo login
    echo ""
    echo "Checking cargo registry authentication..."
    if cargo login --help >/dev/null 2>&1; then
        echo "✓ Cargo is available"
    else
        echo "❌ Cargo not found"
    fi
