#[cfg(test)]
mod task_1_tests {
    use lab3::task1::generate_output;
    #[test]
    fn test_first_line() {
        let output = generate_output();
        let parts: Vec<&str> = output.split('\n').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "Hello, world!")
    }

    #[test]
    fn test_second_line() {
        let output = generate_output();
        let parts: Vec<&str> = output.split('\n').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[1], "And hi again!")
    }

    #[test]
    fn test_third_line() {
        let output = generate_output();
        let parts: Vec<&str> = output.split('\n').collect();
        assert_eq!(parts.len(), 3);
        let exclam_count = parts[2].chars().filter(|ch| *ch == '!').count();
        assert_eq!(exclam_count, parts[2].len());
        assert!(exclam_count >= 5);
        assert!(exclam_count <= 50);
    }
}

#[cfg(test)]
mod task_2_tests {
    use lab3::task2::*;

    #[test]
    fn test_correct_person_reading() {
        let person_buffer = "Name\nSurname\n16\n";
        let person = read_person_from_buffer(person_buffer).unwrap();
        assert_eq!(
            person,
            Person {
                name: "Name".to_string(),
                surname: "Surname".to_string(),
                age: 16,
            }
        )
    }

    #[test]
    fn test_wrong_person_reading_age() {
        let person_buffer = "Name\nSurname\nWRONG!\n";
        let result = read_person_from_buffer(person_buffer);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: invalid digit found in string".to_string()
        );
    }

    #[test]
    fn test_wrong_person_reading_structure() {
        let person_buffer = "Name\nWRONG!\n";
        let result = read_person_from_buffer(person_buffer);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: Invalid buffer structure".to_string()
        );
    }

    #[test]
    fn test_empty_min_max_avg() {
        let people: Vec<Person> = Vec::new();
        let result = get_min_max_avg_age(&people);
        assert!(result.is_none())
    }

    #[test]
    fn test_min_max_avg() {
        let people = vec![
            Person {
                name: "N1".to_string(),
                surname: "SN1".to_string(),
                age: 19,
            },
            Person {
                name: "N2".to_string(),
                surname: "SN2".to_string(),
                age: 28,
            },
            Person {
                name: "N3".to_string(),
                surname: "SN3".to_string(),
                age: 44,
            },
            Person {
                name: "N4".to_string(),
                surname: "SN4".to_string(),
                age: 11,
            },
            Person {
                name: "N5".to_string(),
                surname: "SN5".to_string(),
                age: 36,
            },
            Person {
                name: "N6".to_string(),
                surname: "SN6".to_string(),
                age: 85,
            },
            Person {
                name: "N7".to_string(),
                surname: "SN7".to_string(),
                age: 97,
            },
        ];

        let result = get_min_max_avg_age(&people);

        assert!(result.is_some_and(|tuple| { tuple.0 == 11 && tuple.1 == 97 && tuple.2 == 45.71 }))
    }
}

#[cfg(test)]
mod task_3_tests {
    use lab3::task3::*;
    #[test]
    fn zero_area_test() {
        let result = calculate_area(0.0, 5.0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: A rectangle cannot have side of length zero or of negative length".to_string()
        );
    }

    #[test]
    fn negative_area_test() {
        let result = calculate_area(-10.0, 5.0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Error: A rectangle cannot have side of length zero or of negative length".to_string()
        );
    }
}
