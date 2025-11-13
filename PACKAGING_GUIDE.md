# Package Manager Distribution Guide

This guide covers the next steps to publish work-tuimer v0.3.0 to Homebrew and AUR.

## ✅ Completed

- ✅ Created Homebrew formula (`packaging/homebrew/work-tuimer.rb`)
- ✅ Created AUR PKGBUILD (`packaging/aur/PKGBUILD`)
- ✅ Created AUR .SRCINFO (`packaging/aur/.SRCINFO`)
- ✅ Updated README with installation instructions
- ✅ Created packaging documentation
- ✅ Committed to `feature/package-managers` branch

## 📋 Next Steps

### 1. Test Locally (Before Publishing)

#### Homebrew Testing (macOS)

```bash
# Install from local formula
brew install --build-from-source packaging/homebrew/work-tuimer.rb

# Test the installation
work-tuimer --version
work-tuimer --help

# Clean up
brew uninstall work-tuimer
```

#### AUR Testing (Arch Linux)

You'll need an Arch Linux system or VM to test this:

```bash
cd packaging/aur

# Generate the correct b2sum checksum
makepkg -g

# This will output something like:
# b2sums=('abc123...')

# Manually edit PKGBUILD and replace 'SKIP' with the generated b2sum

# Build and install
makepkg -si

# Test
work-tuimer --version

# Clean up
sudo pacman -R work-tuimer
```

### 2. Merge to Main

Once you're satisfied with the packaging files:

```bash
git checkout main
git merge feature/package-managers
git push origin main
```

### 3. Submit to Homebrew

**Important**: Homebrew prefers first-time submissions to come from the project maintainer.

```bash
# Fork homebrew-core
# Visit: https://github.com/Homebrew/homebrew-core
# Click "Fork"

# Clone your fork
git clone https://github.com/YOUR_USERNAME/homebrew-core
cd homebrew-core

# Create branch
git checkout -b work-tuimer

# Copy formula to correct location
cp /path/to/work-tuimer/packaging/homebrew/work-tuimer.rb Formula/w/work-tuimer.rb

# Commit
git add Formula/w/work-tuimer.rb
git commit -m "work-tuimer 0.3.0 (new formula)

Simple, keyboard-driven TUI for time-tracking"

# Push
git push origin work-tuimer

# Open PR on GitHub
# Visit your fork and click "Contribute" -> "Open pull request"
```

**Homebrew PR Tips:**
- Title: `work-tuimer 0.3.0 (new formula)`
- Description: Keep it brief, mention what the package does
- BrewTestBot will automatically test your formula on multiple macOS versions
- Address any feedback from maintainers promptly
- Typical review time: 1-7 days

### 4. Publish to AUR

**Prerequisites:**
- AUR account (register at https://aur.archlinux.org/register)
- SSH key added to your AUR account

```bash
# Generate b2sum and update PKGBUILD
cd packaging/aur
makepkg -g
# Edit PKGBUILD to replace SKIP with the generated b2sum

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Clone AUR repository (first time only)
git clone ssh://aur@aur.archlinux.org/work-tuimer.git aur-work-tuimer
cd aur-work-tuimer

# Copy files
cp ../PKGBUILD ../.SRCINFO .

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial release: work-tuimer 0.3.0"
git push

# Your package is now live!
# View at: https://aur.archlinux.org/packages/work-tuimer
```

**AUR Notes:**
- You become the maintainer - you'll need to update it for new releases
- Users can install via AUR helpers like `yay -S work-tuimer`
- Consider adding a `.AURINFO` if you want additional metadata

## 📝 Future Releases

For future version updates:

### Homebrew Updates
```bash
# Use brew bump-formula-pr for updates
brew bump-formula-pr --url=https://github.com/Kamyil/work-tuimer/archive/refs/tags/vX.Y.Z.tar.gz work-tuimer
```

### AUR Updates
```bash
cd aur-work-tuimer

# Edit PKGBUILD: Update pkgver, pkgrel=1
# Generate new b2sum: makepkg -g
# Update PKGBUILD with new b2sum
makepkg --printsrcinfo > .SRCINFO

git add PKGBUILD .SRCINFO
git commit -m "Update to vX.Y.Z"
git push
```

## 🆘 Troubleshooting

### Homebrew Issues

**Problem**: Formula audit fails
```bash
# Run audit locally
brew audit --new-formula packaging/homebrew/work-tuimer.rb

# Common issues:
# - Missing license
# - Incorrect SHA256
# - Test block doesn't work
```

**Problem**: Build fails
```bash
# Check the build log
brew install --build-from-source --verbose packaging/homebrew/work-tuimer.rb
```

### AUR Issues

**Problem**: b2sum mismatch
```bash
# Delete any cached sources
rm -rf src/ pkg/

# Clean and regenerate
makepkg -C
makepkg -g
```

**Problem**: Build fails
```bash
# Install dependencies
makepkg -s

# Check for missing dependencies
namcap PKGBUILD
```

## 📚 Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Acceptable Formulae](https://docs.brew.sh/Acceptable-Formulae)
- [AUR Submission Guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [Rust Package Guidelines (Arch)](https://wiki.archlinux.org/title/Rust_package_guidelines)

## 🎉 Success Checklist

- [ ] Homebrew formula tested locally
- [ ] AUR PKGBUILD tested locally (with correct b2sum)
- [ ] Changes merged to main branch
- [ ] Homebrew PR submitted and merged
- [ ] AUR package published
- [ ] Tweet/announce the new installation methods!

Good luck! 🚀
