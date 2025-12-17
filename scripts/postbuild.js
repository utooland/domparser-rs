const fs = require('fs');
const path = require('path');

const rootDir = path.resolve(__dirname, '..');
const crateDir = path.join(rootDir, 'crates', 'domparser_napi');

// Move generated files from crate dir to root if they exist there
const generatedFiles = ['index.js', 'index.d.ts'];
generatedFiles.forEach(file => {
  const src = path.join(crateDir, file);
  const dest = path.join(rootDir, file);
  if (fs.existsSync(src)) {
    console.log(`Moving ${file} to root...`);
    fs.renameSync(src, dest);
  }
});

// 1. Patch index.d.ts
const patchFile = path.join(rootDir, 'patch.d.ts');
const indexDts = path.join(rootDir, 'index.d.ts');

if (fs.existsSync(patchFile) && fs.existsSync(indexDts)) {
  console.log('Patching index.d.ts...');
  const patchContent = fs.readFileSync(patchFile, 'utf8');
  // Check if already patched to avoid duplication
  const currentContent = fs.readFileSync(indexDts, 'utf8');
  if (!currentContent.includes('DOMParser')) {
      fs.appendFileSync(indexDts, patchContent);
  }
}

// 2. Fix index.js require paths (if needed, but napi seems to handle it now)
// We keep this just in case, but updated for domparser
const indexJs = path.join(rootDir, 'index.js');
if (fs.existsSync(indexJs)) {
  console.log('Checking index.js require paths...');
  let content = fs.readFileSync(indexJs, 'utf8');
  
  // Ensure we are using scoped packages if not already
  // Regex: require\('domparser-([a-z0-9-]+)'\)
  // Replacement: require('domparser-$1')
  
  let changed = false;
  content = content.replace(/require\('domparser-([a-z0-9-]+)'\)/g, (match, p1) => {
    changed = true;
    return `require('domparser-${p1}')`;
  });
  
  if (changed) {
      console.log('Fixed index.js require paths.');
      fs.writeFileSync(indexJs, content);
  }
}

// 3. Copy artifact to npm folder (for local testing of package structure)
// Determine platform and arch
const platform = process.platform;
const arch = process.arch;

let targetName = '';
if (platform === 'darwin') {
  targetName = `darwin-${arch}`;
} else if (platform === 'linux') {
  // This is a simplification, might need more logic for musl/gnu
  // But for local dev on standard linux, gnu is likely
  targetName = `linux-${arch}-gnu`; 
} else if (platform === 'win32') {
  targetName = `win32-${arch}-msvc`;
}

if (targetName) {
  const binaryName = `domparser.${targetName}.node`;
  // The binary might be in root or in crate dir?
  // napi build --platform puts it in root usually.
  let srcPath = path.join(rootDir, binaryName);
  if (!fs.existsSync(srcPath)) {
      const cratePath = path.join(crateDir, binaryName);
      if (fs.existsSync(cratePath)) {
          console.log(`Moving ${binaryName} to root...`);
          fs.renameSync(cratePath, srcPath);
          srcPath = path.join(rootDir, binaryName); // Update srcPath to root
      }
  }
  
  const destDir = path.join(rootDir, 'npm', targetName);
  const destPath = path.join(destDir, binaryName);

  if (fs.existsSync(srcPath)) {
    console.log(`Copying ${binaryName} to ${destDir}...`);
    if (!fs.existsSync(destDir)) {
      fs.mkdirSync(destDir, { recursive: true });
    }
    fs.copyFileSync(srcPath, destPath);
  }
}
