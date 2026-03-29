#!/bin/bash
# Build Flint Launcher with bundled Java
# This script bundles Java files into the installer (optional but recommended)

echo ""
echo "=========================================="
echo "Flint Launcher - Build with Java Bundle"
echo "=========================================="
echo ""
echo "This will download and bundle Java (1-2GB)"
echo "Build time: 15-30 minutes"
echo ""
read -p "Press Enter to continue..."

echo "Bundling Java files..."
node src-tauri/bundle-java.js

if [ $? -ne 0 ]; then
    echo "Java bundling failed, but continuing with build..."
else
    echo "Java bundled successfully!"
fi

echo ""
echo "Building Tauri application..."
npm run tauri build

echo ""
echo "=========================================="
echo "Build complete!"
echo "=========================================="
echo ""
