# The Linguistic Time Capsule

This is a work-in-progress implementation of a web app that lets users explore changes in word meanings over the last centuries.
The app combines a machine learning model called "Dynamic Word Embeddings" with cutting edge algorithms for model compression and bundles everything in an easily accessible web frontend using WebAssembly (WASM).
For more details, please see these papers:

- [Bamler and Mandt, *Dynamic Word Embeddings*, ICML 2017](http://proceedings.mlr.press/v70/bamler17a.html). <br>
  Describes the natural language machine learning model that is used to infer time-dependent word embedding vectors from unstructured text corpora.
- [Yang, Bamler, and Mandt, *Variable-Bitrate Neural Compression via Bayesian Arithmetic Coding*, arXiv:2002.08158](https://arxiv.org/abs/2002.08158) <br>
  Describes the compression method that we plan to use to make the fitted machine learning model small enough so that it can be downloaded on the client side.

## Code Structure

The codebase is divided in a frontend and a backend part.

- The **frontend** controls the user interface: input fields, the plotting pane, hover events, etc.
  This functionality is implemented in standard web technologies (HTML, CSS, SVG, JavaScript).
  It uses [Webpack](https://webpack.js.org/) for minification and transpiling.
- The **backend** is concerned with performance critical tasks like decoding the compressed word embeddings and certain linear algebra operations.
  Although we call it "backend", this code runs entirely on the client side.
  For performance reasons, this functionality is implemented in [Rust](https://www.rust-lang.org/) and compiled into a [WebAssembly module](https://webassembly.org/).

When a user interacts with the app, the response may involve either only the frontend or a combination of the frontend and the backend.
For example:

- When the user hovers over the plot legend then the app highlights the corresponding trajectory in the plot pane.
  This is implemented entirely on the frontend since it does not require any new computation (the trajectory is already plotted, we only have to change its opacity and its thickness).
- When the user enters a new word into the text field, then the frontend registers a key event.
  The frontend looks up the ID of the entered word in the vocabulary and then calls into the backend to get some interesting trajectories for the entered word.
  After the backend returns these trajectories to the frontend, the frontend creates SVG elements to plot the trajectories.

## Setting up a Development Environment

Before coding on this app, I strongly recommend going through the [Rust and Webassembly Tutorial](https://rustwasm.github.io/docs/book/introduction.html) up to the point where one has a simple "Hello, World" app running (currently Section 4.2 in the tutorial).

### Installing Required Software

To code on the Linguistic Time Capsule, set up a development environment as follows:

1. [Install the Rust toolchain](https://rustup.rs/).
   The Linguistic Time Capsule uses Rust (compiled to WebAssembly) for performance critical tasks like decoding compressed word embeddings and certain linear algebra operations.
   - If Rust is already installed on your system, make sure you have the latest version by running `rustup update`.
2. [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
   Wasm-pack is a tool that generates glue code between JavaScript and Rust generated WebAssembly.
3. [Install npm](https://www.npmjs.com/get-npm).
   - If you already have npm, you may want to make sure it's up to date by running `npm install npm@latest -g`.
4. (Optional) set up your editor.
   For VS code, I recommend installing the following extensions: [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint), [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust), and [Rust Test Lense](https://marketplace.visualstudio.com/items?itemName=hdevalke.rust-test-lens).
   - If you want to contribute code to the repository, please make sure to use a code formatter.
     In particular, all rust code must be formatted with `rustfmt`.
     Most editors can be configured to run `rustfmt` automatically when saving files.

### Setting up the Project Directory

The following steps need to be done only once in order to set up the project.

1. Clone the Github repository:
   ```bash
   git clone git@github.com:robamler/linguistic-time-capsule.git
   ```
2. Add assets.
   The compressed fitted Dynamic Word Embeddings model is currently not distributed as a part of the git repository due to its size.
   Therefore, you have to manually place the model in the right directory.
   - There should be a directory `linguistic-time-capsule/wasm/www`.
     Create a subdirectory `assets` in this directory.
   - Get the two files `googlebooks_metadata_1800to2008_vocabsize30000` and `step34000_T209_V30000_K100.dwe` from Rob and place them in the `assets` directory you just created.
3. Build the backend:
   ```bash
   cd linguistic-time-capsule/wasm
   wasm-pack build
   ```
   This may take up to a few minutes when run for the first time because it has to compile all the dependencies.
   Subsequent compilations will be much faster because compiled dependencies will be cached.
   - If you use VS Code and open it in the directory `linguistic-time-capsule` then this command should be available as the task "wasm-pack build" (use `Ctrl+Shift+P` → "Run Task" → "wasm-pack build").
4. (Locally) install dependencies of the backend (such as Webpack).
   From within the directory `linguistic-time-capsule/wasm`, run
   ```bash
   cd www
   npm install
   ```
5. Compile the frontend and run a server that serves the app and observes changes to the source code:
   from within the directory `linguistic-time-capsule/wasm/www`, run
   ```bash
   npm run serve
   ```
   - If you use VS Code and open it in the directory `linguistic-time-capsule` then this command should be available as the task "npm: serve" (use `Ctrl+Shift+P` → "Run Task" → "npm: serve").
6. Open a web browser at the URL printed by the above program.
   By default, it's http://localhost:8080/.

## Workflow

Most of the steps under "Setting up a Development Environment" above are only necessary once.
When you come back later to work on the app, the only required steps are:

1. Start the Webpack server:
   ```bash
   cd linguistic-time-capsule/wasm/www
   npm run serve
   ```
   (Or simply run the task "npm: serve" from within VS Code: `Ctrl+Shift+P` → "Run Task" → "npm: serve".)
2. Navigate to http://localhost:8080/ in a web browser.

Once the server runs, use the following development workflow:

- For **frontend development**:
simply save the HTML, CSS, or JavaScript file.
  The Webpack server should automatically detect the change and initiate a reload in the browser (if the browser doesn't reload then most likely you accidentally killed the Webpack server at some point; just restart it.).

- For **backend development**:
if you make any changes to Rust source files (`*.rs`) then you'll have to recompile the WebAssembly module:
  ```bash
  cd linguistic-time-capsule/wasm
  wasm-pack build
  ```
  (Or simply run the task "wasm-pack build" from within VS Code: `Ctrl+Shift+P` → "Run Task" → "wasm-pack build".)
  - Compilations after incremental changes to the Rust source code should be much faster than the initial compilation.
  - If the source code was changed then the compilation will generate a new WebAssembly module, and the Webpack server should automatically detect the changed module and initiate a reload in the browser.
