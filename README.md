```
       __________    _____    _________.____________  
  _____\______   \  /  _  \  /   _____/|   \_   ___ \ 
 /  ___/|    |  _/ /  /_\  \ \_____  \ |   /    \  \/ 
 \___ \ |    |   \/    |    \/        \|   \     \____
/____  >|______  /\____|__  /_______  /|___|\______  /
     \/        \/         \/        \/             \/ 

```

Stupid BASIC interpreter written in rust because I needed one. Yes I know, many exist, but I just couldn't find one in the Arch AURs so I just made one myself. It is most certainly not a serious project, don't attack me. At its current point, it barely has any features, It won't be as complicated as something like `qbasic` or `gwbasic` but I want to be somewhat functionional with graphics and a somewhat comprehensible syntax.

I Decided to use [Rust](https://rust-lang.org) instead of my beloved C for the soul reason that I want to move on to something new, I cannot say I am the largest fan of the language but it looks promissing.

## Usage
Since this project is written in rust we can easily compile it using cargo using `cargo build --release`
```sh
$ sBASIC [FILE]
$ cargo run --release [FILE] # or using cargo
```
Examples are located in the `res/` directory:
```sh
$ sBASIC res/Hello.bas
$ sBASIC res/Fibonacci.bas

# Or using cargo 
$ cargo run --release res/Hello.bas
$ cargo run --release res/Fibonacci.bas
```

## Features to come
- Interactive mode
- More keywords
- Graphics

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
Please make sure to update tests as appropriate.
