# TC37x Peripheral package 
Not full support of target peripherals 

# Building librs from the svd file

The tool svd2rust is needed to build the PAC library from a svd file

to build lib.rs execute
```sh
mkdir tmp
svd2rust --target tricore --feature_peripheral -i TC37X.svd -o tmp
form -i tmp/lib.rs -o src
rustfmt src/*.rs src/*/*.rs
```

To support TriCore in svd2rust a modified version of svd2rust from
HighTec must be used.
This version of svd2rust supports the interrupt macro generation and
in a future release also the support of LDMST or atomic read-modify-write
access to the peripherals will be implemented
