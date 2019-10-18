# DeeDoo

Command line file deduplicator.

## Installation
Install directly with cargo:

``` shell
cargo install deedoo
```

## Usage:
``` shell
USAGE:
    deedoo [FLAGS] [OPTIONS] <directory>

FLAGS:
    -E, --ensure     Runs additional check to verify duplicate.
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show extra logging

OPTIONS:
    -o, --output <out_directory>    Directory for duplicated files.

ARGS:
    <directory>    Directory to scan.
```

