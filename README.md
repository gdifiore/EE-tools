# EE-tools
EE-tools - Tools written to help with my IB CompSci extended essay

## Building

```
git clone https://github.com/gdifiore/EE-tools.git
cargo build --release --verbose
```

## How to use the tools
After building the repository, you can use the `-h` flag like this `ee-tools -h` and it will print this screen:
```
EE-tools for IB CompSci Extended Essay 1.0.1
Gabriel DiFiore <difioregabe@gmail.com>
Can run a variety of NIST randomness tests, write/read/plot data and automatically run a series of tests

USAGE:
    ee-tools.exe --file <file> --test <test_name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>        File containing input data for randomness tests
    -t, --test <test_name>    Test to perform n trials of (frequency_monobit, block_frequency, runs_test)
```

Put all trial data in a singular file, separated by newlines, and the program will automatically parse it and run the tests.

## What's in here?
A singular all-in-one tool for performing statistical analysis of (pseudo)random number generators based on the [NIST Standard](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-22r1a.pdf) and plotting the data on a graph. You could use all the functions provided separately, however that functionality is not built in to this version.

## Why rust?
I'd never used it before (thus the possible non-idiomatic nature of this code) and wanted to try and use it, and thought this was the perfect excuse to learn it.
