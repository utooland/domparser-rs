const fs = require('fs');
const path = require('path');

const packageJson = require('../package.json');
const version = packageJson.version;

const cargoTomlPath = path.join(__dirname, '../crates/domparser/Cargo.toml');
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');

// Replace version = "..." with version = "new_version"
// We match the version key specifically under [package] section ideally, 
// but usually it's the first version key in the file.
const newCargoToml = cargoToml.replace(/^version\s*=\s*".*?"/m, `version = "${version}"`);

if (cargoToml !== newCargoToml) {
  fs.writeFileSync(cargoTomlPath, newCargoToml);
  console.log(`Updated crates/domparser/Cargo.toml to version ${version}`);
} else {
  console.log('crates/domparser/Cargo.toml version is already up to date.');
}
