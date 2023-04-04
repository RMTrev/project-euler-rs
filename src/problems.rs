mod p001;
mod p002;
mod p003;

use super::RunConfig;

pub fn run_problem(config: RunConfig) {
    match config.problem {
        1 => p001::run(config),
        2 => p002::run(config),
        3 => p003::run(config),
        _ => {
            println!("Problem {} not implemented", config.problem);
            ()
        }
    }
}
