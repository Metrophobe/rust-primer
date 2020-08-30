//Tuples group together values of different yypes 
// 12 elements limit 


pub fn run(){
    let person: (&str,&str,i8,&str) = ("Metro","phobe",29,"Robot");

    println!("{:?}",person);
    println!("{}",person.0);
}