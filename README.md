# Ray tracing in Rust

The goal of this workshop is to introduce you to two (hopefully) new things.

1. The Rust programming language
2. Ray tracing

Rust is a systems programming language that focuses on high performance and memory safety.
By leveraging zero-cost abstractions and a strict compiler, the languages goes a long way of providing and ensuring these two things.

A ray tracer is a program that renders a scene by approximating actual light rays and their path through the physical world.
We can use the light gathered by the randomly scattered rays through our scene, so that we are able to project them back to a 2D pixel grid defined by a camera, as illustrated in the picture below.
<img src="imgs/ray-tracer.png" width="500px" alt="Ray tracer" style="display: block; margin: 0 auto;" />

This workshop contains a partly implemented ray tracer that is written in Rust.
Your goal is to finish the tasks that will bring you through the different steps necessary for a basic ray tracer, and by the end of the workshop you should be able to generate some nice looking images!

## Setup
The first thing you need to do is to install Rust, you can find the installation instructions [here](https://www.rust-lang.org/en-US/install.html).
There are language support in most major text editors, and in some IDEs.
Check out [Are we IDE yet](https://areweideyet.com/) for a list of plugin support for your favorite editor.
The only thing you really need is syntax highlighting for the `.rs` file extension.

By default the latest version of Rust will be installed, together with the latest version of Cargo.
Cargo is the build tool and package manager for rust, it lets us build libraries and executables, and run tests and the executables we have defined in the project.

There's three commands you will find usable for this project:
* `cargo build` - this command builds all the files in the project, any compilation warning (and trust me, you will encounter them) will be shown as part of the build output.
* `cargo test` - this command builds the project *and* runs all the tests in the project.
You should run this command while you solve the assignments, they should indicate when you have passed the different steps.
* `cargo run --bin image` - this command builds the project *and* runs the executable file found in `src/bin/image.rs` (this file contains a `main()` function, and is therefore an executable).
You should run this command (starting from **Step 2**), as it will produce the rendered image of our scene.

## Step 1 - Vectors
At the core of any ray tracer lies vector operations.
This includes addition between vectors and scalars, and other operations like multiplication and division.
The `Vec3` class (a 3-dimensional vector) is implemented in the `src/vec.rs` file, and it contains most of the vector operations that we need for our ray tracer.

**Step 1** of our ray tracer is to implement the missing vector operations!
Go to `src/vec.rs` and look for the code that starts with `panic!("Step 1...")`.
`panic!` is a [Rust macro](https://doc.rust-lang.org/beta/book/macros.html) (you can see that by the exclamation mark), and Rust will exit the program immediately when it is encountered at run-time.

*Hint: You can look at the implementation of the other vector operations in the file if you're not sure about the implementation details.*

**Verification step:**
* run the `cargo test` command and verify that all tests labeled `test_1...` run successfully.

## Step 2 - A gradient image
Now that we have implemented the core math our ray tracer, it is time to generate an image - we will start by implementing a simple gradient "sky" for our scene.

**Step 2a**, we need to set up a camera so that we can capture an image of our scene, head to `src/bin/image.rs` to figure out the missing values that is needed to initialize the `Camera`.
As you can see, the `main()` function is rather simple.
All it does is initialize our camera and a scene, and then it calls the `trace_scene()` library routine, which in turn gives us a vector of colors (given by the type `Vec<Color>`) - one `Color` for each pixel in the image.
Finally, it takes these color values and converts them to a BMP image that it save as `scene.bmp`.

**Step 2b**, head to the `src/lib.rs` and navigate to the `trace_scene()` library function - this function contains the main loop of the ray tracer.
This is where we initialize the color vector (`Vec<Color>`), and the for each `x` and `y` coordinate of the image we cast a new ray from the camera and into the scene by calling the `trace_ray_in_scene()` function (see the picture in the top of the README for an illustration).
As you can see, our work here is not yet done, you need to make sure the `trace_ray_in_scene()` function actually gets called and returns the gradient!
**Note:** You can set the `depth` parameter to 0, we will increment this later.

**Verification steps:**
* run the `cargo test` command and verify that the tests labeled `step_2...` run successfully.
* When they do, run the `cargo run --bin image` command and verify that your program executes *without* panicking!
Does it?
If so, navigate to the project directory and open the `scene.bmp` image.
**The image should be a gradient of light blue and white.**

*Does it not run without panicking? Then look over and make sure that all panics marked `Step 2...` have been fixed, or contact one of the helpful persons to help you!*

## Step 3 - Intersection between Rays and Spheres

Even though we now have an image, it is not very exciting to look at.
The next essential step of a ray tracer is `Ray`-`Intersectable` intersection!
Without it, we will not be able to display our `Intersectables` and their colors.

Open the `src/scene.rs` file and take a couple of moments to reflect over the `Intersectable` trait found in the top of the file.
We can see that the trait contains two function signatures that are important to the core of the ray tracing algorithm:
* **`intersects(Ray, min, max) -> Option<Intersection>`** is a function that takes in a `Ray` (in addition to a min and max limit value that helps us determine if the `Ray` intersects or not), and returns an optional `Intersection` struct.
The returned `Intersection` contains the intersection point, the surface normal at the intersection, and the shape that was intersected. This helps us trace new rays recursively from the intersection point.
* **`scatter(Ray, Intersection) -> Option<(Color, Ray)>`** is a function that takes a `Ray` and an `Intersection` as arguments, and returns the `Color` of the `Intersectable`, in addition to a new `Ray` with its origin at the intersection point and its direction pointing further into the scene.

Further down in this file we can see two structs that implement this trait; the `Scene` and the `Sphere`.
The `Scene` simply loops over all its `Intersectable`'s and returns the one that is closest to the origin of the `Ray`.
The `Sphere` intersection is mostly implemented, but you have to do the math for the `Intersection`.

**Step 3a**, complete the implementation of the `point_along_direction()` function on the `Ray` struct found in `src/ray.rs`.

**Step 3b**, complete the implementation of the surface normal in the `create_intersection()` function according to the illustration below.

<img src="imgs/surface-normal.jpeg" width="500px" alt="Calculate surface normal" style="display: block; margin: 0 auto;" />

**Step 3c**, now that we have done the necessary calculations, we need to actually trigger the `Intersection` between the initial `Ray` and the `Scene`.
Open `src/lib.rs` and navigate back to the `trace_ray_in_scene()` function.
Now call the `intersects()` function on the `scene` instead of returning the gradient as you currently do.
Since the `intersects()` function returns an `Option<Intersection>`, we will need to handle both of the cases of the value and map it to a `Color` - this is easily done with a `match` expression!
The `trace_ray_in_scene()` function should now look like this:

```rust
fn trace_ray_in_scene(ray: &Ray, scene: &Scene, depth: u32) -> Color {
    if depth == 50 {
        return Color::black(); // Return black to avoid being stuck with an unlimited recursion
    }
    match scene.intersects(ray, 0.0, f64::MAX) {
        Some(intersection) => {
            Color::black()
        }
        None => gradient(ray),
    }
}
```

**Verification steps:**
* run the `cargo test` command and verify that the tests labeled `step_3...` run successfully.
* When they do, run the `cargo run --bin image` command and look at your picture again.
What do you see?

*The image should now be ~60% black, but the gradient "sky" should still be visible at the top of the image.*
*Is this not what you see? Then ask someone for help, this is where the fun starts! :-)*

## Step 4 - Scatter new rays
We are now almost done with the implementation of our ray tracer!
Now, instead of returning `Color::black()` when your ray intersects with the scene, call the `scatter()` function like so: `intersection.shape.scatter(ray, &intersection)`.

**Step 4a** - remember that the call to `scatter()` returns an `Option<(Color, Ray)>`?
You need to handle this value similarly to what you did in **Step 3c** - return this color instead of black when `scatter()` returns `Some` value, but you can still return black the value returned from `scatter()` is `None`.

*What does your picture look like now? You should be able to see some pretty sorry looking `Spheres`!*

**Step 4b**, our picture has spheres, but just barely (they're more like flat circles).
The next essential step in the ray tracing algorithm we're implementing is *recursion*.
The `scatter()` function returns a new `Ray` with the origin set to the intersection point, but with a new random direction.
All you need to do is to multiply the *scattered color* with the new color value obtained by calling `trace_ray_in_scene()` recursively, with the *scattered ray* as its parameter.

*The picture should begin look pretty good, albeit a little grainy.*

**Step 4c**, we have a very easy fix for this:
Open up `src/bin/image.rs` and give `num_samples` a higher value than 1, what about 100?
**Or if you're feeling ambitious** you can set it to 1000 - but be warned, it might take a little while!

**Verification step:**
* If you're seeing an (objectively) awesome picture, you're done!
You have (partially) implemented your own ray tracer, congratulations!

*Do you not think what you see is very awesome? It might be that you have done something wrong, or you might not not think that spheres are as awesome as some of us others do. Either way, let someone know so they can help you with your concerns!*

## Looking Further
Our ray tracer is done for now, but that does not mean that we are done with cool things.
There are a number of things you can do with this ray tracer as a starting point:

### Video
Check out the video executable in the project, you can find it in `src/bin/video.rs`.
It lets you add animations to the camera and the spheres, you can run the command `cargo run --bin video` and check out the output to get started.

### Optimization
Ray tracing requires a lot of computation, but this library is only using a single CPU core.
If your computer has more than one core, you can use the [rayon](https://github.com/nikomatsakis/rayon) library to optimize the `trace_ray_in_scene()` function.
`rayon` is already installed, you can include it into `src/lib.rs` by adding `extern crate rayon;` to the top of the file, and then the rest is up to you!

### Cooler scenes
We have only provided a single scene in the main file, but you can modify it or create a new one.
Can you come up with any interesting scenes to generate?
E.g. mickey mouse or another shape that can be entirely composed by spheres?

What about combining it with the animation library?

### New Intersectables
Okay, now were getting ahead of ourselves.
But seriously, what about implementing a `Triangle` `Intersectable`?
This is not an easy task, but it is a worthy one.
If you have a `Triangle`, you will be able to describe any kind of complex shape by combining them into a larger mesh object.
This will require more background than what is given in this workshop, but there are a large number of freely available resources online to get started with this.
Happy coding! :-)
