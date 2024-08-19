# Source Code Architecture of Coric's Quest

This document describes some of the high-level concepts contained within the source code of *Coric's Quest*.
If you're a programmer that's curious about the structure of a small-ish game, you're in the right place.

There's two broad things that are worth talking about: *game logic* and *rendering*.
There's a lot more stuff that the code does besides these two things, but they're mostly just lots of little details and shouldn't be too hard to figure out if you stare at them for long enough.

The source code of *Coric's Quest* is about ten thousand lines of Rust, counting blank lines, comments and a file generated from a JSON schema.
The code's kinda light on comments, for which I need to apologize in advance; except for that generated file, I wrote all the code by myself, for myself.
It's also organized strangely in hindsight: I was experimenting with separating struct definitions from impl blocks, and ordering functions alphabetically, both decisions I ended up hating, but were too deeply entrenched to back out of... lessons learned, I guess.

## Game Logic

The most important aspect of game engine code is having some idea about how shared game data changes over time, from frame to frame.
For *Coric's Quest*, this boils down to three things: the *mode stack*, the *script async function* and the *main loop*.

### Mode Stack

A **mode** is a logical grouping of drawn elements that should appear on screen with associated update logic.
Modes can be simple and self-contained like menus and dialog boxes, or more advanced like map drawing/movement and the battle system.
Modes have a *draw method* to render themselves.
Modes also have an *update async method* to modify its own state or the shared game state, possibly in response to input.

The data held by a mode falls into one of two categories: the drawn stuff, and everything else.
For example, a menu mode would hold data for drawn items like its window background, menu text and displayed cursor, while also holding an integer representing the current menu selection.

The **mode stack**, as the name implies, is a stack of modes.
Drawing the mode stack means drawing every mode from bottom to top, painter's algorithm style.
The code will arrange for the update async method of the top-most mode to be called; this is explained in the next section.

A lot of modes in the game are for things like dialogs and menus; other games would call the mode stack here an "interface stack" instead; I draw the current level map with one of these, so the name "interface" doesn't quite fit here.
Yet other games with the same idea would call modes "scenes" and talk about a "scene stack" here; I just call these things "modes" as a carry-over habit from my previous games, so I might switch to this name for this concept in any future games that I make.

### Script Async Function

The **script async function** represents the high-level flow of game logic.
Its job is to push and pop modes onto and off of the mode stack to control what's drawn on screen, since modes can't control the mode stack themselves.

The script async function also runs the update async method of the top-most mode of the mode stack.
That mode's update async method typically deals with things like inputs, and returns a **mode event**, which is an enum that the script async function can react to, e.g. the `YesNoPrompt` mode's update async method can return `YesNoPrompt::Yes` or `YesNoPrompt::No`.

The simplest example of how the script async function uses modes is the `TextBox` mode:

1. The script async function pushes the `TextBox` mode onto the mode stack.
2. The script async function calls the `TextBox::update` async method, awaiting its result.
3. The `TextBox::update` async method runs until the player presses a key, returning `TextBoxEvent::Done`.
4. The script async function checks that `TextBoxEvent::Done` was returned and carries on.

`TextBoxEvent::Done` is the sole possible mode event that `TextBox` returns, so technically it could just return nothing.
I did it for consistency, but I might just arrange for a mode like `TextBox` to return nothing in the future; it matters more for things like figuring out which menu option was chosen in a menu.

For any of this async function business to work properly, it needs to be able to yield control once it's done a frame's worth of work.
If I had my way with the Rust language, this script would be a *coroutine* or at least a *generator* that I could just yield for each frame; unfortunately, the former isn't even on track to stabilization and the latter is going nowhere fast.
Because of this, I have to create a custom *future* that waits exactly once (the `PendingThenReady` struct in `src/async_utils.rs`) then await on it in a helper function (the `wait` async function in the same file).
The one-time setup is a bit clunky, but using it is quite easy.
Most frame-to-frame waiting happens in mode async update methods, so the async script function just needs to await those in turn.

Yielding control is not enough; something needs to *poll* the script async function on each frame in order to drive it forward.

### Main Loop

The **main loop** loops while taking input, running update logic and drawing to the screen.
If you have any interest in game development, you already know about these inside and out, so I won't waste your time here explaining them in depth.

The main loop of *Coric's Quest* is provided by Miniquad, which provides callbacks for the aforementioned inputs, updating and drawing; this is what makes Miniquad a *framework*.

With everything laid out above, the main loop's responsibilities are easy enough to explain:

1. When an input event arrives, it's just buffered to be read later.
2. On update, the script async function is *polled* to do one frame's worth of work, typically via the update async method of the top-most mode of the mode stack that it's running at the time.
3. On draw, the mode stack is rendered one mode at a time from bottom to top.

To poll the async script function, Rust first needs us to *pin* it (i.e. place it at a memory address where it won't move), so it's initialized within a `Box::pin`.
Then to actually poll it, Rust requires us to create a `std::task::Context` that itself needs a `std::task::Waker`.
The async script function is called every single frame unconditionally, so they're completely unneeded, thus we make a dummy context with a dummy waiter while quietly wishing that stable Rust had proper coroutines.

### Sharing Data

Just kidding; there's a fourth thing that matters here!

There's two forms of data sharing in the source code of *Coric's Quest*:

1. between the main loop and the script async function, and
2. between the script async function, its helper functions and the mode async methods.

#### SharedMut

The first point is a big deal.
For example, the game loop needs to read the mode stack in order to draw it, but the script async function needs to modify the mode stack.
There's no way to pass data back-and-forth between an async function and the main loop that polls it, and Rust's references with their static lifetimes won't allow an owned value or mutable reference to coexist with any other kind of reference.

The solution to this is `SharedMut<T>`: a custom type that just wraps an `Rc<UnsafeCell<T>>` and implements the `Deref` and `DerefMut` traits.
The main loop makes a `SharedMut<ModeStack>` for itself, and creates a clone that it passes to the script async function to manipulate.
Miniquad is single-threaded, so all of this is safe without any need for synchronization.

Besides the mode stack, the input, audio, current level, actors and amount of color fade over the screen are all shared like this between the main loop and the script async function.

#### Contexts

Turning to the script async function itself, there's a lot of data that needs to be passed around between it and its various helper functions, not to mention the mode update async methods; passing all of it around as separate arguments would be miserable.

Fortunately, there's a better way.
Said misery is avoided with the help of a **context struct**, whose only job is to hold onto all the little bits of data that's needed throughout the game logic; this is what `ScriptContext` is in `src/contexts.rs`.
It's typed a lot, so it's given the short name of `sctx` throughout the source code.

Unfortunately, the `ScriptContext` also contains the `ModeStack`, which is mutably borrowed when running the update async method of its top-most mode, but we also want access to things in the `ScriptContext` when running a mode's update async method.
Rust doesn't permit this outright, but we can *reborrow* every field of `ScriptContext` except for the `ModeStack`, leading to `ModeContext`, which exists for this exact purpose.
Rust currently lacks partial borrows of structs, which would make this somewhat easier to deal with.

Looking back, it'd probably be easier to split the `ModeStack` from the `ScriptContext` entirely, which would remove the need for `ModeContext` to exist.
For any future games, I'd probably also rename `ScriptContext` into just `Game`, and its common `sctx` variable name into just `g` for easier typing.

## Rendering

*Coric's Quest* is drawn with the help of Miniquad's rendering API.
Specifically, it uses the `GlContext` rendering backend, which means that the game uses GLSL shaders together with either OpenGL on desktop platforms or WebGL with WebAssembly running in a web browser.

Before I get too deep into this, a word of warning:
I'm going to oversimplify and probably misuse a whole lot of terms here for the sake of keeping this explanation relatively short; rendering with graphics APIs is just a really big, messy topic.

Like most games, *Coric's Quest* renders everything on-screen every single frame.
This is done with two *render passes*, followed by a *frame commit* to ensure everything appears on screen.

The first render pass targets a small off-screen texture whose size is the internal resolution of the game.
This is where the mode stack is drawn, which draws each mode, which in turn draws each drawable element that it holds.

The second render pass targets the visible window.
It takes the off-screen texture that was just rendered to and applies aspect-correct scaling to fit it in the window, with letter-boxing if needed.

Drawing the mode stack is just a function that triggers drawing individual modes that are also just functions.
The interesting part is how the drawable elements are rendered: text, sprites, windows and the like.
Despite their variety, they all really just boil down to four steps:

1. Apply their *pipeline* (shaders and data layouts).
2. Apply their *bindings* (buffers and textures).
3. Apply their *uniforms* (non-vertex shared data).
4. Issue a *draw call*, possibly with instancing.

### Pipelines

A **pipeline** is made up of a *vertex shader*, a *fragment shader*, and layout descriptions of the bindings and uniforms it expects.

A **vertex shader** is a program that is run by the graphics API whose job is to position vertices onto the render target and associate them with any extra info needed for rendering later on, e.g. texture coordinates and colors.
A vertex shader reads data from the *vertex buffers* supplied in the bindings in the form of **vertex attributes**, combining them with data from the uniforms to produce vertex data.

Each set of three vertices from the vertex shader produces a triangle.
The area inside each triangle covers *fragments* (read: pixels) of the render target.
The extra data of each vertex (e.g. texture coordinates and colors) is "smoothed out" over the area of the triangle for each fragment, becoming the inputs to the *fragment shader*.

A **fragment shader** runs on each fragment (pixel), reading the smoothed-out inputs and outputting a single color.

There are three shaders used by *Coric's Quest*:

1. `quad_shader`: Draws two triangles arranged into a quad (rectangle), supports instancing.
2. `layer_shader`: Draws an entire tile map layer with a tile set and a tile data texture.
3. `screen_shader`: Like `quad_shader`, but just for rendering the off-screen texture to the screen proper.

It wouldn't be too hard to get rid of the `screen_shader` and just use the `quad_shader` for the same purpose, but what's there already works and doesn't cause any issues, so there's no pressing need to revisit it.

The pipelines are created once at the beginning of the game and have their handles copied to whatever drawable element needs them.
We never bother freeing them; program termination handles it for us.

### Bindings

A **bindings** struct holds the vertex buffers and textures needed to draw something or a set of things onto a render target.
Specifically, they're made up of one or more *vertex buffers*, a single *index buffer* and zero or more *textures*.

A **vertex buffer** is an array of data, typically floating point numbers, that's broken down into vertex attributes for consumption by the vertex shader.

The vertex buffers could just be read in the order they appear, but instead they're indexed in the order defined by the **index buffer**, which is just an array of integer index values that point into the vertex buffers.

Each set of three indices traces out a triangle.
The most commonly-drawn shape in *Coric's Quest* is the quad, defined by a vertex buffer with four points (named `quad_vbuf`) and an index buffer (named `quad_ibuf`) with six indices: three for each of the two triangles that make it up.
This pair of vertex buffer and index buffer are shared to just about everything that ever shows up on screen.

A **texture** is 2D data; this is almost always an image, but the `layer_shader` also uses an extra one to hold tile data.
Textures are *sampled* at specific texture coordinates by the fragment shader to determine the color of each fragment (pixel).

Bindings are intended for use with a specific shader; at least, that's the way that *Coric's Quest* treats them.

The vertex buffers, index buffer and textures that go into bindings are typically allocated and filled well in advance of any drawing.
Making buffers and textures allocates memory in graphics API memory space, so *Coric's Quest* goes out of its way to hold onto and reuse them whenever practical.

### Uniforms

**Uniforms** are just a bit of extra data needed for drawing something that isn't specific to any vertex, or a texture outright.
They're a bit like global variables that apply to a single draw call, except that they can't be changed mid-call.

Examples of uniforms are things like coordinate offsets that are applied to every vertex position, and the size of the off-screen render target to ensure everything is scaled to appear on the screen correctly.

Unlike buffers and textures, uniforms are typically set just before the draw call that needs them.
If uniforms need to be persistent, a drawable element will just hold onto the data itself and use it to set the uniforms each time it's asked to draw itself.

### Draw Calls

Once the pipeline, bindings and uniforms have been applied to draw something (or a set of things), a **draw call** tells the graphics API to put it all together and render it to the render target that was previously set by the render pass all the way at the beginning of the process.

Miniquad's API for issuing draw calls takes three arguments, but we only care about two of them: the *number of vertices* and the *number of instances*.

Since the API only ever draws triangles, the number of vertices is always a multiple of three; in fact, since the game almost exclusively draws two triangles in a quad arrangement, this number is always exactly `6`.

Using `1` as the number of instances for a draw call does exactly what you'd expect.
When it needs to be more than `1`, a little more setup is needed.

### Instancing

**Instancing** is like issuing multiple draw calls in a loop, except doing that outright would be tremendously inefficient.
Instead, the looping is done on the graphics API side with a specially-prepared buffer that holds the data that's different for each instance ("loop" iteration).

Instancing first needs support in a pipeline, and the `quad_shader` is set up for exactly this purpose.
The pipeline needs to expect an **instanced vertex buffer** whose step function is set to change per instance ("loop" iteration) instead of per vertex; otherwise they're just like normal vertex buffers, feeding their data to the vertex shader via vertex attributes.
For example, a single bit of data from an instanced vertex buffer will be seen by all six vertices of the two triangles that make up a quad.

The vertex attributes expected by the pipeline also need to be pointed to the right index in the list of vertex buffers passed to the pipeline, since instancing typically involves more than one vertex buffer.

The biggest user of instancing in *Coric's Quest* is text: every glyph that appears on-screen has an element in an instanced vertex buffer, with the offset of its glyph in the font glyph texture (source position) and the position it should show up at in the off-screen render target.
The number of instances specified during its draw call is just the number of glyphs of text that should be drawn; the game manipulates this in certain places for messages that appear letter-by-letter.

### Tile Map Rendering with Data Textures

A map in *Coric's Quest* is called a `Level` internally, each of which holds a list of `Layer` structs.
Each `Layer` could be rendered with instancing, at least in theory, but *Coric's Quest* takes a different approach.

The `layer_shader` pipeline takes not one, but *two* textures.
The first texture is the tile set with the graphics for each tile, like you'd expect.

The second texture is a **tile data texture**.
Each pixel of this texture is a tile on the layer; the red value is the `x` position of the tile, while the green value holds the `y` value, both counted in whole-tile units.

The fragment shader of the `layer_shader` pipeline first consults this tile data texture for the offset of the tile to render.
A fragment-specific pixel offset is added to this tile offset to find the final offset in the tile set texture to use for the fragment's color.

Doing things this way allows an entire tile layer to not only be drawn as a single instance, but as a single quad.
Is this faster than instancing?
I dunno, but it seems fast enough in practice, which is good enough for me.

## Conclusion

Well, those are the two big ideas underpinning the architecture of *Coric's Quest*.
This arrangement of game logic and rendering was the result of prior games and experiments, and it all worked out quite nicely; once the foundations were laid, menus, dialogs and one-off triggers, animations and cutscenes were created in minutes instead of hours.
