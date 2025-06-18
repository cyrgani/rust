//@ known-bug: #140793
use run_make_support::rustc;

fn main() {
    rustc().input("140793.rs").incremental("incremental").run();
    rustc().input("140793.rs").arg("--cfg=a").incremental("incremental").run_ice();
}
