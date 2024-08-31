#![feature(test)]

extern crate test;

use std::sync::LazyLock;
use test::bench::Bencher;

#[allow(unused)]
struct S {
    a: String,
    b: usize,
    c: Vec<T>,
}

#[derive(Clone)]
struct SP {
    a: String,
    b: usize,
    c: Vec<T>,
}

#[allow(unused)]
#[derive(Clone)]
struct T {
    d: usize,
    e: String,
}

static CASES: LazyLock<[SP; 100]> = LazyLock::new(|| {
    use rand::{thread_rng, Rng};
    
    fn random_string() -> String {
        use rand::distributions::{DistString, Alphanumeric};
        let len = thread_rng().gen_range(0..100);
        Alphanumeric.sample_string(&mut thread_rng(), len)
    }

    fn random_uint() -> usize {
        thread_rng().gen::<usize>()
    }

    (0..100).map(|_| SP {
        a: random_string(),
        b: random_uint(),
        c: (0..100).map(|_| T {
            d: random_uint(),
            e: random_string()
        }).collect()
    }).collect::<Vec<_>>().try_into().ok().unwrap()
});

#[bench]
fn transfer_by_hand(b: &mut Bencher) {
    test::black_box(&*CASES);
    b.iter(|| -> [S; 100] {
        CASES.clone().map(|sp| test::black_box(
            S { a: sp.a, b: sp.b, c: sp.c }
        ))
    })
}

#[bench]
fn transfer_by_mem_transmute(b: &mut Bencher) {
    test::black_box(&*CASES);
    b.iter(|| -> [S; 100] {
        CASES.clone().map(|sp| test::black_box(
            unsafe {std::mem::transmute(sp)}
        ))
    })
}
