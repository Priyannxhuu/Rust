fn main(){
    let (course , category) = ("Rust" , "Course"); //Assign multiple values
    println!("This is a {} programming {}" , course , category); // Print the value
}

// Note: If a variable is kept un-assigned or unused, you’ll get a warning. To remove such a warning write the expression #[allow(unused_variables, unused_mut)] at the start of the program code. However, it’s not a good practice to keep unassigned/unused variables.