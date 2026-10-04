#!/bin/bash
# svd files generated useding msp430_svd:
# cargo run -- msp430fr2355

svd2rust --reexport-interrupt --atomics -g --impl-defmt defmt --target=msp430 -i msp430fr2355.svd

rm -rf src/

form -i lib.rs -o src/ && rm lib.rs

mv generic.rs src/

cargo fmt
