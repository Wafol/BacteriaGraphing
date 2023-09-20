## How to navigate this
In this readme are all the steps I made before making this BacteriaGraphing project. I included all the things that I have done before but not all of them are necessary to run this project.
Be careful, some installation in one step is neccessary to run the other step.

Content (my steps):
1. Plotters Rust demo
2. Brand new Rust-Wasm project
3. Simple as possible Plotters rust-wasm Project
4. Rust websocket communication (links)
5. Bacteria Graphing project


## Plotters Rust demo (6.9. 2023)
Main Plotters GitHub repo: https://github.com/plotters-rs/plotters

Useful GitHub repos:
https://github.com/plotters-rs/plotters
https://github.com/plotters-rs/plotters-wasm-demo

https://plotters-rs.github.io/book/basic/animate.html

I am working on Ubuntu 22.04.3 LTS

#### Running Plotters Rust demo according to https://github.com/plotters-rs/plotters-wasm-demo 
###### Setting up Rust-WebAssembly tool-chain
1. Install **Rust tool-chain** in terminal with: ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`` (It prompted which type of installation i want, select: *1) Proceed with installation*) according to: https://www.rust-lang.org/learn/get-start
	
	(Maybe you will need to restart the terminal to be able to use following commands)
	 Check if everything correctly installed with: ``rustup --version`` and ``cargo --version`` and ``rustc --version`` 

2. Install **wasm-pack** with command ``curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`` and check correct installation with ``wasm-pack --version`` 

3. Install **cargo-generate** with commd: ``cargo install cargo-generate`` and check: ``cargo-generate`` 

4. Install **npm** with commd: ``sudo apt install npm`` and check ``npm --version`` 

4. Clone this repo https://github.com/plotters-rs/plotters-wasm-demo and in this repo run ```./start-server.sh```  (I have used - ## Commits on Jun 27, 2022: 38523cdba80ab5c0e65db62edee275901c27ce90) 
	
	After running the command I got an error:
	```error: binary `wasm-pack` already exists in destination
	Add --force to overwrite```
	So I opened the file "start-server.sh" and changed line: ``cargo install wasm-pack`` to ``cargo install wasm-pack --force``.
	then run ``./start-server.sh`` again.
	*This problem occurred probably only bcs I have installed wasm-pack before. So if you haven't installed it you should be without this error.*

	Another error (build error): ``... item not found in FontTransform`` It is mentioned in GitHub issues: (https://github.com/plotters-rs/plotters-wasm-demo/issues/1) JuliDi writes: *Something wrong with the git version that is linked.* And suggests getting plotters from crates.io instead from GitHub. SOLUTION that worked for me: 
	- go to Cargo.toml file in project folder
	- replace dependencies lines ``plotters =....`` and ``[dependencies.plotters-canvas]\n git = "https://github.com/plotters-rs/plotters-canvas"`` with ``plotters = "0.3.2"`` and ``plotters-canvas = "0.3.0"``
	- Final dependencies look for me like this:
```toml
[dependencies]
plotters = "0.3.2"
#plotters = {git = "https://github.com/plotters-rs/plotters"}

wasm-bindgen = "0.2.62"
wee_alloc = "0.4.5"
web-sys = { version = "0.3.39", features = ["HtmlCanvasElement"] }

plotters-canvas = "0.3.0"
#[dependencies.plotters-canvas]
#git = "https://github.com/plotters-rs/plotters-canvas"
```

After resolving all errors, run ``./start-server.sh`` again. When compiled successfully, server should start. you can find it on: http://localhost:8080


## Brand new Rust-Wasm project (7.9.2023)
I am doing this after following the [[Plotters Rust demo (6.9. 2023)]], so this is not containing complete guide to setup rust project from start.

Good source: https://wasmbyexample.dev/examples/hello-world/hello-world.rust.en-us
(https://rustwasm.github.io/docs/book/game-of-life/introduction.html)

1. ``mkdir NewProject``
2. ``cd NewProject`` 
3. ``cargo init`` (this will generate simple *Cargo.toml* and *main.rs* file)
4. write this to *Cargo.toml*:
```toml 
[package]
name = "NewProject"
version = "0.1.0"
authors = ["Your Name <your@name.com>"]
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.87"
```

5. ``mv src/main.rs src/lib.rs``
6. write this to *lib.rs*:
```rust
// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}
```

7. ``wasm-pack build --target web`` 
8. ``mkdir www`` 

    It is generally prefered to genereate the www dir (html, css, js) with npm as shown in [[Simple as possible Plotters rust-wasm Project]]. Here I am doing it manually.

9. ``cd www``
10. ``touch index.js``
11. to *index.js* put this code:
```js
// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import init from "../pkg/NewProject.js";

const runWasm = async () => {
	// Instantiate our wasm module
	const newProject = await init("../pkg/NewProject_bg.wasm");
	
	// Call the Add function export from wasm, save the result
	const addResult = newProject.add(24, 24);
	
	// Set the result onto the body
	document.body.textContent = `Hello World! addResult: ${addResult}`;
};

runWasm();
```

12. ``touch index.html``
13. to *index.html* put this code:
```html
<!DOCTYPE html>

<html>
	<head>
		<meta charset="UTF-8" />
		<title>Hello World - Rust</title>
		
		<script type="module" src="./index.js"></script>
	</head>
	<body></body>
</html>
```

Now you should be able to run the website.


## Simple as possible Plotters rust-wasm Project
My GitHub repo with Simple as possible rust-wasm Project: https://github.com/Wafol/SimplePlottersProject (run with calling ``./start-server.sh``)

The tool-chain I use is setup according to [[Plotters Rust demo (6.9. 2023)]]

1. ``mkdir SimplePlottersProject``
2. ``cd SimplePlottersProject``
3. ``cargo init``
4. Add Plotters to *Cargo.toml*
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.87"

###add this to dependecies###
wee_alloc = "0.4.5"
web-sys = { version = "0.3.39", features = ["HtmlCanvasElement"] }

plotters = "0.3.5"
plotters-canvas = "0.3.0"
#############################

###add profile.release###
[profile.release]
lto = true
#########################
```

5. ``mv src/main.rs src/lib.rs``
9.  paste this to lib.rs:
```rust
use wasm_bindgen::prelude::*;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

#[wasm_bindgen]
pub fn draw() {
	let canvas_backend = CanvasBackend::new("canvas").expect("cannot find canvas");
	let root_drawing_area = canvas_backend.into_drawing_area();
	root_drawing_area.fill(&WHITE).unwrap();
	
	let mut chart = ChartBuilder::on(&root_drawing_area)
	.build_cartesian_2d(0..100, 0..100)
	.unwrap();
	
	chart.draw_series(
	LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
	).unwrap();
}
```

11. ``wasm-pack build``
12. ``npm init wasm-app www``
13. ``cd www``
14. make bootstrap.js look like this:
```js
init();

async function init() {
	if (typeof process == "object") {
		// We run in the npm/webpack environment.
		const [{}, {main}] = await Promise.all([
		import("../pkg/SimplePlottersProject.js"),
		import("./index.js"),
		]);
		
		main();
	} else {
		const [{default: init}, {main}] = await Promise.all([
		import("../pkg/SimplePlottersProject.js"),
		import("./index.js"),
		]);
		
		await init();
		main();
	}
}
```

15. make index.js look like this:
```js
import { draw } from "../pkg/SimplePlottersProject";

/** Main entry point */
export function main() {
	draw();
}
```

16. make index.html look like this:
```html
<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="utf-8">
		<title>SimplePlottersProject</title>
		
		<script src="./bootstrap.js"></script>
	</head>
	<body>
		<main>
			<canvas id="canvas" width="600" height="400"></canvas>
		</main>
	</body>
</html>
```

17. ``npm start`` This command should start webserver on localhost and on it shouls simple graph show

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

That's NOT NECESSARY, but i like to add this script:
```sh
rustup target add wasm32-unknown-unknown

if [ -z "$(cargo install --list | grep wasm-pack)" ]
then
	cargo install wasm-pack
fi

if [ "${CONFIG}" = "release" ]
then
	wasm-pack build
else
	wasm-pack build --release
fi

cd www
npm install
npm start
```


## Rust websocket communication (links)
(https://rustwasm.github.io/wasm-bindgen/examples/websockets.html)

https://docs.rs/wasm-sockets/latest/wasm_sockets/

closures move
https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/closures.html


## Bacteria Graphing project
My GitHub repo: https://github.com/Wafol/BacteriaGraphing

#### How to run:
1. Run the FastAPI python server: ``uvicorn bacteria_server:app --reload``
2. Run the Rust Wasm Plotters client: ``./start-server.sh`` (in different terminal)

You should be good to go. (open localhost)

#### Main code
Most important code is in:
1. src/lib.rs (rust client)
2. bacteria_server.py (python server)

#### Useful resources
how to work with different coordination systems
https://plotters-rs.github.io/rustdoc/plotters/coord/index.html

How to do log scale (on covid graph)
https://github.com/plotters-rs/plotters/blob/master/plotters/examples/tick_control.rs

Great plotters documentation(handbook)
https://plotters-rs.github.io/book/

another source with tutorials 
https://towardsdatascience.com/rustic-data-data-visualization-with-plotters-part-1-7a34b6f4a603#3f49

heatmap
https://users.rust-lang.org/t/plotters-creating-a-spectrogram-heatmap-with-log-scaling/57129/6