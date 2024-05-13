/* Note: Variable shadowing is a technique in which a variable
declared within a certain scope has the same name as a variable
declared in an outer scope. This is also known as masking. This
outer variable is said to be shadowed by the inner variable,
while the inner variable is said to mask the outer variable.
*/

fn main(){
    let outer = 7; // Defining global variable
    {
        let inner = 8; // Defining local variable
        println!("Block inner variable {}" , inner); // Printing local variable
        let outer = 9; // Masking global variable inside code block
        println!("Block outer variable {}" , outer); // Printing masked variable
    }
    println!("Outer variable {}" , outer); // Printing global variable outside block
}