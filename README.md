<div align="center">
  <h1>DioxusComponents</h1>
  <p><strong>Accessible, customizable components for Dioxus.</strong></p>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus-kit-core">
    <img src="https://img.shields.io/crates/v/dioxus-kit-core.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus-kit-core">
    <img src="https://img.shields.io/crates/d/dioxus-kit-core.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/dioxus-kit-core">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

---

<br/>

Dioxus Components is a shadcn style component library for Dioxus built on top of the unstyled [`dioxus-kit-core`](https://crates.io/crates/dioxus-kit-core) library and shipped as [`dioxus-kit`](https://crates.io/crates/dioxus-kit). The unstyled primitives serve as the foundation for building accessible and customizable UI components in Dioxus applications. The styled versions serve as a starting point to develop your own design system.

## Getting started

First, explore the [component gallery](https://dioxuslabs.github.io/dioxus-components/) to find the components you want to use.

Once you find a component, you can add it to your project with the Dioxus CLI. If you don't already have `dx` installed, you can do so with:

```
cargo install dioxus-cli
```

Then, you can add a component to your project with:

```
dx components add button
```

This will create a `components` folder in your project (if it doesn't already exist) and add the `Button` component files to it. If this is your first time adding a component, it will also prompt you to add a link to `/assets/dx-components.css` at the root of your app to provide the theme for your app.

## Contributing

### Project structure

This repository contains three main crates:
- `dioxus-kit-core`: The core unstyled component library.
- `dioxus-kit`: A shadcn-styled component library built on top of `dioxus-kit-core`.
- `preview`: A Dioxus application that showcases the components from `dioxus-kit` along with documentation and variant demos.

### Adding new components

If you want to add a new component, you should:
1. If there is any new interaction logic or accessibility features required, implement an unstyled component in the `dioxus-kit-core` crate. When adding components to the core library, ensure:
    - It adheres to the [WAI-ARIA Authoring Practices for accessibility](https://www.w3.org/WAI/standards-guidelines/aria/).
    - All styling can be modified via props. Every element should spread attributes and children from the props
2. In the `dioxus-kit` crate, create a styled version of the component using shadcn styles. This will serve as the styled version `dx components` will add to projects.
3. In the `preview` crate, add `docs.md` and one or more `variants/` demos under `src/components/<name>/` to showcase the component.
4. Add tests in `playwright` to ensure the component behaves as expected.

### Testing changes

The components use a combination of unit tests with cargo, css linting, and end-to-end tests with Playwright.

To run the unit tests for the `dioxus-kit-core` crate, use:

```sh
cargo test -p dioxus-kit-core
```

To run the CSS linting, use:

```sh
cd preview
npm install
npx stylelint "src/**/*.css"
```

To run the Playwright end-to-end tests, use:

```sh
cd playwright
npm install
npx playwright test
```

### Running the preview

To test your changes, you can run the preview application. For a desktop build, use:

```sh
dx serve -p preview --desktop
```

or for the web build:

```sh
dx serve -p preview --web
```

## License

This project is dual licensed under the [MIT](./LICENSE-MIT) and [Apache 2.0](./LICENSE-APACHE) licenses.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this repository, by you, shall be licensed as MIT or Apache 2.0, without any additional terms or conditions.
