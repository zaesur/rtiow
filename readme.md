# Ray Tracing in One Weekend

![render](https://github.com/zaesur/rtiow/blob/main/render.png?raw=true)
https://github.com/zaesur/rtiow/tree/V1?tab=readme-ov-file#compilation
## Index
1. [Description](https://github.com/zaesur/rtiow/tree/main?tab=readme-ov-file#description)
2. [Usage](https://github.com/zaesur/rtiow/tree/main?tab=readme-ov-file#usage)
3. [Sources](https://github.com/zaesur/rtiow/tree/main?tab=readme-ov-file#sources)

## Description
An implementation of Ray Tracing in One Weekend, translated to Rust.

I had wanted to implement a ray tracer for a long time, and figured it would be interesting to try this in a new language.
Because of its performance and portability, as well as my preference for functional programming patterns, I figured Rust would be a good choice.

Completing the first book took a little over a weekend, due to simultaneously exploring a new domain as well as learning an a new programming language.
I have tried to follow the naming conventions of the original tutorial, making use of various Rust features where it made sense (notably, the 'Option' type).
While I don't expect the code to be fully optimized, I hope it is declarative and easy to follow.

I have also decided to use the [GLM](https://docs.rs/nalgebra-glm/latest/nalgebra_glm/) library,
as there is no point in reinventing the wheel and implementing my own linear algebra library which is as performant and numerically stable as GLM.
As a bonus point, this library shares the same interface as the C++ and OpenGL variants, so familiarizing myself with the API will hopefully pay off
as I continue my studies.

Some time after finishing the first book, I was dissatistied with the camera implementation suggested by the book,
which features a long list of variables with unruly names and lots of procedural code.
While looking around for alternatives, I stumbled upon various theoretical discussions of raytracing cameras,
and discovered that, in essence, they are a matrix which transforms points in 2D screen space into 3D space,
such that rays can be cast into the scene.

I have therefore refactored the original camera implementation into a new 'matrix' implementation,
which follows the mathematical principles of raytracing more faithfully.
In doing so, I have refreshed my knowledge of linear algebra, and deepened my understanding of the inner workings of raytracing,
rather than blindly 'translating' the original code to Rust.

While the new implementation has taken a bit of a performance penalty, due to the conversion from vector multiplications to
four dimensional matrix multiplication, I am happy that the resulting code is more expressive and uses conventional naming terms
(such as 'camera-to-world matrix', 'look at', and so on).
This will make it easier to apply these lessons to future projects.

Those interested in the original implementation may still find it at the [V1][https://docs.rs/nalgebra-glm/latest/nalgebra_glm/] branch.

## Usage

### Requirements
* Rust
* ImageMagick (Optional: to display PPM files)

### Compilation
To compile an executable, run `cargo build -r`.

### Usage
To run immediately, run `cargo run -r > output.ppm`.

To display the output image, run `display output.ppm`, or convert it to PNG with `convert output.ppm output.png`.

## Sources
* [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
* [_Generating Camera Rays with Ray-Tracing_](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html)
* [_Placing a Camera: the LookAt Function_](https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function/framing-lookat-function.html)
