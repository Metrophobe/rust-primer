pub fn run() {
    println!("Hello from the print.rs file.....");
    println!("This is what was printed in placeholder {} and placeholder {}","one","two");
    println!("This is what was printed using positional placeholders {1} , {0} , {1} ,{0}","one","two"); //positional arguments 
    println!("This is what was printed using named placeholders {one} , {two}",one="one",two="two"); //named arguments 
    println!("This is what was printed using placeholder traits for Binary:{:b} , Hex: {:x} Octal: {:o}",10,10,10); //traits arguments 
    println!("This is what was printed using debug traits {:?}",(12,true,"hello")); //debug
} 