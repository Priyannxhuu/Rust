// IF LET Expression

/*
What Is an if let Expression?
if let is a conditional expression that allows pattern matching. The block of code in the construct executes if the pattern in the condition matches with that of scrutinee expression.
*/

// Syntax
/* 
if let(val1 , val2) = match_expression{
    statement;
}
else{
    statements;
}
*/

/* Note: When it says matching of pattern, it means that the defined
pattern has the same number of values as that of the scrutinee expression.
*/

fn main(){
    let course = ("Rust" , "Beginner" , "Course");

    if let("Rust" , "Beginner" , "Course") = course{
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    }
    else{
        println!("Values Unmatched");
    }
}

// If the first and second value matched then it can guess the third value

// If the firsr matched then also it can guess remaining

//