#![allow(dead_code)]

use serde::Deserialize;
use ts_rs::TS;

#[derive(Deserialize, TS)]
struct A {
    a: i32,
    b: i32,
}

#[derive(Deserialize, TS)]
struct B {
    a: String,
    b: String,
}

#[derive(Deserialize, TS)]
#[serde(untagged)]
enum C {
    A(A),
    B(B),
}

#[derive(TS)]
struct D {
    #[ts(flatten)]
    c: C,
    d: i32,
}

#[test]
fn test_decl() {
    assert_eq!(D::decl(), "type D = { d: number, } & (C)");
}
