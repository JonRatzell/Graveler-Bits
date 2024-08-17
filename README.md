# Graveler-Bits

A [Rust](https://www.rust-lang.org/) submission to [The SCIENCE of Soft Lock Picking](https://youtu.be/M8C8dHQE2Ro) [coding challenge](https://github.com/arhourigan/graveler).

## Performing 1 billion iterations in about ~1.33s[^1]

![image](https://github.com/user-attachments/assets/4cdb2436-0e16-4f3c-80a4-8dea21e10f4b)

### Benchmark

![image](https://github.com/user-attachments/assets/393bc564-f2ae-4570-a0c8-daccf67bf66e)


Made using [rayon](https://github.com/rayon-rs/rayon) for easy parallel processing and [xoshiro256+](https://prng.di.unimi.it/) from [rand_xoshiro](https://github.com/rust-random/rngs/tree/master/rand_xoshiro) for generating random numbers.

## Building
Requires [Rust toolchain](https://www.rust-lang.org/tools/install)

- Clone/Download the repo
- `cargo run --release` to run the binary or `cargo bench` to run included benchmark


## Bonus
Cranking TOTAL_ROLLS up in [main.rs](https://github.com/JonRatzell/Graveler-Bits/blob/main/src/main.rs) turns this program into an excellent stress test for your CPU's temps.[^2]



[^1]: Best time on my system. YMMV
[^2]: Nothing gets temps going quite like running every core at 100% for minutes.
