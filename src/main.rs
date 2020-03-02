
extern crate serde;
extern crate serde_yaml;
extern crate serde_json;
#[macro_use] extern crate structopt;
#[macro_use] extern crate failure;

use std::{fs, path, io};

use failure::{Fail, ResultExt, Error};
use structopt::StructOpt;



#[derive(Debug, StructOpt)]
#[structopt(name = "yaml2json", about = "")]
struct Opt {

    #[structopt(parse(from_os_str))]
    input : path::PathBuf

}

fn load_file<P : AsRef<path::Path>>(f : &P) -> Result<serde_json::Value, Error> {
    let mut f = fs::OpenOptions::new().read(true).open(f)?;

    Ok(serde_yaml::from_reader(f)?)
}

fn main() {
    let opt = Opt::from_args();

    let json = serde_json::to_string_pretty(&load_file(&opt.input).unwrap()).unwrap();
    println!("{}", json)
}
