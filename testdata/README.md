# Test Redcode Warriors

These files exist to test the program against real-world examples of warriors,
all freely available online.

## Directory Structure

- `input`: used for reading warrior files
- `expected_output`: the expected "load file" format for a given output.
  Matches the `input` directory structure approximately one-to-one. These files
  are generated using a pMars with `generate.sh`.

Within each directory are some subdirectories:

### Unimplemented

This directory matches the main directory structure but acts as a placeholder for
tests cases that will fail without some feature support (mostly, `FOR` macros).

### Simple

Some "non-real-world" warriors created to exercise specific codepaths, or perform
a kind of smoke test.

### Wilkie

Taken from [Wilkie's benchmark](http://www.koth.org/wilkies/) on KOTH.

### WilMoo Benchmark

Taken from the [WilMoo benchmark](http://www.koth.org/wilmoo/) on KOTH.
