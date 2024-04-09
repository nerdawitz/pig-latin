use std::io;

fn main() {
    loop {
        println!("\n This program converts english words into pig latin. \n For example, if you input the word 'first', it will become 'irst-fay'.");
        let mut user_input = String::new();
        println!("Enter an english word:");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");
        let validated_input: String = user_input.trim().to_string();

        if !alphabet_check(&validated_input) {
            println!("Invalid input, please enter a word containing only alphabetical characters.");
            return;
        }

        let first_character = &validated_input[0..1];
        match first_character {
            "a" | "e" | "i" | "o" | "u" | "A" | "E" | "I" | "O" | "U" => {
                println!("{}", vowel_input(&validated_input))
            }
            _ => println!("{}", const_input(&validated_input)),
        }
    }
}

fn alphabet_check(input: &str) -> bool {
    input.chars().all(|c| c.is_alphabetic())
}

fn vowel_input(input: &str) -> String {
    let new_input = input.to_string();
    let added_string = String::from("-hay");
    let latin_word = new_input + &added_string;
    latin_word
}

fn const_input(input: &str) -> String {
    let new_input = input.to_string();
    let (first_char, rest_of_word) = new_input.split_at(1);
    let string_rest_word: String = rest_of_word.to_string();
    let latin_word = string_rest_word + "-" + &first_char + "ay";
    latin_word
}
