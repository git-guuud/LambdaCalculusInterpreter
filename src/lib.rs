//      __           __      __    
//     / /__ ___ _  / /  ___/ /__ _
//    / / _ `/  ' \/ _ \/ _  / _ `/
//   /_/\_,_/_/_/_/_.__/\_,_/\_,_/ 
//            __         __        
//  _______ _/ /_____ __/ /_ _____ 
// / __/ _ `/ / __/ // / / // (_-< 
// \__/\_,_/_/\__/\_,_/_/\_,_/___/ 
// 
// 
mod parse;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_tree(input: String) -> String 
{
    let tokens = parse::tokenize(input).expect("Parsing failed");
    let p_tokens = parse::parenthise(tokens).unwrap();
    let t_tokens = parse::treeify(p_tokens).expect("Failed to form tree");
    t_tokens.to_string()
}


// use rocket::Rocket;
// use std::fs::OpenOptions;
// use std::io::Write;

// #[macro_use] extern crate rocket;

// #[get("/api")]
// fn index() -> String {
//     // let mut code = OpenOptions::new()
//     //     .read(true)
//     //     .append(true)
//     //     .create(true)
//     //     .open("code.txt")
//     //     .expect("unable to create code file");
//     // code.set_len(0);
//     // code.write("(\\x.x)w".to_string().as_bytes()).expect("Unable to write code");

//     // let mut out_json = OpenOptions::new()
//     //     .write(true)
//     //     .truncate(true)
//     //     .create(true)
//     //     .open("out.json")
//     //     .expect("unable to create json file");


//     let file_path = "code.txt";


//     let tokens = parse::tokenize(file_path).expect("Parsing failed");
//     let p_tokens = parse::parenthise(tokens).unwrap();
//     // for token in p_tokens.iter() {
//     //     print!("{}", token);
//     // }

//     // println!();

//     let t_tokens = parse::treeify(p_tokens).expect("Failed to form tree");
//     // out_json.write(t_tokens.to_string().as_bytes()).expect("unable to write to json file");
//     t_tokens.to_string()
// }

// #[launch]
// fn rocket() -> Rocket<rocket::Build>
// {
//     rocket::build().mount("/", routes![index])
// }



// fn main() {
//     let file_path = "code.txt";
//     let contents = fs::read_to_string(file_path).expect("File no readable");

//     let tokens = parse::tokenize(contents).expect("Parsing failed");
//     let mut p_tokens = parse::parenthise(tokens).unwrap();
//     // for token in p_tokens.iter() {
//     //     print!("{}", token);
//     // }

//     // println!();

//     let mut t_tokens = parse::treeify(p_tokens).expect("Failed to form tree");
//     println!("{}", t_tokens);
//     // p_tokens =parse::detree(t_tokens.as_ref());
//     // for token in p_tokens.iter() {
//     //     print!("{}", token);
//     // }

//     // println!();

//     // for _ in 0..10 
//     // {
//     //     parse::beta_reduce_once(t_tokens.as_mut());
//     //     p_tokens =parse::detree(t_tokens.as_ref());
//     //     for token in p_tokens.iter() {
//     //         print!("{}", token);
//     //     }

//     //     println!();
//     // }
// }
