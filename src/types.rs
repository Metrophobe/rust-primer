
//Common types include 
//u8,16,32,64,128 - unsigned integer
//i8,16,32,64,128 - integer
//f32,f64 - float 
//bool - boolean 
//char - character 
//strings - see example 4 
//tuples - lists 
//arrays - arrays (fixed length)
//vectors - resizable arrays 
//rust is strongly typed however the compiler typically can infer the types for you ...

pub fn run(){

    let x = 1; //i32

    let y = 3.5; //f64

    let z:f32 = 3.4; //explicit type 

    let is_active:bool = true;

    let letter:char = 'p';

    let smiley = '\u{1f600}';
    
    //max size 
    println!("Max i32 {}",std::i32::MAX);
    println!("Max i64 {}",std::i64::MAX);
    println!("{:?}",(x,y,z,is_active,letter,smiley));




}