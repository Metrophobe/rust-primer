//strings can be immutable or
//strings can be growable as in other languages 


pub fn run(){

    let a = "hello this is a fixed length string";
    println!("{:?}",a);
    //a = "or is it ?";
    //println!("{:?}",a); //uncomment ... this gives an error ... as is with other variables 

    let mut b = String::from("Hello this is a growable string .... ");
    println!("{},{}",b,b.len());
    b.push('M'); //pushig a character
    println!("{},{}",b,b.len()); 
    b.push_str("etrophobe"); //pushing a string
    println!("{},{}",b,b.len()); //length
    println!("{}",b.capacity()); //capacity
    println!("{}",b.is_empty()); //is empty?
    println!("{}",b.contains("Metro")); //is empty?
    println!("{}",b.contains("metro")); //is empty? .... case sensitive .... 
    println!("{} {}",b.replace("Metro","metro"),b.len()); //replace..... 

    for elem in b.split_whitespace() {
        println!("{}",elem);
    } //splitting sentence up.... 
    
    let mut c = String::with_capacity(10); //creating string with a certain amount of capacity .... 

    c.push('a');
    c.push('b');
    c.push('c');

    assert_eq!(3,c.len()); //testing for equality 
    assert_ne!(2,c.len()); //testing for inequality

} 