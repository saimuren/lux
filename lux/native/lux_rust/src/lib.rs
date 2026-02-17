
use rustler::{Atom, Encoder, Env, Term, NifResult};

mod atoms {
    rustler::atoms! {
        ok,
        error
    }
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

rustler::init!("Elixir.Lux.Rust", [add]);
