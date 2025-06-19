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




fn main() {
    let file_path = "code.txt";

    
    let tokens = parse::tokenize(file_path).expect("Parsing failed");
    let mut p_tokens = parse::parenthise(tokens).unwrap();
    // for token in p_tokens.iter() {
    //     print!("{}", token);
    // }

    // println!();

    let mut t_tokens = parse::treeify(p_tokens).expect("Failed to form tree");
    // println!("{}", t_tokens);
    p_tokens =parse::detree(t_tokens.as_ref());
    for token in p_tokens.iter() {
        print!("{}", token);
    }

    println!();

    for _ in 0..10 
    {
        parse::beta_reduce_once(t_tokens.as_mut());
        p_tokens =parse::detree(t_tokens.as_ref());
        for token in p_tokens.iter() {
            print!("{}", token);
        }

        println!();
    }
}