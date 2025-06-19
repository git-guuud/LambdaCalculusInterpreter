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




use std::fs;
use std::fmt;

mod parse;

#[derive(PartialEq, Copy, Clone)]
enum Expr
{
    Name(&'static str),
    Value(i64),
}

impl Expr
{
    fn make_sub(&self, expr1: Expr, expr2: Expr) -> Expr
    {
        match expr1
        {
            Expr::Name(_) if expr1==*self => expr2,
            _ => *self,
        }
    }
}

impl fmt::Display for Expr
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        match self
        {
            Expr::Name(x) => write!(f, "{}", x),
            Expr::Value(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Copy, Clone)]
struct Function
{
    variable: Expr,
    body: Expr,
}

impl fmt::Display for Function
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "λ{} . {}", self.variable, self.body)
    }
}

#[derive(Copy, Clone)]
struct Application
{
    arg: Expr,
    func: Function,
}

impl fmt::Display for Application
{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "( {} ){}", self.func, self.arg)
    }
}

impl Application
{
    fn evaluate(&self) -> Expr 
    {
        self.func.body.make_sub(self.func.variable, self.arg)
    }
}


fn main() {
    let file_path = "code.txt";
    // let contents = fs::read_to_string(file_path).expect("file not readable");

    // println!("{}", contents);
    
    // let one = Expr::Value(1);
    // let x = Expr::Name("x");

    // let identity = Function{variable:x, body:x};
    // let o = Function{variable:x, body:one};

    // let res = Application{arg:one, func:o};

    // println!("{}", res);
    // println!("{}", res.evaluate());

    let tokens = parse::tokenize(file_path).expect("Parsing failed");
    let p_tokens = parse::parenthise(tokens).unwrap();
    for token in p_tokens.iter() {
        print!("{}", token);
    }

    let t_tokens = parse::treeify(p_tokens).expect("Failed to form tree");
    println!("{}", t_tokens);

}