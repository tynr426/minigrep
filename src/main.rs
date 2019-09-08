use std::env;
use std::process;
mod lib;
use lib::Config;
fn main() {
    //第一种方法
    //let args: Vec<String> = env::args().collect();
    // let result = Config::new(&args);
    //第二种
    let result=Config::new(env::args());
    let config = result.unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("query={:?},file_name={:?}", config.query, config.filename);
    if let Err(e) =Config::run(&config){
        println!("Application error: {}", e);

        process::exit(1);
    };
}

