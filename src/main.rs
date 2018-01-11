#[macro_use]
extern crate error_chain;
extern crate futures;
#[macro_use]
extern crate log;
extern crate loggerv;
extern crate petgraph;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate tokio_core;
extern crate tokio_process;
use tokio_core::reactor::Core;
use structopt::StructOpt;

use std::fs::File;
use std::io::prelude::*;

mod rule;
use rule::Rule;

#[derive(StructOpt, Debug)]
#[structopt(name = "knight")]
struct Options {
    #[structopt(short = "v", long = "verbose")] verbosity: u64,
    #[structopt(short = "f", long = "file", default_value = "build.ninja")] manifest: String,
    // #[structopt(short = "j", long = "max_jobs", default_value = "1")]
    // max_jobs: u64,
}

fn open(path: &str) -> Result<File> {
    File::open(path).chain_err(|| format!("Can't open `{}`", path))
}

fn read(path: &str) -> Result<String> {
    let mut result = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut result)?;
    Ok(result)
}

fn main() {
    if let Err(ref e) = knight() {
        use error_chain::ChainedError;

        error!("{}", e.display_chain());
        ::std::process::exit(1);
    }
}

fn knight() -> Result<()> {
    let opts = Options::from_args();
    loggerv::init_with_verbosity(opts.verbosity)?;
    debug!("{:?}", opts);

    // let manifest = read(&opts.manifest)?;

    info!("It's all good!");
    let mut core = Core::new().unwrap();
    let core_handle = &core.handle();
    let example = Rule {
        cmd: "echo".to_string(),
        args: vec!["a".to_string()],
        children: vec![
            Rule {
                cmd: "echo".to_string(),
                args: vec!["b".to_string()],

                children: vec![
                    Rule {
                        cmd: "echo".to_string(),
                        args: vec!["c".to_string()],

                        children: vec![],
                    },
                ],
            },
            Rule {
                cmd: "echo".to_string(),
                args: vec!["d".to_string()],
                children: vec![
                    Rule {
                        cmd: "echo".to_string(),
                        args: vec!["e".to_string()],
                        children: vec![],
                    },
                    Rule {
                        cmd: "echo".to_string(),
                        args: vec!["f".to_string()],

                        children: vec![],
                    },
                ],
            },
        ],
    };
    let c = (&example).run(core_handle);
    let res = core.run(c);
    info!("{:?}", res);
    Ok(())
}

error_chain! {
    foreign_links {
        Log(::log::SetLoggerError);
        Io(::std::io::Error);
    }
}
