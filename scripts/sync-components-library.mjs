#!/usr/bin/env node

import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(fileURLToPath(import.meta.url)));
const registryManifestPath = join(repoRoot, 'component.json');
const previewComponentsDir = join(repoRoot, 'preview/src/components');
const previewAssetsDir = join(repoRoot, 'preview/assets');
const crateRoot = join(repoRoot, 'dioxus-components');
const crateComponentsDir = join(crateRoot, 'src/components');

const registryManifest = JSON.parse(readFileSync(registryManifestPath, 'utf8'));
const componentEntries = registryManifest.members
  .map((member) => member.replace(/^preview\/src\/components\//, ''))
  .sort((left, right) => left.localeCompare(right));

rmSync(crateComponentsDir, { recursive: true, force: true });
mkdirSync(crateComponentsDir, { recursive: true });
mkdirSync(join(crateRoot, 'assets'), { recursive: true });

for (const componentName of componentEntries) {
  const sourceDir = join(previewComponentsDir, componentName);
  const targetDir = join(crateComponentsDir, componentName);

  mkdirSync(targetDir, { recursive: true });
  for (const fileName of ['component.rs', 'mod.rs', 'style.css']) {
    cpSync(join(sourceDir, fileName), join(targetDir, fileName));
  }

  const componentSource = join(targetDir, 'component.rs');
  const moduleSource = join(targetDir, 'mod.rs');
  if (existsSync(componentSource) && readFileSync(componentSource, 'utf8').trim() === '') {
    writeFileSync(moduleSource, 'mod component;\n', 'utf8');
  } else if (existsSync(componentSource)) {
    writeFileSync(
      componentSource,
      convertCssModuleClasses(readFileSync(componentSource, 'utf8')),
      'utf8',
    );
    writeCssModuleCacheMarker(componentSource, join(targetDir, 'style.css'));
  }
}

cpSync(
  join(previewAssetsDir, 'dx-components-theme.css'),
  join(crateRoot, 'assets/dx-components-theme.css'),
);

const publicModules = [];
const publicReexports = [];
const globalStyleComponents = [];
for (const componentName of componentEntries) {
  publicModules.push(`pub mod ${componentName};`);

  const componentSource = join(crateComponentsDir, componentName, 'component.rs');
  if (existsSync(componentSource) && readFileSync(componentSource, 'utf8').trim() !== '') {
    publicReexports.push(`pub use ${componentName}::*;`);
  }

  if (!componentUsesCssModule(componentSource)) {
    globalStyleComponents.push(componentName);
  }
}

writeFileSync(
  join(crateComponentsDir, 'mod.rs'),
  `${publicModules.join('\n')}\n\n${publicReexports.join('\n')}\n`,
  'utf8',
);

writeFileSync(
  join(crateRoot, 'src/styles.rs'),
  `pub const COMPONENT_CSS: &str = concat!(\n${globalStyleComponents
    .map((componentName) => `    include_str!("components/${componentName}/style.css"),`)
    .join('\n')}\n);\n\npub const THEME_CSS: &str = include_str!("../assets/dx-components-theme.css");\n`,
  'utf8',
);

console.log(`Synced ${componentEntries.length} components into dioxus-components.`);

function componentUsesCssModule(componentSource) {
  return (
    existsSync(componentSource) &&
    /#\[css_module\("[^"]+"\)\]/.test(readFileSync(componentSource, 'utf8'))
  );
}

function writeCssModuleCacheMarker(componentSource, styleSource) {
  if (!componentUsesCssModule(componentSource)) {
    return;
  }

  const markerPattern = /\n?\/\* dioxus-components-css-module-source: .* \*\/\n?$/;
  const marker = `/* dioxus-components-css-module-source: ${realpathSync(styleSource)} */`;
  const css = readFileSync(styleSource, 'utf8').replace(markerPattern, '').trimEnd();
  writeFileSync(styleSource, `${css}\n\n${marker}\n`, 'utf8');
}

function convertCssModuleClasses(source) {
  let converted = source;

  if (converted.includes('.inner')) {
    converted = converted.replace(
      /fn to_class\(self\) -> &'static str/g,
      'fn to_class(self) -> String',
    );
  }

  return converted
    .replace(/Styles::([A-Za-z0-9_]+)\.inner/g, 'Styles::$1.to_string()')
    .replace(/\bStyles::([A-Za-z0-9_]+)\b(?!\s*(?:\.|::|\())/g, 'Styles::$1.to_string()');
}
