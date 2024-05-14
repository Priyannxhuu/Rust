// Array

/*
An array is a homogenous sequence of elements. Being a compound
type, it is used when the collection of values of the same type
are to be stored in a single variable. In Rust, an array can only
be of a fixed length. Like all other languages, each element in
the array is assigned an index. By default, the first element is always at index 0.
*/

// Note: By default, arrays are immutable.

#[allow(unused_variables , unused_mut)]
fn main(){
    let arr:[i32;4] = [1,2,3,4];
    println!("Array: {:?}" , arr); // Debug Formatting is used for displaying array to console
    println!("Array: {}" , arr[3]); // Getting particular index value
    println!("Array: {:?}" , arr[2]);
    println!("Array Length: {}" , arr.len()); // Array Length
    let slice_array:&[i32] = &arr[0..3]; // Array slice
    println!("Slice of an array: {:?}" , slice_array);

}

// let mut arr:[i32;4] = [1,2,3,4] 
// Now this array is mutable and its elements can be modified
