use std::io::{stdin,stdout,Write};
mod arbitary;
fn main() {
    println!("CAScubirb CLI Interface - V0.0.0 ");
    let mut count = 0;
    loop{
        let mut s=String::new();
        print!("In[{}]:= ",count);
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
<<<<<<< Updated upstream
        if s == "exit" || s=="Exit"{
            std::process::exit(0);
        }else{
            println!("Out[{}]: {}",count,s);
        }
=======
        if s == "exit".to_string() {std::process::exit(0);}
        println!("Out[{}]: {}",count,s);

>>>>>>> Stashed changes
        count += 1;
        
    }

}
