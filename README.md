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
EE-tools for IB CompSci Extended Essay 1.1.0
Gabriel DiFiore <difioregabe@gmail.com>
Program to automatically run a series of NIST randomness tests from an input file and plot the data

USAGE:
    ee-tools.exe --file <file> --source <source> --test <test_name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>         File containing input data for randomness tests
    -t, --test <test_name>    Test to perform n trials of (frequency_monobit, block_frequency, runs_test)
    -s, --source <source>     Source of Data (random.org, HotBits, etc.)
```

Put all trial data in a singular file, separated by newlines, and the program will automatically parse it and run the tests.

## Benchmarking
Performing the Runs Test on an input file of 40 lines (n=100), it took 4.8 seconds from opening file to graphing.

## What's in here?
A singular all-in-one tool for performing statistical analysis of (pseudo)random number generators based on the [NIST Standard](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-22r1a.pdf) and plotting the data on a graph. You could use all the functions provided separately, however that functionality is not built in to this version.

## Why rust?
I'd never used it before (thus the possible non-idiomatic nature of this code) and wanted to try and use it, and thought this was the perfect excuse to learn it.
