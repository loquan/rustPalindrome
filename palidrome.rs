use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
   
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => print!("load{}  ", display),
		
    }
	
	//remove space
	let q= s.replace(" ", "");	
	
	let mut vec = Vec::new();
	for word in q.split('\n') {
	    vec.push(word);
    }

	
	let length=vec.len()-1;
	
    let mut rev_vec = Vec::new();	

	//create the reverse string
	for x in 0..length  {
		let mut rev_s = String::new();
		
		let mut count=0;
		for c in vec[x].chars().rev() {
			if count==0
			{
			  count = 1;
			}
			else
			{
			  
			  rev_s.push(c);
			}	
			
		}
		 
		count = 0;
		
		for c in vec[x].chars().rev() {
			if count==0
			{
			  count = 1;
			  rev_s.push(c);
			}
			
		}
	    //push the revers string into a vector
		rev_vec.push(rev_s);
	
		
	}
	println!("");
	for x in 0..length  {
		
	   //sort each string	
       let mut sort_vect=Vec::new(); 
	   for c in vec[x].chars() {
			sort_vect.push(c);
	   }
		 
	    
	   
		// tried using print! some how it is not working with the if statement
		 if rev_vec[x]==vec[x]
		{
		  println!("av|");
		}
		else{
		  println!("nav|"); 
		}
		
		 sort_vect.sort();
		 for c in sort_vect {
			print!("{}", c);
		 }
		 
		println!("");
		
		 
	
	} 
	
	
	//print!("{} value:\n{}", display, s);
    // `file` goes out of scope, and the "hello.txt" file gets closed
}