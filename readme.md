# Ray Tracing in One Weekend
An implementation of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html), translated to Rust.

![render](https://github.com/zaesur/rtiow/blob/main/render.png?raw=true)

## Requirements
* Rust
* ImageMagick (Optional: to display PPM files)

## Compilation
To compile an executable, run `cargo build -r`.

## Usage
To run immediately, run `cargo run -r > output.ppm`.

To display the output image, run `display output.ppm`, or convert it to PNG with `convert output.ppm output.png`.
