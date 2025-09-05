# ock
A simpler, faster replacement for most awk use cases, ock is a command line utility for working with
table-like data.

## Installation
```
cargo install ock
```

## Usage
### Select a column
```
ps aux | ock -c 2
```
```
ps aux | ock -c pid
```

### Select a row
```
ps aux | ock -r caffeinate
```

### Selecting ranges
```
ps aux | ock -c pid -r 0:10
```

### Selecting multiple ranges
```
ps aux | ock -c name, pid -r 0:10
```

### Regex
```
ps aux | ock -c name, pid -r "python(2|3)"
```

### Delimiters
```
ock  -r 1:10:2 -c 1,5 --column-delimiter "," data.csv
```

### Out-of-bounds selections
Selecting columns that are not present in the input produces no output and
emits a warning.
