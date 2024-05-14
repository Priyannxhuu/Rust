// Constant Variable

/*Constant variables are ones that are declared constant
throughout the program scope, meaning, their value cannot be
modified. They can be defined in global and local scope.*/

// Syntax
// const varname = value;
// const varname : type = value;

// variable name should be uppercase

const ID_1 : i32 = 4; // Define the global variable
fn main(){
    const ID_2 : i32 = 7; // Define the local variable
    println!("Id : {}" , ID_1); // Print the global variable
    println!("Id : {}" , ID_2); // Print the local variable
}

// const variable can not be mutable