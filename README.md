# CAL Shuffle
cc1, as, ld scrambler.

This project is a wrapper around GCC/G++ meant to check compatilbity of
different versions of cc1, as, and ld. Each invocation will have its args hashed
and use that hash to randomly select a cc1, as, and ld combination.

## Usage

Normal command:
`/path/to/g++ test.c -o exec.out`

Equivalent wrapper command:
`CXX_RAND=/path/to/g++ cxx_rand test.c -o exec.out`

Change the seed to try different combinations for each unique command:
`CXX_RAND=/path/to/g++ SEED=1 cxx_rand test.c -o exec.out`

## Replicating Results
Each invocation hashes the input flags and combines the resulting hash with the
user-provided seed. That seed is used by a random selector that chooses the cc1,
as, and ld for a given invocation. As long as the flags and seed are not edited,
the same cc1, as, and ld will be used to compile every time.

## Building
`make rv64`

Use the binaries that are copied to the top level directory (./cc_rand and
./cxx_rand).
