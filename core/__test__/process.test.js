const { default: test } = require('ava');
const fs = require('fs');
const path = require('path');

const { processImage } = require('../index.js');

test.serial('should process image with grayscale and blur', t => {
    const inputPath = path.join(__dirname, '../cobra.png');

    if (!fs.existsSync(inputPath)) {
        t.fail(`Missing test image at: ${inputPath}`);
        return;
    }

    // Optional fields (Option<T> in Rust) must be passed as 'undefined' in JavaScript.
    // Passing 'null' will throw a NAPI type conversion error.
    const outputFilename = processImage(inputPath, {
        blur: 4.5,
        grayscale: true,
        invert: false,
        resize: undefined,
        rotate: undefined,
        outputFormat: 'png'
    });

    // Verify that the Rust function returned the generated filename string
    t.is(typeof outputFilename, 'string');

    // Verify that the output file was successfully written to the disk
    const outputPath = path.join(process.cwd(), outputFilename);
    t.true(fs.existsSync(outputPath), `Generated file ${outputFilename} does not exist`);
});