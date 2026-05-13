//! Benchmarks for the various formatting paths in `hex-display`.
//!
//! Run with `cargo bench --features alloc`. Each benchmark group sweeps input size
//! so we can see both per-call overhead (small inputs) and throughput (large inputs).

#![allow(
    missing_docs,
    reason = "Criterion macros generate undocumented functions"
)]

use core::fmt::Write as _;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use hex_display::{Hex, HexDisplayExt};

const SIZES: &[usize] = &[16, 256, 65_536];

fn make_input(size: usize) -> Vec<u8> {
    // Deterministic, non-uniform pattern so the optimizer can't constant-fold
    // and so byte values cover the full 0..=255 range.
    (0..size)
        .map(|i| black_box(i.to_le_bytes()[0].wrapping_mul(31) ^ 0xA5) as u8)
        .collect()
}

/// Lowercase hex via `Display` (the most common path).
fn bench_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("display_lower");
    for &size in SIZES {
        let input = make_input(size);
        // Pre-size the sink so we measure formatting, not reallocation.
        let mut sink = String::with_capacity(size * 2);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                sink.clear();
                write!(sink, "{}", Hex(black_box(input))).unwrap();
                black_box(&sink);
            });
        });
    }
    group.finish();
}

/// Uppercase hex via `{:X}`.
fn bench_upper_hex(c: &mut Criterion) {
    let mut group = c.benchmark_group("upper_hex");
    for &size in SIZES {
        let input = make_input(size);
        let mut sink = String::with_capacity(size * 2);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                sink.clear();
                write!(sink, "{:X}", Hex(black_box(input))).unwrap();
                black_box(&sink);
            });
        });
    }
    group.finish();
}

/// `hex_string()` — the realistic end-user path, including the allocation.
fn bench_hex_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("hex_string_alloc");
    for &size in SIZES {
        let input = make_input(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| black_box(black_box(input.as_slice()).hex_string()));
        });
    }
    group.finish();
}

/// Baseline: same operation, but skipping the `Hex` wrapper and using the
/// stdlib `{:02x}` per byte directly. Lets us see how much overhead the
/// wrapper itself adds vs. any future optimizations.
fn bench_baseline_stdlib(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_stdlib_per_byte");
    for &size in SIZES {
        let input = make_input(size);
        let mut sink = String::with_capacity(size * 2);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| {
                sink.clear();
                for byte in black_box(input) {
                    write!(sink, "{:02x}", byte).unwrap();
                }
                black_box(&sink);
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_display,
    bench_upper_hex,
    bench_hex_string,
    bench_baseline_stdlib
);
criterion_main!(benches);
