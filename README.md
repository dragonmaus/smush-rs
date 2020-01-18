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
use std::time::Instant;

const TEST_DATA: &[u8] = include_bytes!("../src/ipsum.txt");

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
                encode_elapsed.as_secs_f32(),
                decode_elapsed.as_secs_f32()
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
[level1] - base58 is 36.57% larger than identity - encode: 43.33 ms, decode: 14.23 ms
[level1] - bincode is 0.14% larger than identity - encode: 6.10 μs, decode: 9.20 μs
[level1] - brotli is 53.33% smaller than identity - encode: 165.70 μs, decode: 172.00 μs
[level1] - deflate is 53.08% smaller than identity - encode: 174.30 μs, decode: 64.40 μs
[level1] - gzip is 52.76% smaller than identity - encode: 138.50 μs, decode: 53.60 μs
[level1] - lz4 is 40.96% smaller than identity - encode: 100.00 μs, decode: 49.10 μs
[level1] - xz is 60.63% smaller than identity - encode: 1.58 ms, decode: 205.90 μs
[level1] - zlib is 52.97% smaller than identity - encode: 127.50 μs, decode: 57.40 μs
[level1] - zstd is 59.80% smaller than identity - encode: 145.80 μs, decode: 59.70 μs
*********************
Default Quality
*********************
[default] - base58 is 36.57% larger than identity - encode: 42.93 ms, decode: 13.79 ms
[default] - bincode is 0.14% larger than identity - encode: 3.70 μs, decode: 9.10 μs
[default] - brotli is 63.32% smaller than identity - encode: 1.37 ms, decode: 92.30 μs
[default] - deflate is 63.12% smaller than identity - encode: 217.50 μs, decode: 47.60 μs
[default] - gzip is 62.80% smaller than identity - encode: 201.00 μs, decode: 61.20 μs
[default] - lz4 is 46.67% smaller than identity - encode: 267.80 μs, decode: 33.60 μs
[default] - xz is 62.06% smaller than identity - encode: 4.31 ms, decode: 258.40 μs
[default] - zlib is 63.01% smaller than identity - encode: 170.00 μs, decode: 57.80 μs
[default] - zstd is 62.30% smaller than identity - encode: 566.00 μs, decode: 80.10 μs
*********************
Maximum Quality
*********************
[maximum] - base58 is 36.57% larger than identity - encode: 43.50 ms, decode: 13.85 ms
[maximum] - bincode is 0.14% larger than identity - encode: 6.60 μs, decode: 8.60 μs
[maximum] - brotli is 65.12% smaller than identity - encode: 8.82 ms, decode: 170.70 μs
[maximum] - deflate is 63.12% smaller than identity - encode: 187.40 μs, decode: 66.40 μs
[maximum] - gzip is 62.80% smaller than identity - encode: 172.00 μs, decode: 55.90 μs
[maximum] - lz4 is 46.67% smaller than identity - encode: 350.10 μs, decode: 39.70 μs
[maximum] - xz is 62.06% smaller than identity - encode: 10.91 ms, decode: 895.00 μs
[maximum] - zlib is 63.01% smaller than identity - encode: 166.70 μs, decode: 46.50 μs
[maximum] - zstd is 63.12% smaller than identity - encode: 12.99 ms, decode: 124.50 μs
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
[level1] - brotli is 53.33% smaller than identity - encode: 302.30 μs, decode: 209.20 μs
[level1] - deflate not enabled
[level1] - gzip not enabled
[level1] - lz4 is 40.96% smaller than identity - encode: 119.40 μs, decode: 53.40 μs
[level1] - xz not enabled
[level1] - zlib not enabled
[level1] - zstd is 59.80% smaller than identity - encode: 205.30 μs, decode: 96.90 μs
*********************
Default Quality
*********************
[default] - base58 not enabled
[default] - bincode not enabled
[default] - brotli is 63.32% smaller than identity - encode: 1.57 ms, decode: 80.20 μs
[default] - deflate not enabled
[default] - gzip not enabled
[default] - lz4 is 46.67% smaller than identity - encode: 241.80 μs, decode: 74.70 μs
[default] - xz not enabled
[default] - zlib not enabled
[default] - zstd is 62.30% smaller than identity - encode: 446.50 μs, decode: 73.50 μs
*********************
Maximum Quality
*********************
[maximum] - base58 not enabled
[maximum] - bincode not enabled
[maximum] - brotli is 65.12% smaller than identity - encode: 8.84 ms, decode: 111.80 μs
[maximum] - deflate not enabled
[maximum] - gzip not enabled
[maximum] - lz4 is 46.67% smaller than identity - encode: 225.90 μs, decode: 78.30 μs
[maximum] - xz not enabled
[maximum] - zlib not enabled
[maximum] - zstd is 63.12% smaller than identity - encode: 12.53 ms, decode: 110.60 μs
```
