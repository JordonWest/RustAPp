use text_io::read;


pub fn run(){
    crate::welcome::run();
    println!("1: Git License Administration");
    println!("\n2: See my notes");
    let word: String = read!("{}\n");
    if word == "1"{
        crate::git_admin::run();
    }else if word == "2"{
        crate::curl_help::run();
    }
}
