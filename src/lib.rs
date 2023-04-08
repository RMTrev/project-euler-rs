pub mod problems;
pub mod common;

#[derive(Debug)]
pub struct RunConfig {
    problem: u16,
    params: Vec<String>,
}

impl RunConfig {
    pub fn new(mut args: std::env::Args) -> Result<RunConfig, &'static str> {
        args.next();

        let problem = match args.next() {
            Some(val) => val,
            None => {
                return Err("Problem number required");
            }
        };

        let problem: u16 = match problem.parse() {
            Ok(val) => val,
            Err(_) => {
                return Err("Problem number must be an integer");
            }
        };

        let mut params: Vec<String> = Vec::new();

        loop {
            let param = match args.next() {
                Some(val) => val,
                None => {
                    break;
                }
            };

            params.push(param);
        }

        Ok(RunConfig {
            problem,
            params,
        })
    }
}
