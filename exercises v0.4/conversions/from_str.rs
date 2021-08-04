// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::error;
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}



// Steps:
// 1. If the length of the provided string is 0, an error should be returned
// 2. Split the given string on the commas present in it
// 3. Only 2 elements should be returned from the split, otherwise return an error
// 4. Extract the first element from the split operation and use it as the name
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`
// 6. If while extracting the name and the age something goes wrong, an error should be returned
// If everything goes well, then return a Result of a Person object


#[derive(Debug, Clone)]
struct NameAndAgeNeeded;

impl std::fmt::Display for NameAndAgeNeeded {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Must and only give name and age with commas splited")
    }
}

impl error::Error for NameAndAgeNeeded{}

impl FromStr for Person {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if !s.contains(",") || s.len() == 0 {
            return Err(NameAndAgeNeeded.into());
        }
        let s  =s.split(",").collect::<Vec<_>>();
        if s.len() !=2 {return Err(NameAndAgeNeeded.into())}
        if s[0] ==""{
            return Err(NameAndAgeNeeded.into());
        }
        if let Ok(age) = s[1].parse::<usize>(){
            return Ok(Person{
                name: String::from(s[0]),
                age: age
            })
        }
        else{
            return Err(NameAndAgeNeeded.into());
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!("John,".parse::<Person>().is_err());
    }

    #[test]
    fn invalid_age() {
        assert!("John,twenty".parse::<Person>().is_err());
    }

    #[test]
    fn missing_comma_and_age() {
        assert!("John".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name() {
        assert!(",1".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_age() {
        assert!(",".parse::<Person>().is_err());
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(",one".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma() {
        assert!("John,32,".parse::<Person>().is_err());
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert!("John,32,man".parse::<Person>().is_err());
    }
}
