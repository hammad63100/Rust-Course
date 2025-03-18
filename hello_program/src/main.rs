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
fn main() {
    // let string_latraliy = "Hello, Hammad";
    // println!("{}", string_latraliy);


    let mut string_object:String = String::from("Hello, Hammad");

    string_object.push_str(" How are you?");
    println!("{}", string_object);

}