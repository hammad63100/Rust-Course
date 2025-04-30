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
// fn main(){
//     let emp_info:(&str, u8) = ("Hammad", 25);

//     let emp_name = emp_info.0;
//     let emp_age = emp_info.1;

//     println!("{} is {} years old", emp_name, emp_age);

//     // destructuring

//     let (employee_name, employee_age) = emp_info;
//     println!("{} is {} years old", employee_name, employee_age);

// }


// function overview

// fn main(){
    
//     print_value(5);
// }

// // fn print_value(){
// //     println!("{}", 6+6);
// // }
// fn print_value(item : u8){
//     println!("{}",item);
// }


// fn main() {
//     let num1:u8 = 5;
//     let num2:u8 = 10;
//     let result:u8 = add(num1, num2);
//     println!("The sum of {} and {} is {}", num1, num2, result);
// }

// fn add(num1: u8, num2: u8) -> u8 {
//     num1 + num2
// }


// const  GOLOBAL_VARIBALE:u8 = 100;
// fn  main() {
//     let out_sidevariable = 5;

//     {
//         let inside_variable = 10;
//         println!("Prints inside variable {}", inside_variable);
//     println!("Print GOLOBAL_VARIBALE in inside fn {}",GOLOBAL_VARIBALE);
        
//     }
    
//     println!("Print Outside variable {}", out_sidevariable);
//     println!("Print GOLOBAL_VARIBALE in main fn {}",GOLOBAL_VARIBALE);
   

// }

// fn vlu(){
//     println!("Print GOLOBAL_VARIBALE fully outside {}",GOLOBAL_VARIBALE);
// }



