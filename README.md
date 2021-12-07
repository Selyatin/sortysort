# Sortysort

## Try it in your browser from [here](https://selyatin.github.io/sortysort/docs)

## Testing locally

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel`

### Compiling for the web

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page. For this you need to set up some tools. There are a few simple scripts that help you with this:

``` sh
./setup_web.sh
./build_web.sh
./start_server.sh
open http://127.0.0.1:8080/
```

* `setup_web.sh` installs the tools required to build for web
* `build_web.sh` compiles your code to wasm and puts it in the `docs/` folder (see below)
* `start_server.sh` starts a local HTTP server so you can test before you publish
* Open http://127.0.0.1:8080/ in a web browser to view

