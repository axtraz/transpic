const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, '..', 'bin');
const targetReleaseDir = path.join(__dirname, '..', 'target', 'release');

const sourceName = process.platform === 'win32' ? 'transpic-cli.exe' : 'transpic-cli';
const destName = process.platform === 'win32'
    ? 'transpic-cli.exe'
    : `transpic-cli-${process.platform}`;

const source = path.join(targetReleaseDir, sourceName);
const destination = path.join(binDir, destName);

if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

if (!fs.existsSync(source)) {
    console.error(`Error: Source binary not found at ${source}`);
    console.error(`Please run 'cargo build --release' first on this platform.`);
    process.exit(1);
}

try {
    fs.copyFileSync(source, destination);
    console.log(`✓ Binary successfully copied to: ${destination}`);

    if (process.platform !== 'win32') {
        fs.chmodSync(destination, '755');
        console.log(`✓ Execution permissions (chmod 755) applied.`);
    }
} catch (err) {
    console.error(`Error during binary deployment: ${err.message}`);
    process.exit(1);
}