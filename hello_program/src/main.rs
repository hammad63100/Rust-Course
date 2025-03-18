// fn main() {
//     println!("Hello, world!");
//     println!("Hello, Hammad");
// }


// fn main() {
    
//     let num:u32= 255;
//     print!("The number is: {}\n", num);   

//     let num:u32 = 200;

//     print!("The number is: {}", num);
// }



// &str &  string data type
//String - Dynamic Length Strings - Heap Allocated
//&str Fixed Length Strings Special memory
// fn main() {
//     // let string_latraliy = "Hello, Hammad";
//     // println!("{}", string_latraliy);


//     let mut string_object:String = String::from("Hello, Hammad");

//     string_object.push_str(" How are you?");
//     println!("{}", string_object);

// }



//tupple
fn main() {
    let employee: (&str, &str, u32) = ("Hammad", "Software Engineer", 100000);
    println!("{} is a {} and he earns {}", employee.0, employee.1, employee.2);
}