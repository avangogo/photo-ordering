# photo-ordering

This is my solution for the photo ordering problem.
It is designed to be fast enough for `n <= 15` photos while staying reasonnably simple.

## Compiling
To compile the code, you only need Cargo. You could typically do:
```
git clone git@github.com:avangogo/photo-ordering.git
cd photo-ordering
cargo test
```

## Running on a particular input
Examples are in the `examples` directory.
You can for instance run the program on the first example as follows:
```
cargo run --release examples/example1
```
The output should be 3.

## Python version
A python translation is in the `python` directory. It can be tested from the root of the project by:
```
python3 python/main.py examples/example1
```
