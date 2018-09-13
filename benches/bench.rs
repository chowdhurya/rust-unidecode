#![feature(test)]

extern crate deunicode;
extern crate test;
use test::Bencher;
use deunicode::*;

#[bench]
fn bench_str(b: &mut Bencher) {
    b.iter(|| {
        test::black_box("hęllo world — げんまい茶茶茶! 🦄☣…").to_ascii_lossy().len()
    })
}

#[bench]
fn bench_ascii(b: &mut Bencher) {
    b.iter(|| {
        test::black_box("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.").to_ascii_lossy().len()
    })
}
