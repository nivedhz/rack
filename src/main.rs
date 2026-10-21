use rack::run;

// The top level error propagator, this is unlikely to change but is unnecessary as it lib can be
// merged into main but we will not merge it.
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
