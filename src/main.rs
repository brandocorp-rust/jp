extern crate jsonpath_lib as jsonpath;

use clap::{crate_description, crate_name, crate_version, App, Arg};
use log::{debug, Level};
use serde_json::Value;
use serde_json::to_string_pretty as pretty_print;
use std::io;
use std::io::Read;

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("expr")
            .index(1))
}

fn init() {
    simple_logger::init_with_level(Level::Info).unwrap();
    debug!("Version {} initializing", crate_version!());
}

fn read_stdin() -> Value {
    let mut data = String::new();

    // Read data from stdin
    io::stdin()
        .lock()
        .read_to_string(&mut data)
        .unwrap();

    serde_json::from_str(data.as_str()).unwrap()
}

fn main() {
    init();
    let args = app().get_matches();
    let expr = args.value_of("expr").unwrap();
    let data = read_stdin();

    let mut filter = jsonpath::selector(&data);

    match filter(expr) {
        Ok(value) => {
            if value.len() == 1 {
                println!("{}", pretty_print(&value[0]).unwrap());
            } else {
                println!("{}", pretty_print(&value).unwrap());
            }
        },
        Err(error) => panic!("error: {}", &error),
    };



}
