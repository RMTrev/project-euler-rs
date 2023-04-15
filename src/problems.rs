mod p001; mod p002; mod p003; mod p004; mod p005;
mod p006; mod p007; mod p008; mod p009; mod p010;
mod p011; mod p012;

use super::RunConfig;

type Action = fn(RunConfig) -> ();

static PROBLEMS: &'static [Action] = &[
    p001::run,
    p002::run,
    p003::run,
    p004::run,
    p005::run,
    p006::run,
    p007::run,
    p008::run,
    p009::run,
    p010::run,
    p011::run,
    p012::run,
];

pub fn run_problem(config: RunConfig) {
    match PROBLEMS.get((config.problem as usize) - 1) {
        Some(act) => act(config),
        None => {
            println!("Problem {} not implemented", config.problem);
            ()
        }
    }
}
