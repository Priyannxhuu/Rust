// Problem Statement

/*
Write a code which will check whether _a given integer number is even or odd. If the number is even print
“Number a is even” and if it’s an odd print “Number a is odd”.
*/

fn main(){
    let _a = 3;
    if _a%2 == 0 {
        println!("Number {} is even" , _a);
    }
    else{
        println!("Number {} is odd" , _a);
    }
}