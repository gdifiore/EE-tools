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
EE-tools for IB CompSci Extended Essay 0.3.0
Gabriel DiFiore <difioregabe@gmail.com>
Can run a variety of NIST randomness tests, write/read/plot data and automatically run a series of tests

USAGE:
    ee-tools.exe [FLAGS] [OPTIONS]

FLAGS:
        --all                       Runs all tests on given data
        --block_frequency_test      Runs Frequency Test within a Block on given data
        --data_plot                 for testing data plot functions on given data
        --frequency_monobit_test    Runs Frequency (Monobit) Test on given data
    -h, --help                      Prints help information
    -p                              If supplied data will be automatically plotted and outputted
        --runs_test                 Runs Runs Test on given data
    -V, --version                   Prints version information
    -w                              If supplied data will be written to file

OPTIONS:
    -a, --data <DATA>    File containing witten data of randomness test results
    -f, --file <FILE>    File containing input data for randomness tests
```

## What's in here?
This collection is tools for statistical analysis of random number generators based on the [NIST Standard](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-22r1a.pdf), and tools I use to manage and display the data.

## Why rust?
I'd never used it before (thus the non-idiomatic nature of this code) and wanted to try and use it, and thought this was the perfect excuse to learn it.
