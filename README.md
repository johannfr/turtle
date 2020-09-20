A Rust program for turning turtle_graphics commands into 
SVG graphics.

Made as part of our internal Rust Training program.

# Syntax
Here is an example of the input syntax:
```
    go_forward(15)
    turn(45)
    pen_up
    go_forward(10) 
    pen_down
    color(RED)
    turn(45)
    go_forward(30)
```

## go_forward(DISTANCE)

Move forward DISTANCE pixels, drawing a line if pen is down.

## turn(ANGLE_IN_DEGREES)

Turn ANGLE_IN_DEGREES degrees (0..359).

## pen_up

Stop drawing when subsequently moving forward.

## pen_down

Start drawing when subsequently moving forward.

# Running the example
```bash
cargo build
cargo run examples/test.turtle
```

You will now have an output-file called svg_rendering.html

Open that up with your favourite browser, e.g. on macOS:
```bash
open svg_rendering.html
```
