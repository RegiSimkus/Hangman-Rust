use rand::Rng;

extern crate system_shutdown;
use system_shutdown::shutdown;

const WORD_LIST: [&str; 5] = [
    "Word list",
    "Jesus",
    "Saturday 27th of November",
    "Kabaragoya",
    "Green"
];

fn generate_answer() -> String
{
    return WORD_LIST[rand::thread_rng().gen_range(0..4)].to_string();
}

fn replace_char_at(string: String, position: i8, ch: char) -> String
{
    let mut new: String = String::new();
    for i in 0..string.chars().count() {
        if i as i8 == position
        {
            new.push(ch);
        }
        else 
        {
            new.push(string.as_bytes()[i] as char);
        }
    }
    return new; 
}

fn add_progress(ans: &str, prog: &str, c: char) -> String
{
    let mut newprog: String = prog.to_string();
    for i in 0..ans.chars().count() {
        let ch: char = ans.as_bytes()[i] as char;
        if ch.to_ascii_lowercase() as char == c && prog.as_bytes()[i].to_ascii_lowercase() as char == '_'
        {
            newprog = replace_char_at(prog.to_string(), i as i8, ch);
            return add_progress(&ans, &newprog, ch);
        }
    }
    println!("Made all changes -> {}", newprog);
    return newprog; 
}

fn str_to_underscores(word: &String) -> String
{
    let mut underscores: String = String::new();    
    for i in 0..word.chars().count() {
        if word.as_bytes()[i] as char == ' '
        {
            underscores.push(' ')
        }
        else
        {
            underscores.push('_');
        }
    }
    return underscores;
}

fn main() { // Handle the game
    let ans: String = generate_answer();
    let mut prog: String = str_to_underscores(&ans);

    println!("Your new word is: {}", prog);
    while !ans.eq(&prog) // While they're still guessing
    {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let c: u8 = input.as_bytes()[0];
        prog = add_progress(&ans, &prog, c as char).to_string();
    }

    match shutdown() {
        Ok(_) => println!("cya. :)"),
        Err(_) => println!("Congrats, you won!")
    };
}