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



// //tupple
// fn main() {
//     let employee: (&str, &str, u32) = ("Hammad", "Software Engineer", 100000);
//     println!("{} is a {} and he earns {}", employee.0, employee.1, employee.2);
// }

//tupple
fn main(){
    let emp_info:(&str, u8) = ("Hammad", 25);

    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("{} is {} years old", emp_name, emp_age);

    // destructuring

    let (employee_name, employee_age) = emp_info;
    println!("{} is {} years old", employee_name, employee_age);

}