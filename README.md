![logo](https://raw.githubusercontent.com/hh-Naram/sBASIC/master/Branding/Logo.png)

A stupid BASIC interpreter **(sBASIC)** written in rust because I needed one. Yes I know, many exist, but I just couldn't find one in the Arch AURs so I just made one myself. It is most certainly not a serious project, don't attack me. At its current point, it barely has any features, It won't be as complicated as something like `qbasic` or `gwbasic` but I want to be somewhat functionional language with graphics and a somewhat comprehensible syntax.

I decided to use [Rust](https://rust-lang.org) instead of my beloved C for the soul reason that I want to move on to something new, I cannot say I am the largest fan of the language but it looks promissing. I was debating if I should use `Vulkan` instead of `OpenGL` but using `Vulkan` seemed maybe a bit overkill, maybe in a future update I'll make the switch to get better performance? But I ended up using [SDL2](https://www.libsdl.org/) since it uses an `OpenGL` backend and is relatively simple for the scale of this project.

## Usage
Since this project is written in rust we can easily compile it using cargo using `cargo build --release`
```sh
$ sBASIC [FILE]
$ cargo run --release [FILE] # or using cargo
```
Examples are located in the `examples/` directory:
```sh
$ sBASIC examples/Hello.bas
$ sBASIC examples/Fibonacci.bas
$ sBASIC examples/Calculator.bas
$ sBASIC examples/Graphics.bas
...

# Or using cargo 
$ cargo run --release examples/Hello.bas
$ cargo run --release examples/Fibonacci.bas
$ cargo run --release examples/Calculator.bas
$ cargo run --release examples/Graphics.bas
...
```

## Features
- Somewhat of a modern syntax.
- Basic graphics capabilities.
- Performance and speed.

## Features to come
- Better syntax for commas.
- Functions.
- Maths functions _(sin/arcsin, cos/arccos, tan/arctan, ...)_.

## Keywords
Basic operations:
- `PRINT`/`INPUT` : Print data to the console, or read data from the console.
- `IF`/`THEN`/`GOTO` : Conditional operations and jumps.
- `LET` : Create/Modify a variable.
- `END` : End program.
- `REM` : Comment

Graphics Operations:
- `SCREEN` : Setup and initialize a window.
- `CLEAR` : Clear window with a clear color.
- `COLOR` : Set draw color.
- `LINE`/`TO` : Render a line from `(x1, y1)` to `(x2, y2)`.
- `DOT` : Render a dot.
- `CIRCLE` : Render a circle.

## Graphics example screenshot
This is what the example in `examples/Graphics.bas` gives, I am pretty proud of it so I decided to include this screenshot.
![graphics_screenshot](https://raw.githubusercontent.com/hh-Naram/sBASIC/master/screenshots/Graphics.png)

## References
- [Wiki: BASIC](https://en.wikipedia.org/wiki/BASIC)
- [Wiki: Graphics BASIC](https://en.wikipedia.org/wiki/Graphics_BASIC)
- [Github: gwBASIC](https://github.com/microsoft/GW-BASIC)
- [Github: rBasic](https://github.com/travisbhartwell/rbasic)
- [Crafting Interpreters](https://craftinginterpreters.com/)
- [gwBASIC User Manual](http://www.antonis.de/qbebooks/gwbasman/)

## Contributing
Pull requests are welcome as working on this project solo is not as fun as you can imagine, _also I basically never finish projects_. For major changes, please open an issue first to discuss what you would like to change.
Please make sure to update tests as appropriate.
