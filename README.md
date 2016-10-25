# NaCl Selective Keygen

Generate a NaCl keypair (using brute force) where the public key matches a
certain pattern.

Usage:

    $ cargo run --release -- 123
    Searching for keypair that starts with 123...

    Iteration 0...
    Iteration 1000...
    Iteration 2000...
    Iteration 3000...
    Iteration 4000...

    ==> Public: 12323a4b024fa4a6a8f35fa0f03257434e23cbe7b3c1de16ddbacb7f355e001e
    ==> Secret: 40e910e93edd998134a4348cd923d3e17bffb8b80a28be985d23dd4b12d5f884

    Found key after 4837 iterations.
