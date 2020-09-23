// Copyright 2020 Riad S. Wahby <rsw@cs.stanford.edu>
//
// This file is part of fffft.
//
// Licensed under the Apache License, Version 2.0 (see
// LICENSE or https://www.apache.org/licenses/LICENSE-2.0).
// This file may not be copied, modified, or distributed
// except according to those terms.

use super::FieldFFT;
use crate::tests::ft::*;

use ff::Field;
use test::{black_box, Bencher};

const LOG_BENCH_SIZE: usize = 20;

#[bench]
fn roots_of_unity_ser(b: &mut Bencher) {
    rou_bench(b, super::roots_of_unity_ser);
}

#[bench]
fn roots_of_unity_par(b: &mut Bencher) {
    rou_bench(b, super::roots_of_unity);
}

fn rou_bench<F>(b: &mut Bencher, mut f: F)
where
    F: FnMut(Ft, u32, u32) -> Vec<Ft>,
{
    let root = <Ft as FieldFFT>::root_of_unity();
    let s = <Ft as FieldFFT>::S;

    b.iter(|| {
        black_box(f(root, LOG_BENCH_SIZE as u32, s));
    });
}

#[bench]
fn io_help_ser(b: &mut Bencher) {
    fft_bench(b, super::io_help_ser);
}

#[bench]
fn io_help_par(b: &mut Bencher) {
    fft_bench(b, super::io_help);
}

#[bench]
fn oi_help_ser(b: &mut Bencher) {
    fft_bench(b, super::oi_help_ser);
}

#[bench]
fn oi_help_par(b: &mut Bencher) {
    fft_bench(b, super::oi_help);
}

fn fft_bench<F>(b: &mut Bencher, mut f: F)
where
    F: FnMut(&mut [Ft], Ft, u32, u32),
{
    use std::iter::repeat_with;

    let mut rng = rand::thread_rng();
    let mut xi: Vec<Ft> = repeat_with(|| Ft::random(&mut rng))
        .take(1 << LOG_BENCH_SIZE)
        .collect();
    let root = <Ft as FieldFFT>::root_of_unity();
    let s = <Ft as FieldFFT>::S;

    b.iter(|| {
        f(xi.as_mut(), root, LOG_BENCH_SIZE as u32, s);
    });
}
