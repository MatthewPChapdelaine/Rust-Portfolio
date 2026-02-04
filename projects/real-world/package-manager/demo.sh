#!/bin/bash
# Demo script for package manager

cd "$(dirname "$0")"

echo "=== Package Manager Demo ==="
echo ""

# Build if not already built
if [ ! -f target/release/pkgmgr ]; then
    echo "Building package manager..."
    cargo build --release --quiet
fi

echo "1. Listing available packages:"
./target/release/pkgmgr registry list
echo ""

echo "2. Searching for 'web' packages:"
./target/release/pkgmgr registry search web
echo ""

echo "3. Getting info for 'tokio':"
./target/release/pkgmgr registry info tokio
echo ""

echo "4. Creating a new test project:"
mkdir -p demo-project
cd demo-project
../target/release/pkgmgr init demo-app

echo ""
echo "5. Adding dependencies to Package.toml:"
cat >> Package.toml << 'EOF'

[dependencies]
serde = "^1.0"
tokio = "^1.0"
EOF

cat Package.toml
echo ""

echo "6. Installing dependencies:"
../target/release/pkgmgr install
echo ""

echo "7. Viewing dependency tree:"
../target/release/pkgmgr tree
echo ""

echo "8. Checking installed packages:"
ls -la pkg_modules/
echo ""

echo "=== Demo Complete ==="
