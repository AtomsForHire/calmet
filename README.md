# Calmet

> [!CAUTION]
I'm still actively converting my personal python script into this CLI utility, so
I haven't tested much just yet.

> [!WARNING]
Compilation has been tested on Mac Sequoia 15.5 only, at the moment.

# Installation

## Dependencies
1. Make sure cargo is installed on your system.
2. You will also need cfitsio >= 3.37 for the [rust-fitsio
   crate](https://github.com/simonrw/rust-fitsio).
   * If you have a C compiler + autotools + make, you can use the `fitsio-src`
   feature to have cargo compile cfitsio from source (following the rust-fitsio
   documentation).

## Example commands
1. Install with cfitsio already available on your system
```sh
> git clone git@github.com:AtomsForHire/calmet.git
> cd calmet && cargo install --path .
```

2. Install with cargo compiling cfitsio for you
```sh
> git clone git@github.com:AtomsForHire/calmet.git
> cd calmet && cargo install --path . --features=fitsio-src
```

# Usage
To use this tool, simply run `calmet` to output available subcommands and their
description. Running `calmet <subcommand>` will also print out available
options. Currently, all commands require and input for the `-f` or `--files`
options.

Example:
```
$ calmet
Calculates metrics from hyperdrive solutions

Usage: calmet <COMMAND>

Commands:
  cal-metrics    Calculate all calibration metrics
  amp-metrics    Calculate only EW and NS gain smoothness
  phase-metrics  Calculate only EW and NS phase metrics
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` 
$ calmet cal-metrics
Calculate all calibration metrics

Usage: calmet cal-metrics [OPTIONS]

Options:
  -f, --files <FILES>...
  -h, --help              Print help
```

```
$ calmet cal-metrics -f *.fits
Calculating amplitude smoothness, phase RMSE, and phase average euclidean distance
Finished
```
