# mgend-rdf

## Usage

```
USAGE:
    mgend-rdf [FLAGS] <input> --assembly <assembly> --directory <directory> --format <format> --hgnc <hgnc>

FLAGS:
    -h, --help         Prints help information
        --rehearsal    Process only one line
    -V, --version      Prints version information

OPTIONS:
        --assembly <assembly>      Assembly [possible values: GRCh37, GRCh38]
        --directory <directory>    Path to output directory
        --format <format>          Output format [possible values: jsonld, turtle]
        --hgnc <hgnc>              Path to hgnc_complete_set.txt (wget
                                   ftp://ftp.ebi.ac.uk/pub/databases/genenames/new/tsv/hgnc_complete_set.txt)

ARGS:
    <input>    Path to input file [*.tsv | *.tsv.gz]
```

1. Obtain `hgnc_complete_set.txt`

   ```
   $ wget ftp://ftp.ebi.ac.uk/pub/databases/genenames/new/tsv/hgnc_complete_set.txt
   ```

1. Run converter

   ```
   $ mgend-rdf --assembly GRCh38 --format turtle --hgnc hgnc_complete_set.txt --directory output MGeND_hg38.tsv.gz
   ```

## Schema

![schema.svg](schema.svg)

## How to build

Prerequisites

* rustup

```
$ git clone https://github.com/med2rdf/mgend.git
$ cargo build --release
$ ./target/release/mgend-rdf --help
```
