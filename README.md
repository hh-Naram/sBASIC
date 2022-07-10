![logo](https://raw.githubusercontent.com/hh-Naram/sBASIC/master/Branding/Logo.png)

Stupid BASIC interpreter written in rust because I needed one. Yes I know, many exist, but I just couldn't find one in the Arch AURs so I just made one myself. It is most certainly not a serious project, don't attack me. At its current point, it barely has any features, It won't be as complicated as something like `qbasic` or `gwbasic` but I want to be somewhat functionional language with graphics and a somewhat comprehensible syntax.

I decided to use [Rust](https://rust-lang.org) instead of my beloved C for the soul reason that I want to move on to something new, I cannot say I am the largest fan of the language but it looks promissing. I was debating if I should use `Vulkan` instead of `OpenGL` but using `Vulkan` seemed maybe a bit overkill, maybe in a future update I'll make the switch to get better performance?

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
## References
- [Wiki: BASIC](https://en.wikipedia.org/wiki/BASIC)
- [Wiki: Graphics BASIC](https://en.wikipedia.org/wiki/Graphics_BASIC)
- [GITHUB: gwBASIC](https://github.com/microsoft/GW-BASIC)
- [GITHUB: rBasic](https://github.com/travisbhartwell/rbasic)
- [Crafting Interpreter](https://craftinginterpreters.com/)k
- [gwBASIC User Manual](http://www.antonis.de/qbebooks/gwbasman/)

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
Please make sure to update tests as appropriate.
