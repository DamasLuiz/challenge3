/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.
 
Cast the i32 to an i16 integer and assign the result
to a separate variable.
 
Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.
 
Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.
 
Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.
 
Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.
 
Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.
 
Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/
#![allow(unused_variables)]

fn main() {
    let a_number: i32 = 1_337;
    let another_number = a_number as i16;

    let float_number: f64 = 6.2639017267633;
    println!("{:.3}", float_number);

    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee: bool = with_milk == true && with_sugar == true;
    let is_acceptable_coffee: bool = with_milk == true || with_sugar == true;

    let array_i8: [i8; 4] = [7, 1, 21, 14];
    let the_tuple = (a_number, another_number, is_my_type_of_coffee, array_i8);

    println!("\n\nThe array is:\n{:?}", array_i8);
    println!("{:#?}", array_i8);
    dbg!(array_i8);
    println!("\n\nThe tuple is:\n{:?}", the_tuple);
    println!("{:#?}", the_tuple);
    dbg!(the_tuple);
}
