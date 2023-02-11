# Hello World using LLVM (13) In Rust

Adapted from this article [Using LLVM from Rust to generate WebAssembly binaries](https://medium.com/@jayphelps/using-llvm-from-rust-to-generate-webassembly-93e8c193fdb4)

Just prints "Hello World" in an executable, currently set up for arm64, but should be straight forward to set up for other architectures

# Run (MacOS arm64)

Install LLVM 13
```
brew install llvm@13
```

Set LLVM_SYS_130_PREFIX variable
```
export LLVM_SYS_130_PREFIX=/PATH/TO/LLVM13/VERSION
```

We can then generate the executable  
```
cargo run
```

Then run the executable
```
./main
```
Which should print "Hello world!"

