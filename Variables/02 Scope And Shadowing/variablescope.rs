// There are two types of variables in term of scope

// Local variable
/*
A variable within block of code that can not be accessed 
outside of that block after closing {} the variable is freed
and memory is disallocated
*/

fn main(){
    {
        let inner_variable = "Thala for a reason"; // Declaring a local variable this can not be accessed outside of this block
        println!("Inner Variable: {}" , inner_variable); // Printing local variable
    }
}

// Global Variable
/*
A variable that are declared outside any block of code and 
can be accessed withing any block are known as global variable
*/

fn main(){
    let outer_variable = "Thala for a reason"; // Declaring global variable this can be used inside code blocks or outside code blocks
    {
        let inner_variable = "Thala for a reason"; // Declaring a local variable this can not be accessed outside of this block
        println!("Inner Variable: {}" , inner_variable); // Printing local varibale
        println!("Outer Variable: {}" , outer_variable); // Printing global variable inside a code block
    }
    println!("Outer Variable: {}" , outer_variable); // Printing global variable
}