# smush

![Build Status](https://github.com/gwihlidal/smush-rs/workflows/CI/badge.svg)
[![Latest version](https://img.shields.io/crates/v/smush.svg)](https://crates.io/crates/smush)
[![Documentation](https://docs.rs/smush/badge.svg)](https://docs.rs/smush)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Common rust abstraction around a variety of encoding and compression codecs.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
smush = "0.1.5"
```

Example:

```rust
use smush::{
    decode, encode, is_codec_enabled,
    Codec::{self, *},
    Quality,
};
use std::time::{Duration, Instant};

const TEST_DATA: &[u8] = include_bytes!("../src/ipsum.txt");

fn format_timing(d: &Duration) -> String {
    let mut t = d.as_secs_f32();
    let mut i = 0;
    let prefix = vec!["", "m", "μ", "n"];

    while t < 1.0 && i < prefix.len() - 1 {
        t *= 1000.0;
        i += 1;
    }

    format!("{:.2} {}s", t, prefix[i])
}

fn print_delta(identity: f32, encoded: f32, codec: Codec, quality: Quality, timings: &str) {
    let delta = (identity - encoded) / identity * 100f32;
    if delta > 0f32 {
        println!(
            "[{}] - {} is {:.2}% smaller than identity - {}",
            quality, codec, delta, timings
        );
    } else {
        println!(
            "[{}] - {} is {:.2}% larger than identity - {}",
            quality,
            codec,
            delta.abs(),
            timings
        );
    }
}

fn run_test(encoding: Codec, quality: Quality) {
    if is_codec_enabled(encoding) {
        let (encode_elapsed, encoded) = {
            let start_time = Instant::now();
            let e = encode(&TEST_DATA, encoding, quality).unwrap();
            (start_time.elapsed(), e)
        };
        assert_ne!(&TEST_DATA, &encoded.as_slice());

        let (decode_elapsed, decoded) = {
            let start_time = Instant::now();
            let d = decode(&encoded, encoding).unwrap();
            (start_time.elapsed(), d)
        };
        assert_eq!(&TEST_DATA, &decoded.as_slice());

        let encoded_len = encoded.len() as f32;
        print_delta(
            TEST_DATA.len() as f32,
            encoded_len,
            encoding,
            quality,
            &format!(
                "encode: {}, decode: {}",
                format_timing(&encode_elapsed),
                format_timing(&decode_elapsed)
            ),
        );
    } else {
        println!("[{}] - {} not enabled", quality, encoding);
    }
}

fn run_tests(quality: Quality) {
    run_test(Base58, quality);
    run_test(BinCode, quality);
    run_test(Brotli, quality);
    run_test(Bzip2, quality);
    run_test(Deflate, quality);
    run_test(Gzip, quality);
    run_test(Lz4, quality);
    run_test(Xz, quality);
    run_test(Zlib, quality);
    run_test(Zstd, quality);
}

fn main() {
    println!("*********************");
    println!("Level 1 Quality");
    println!("*********************");
    run_tests(Quality::Level1);

    println!();
    println!("*********************");
    println!("Default Quality");
    println!("*********************");
    run_tests(Quality::Default);

    println!();
    println!("*********************");
    println!("Maximum Quality");
    println!("*********************");
    run_tests(Quality::Maximum);
}
```

## Example

```shell
$ cargo run --release --example main

*********************
Level 1 Quality
*********************
[level1] - base58 is 36.57% larger than identity - encode: 58.17 ms, decode: 14.93 ms
[level1] - bincode is 0.14% larger than identity - encode: 4.43 μs, decode: 7.25 μs
[level1] - brotli is 53.33% smaller than identity - encode: 133.21 μs, decode: 110.13 μs
[level1] - bzip2 is 67.11% smaller than identity - encode: 756.55 μs, decode: 186.55 μs
[level1] - deflate is 53.08% smaller than identity - encode: 185.62 μs, decode: 44.70 μs
[level1] - gzip is 52.76% smaller than identity - encode: 95.29 μs, decode: 42.35 μs
[level1] - lz4 is 40.96% smaller than identity - encode: 38.62 μs, decode: 8.54 μs
[level1] - xz is 60.63% smaller than identity - encode: 1.92 ms, decode: 167.27 μs
[level1] - zlib is 52.97% smaller than identity - encode: 113.27 μs, decode: 42.62 μs
[level1] - zstd is 59.83% smaller than identity - encode: 73.32 μs, decode: 28.68 μs

*********************
Default Quality
*********************
[default] - base58 is 36.57% larger than identity - encode: 55.72 ms, decode: 15.16 ms
[default] - bincode is 0.14% larger than identity - encode: 4.01 μs, decode: 7.11 μs
[default] - brotli is 63.32% smaller than identity - encode: 1.23 ms, decode: 66.27 μs
[default] - bzip2 is 67.11% smaller than identity - encode: 726.58 μs, decode: 177.74 μs
[default] - deflate is 63.12% smaller than identity - encode: 178.31 μs, decode: 37.10 μs
[default] - gzip is 62.80% smaller than identity - encode: 167.54 μs, decode: 36.45 μs
[default] - lz4 is 46.67% smaller than identity - encode: 102.30 μs, decode: 8.43 μs
[default] - xz is 62.06% smaller than identity - encode: 4.48 ms, decode: 180.16 μs
[default] - zlib is 63.01% smaller than identity - encode: 198.37 μs, decode: 36.10 μs
[default] - zstd is 62.37% smaller than identity - encode: 123.16 μs, decode: 24.73 μs

*********************
Maximum Quality
*********************
[maximum] - base58 is 36.57% larger than identity - encode: 55.54 ms, decode: 14.90 ms
[maximum] - bincode is 0.14% larger than identity - encode: 3.77 μs, decode: 7.25 μs
[maximum] - brotli is 65.12% smaller than identity - encode: 11.45 ms, decode: 83.31 μs
[maximum] - bzip2 is 67.11% smaller than identity - encode: 700.11 μs, decode: 179.40 μs
[maximum] - deflate is 63.12% smaller than identity - encode: 198.41 μs, decode: 36.18 μs
[maximum] - gzip is 62.80% smaller than identity - encode: 165.79 μs, decode: 36.04 μs
[maximum] - lz4 is 46.67% smaller than identity - encode: 112.02 μs, decode: 8.96 μs
[maximum] - xz is 62.06% smaller than identity - encode: 4.81 ms, decode: 187.81 μs
[maximum] - zlib is 63.01% smaller than identity - encode: 203.94 μs, decode: 37.16 μs
[maximum] - zstd is 63.12% smaller than identity - encode: 12.05 ms, decode: 29.43 μs
```
By default, all codecs are enabled. It may be desirable to only enable the codecs that you want.

You can specify `--no-default-features` / `default-features = false` to disable all codecs, and then opt in to the feature names for the codecs you want.

Available codec feature names:

- base58_support
- bincode_support
- brotli_support
- deflate_support
- gzip_support
- lz4_support
- xz_support
- zlib_support
- zstd_support

As an example, the following shows support for only `brotli`, `lz4`, and `zstd`:

```shell
$ cargo run --release --example main --no-default-features --features=brotli_support,lz4_support,zstd_support

*********************
Level 1 Quality
*********************
[level1] - base58 not enabled
[level1] - bincode not enabled
[level1] - brotli is 53.33% smaller than identity - encode: 166.26 μs, decode: 122.33 μs
[level1] - bzip2 not enabled
[level1] - deflate not enabled
[level1] - gzip not enabled
[level1] - lz4 is 40.96% smaller than identity - encode: 58.69 μs, decode: 12.31 μs
[level1] - xz not enabled
[level1] - zlib not enabled
[level1] - zstd is 59.83% smaller than identity - encode: 120.04 μs, decode: 40.39 μs

*********************
Default Quality
*********************
[default] - base58 not enabled
[default] - bincode not enabled
[default] - brotli is 63.32% smaller than identity - encode: 1.10 ms, decode: 65.12 μs
[default] - bzip2 not enabled
[default] - deflate not enabled
[default] - gzip not enabled
[default] - lz4 is 46.67% smaller than identity - encode: 175.33 μs, decode: 8.12 μs
[default] - xz not enabled
[default] - zlib not enabled
[default] - zstd is 62.37% smaller than identity - encode: 333.07 μs, decode: 27.98 μs

*********************
Maximum Quality
*********************
[maximum] - base58 not enabled
[maximum] - bincode not enabled
[maximum] - brotli is 65.12% smaller than identity - encode: 10.75 ms, decode: 83.04 μs
[maximum] - bzip2 not enabled
[maximum] - deflate not enabled
[maximum] - gzip not enabled
[maximum] - lz4 is 46.67% smaller than identity - encode: 182.65 μs, decode: 22.83 μs
[maximum] - xz not enabled
[maximum] - zlib not enabled
[maximum] - zstd is 63.12% smaller than identity - encode: 14.32 ms, decode: 53.09 μs
```
