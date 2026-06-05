const fs = require('fs');
const path = require('path');

const binDir = path.join(__dirname, '..', 'bin');
const targetReleaseDir = path.join(__dirname, '..', 'target', 'release');

const binaryName = process.platform === 'win32' ? 'transpic-cli.exe' : 'transpic-cli';

const source = path.join(targetReleaseDir, binaryName);
const destination = path.join(binDir, binaryName);

if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
}

try {
    fs.copyFileSync(source, destination);
    console.log(`✓ Binary successfully copied to: ${destination}`);
} catch (err) {
    console.error(`Error copying binary: ${err.message}`);
    process.exit(1);
}