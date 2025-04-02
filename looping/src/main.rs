use std::io;

fn main() {
    let mut x = 0;
    let mut answer = String::new();

    loop{
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read line");
        x+=1;
        if answer == "The letter e\n" {
            break println!("Number of trials {}", x);
        }
    }
}

