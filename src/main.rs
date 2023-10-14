//project will recongnise this file
mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;

const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}!", OUR_COURSE);
    // Stack
    let x: i32 = 4;
    println!("x is {}", x);

    // For Loop
    for i in 0..=x {
        if i != 4 {
            print!("{}, ", i)
        } else {
            println!("{}", i)
        }
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but is now {} ", z);

    let freezing_temp: f64 = -2.4;
    println!("The freezing temp is {} ", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("The is_zero_remainder is {} ", is_zero_remainder);

    //use single quotes for
    let my_char: char = 'z';
    println!("The my_char is {} ", my_char);

    let my_emoji_char: char = '😂';
    println!("The my_emoji_char is {} ", my_emoji_char);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("The my_ints is {:#?} ", my_floats);

    let my_float_new: [f32; 10] = my_floats.map(|n: f32| n + 2.0);
    println!("The my_float_new is {:#?} ", my_float_new);

    let name: &str = "Sghaud";
    println!("Name: {:?} ", name);

    let name: &str = "Chanel";
    println!("Name: {:?} ", name);

    let dynamic_name: String = String::from("Chanel Moreno");
    println!("dynamic_name: {:?} ", dynamic_name);
    println!("dynamic_name: {:?} ", &dynamic_name);
}
