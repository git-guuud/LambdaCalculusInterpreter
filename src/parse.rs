// use std::fs;
use std::fmt;
use std::vec::Vec;

#[derive(PartialEq, Clone)]
pub enum Token 
{
    Lambda,
    Variable(String),
    Dot,
    OpenParen,
    CloseParen,
}

impl fmt::Display for Token
{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Token::Lambda => write!(f, "λ"),
            Token::Variable(var) => write!(f, "{}", var),
            Token::Dot => write!(f, "."),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Node
{
    pub token: Token,
    pub children: Vec<Box<Node>>,
    // pub parent: Option<Rc<Node>>,
}

impl fmt::Display for Node 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        let mut s = "{".to_string();

        s = format!("{}\"name\":\"{}\", \"children\":[", s, self.token);
        if self.children.len() > 0 
        {
            for child in &self.children 
            {
                s = format!("{}{},", s, child);
            }
            s.pop();
        }
        write!(f,"{}]}}",s)
    }
}


pub fn tokenize(contents: String) -> Result<Vec<Token>, String>
{
    // This function will parse the input string and return an Expr
    // For now, we will return an error to indicate that this is not implemented
    
    
    let mut tokens = Vec::new();

    let mut current_token = String::new();

    for c in contents.chars() 
    {
        match c 
        {
            'λ' | '\\' =>
            {
                if !current_token.is_empty()
                {
                    tokens.push(Token::Variable(current_token.clone()));
                    current_token.clear();
                }
                tokens.push(Token::Lambda);
            }
            '.' => 
            {
                if !current_token.is_empty()
                {
                    tokens.push(Token::Variable(current_token.clone()));
                    current_token.clear();
                }
                tokens.push(Token::Dot);
            }
            '(' => 
            {
                if !current_token.is_empty()
                {
                    tokens.push(Token::Variable(current_token.clone()));
                    current_token.clear();
                }
                tokens.push(Token::OpenParen);
            }
            ')' => 
            {
                if !current_token.is_empty()
                {
                    tokens.push(Token::Variable(current_token.clone()));
                    current_token.clear();
                }
                tokens.push(Token::CloseParen);
            }
            ' ' | '\n' | '\t' => 
            {
                if !current_token.is_empty()
                {
                    tokens.push(Token::Variable(current_token.clone()));
                    current_token.clear();
                }
            }
            _ => 
            {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty() 
    {
        tokens.push(Token::Variable(current_token));
    }

    return Ok(tokens);
}


pub fn parenthise(tokens: Vec<Token>) -> Result<Vec<Token>, String>
{
    let mut output = Vec::new();
    let mut paren_count = 0;
    let mut last_token = None;
    let mut extra_paren = vec![0];
    for token in tokens
    {
        if (last_token == Some(Token::Dot) && token != Token::OpenParen) 
            || (token == Token::Lambda && last_token != Some(Token::OpenParen))
        {
            extra_paren.last_mut().map(|x| *x += 1);
            output.push(Token::OpenParen);
        }
        else if token == Token::OpenParen
        {
            paren_count += 1;
            extra_paren.push(0);
        }
        
        if token == Token::CloseParen 
        {
            paren_count -= 1;
            if paren_count < 0
            {
                return Err("Unmatched closing parenthesis".to_string());
            }
            let count = extra_paren.pop().unwrap() + 1;
            for _ in 0..count
            {
                output.push(Token::CloseParen);
            }
        }
        else 
        {
            output.push(token.clone());
        }

        last_token = Some(token);
    }

    if paren_count > 0 
    {
        return Err("Unmatched opening parenthesis".to_string());
    }
    
    let count = extra_paren.pop().unwrap();
    for _ in 0..count
    {
        output.push(Token::CloseParen);
    } 

    return Ok(output);
}

pub fn treeify(tokens: Vec<Token>) -> Result<Box<Node>, String>
{
    let mut n = tokens.len();
    if n == 0 
    {
        return Err("No tokens to parse.".to_string());
    }
    let mut i = 0;
    if tokens[i] == Token::OpenParen && tokens[n-1] == Token::CloseParen 
    {
        let mut paren_count = 0;
        let mut min_paren = n;
        for token in &tokens
        {
            match token 
            {
                Token::OpenParen => 
                {
                    if paren_count!=i {min_paren = if paren_count<min_paren {paren_count} else {min_paren};}
                    paren_count+=1;
                }
                Token::CloseParen => paren_count -=1,
                _ => {min_paren = if paren_count<min_paren {paren_count} else {min_paren};}
            }
            i+=1;
            // if min_paren == 0 {break};
        }
        i=min_paren;
        n-=min_paren;
    }

    if i==n 
    {
        return Ok(Box::new(Node{
            token: Token::OpenParen,
            children: Vec::new(),
        }))
    }

    if i==n-1
    {
        return Ok(Box::new(Node{
            token: tokens[i].clone(),
            children: Vec::new(),
        }))
    }

    let mut root = Box::new(
        Node{token:Token::OpenParen, 
            children:Vec::new()}
    );
    if tokens[i] == Token::Lambda
    {
        i+=1;
        root = Box::new(
            Node{token:Token::Lambda, children:Vec::new()}
        );
        let mut var = Vec::new();
        let mut num_paren = 0;
        loop {
            if tokens[i] == Token::OpenParen
            {
                num_paren+=1;
            }
            else if tokens[i] == Token::CloseParen
            {
                num_paren-=1;
            }
            var.push(tokens[i].clone());
            i+=1;
            if num_paren==0
            {
                break;
            }
            if i==n
            {
                return Err("Unexpected end to tokens.".to_string());
            }
        }

        if tokens[i]!=Token::Dot 
        {
            return Err("Expected dot not found.".to_string());
        }
        else 
        {
            i+=1;
        }

        let mut body = Vec::new();
        loop {
            if tokens[i] == Token::OpenParen
            {
                num_paren+=1;
            }
            else if tokens[i] == Token::CloseParen
            {
                num_paren-=1;
            }
            body.push(tokens[i].clone());
            i+=1;
            if num_paren==0
            {
                break;
            }
            if i==n
            {
                return Err("Unexpected end to tokens.".to_string());
            }
        }

        root.children.push(match treeify(var) {
            Ok(node) => node,
            Err(e) => return Err(e),
        });
        root.children.push(match treeify(body) {
            Ok(node) => node,
            Err(e) => return Err(e),
        });
    }
    else if tokens[i] == Token::OpenParen 
    {
        let mut func = Vec::new();
        let mut num_paren = 0;
        loop {
            if tokens[i] == Token::OpenParen
            {
                num_paren+=1;
            }
            else if tokens[i] == Token::CloseParen
            {
                num_paren-=1;
            }
            func.push(tokens[i].clone());
            i+=1;
            if num_paren==0
            {
                break;
            }
            if i==n
            {
                return Err("Unexpected end to tokens.".to_string());
            }
        }

        root.children.push(match treeify(func) {
            Ok(node) => node,
            Err(e) => return Err(e),
        });
    }
    let mut num_paren = 0;
    if i==n {return Ok(root)}
    'args: loop 
    {
        let mut arg = Vec::new();
        'arg: loop {
            if tokens[i] == Token::OpenParen
            {
                num_paren+=1;
            }
            else if tokens[i] == Token::CloseParen
            {
                num_paren-=1;
            }
            arg.push(tokens[i].clone());
            i+=1;
            if num_paren<=0
            {
                break 'arg;
            }
            if i==n
            {
                return Err("Unexpected end to tokens.".to_string());
            }
        }

        root.children.push(match treeify(arg) {
            Ok(node) => node,
            Err(e) => return Err(e),
        });
        if i==n {break 'args;}
    }
    

    return Ok(root);
}

pub fn detree(root: &Node) -> Vec<Token> 
{
    let mut output = Vec::new();
    if root.token == Token::OpenParen 
    {
        output.push(Token::OpenParen);
        for child in &root.children 
        {
            output.append(&mut detree(child.as_ref()));
        }
        output.push(Token::CloseParen);
    }
    else if root.token == Token::Lambda
    {
        output.push(Token::OpenParen);
        output.push(Token::Lambda);
        let var = root.children[0].clone();
        let body = root.children[1].clone();
        output.append(&mut detree(var.as_ref()));
        output.push(Token::Dot);
        // output.push(Token::OpenParen);
        output.append(&mut detree(body.as_ref()));
        // output.push(Token::CloseParen);
        output.push(Token::CloseParen);
    }
    else
    {
        output.push(root.token.clone());
    }

    return output;
}

pub fn beta_reduce_once(root: &mut Node) -> Result<(), ()>
{
    match root.token 
    {
        Token::Variable(_) => return Err(()),
        Token::Lambda => {
            return beta_reduce_once(root.children[1].as_mut());
        },
        _ => {}
    };
    if root.children.len() == 1 && root.token == Token::OpenParen
    {
        match beta_reduce_once(root.children[0].as_mut())
        {
            Ok(_) => return Ok(()),
            Err(_) => {
                root.token = root.children[0].token.clone();
                root.children = root.children[0].children.clone();
                return Ok(());
            }
        };
        
    }
    let mut lambda_found = false;
    for i in 0..root.children.len()-1
    {
        if root.children[i].token == Token::Lambda
        {
            lambda_found = true;
            if beta_reduce_once(root.children[i].children[1].as_mut()).is_err()
            {
                let mut body = root.children[i].children.remove(1);
                replace(root.children[i].children[0].clone(), root.children.remove(i+1), body.as_mut());
                root.children[i] = body;
            }
            return Ok(());
        }
        else if root.children[i].token == Token::OpenParen 
        {
            return beta_reduce_once(root.children[i].as_mut());
        }
    }

    if !lambda_found
    {
        for child in &mut root.children
        {
            if child.token == Token::OpenParen
            {
                return beta_reduce_once(child.as_mut());
            }
            else if child.token == Token::Lambda
            {
                return beta_reduce_once(child.children[1].as_mut());
            }
        }
    }
    return Err(());
}

fn replace(var: Box<Node>, arg: Box<Node>, body: &mut Node) -> () 
{
    if *body == *var
    {
        *body = *(arg.clone());
    }
    else
    {
        for child in &mut body.children
        {
            replace(var.clone(), arg.clone(), child);
        }
    }
}