# DeeDoo

Command line file deduplicator.

## About

The deduplicator does not delete any files. Any duplicates it finds are moved to the `rejects` directory with the full path preserved. The `rejects` directory location can be specified with the `-o` option. The program finds the duplicates by performing `crc32` calculation on the file content. If the `-E` flag is specified program will perform full byte-by-byte comparison to ensure files are the same. 

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

