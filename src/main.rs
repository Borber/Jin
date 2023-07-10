use boa_engine::{Context, Source};
use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Use as:\n./jin \"1 + 1\"");
        return;
    }

    let mut context = Context::default();
    let arg = args.get(1).unwrap().to_owned();
    let p = Path::new(&arg);

    let code = if p.exists() {
        std::fs::read_to_string(p).unwrap()
    } else {
        arg
    };

    let source = Source::from_bytes(&code);

    match context.eval(source) {
        Ok(result) => println!("{}", result.display()),
        Err(e) => println!("{e}"),
    }
}
