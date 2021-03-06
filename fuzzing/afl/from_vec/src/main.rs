#[macro_use]
extern crate afl;
extern crate graph_harness;
use graph_harness::*;

fn main() {
    fuzz!(|data: FromVecHarnessParams| {
        // We ignore this error because we execute only the fuzzing to find
        // the panic situations that are NOT just errors, but unhandled errors.
        let _ = from_vec_harness(data);
    });
}
