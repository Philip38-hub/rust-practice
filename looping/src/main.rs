use std::io;

fn main() {
    let mut x = 0;
    
    loop{
        let mut answer = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read line");
        x+=1;
        let answer = answer.replace("\n", "");
        if answer == "The letter e" {
            break println!("Number of trials: {}", x);
        }
    }
}

