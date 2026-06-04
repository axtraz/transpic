#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binaryName = process.platform === 'win32' ? 'transpic-cli.exe' : 'transpic-cli';
const binaryPath = path.join(__dirname, '..', 'target', 'release', binaryName);

if (!fs.existsSync(binaryPath)) {
  console.error("Error: Rust binary not found.");
  console.error("Please run 'npm run build' or 'cargo build --release' first.");
  process.exit(1);
}

const args = process.argv.slice(2);

const child = spawn(binaryPath, args, { 
  stdio: 'inherit'
});

child.on('close', (code) => {
  process.exit(code ?? 0);
});

child.on('error', (err) => {
  console.error("Failed to start transpic:", err.message);
  process.exit(1);
});