
//Variables in rus hold primitive data or references to data 
//Variables are readonly by default (so you cannot change them once assigned)
//Scoping is similar to java / c#
//There are vars , conts and mutable vars (can be changed)


pub fn run(){
    let name = "Metro";
    let mut age = 29; //making variable mutable.....
    println!("My name is {} and my age is {}",name,age);
    age = 30;
    println!("My name is {} and my age is {}",name,age);

    //These are constants and you have to assign a type to them to be able to use them.....
    const ID: bool = true;

    //You can also assign multiple fields at the same time 
    let (is_alive,is_kicking) = ("Alive","Kicking");
    println!("{} is {}",name,is_alive);
    println!("{} is {} and its {}",name,is_kicking,ID);



    

}