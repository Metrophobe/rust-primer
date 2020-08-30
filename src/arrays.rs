use std::mem;

pub fn run(){
  let mut numbers: [i32;5] = [1,2,3,4,5];  //these are strongly typed and of fixed length so the number of elements has to be exactly matching the size  
  println!("{:?}",numbers); //printing of all array
  numbers[1] = 20; //modifying an element 
  println!("{:?}",numbers); //printing of all array
  println!("{:?}",numbers[2]);//printing third element 
  println!("{:?}",numbers.len());//length of array 
  //println!("{:?}",std::mem::size_of_val(&numbers));//capacity in bytes of array 
  println!("{:?}",mem::size_of_val(&numbers));//capacity in bytes of array 

  let portion: &[i32] = &numbers[1..3]; //more on these & symbols later .... 
  println!("{:?}",portion);//capacity in bytes of array 


  
}