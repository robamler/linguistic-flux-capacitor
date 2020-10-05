# The Linguistic Flux Capacitor

![Test and Deploy status](https://github.com/robamler/linguistic-flux-capacitor/workflows/Test%20and%20Deploy/badge.svg)

The Linguistic Flux Capacitor is a free web application that lets you explore how the meaning of words in the English language has changed over the past two centuries.

**Try it out (no installation required):**
<https://robamler.github.io/linguistic-flux-capacitor>

The app is a technical demonstration of what can be done when you combine a [probabilistic natural language model](https://robamler.github.io/files/bamler-dynamic-word-embeddings-icml-2017.pdf) with a [novel compression algorithm](https://robamler.github.io/files/yang-vbq-icml-2020.pdf).
The app runs a machine learning model with 600 million parameters directly in your browser by leveraging WebAssembly (wasm) and a novel compression algorithm called [Variational Bayesian Quantization](https://robamler.github.io/files/yang-vbq-icml-2020.pdf).

## References / How to Cite

Technical details are described in the following papers:

- Bamler and Mandt, *Dynamic Word Embeddings*, ICML 2017. <br>
  *(see [PDF](https://robamler.github.io/files/bamler-dynamic-word-embeddings-icml-2017.pdf) or [video presentation](https://vimeo.com/240776794))* <br>
  Presents the machine learning model (natural language model) used by the app.
- Yang, Bamler, and Mandt, *Variational Bayesian Quantization*, ICML 2020. <br>
  *(see [PDF](https://robamler.github.io/files/yang-vbq-icml-2020.pdf) or [video presentation](https://icml.cc/virtual/2020/poster/6764))* <br>
  Presents the compression method that we use to transmit the entire trained model to the client side, thus enabling an immersive experience with near-instant model predictions.
- Bamler and Mandt, *Improving Optimization for Models With Continuous Symmetry Breaking*, ICML 2018. <br>
  *(see [PDF](https://robamler.github.io/files/bamler-goldstone-gd-icml-2018.pdf) or [related video presentation](https://robamler.github.io/videos/bamler-mandt-qft-of-representation-learning.mp4))* <br>
  Presents an advanced optimization algorithm inspired by Higgs Mechanism in physics, which is needed for optimal performance of our model.

## Lightweight Explanation

If the above scientific papers are too dense then you can find a more informal blog-post-style explanation of the employed techniques [on the app website](https://robamler.github.io/linguistic-flux-capacitor) (just scroll down from the app).

## Contributing

Pull requests with simple bug fixes or feature additions are very welcome.
If you're interested in more complex contributions I recommend [reaching out to me](https://robamler.github.io) first to prioritize projects and discuss possibilities for attribution and/or monetary compensation (for students).

### License

The source code of The Linguistic Flux Capacitor is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
See files [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
If you open a pull request, we will interpret this as consent that your contributions may be licensed under the same existing terms unless you explicitly express your objection in the description of the pull request.

### Code Structure

The codebase is divided into a frontend and a backend part.

- The **frontend** controls the user interface: input fields, the plotting pane, hover events, etc.
  This functionality is implemented in standard web technologies (HTML, CSS, SVG, JavaScript).
  It uses [Webpack](https://webpack.js.org/) for transpiling/minification/bundling.
- The **backend** is concerned with performance critical tasks such as decompressing and running the machine learning model.
  Don't let the name "backend" fool you, this code runs entirely on the client side.
  For performance reasons, this functionality is implemented in [Rust](https://www.rust-lang.org/) and compiled to a [WebAssembly module](https://webassembly.org/).


### Development Resources

Before coding on this app, I strongly recommend going through the [Rust and Webassembly Tutorial](https://rustwasm.github.io/docs/book/introduction.html) up to the point where one has a simple "Hello, World" app running (currently Section 4.2 in the tutorial).

### Installing Required Software

To code on the Linguistic Flux Capacitor, set up a development environment as follows:

1. [Install the Rust toolchain](https://rustup.rs/).
   The Linguistic Flux Capacitor uses Rust (compiled to WebAssembly) for performance critical tasks like decompressing and running the machine learning model.
   - If Rust is already installed on your system, make sure you have the latest version by running `rustup update`.
2. [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
   Wasm-pack is a tool that generates glue code between JavaScript and Rust generated WebAssembly.
3. [Install npm](https://www.npmjs.com/get-npm).
   - If you already have npm, you may want to make sure it's up to date by running `npm install npm@latest -g`.
4. (Optional) set up your editor.
   For VS code, I recommend installing the following extensions: [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint) and [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer).
   - If you want to contribute code to the repository, please make sure to use a code formatter.
     In particular, all rust code must be formatted with `rustfmt` (or `cargo fmt`) or else our CI system won't allow merging your pull request.
     Most editors can be configured to run `cargo fmt` automatically when saving files.

### Setting up the Project Directory

The following steps need to be done only once in order to set up the project.

1. Clone the Github repository:
   ```bash
   git clone git@github.com:robamler/linguistic-flux-capacitor.git
   ```
3. Build the backend:
   ```bash
   cd linguistic-flux-capacitor/wasm
   wasm-pack build
   ```
   This may take up to a few minutes when run for the first time because it has to compile all the dependencies.
   Subsequent compilations will be much faster because compiled dependencies will be cached.
   - If you use VS Code and open it in the directory `linguistic-flux-capacitor` then this command should be available as the task "wasm-pack build" (use `Ctrl+Shift+P` → "Run Task" → "wasm-pack build").
4. (Locally) install dependencies of the frontend (such as Webpack):
   ```bash
   cd linguistic-flux-capacitor/wasm/www
   npm install
   ```
5. Compile the frontend and run a server that serves the app and observes changes to the source code:
   ```bash
   cd linguistic-flux-capacitor/wasm/www
   npm run serve
   ```
   - If you use VS Code and open it in the directory `linguistic-flux-capacitor` then this command should be available as the task "npm: serve" (use `Ctrl+Shift+P` → "Run Task" → "npm: serve").
6. Open a web browser at the URL printed by the above program.
   By default, it'll be http://localhost:8080/.

### Workflow

Most of the steps under "Setting up a Development Environment" above are only necessary once.
When you come back later to work on the app, the only required steps are:

1. Start the Webpack server:
   ```bash
   cd linguistic-flux-capacitor/wasm/www
   npm run serve
   ```
   (Or simply run the task "npm: serve" from within VS Code: `Ctrl+Shift+P` → "Run Task" → "npm: serve".)
2. Navigate to http://localhost:8080/ in a web browser.

Once the server runs, use the following development workflow:

- For **frontend development**:
simply save the HTML, CSS, or JavaScript file.
  The Webpack server should automatically detect the change and initiate a reload in the browser (if the browser tab doesn't reload then most likely you accidentally killed the Webpack server at some point; just restart it.).

- For **backend development**:
if you make any changes to Rust source files (`*.rs`) then you'll have to recompile the WebAssembly module:
  ```bash
  cd linguistic-flux-capacitor/wasm
  wasm-pack build
  ```
  (Or simply run the task "wasm-pack build" from within VS Code: `Ctrl+Shift+P` → "Run Task" → "wasm-pack build".)
  - Compilations after incremental changes to the Rust source code should be much faster than the initial compilation.
  - If the source code was changed then the compilation will generate a new WebAssembly module, and the Webpack server should automatically detect the changed module and initiate a reload in the browser.
