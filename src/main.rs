use std::env;
use std::fs;
fn main() {
    let args:Vec<String> = env::args().collect();
    let arr:Vec<String> =vec!["ssss".to_string(),"333".to_string()];
    let query=&args[1];
    let file_name=&args[2];
    println!("query={:?},file_name={:?}",query,file_name);
    let content=fs::read_to_string(file_name).expect("Something went wrong reading the file");
    println!("content:{:?}",content);

}
