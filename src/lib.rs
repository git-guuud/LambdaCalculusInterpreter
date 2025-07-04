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

use std::sync::{Mutex, OnceLock};

static ROOT: OnceLock<Mutex<Box<parse::Node>>> = OnceLock::new(); //thread safety black magic

#[wasm_bindgen]
pub fn generate_tree(input: String) -> String 
{
    let tokens = match parse::tokenize(input)
    {
        Ok(tokens) => tokens,
        Err(e) => {
            return format!("Error parsing input: {}", e);
        }
    };
    let p_tokens = match parse::parenthise(tokens){
        Ok(p_tokens) => p_tokens,
        Err(e) => {
            return format!("Error parsing input: {}", e);
        }
    };
    let t_tokens = match parse::treeify(p_tokens) {
        Ok(t_tokens) => t_tokens,
        Err(e) => {
            return format!("Error parsing input: {}", e);
        }
    };
    ROOT.get_or_init(|| Mutex::new(t_tokens.clone()));
    let root = ROOT.get().unwrap();
    let mut node = root.lock().unwrap();
    *node = t_tokens;
    (*node).to_string()
}

#[wasm_bindgen]
pub fn beta_reduce_once() -> String
{
    let root = ROOT.get().unwrap();
    let mut node = root.lock().unwrap();
    parse::beta_reduce_once(&mut node);
    (*node).to_string()
}

#[wasm_bindgen]
pub fn get_token_rep() -> String
{
    let root = ROOT.get().unwrap();
    let node = root.lock().unwrap();
    let tokens = parse::detree(node.as_ref());
    let mut s = "".to_string();
    for token in tokens.iter()
    {
        s.push_str(&token.to_string());
        match token {
            parse::Token::Dot | parse::Token::Variable(_) => {
                s.push(' ');
            },
            _ => {}
        }
    }
    s
}

// fn main()
// {
//     generate_tree("f (\\x.\\y.x) (\\x.\\y.\\z.\\t.(x (y z) t)) x (\\f.\\x.f x) (\\f.\\x.f (f x))".to_string());
//     println!("{}", get_token_rep());
//     beta_reduce_once();
// }

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
