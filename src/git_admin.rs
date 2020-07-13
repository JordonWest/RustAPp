use text_io::read;

pub fn run(){
    println!("\n\n\n\nGit License Administration");
    println!("--------------------------\n");
    println!("1: Add a license to a user");
    println!("2: Remove a license from a user");
    println!("3: Return to main menu");
    let word: String = read!("{}\n");
    if word == "1"{
        println!("\n\nUser added\n\n")
    }else if word == "2"{
        println!("\n\nUser removed\n\n")
    }else if word == "3"{
        crate::menu::run();
    }else{
        println!("\n\n\n\n\n\n\n\n\nPlease select a valid choice");
        run();
    }
}