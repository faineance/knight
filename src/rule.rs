use futures::Future;
use futures::future;
use tokio_core::reactor::Handle;
use tokio_process::CommandExt;
use std::io;

use std::process::{Command, Output};

#[derive(Debug)]
pub struct Rule {
    pub cmd: String,
    pub args: Vec<String>,
    pub children: Vec<Rule>,
}

impl Rule {
    pub fn run<'a>(
        &'a self,
        handle: &'a Handle,
    ) -> Box<Future<Item = (Vec<Output>, Output), Error = io::Error> + 'a> {
        Box::new(
            future::join_all(self.children.iter().map(move |child| {
                // insert recursion here
                let todo = child.run(handle);
            
                Command::new(&child.cmd)
                    .args(&child.args)
                    .output_async(handle)
            })).and_then(move |x| {
                Command::new(&self.cmd)
                    .args(&self.args)
                    .output_async(handle)
                    .map(|y| (x, y))
            }),
        )
    }
}
