extern crate shrust;

mod spooky;

use shrust::ShellIO;

fn main() {
    if let Err(e) = std::env::var("GMO_COIN_API_KEY") {
        println!("{:?}: You need set API_KEY", e);
        std::process::exit(1);
    }

    let mut shell = spooky::init();

    shell.run_loop(&mut ShellIO::default())
}

