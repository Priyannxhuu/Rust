// IF expression

/*
There can be multiple conditional constructs using an if statement.

If expression

If…else expression

If…else if…else expression

Nested if expression

Shorthand if expression
*/

// IF

// Syntax
/*
if condition{
    statement;
}
*/

fn main(){
    let learn_lang = "Rust";
    if learn_lang == "Rust"{
        println!("You are learning Rust");
    }
}

// IF ELSE

// Syntax
/*
if condition{
    statement;
}
else{
    statements;
}
*/

fn main(){
    let learn_lang = "Rust";
    if learn_lang == "Rust"{
        println!("You are learning Rust");
    }
    else{
        println!("You are learning some other language");
    }
}

// IF ELSE IF ELSE

// Syntax
/*
if condition{
    statement;
}
else if condition{
    statements;
}
else{
    statements;
}
*/

fn main(){
    let learn_lang = "Rust";
    if learn_lang == "Rust" {
        println!("You are learning Rust");
    }
    else if learn_lang == "Python" {
        println!("You are learning Python");
    }
    else{
        println!("You are learning some other language");
    }
}

// NESTED IF

// Syntax
/*
if condition{
    if condition{
    statements;
    }
}
*/

fn main(){
    let learn_lang1 = "Rust";
    let learn_lang2 = "Python";
    if learn_lang1 == "Rust"{
        if learn_lang2 == "Python"{
            println!("You are learning Rust and Python");
        }
    }
    else{
        println!("You are learning some other language");
    }
}

// SHORTHAND IF
// Instead of writing a lengthy if-else construct, we can use a shorthand if.

// syntax let x = if (condition){statement} else {statement}

fn main(){
    let learn_lang = "Rust";
    let res = if learn_lang == "Rust" {"You are learning Rust"} else{"You are learning Some other language"};
    println!("{}",res);
}