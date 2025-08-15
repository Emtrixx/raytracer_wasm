# Raytracer Wasm

Raytracer written in Rust for WebAssembly

# Requirements

```bash
 rustup target add wasm32-unknown-unknown
 ```

# Build

```bash
wasm-pack build --target web
```


# Run

Web files are in the `www` directory. To run the web server, you run the "start" command from package.json in the `www` directory.

```bash
 cd www
 npm start
```

```bash
 cd www
 wasm-pack build --target web
 wasm-pack serve
````


# Build web server

```bash
 cd www
 npm run build
```

# Copy js files

Copy js and wasm files in `www/dist` to homepage raytracer dir. Insert html content into ejs template.