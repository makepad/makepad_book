# Makepad Example Apps

As part of the project, Makepad includes several "example" applications built using the Makepad Framework. These apps are located in the `/examples` directory under the main Makepad project root.

## makepad-example-simple

Very simple example app showing usages of buttons, label text, and dynamic updating of text label via button press action.

To see it in action on macOS or Windows desktops:

```bash
cd ~/projects/makepad
cargo run --bin makepad-example-simple
```

## makepad-example-news-feed

A basic scrolling view app that demonstrates the usage of PortalList widget and infinite scrolling of text and images.

To see it in action on macOS or Windows desktops:

```bash
cd ~/projects/makepad
cargo run --bin makepad-example-news-feed
```

## makepad-example-ironfish

A digital synthesizer app with complex grid layouts, many buttons, charts, buttons, dropdowns, sliders, and even a piano keyboard, plus audio and graphs.

To see it in action on macOS or Windows desktops:

```bash
cd ~/projects/makepad
cargo run --bin makepad-example-ironfish
```

## makepad-example-slides

An app that doubles as a powerpoint or slides presentation. Use the arrow keys to move between the slides.

To see it in action on macOS or Windows desktops:

```bash
cd ~/projects/makepad
cargo run --bin makepad-example-slides
```

## makepad-example-simple-shader

A simple app that demonstrates the shader drawing capabilities of Makepad. Modify the numbers inside the `<MyWidget>` DSL code to see the drawing effects change  in real-time.

To see it in action on macOS or Windows desktops:

```bash
cd ~/projects/makepad
cargo run --bin makepad-example-simple-shader
```

## makepad-example-fractal-zoom

An app that demonstrates drawing of Mandelbrot fractal.
Press and hold down the mouse button in different places to zoom-in to the fractal.

To see it in action on macOS or Windows desktops:

```bash
cd ~/projects/makepad
cargo run --bin makepad-example-fractal-zoom
```

## Other Platforms

Most of the example apps list above can also be run on Android, iOS and the web browser (WASM). Please see the [Getting Started](quick_start.html) guide for detailed instructions.
