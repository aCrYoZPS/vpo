use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub surname: String,
    pub age: usize,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.surname, self.age)
    }
}

#[derive(Debug)]
pub struct PersonReadingError {
    message: String,
}

impl fmt::Display for PersonReadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl std::error::Error for PersonReadingError {}

pub fn read_person_from_buffer(buffer: &str) -> Result<Person, PersonReadingError> {
    let parts: Vec<&str> = buffer.trim().split("\n").collect();
    if parts.len() != 3 {
        return Err(PersonReadingError {
            message: "Invalid buffer structure".to_string(),
        });
    }

    let age: usize = match usize::from_str_radix(parts[2], 10) {
        Ok(read_age) => read_age,
        Err(error) => {
            return Err(PersonReadingError {
                message: error.to_string(),
            });
        }
    };

    return Ok(Person {
        name: parts[0].trim().to_string(),
        surname: parts[1].trim().to_string(),
        age,
    });
}

fn round_to_two_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn get_min_max_avg_age(people: &Vec<Person>) -> Option<(usize, usize, f64)> {
    if people.is_empty() {
        return None;
    }

    let ages = people.iter().map(|p| p.age);
    let min_age = ages.clone().min()?;
    let max_age = ages.clone().max()?;
    let avg_age = round_to_two_decimal_places(ages.sum::<usize>() as f64 / people.len() as f64);

    return Some((min_age, max_age, avg_age));
}
