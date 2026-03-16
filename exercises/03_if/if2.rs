// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else if food == "broccoli" {
        "No thanks!"
    } else if food == "gummy bears" {
        "No thanks!"
    } else {
        "No thanks!"
    }
}

fn main() {
    // You can optionally experiment here.
    println!("{}", picky_eater("strawberry"));
    println!("{}", picky_eater("potato"));
    println!("{}", picky_eater("broccoli"));
    println!("{}", picky_eater("gummy bears"));
    println!("{}", picky_eater("bread"));
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "strawberry" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
