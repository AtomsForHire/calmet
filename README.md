# Calmet

[!CAUTION]
I'm still actively converting my personal python script into this CLI utility, so
I haven't tested much just yet.

[!WARNING]
Compilation has been tested on Mac Sequoia 15.5 only, at the moment.
Additionally, it only supports calibration solutions from Hyperdrive (at the
moment). 

# Installation

Make sure cargo is installed on your system.

1. `git clone git@github.com:AtomsForHire/calmet.git`
2. `cd calmet && cargo install --path .`

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
  all              Calculate all calibration metrics
  gain-smoothness  Calculate only EW and NS gain smoothness
  phase-metrics    Calculate only EW and NS phase metrics
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

``` 
$ calmet all
Calculate all calibration metrics

Usage: calmet all [OPTIONS]

Options:
  -f, --files <FILES>...
  -h, --help              Print help
```

```
$ calmet all -f *.fits
Calculating amplitude smoothness, phase RMSE, and phase average euclidean distance
Finished
```
