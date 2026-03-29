#!/usr/bin/env node

/**
 * Tauri build script to pre-download and bundle Java runtimes
 * Downloads Java from Mojang servers and includes in the installer
 * 
 * This runs as part of the Tauri build process:
 * - Downloads Java components to src-tauri/resources/java-runtime/
 * - Java files are bundled into the MSI/EXE installer
 * - On first app launch, Java is copied to %APPDATA%/.flint/runtime/
 */

import fs from 'fs';
import path from 'path';
import https from 'https';
import zlib from 'zlib';
import { pipeline } from 'stream';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const JAVA_MANIFEST_URL = 'https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json';
const RESOURCES_DIR = path.join(__dirname, 'resources', 'java-runtime');

// Java components to bundle (these are used by Minecraft)
const COMPONENTS_TO_BUNDLE = [
    'jre-legacy',           // For older Minecraft versions
    'java-runtime-alpha',   // For newer versions (Java 16)
    'java-runtime-gamma',   // Alternative Java runtime
];

function ensureDir(dir) {
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

function downloadJson(url) {
    return new Promise((resolve, reject) => {
        https.get(url, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => {
                try {
                    resolve(JSON.parse(data));
                } catch (e) {
                    reject(e);
                }
            });
        }).on('error', reject);
    });
}

function downloadFile(url, filepath) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(filepath);
        
        https.get(url, (res) => {
            // Handle redirects
            if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
                file.destroy();
                return downloadFile(res.headers.location, filepath).then(resolve).catch(reject);
            }
            
            if (res.statusCode !== 200) {
                file.destroy();
                fs.unlink(filepath, () => {});
                reject(new Error(`HTTP ${res.statusCode}: ${url}`));
                return;
            }

            // Handle compression based on content-encoding header
            let stream = res;
            if (res.headers['content-encoding'] === 'gzip') {
                stream = res.pipe(zlib.createGunzip());
            }

            stream.pipe(file);
            
            file.on('finish', () => {
                file.close();
                resolve();
            });

            file.on('error', (err) => {
                fs.unlink(filepath, () => {});
                reject(err);
            });

            stream.on('error', (err) => {
                fs.unlink(filepath, () => {});
                reject(err);
            });
        }).on('error', (err) => {
            file.destroy();
            fs.unlink(filepath, () => {});
            reject(err);
        });
    });
}

async function downloadJavaComponent(component) {
    console.log(`\n📥 Downloading ${component}...`);
    
    try {
        // Fetch manifest
        const allRuntimes = await downloadJson(JAVA_MANIFEST_URL);
        const runtimeList = allRuntimes['windows-x64'][component];
        
        if (!runtimeList || runtimeList.length === 0) {
            console.warn(`⚠️  No ${component} found for windows-x64`);
            return;
        }

        // Get latest version
        const runtime = runtimeList[runtimeList.length - 1];
        const manifestUrl = runtime.manifest.url;
        const componentDir = path.join(RESOURCES_DIR, component);
        
        ensureDir(componentDir);
        
        // Fetch file manifest
        console.log(`   Fetching manifest from ${manifestUrl}`);
        const fileManifest = await downloadJson(manifestUrl);
        const files = fileManifest.files || {};
        
        let downloadedCount = 0;
        const fileEntries = Object.entries(files);
        const totalFiles = fileEntries.filter(([_, info]) => info.type === 'file').length;
        
        console.log(`   Found ${totalFiles} files to download`);
        
        // Download files
        for (const [filePath, fileInfo] of fileEntries) {
            if (fileInfo.type === 'directory') {
                const dirPath = path.join(componentDir, filePath.replace(/\//g, path.sep));
                ensureDir(dirPath);
            } else if (fileInfo.type === 'file') {
                const fileUrl = fileInfo.downloads?.raw?.url;
                if (!fileUrl) continue;

                const fullPath = path.join(componentDir, filePath.replace(/\//g, path.sep));
                const dir = path.dirname(fullPath);
                ensureDir(dir);

                // Skip if already exists
                if (fs.existsSync(fullPath)) {
                    downloadedCount++;
                    if (downloadedCount % 50 === 0) {
                        process.stdout.write(`\r   ${downloadedCount}/${totalFiles} files`);
                    }
                    continue;
                }

                try {
                    await downloadFile(fileUrl, fullPath);
                    downloadedCount++;
                    if (downloadedCount % 50 === 0) {
                        process.stdout.write(`\r   ${downloadedCount}/${totalFiles} files`);
                    }
                } catch (err) {
                    console.error(`\n   ❌ Failed to download ${filePath}: ${err.message}`);
                    // Continue with other files
                }
            }
        }
        
        console.log(`\r   ✅ ${component} downloaded (${downloadedCount}/${totalFiles} files)`);
        
    } catch (err) {
        console.error(`❌ Failed to download ${component}: ${err.message}`);
        throw err;
    }
}

async function main() {
    console.log(`\n${'='.repeat(60)}`);
    console.log('☕ Flint Launcher - Java Runtime Bundling');
    console.log(`${'='.repeat(60)}`);
    console.log(`\nBundling location: ${RESOURCES_DIR}`);
    console.log(`Components to bundle: ${COMPONENTS_TO_BUNDLE.join(', ')}`);
    console.log('\nThis may take 5-15 minutes depending on internet speed...\n');

    ensureDir(RESOURCES_DIR);

    // Check if bundling is disabled
    const skipBundleFile = path.join(__dirname, '.skip-java-bundle');
    if (fs.existsSync(skipBundleFile)) {
        console.log('⏭️  Java bundling disabled (create an empty file named .skip-java-bundle to disable)');
        return;
    }

    let successCount = 0;
    
    for (const component of COMPONENTS_TO_BUNDLE) {
        try {
            await downloadJavaComponent(component);
            successCount++;
        } catch (err) {
            console.warn(`⚠️  Skipping ${component} - will be downloaded on first app launch`);
        }
    }

    console.log(`\n${'='.repeat(60)}`);
    if (successCount > 0) {
        console.log(`✅ Java bundling complete! (${successCount}/${COMPONENTS_TO_BUNDLE.length} components)`);
        console.log('   These files will be included in the MSI installer');
        console.log('   On first app launch, Java will be extracted to:');
        console.log('   %APPDATA%/.flint/runtime/');
    } else {
        console.log('⚠️  No Java components were bundled');
        console.log('   Java will be downloaded on first app launch');
    }
    console.log(`${'='.repeat(60)}\n`);
}

main().catch(err => {
    console.error(`\n❌ Fatal error: ${err.message}`);
    // Don't fail the build - Java downloads on demand if bundling fails
    process.exit(0);
});

