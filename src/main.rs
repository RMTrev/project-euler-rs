use project_euler_rs::RunConfig;
use project_euler_rs::problems::run_problem;

use std::env;
use std::process;

fn main() {
    let config = RunConfig::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        eprintln!("USAGE: cargo run problem_num [...args]");
        process::exit(1);
    });

    println!("{:?}", config);

    run_problem(config);
}
