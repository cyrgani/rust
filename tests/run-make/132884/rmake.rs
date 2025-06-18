//@ known-bug: #132884
use run_make_support::rustc;

fn main() {
    rustc().input("132884.rs").incremental("incremental").output("o1").run();
    rustc().input("132884.rs").incremental("incremental").output("o2").run_ice();
}
