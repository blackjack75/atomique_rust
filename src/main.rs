use inquire::{Text, validator::{StringValidator, Validation} , error::InquireError, Select};

fn main() {

    let options: Vec<&str> = vec!["Banana", "Apple", "Strawberry", "Grapes",
    "Lemon", "Tangerine", "Watermelon", "Orange", "Pear", "Avocado", "Pineapple",
];

let ans: Result<&str, InquireError> = Select::new("What's your favorite fruit?", options).prompt();

match ans {
    Ok(choice) => println!("{}! That's mine too!", choice),
    Err(_) => println!("There was an error, please try again"),
}

    let validator = |input: &str| if input.chars().count() > 140 {
        Ok(Validation::Invalid("You're only allowed 140 characters.".into()))
    } else {
        Ok(Validation::Valid)
    };

    let status = Text::new("What are you thinking about?")
        .with_validator(validator)
        .prompt();

    match status {
        Ok(status) => println!("Your status is being published..."),
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}
