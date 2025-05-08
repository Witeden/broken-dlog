# Solution
The main hint in this challenge is that the random exponent `r` lies within [0, 2**50].
Since the only missing inputs are `r` and `x`, knowing `x` gives direct access to `x` by computing
```
x = (t-r)/c
```
Let us denote by b the bitwidth of r (b=50 here).

## Computing r
There are 2 algorithms that break the discrete logarithm while being light to implement: Baby-Steps-Giant-Steps and Pollard's rho.
Since here we know that `r` is bounded, Baby-Steps-Giant-Steps appears to be the most direct algorithm to solve the problem.
Both algorithm reduce the problem of the discrete logarithm to a complexity in O(√p). But Baby-Steps-Giant-Steps directly 
reduces the complexity to O(√b). (Pollard's rho may also do the job, but Baby-Steps-Giant-Steps does it in a more trivial way.)

This solution implements Baby-Steps-Giant-Steps in Rust using 2 crates:
- `ark-ff` for finite field arithmetic,
- and `rayon` to parallelize all computations on the CPU.  
The computation was run on a Ryzen 7700 and took around half an hour.
The implementation gives `r = 996179739629170`.

## Computing x
```
x = (t-r)/c
  = 129741816436586536192511069033522723797805991085207391260653840826086090109
```
We can make sure that the result is correct by checking that `g^x = h`.
