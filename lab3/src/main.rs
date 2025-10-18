mod task1;
mod task2;
mod task3;
mod task4;
mod task5;
mod task6;
use std::{fs, io};

fn first_task() {
    println!("{}", task1::generate_output());
}

fn second_task() {
    let mut people: Vec<task2::Person> = Vec::new();
    let mut input_buffer: String = String::new();
    'outer_loop: loop {
        let mut person_buffer: String = String::new();
        let messages = vec![
            "Input name (-1 to exit): ",
            "Input surname (-1 to exit): ",
            "Input age (-1 to exit): ",
        ];
        for i in 0..3 {
            input_buffer.clear();
            println!("{}", messages[i]);
            match io::stdin().read_line(&mut input_buffer) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("Error reading from stdin: {}", error);
                    continue;
                }
            }

            if input_buffer.trim() == "-1" {
                break 'outer_loop;
            }

            person_buffer += &input_buffer;
        }

        match task2::read_person_from_buffer(person_buffer.trim()) {
            Ok(person) => people.push(person),
            Err(error) => {
                eprintln!("Error parsing buffer to person: {}", error);
                continue;
            }
        }
    }

    println!("-------People-------");
    for person in &people {
        println!("{}", person);
    }
    println!("--------Data--------");
    match task2::get_min_max_avg_age(&people) {
        None => (),
        Some((min, max, avg)) => {
            println!("Min: {} Max: {} Average: {:.2}", min, max, avg);
        }
    }
    println!("--------------------");
}

fn third_task() {
    let a: f64;
    let b: f64;
    loop {
        println!("Input rectangle length:");
        match task3::input_f64() {
            Ok(f) => {
                a = f;
                break;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }

    loop {
        println!("Input rectangle width:");
        match task3::input_f64() {
            Ok(f) => {
                b = f;
                break;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }

    let area = match task3::calculate_area(a, b) {
        Ok(area) => area,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    println!(
        "The rectangle with specified sides has an area of {:.2}",
        area
    )
}

fn fourth_task() {
    let rows: u8;
    loop {
        println!("Input discretion level (0 - 255):");
        match task4::input_u8() {
            Ok(r) => {
                rows = r;
                break;
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }

    let file_path = "table.html";
    match fs::write(file_path, task4::generate_html_file(rows, 3)) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}

fn fifth_task() {
    let dir_path: std::path::PathBuf;
    loop {
        println!("Input directory path:");
        match task5::input_path() {
            Ok(path) => {
                dir_path = path;
                break;
            }
            Err(err) => eprintln!("{}", err),
        };
    }

    println!("Input extension: ");
    let mut input_buffer: String = String::new();
    match io::stdin().read_line(&mut input_buffer) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error reading from stdin: {}", error);
            return;
        }
    }

    let files = task5::get_all_files_with_extension(dir_path, &input_buffer.trim().to_string());
    for file in files {
        println!("{}", file.display());
    }
}

fn sixth_task() {
    println!("Input url: ");
    let mut input_buffer: String = String::new();
    match io::stdin().read_line(&mut input_buffer) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error reading from stdin: {}", error);
            return;
        }
    }

    let bytes = match task6::get_file_content(&input_buffer.trim().to_string()) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match task6::create_file(&bytes) {
        Ok(_) => println!("Content saved to file response.txt"),
        Err(err) => eprintln!("{}", err),
    }
}

fn main() {
    loop {
        println!("Enter task number, -1 to exit");
        let mut input_buffer: String = String::new();
        match io::stdin().read_line(&mut input_buffer) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Error reading from stdin: {}", error);
                continue;
            }
        }
        match i32::from_str_radix(input_buffer.trim(), 10) {
            Ok(task_number) => match task_number {
                -1 => break,
                1 => first_task(),
                2 => second_task(),
                3 => third_task(),
                4 => fourth_task(),
                5 => fifth_task(),
                6 => sixth_task(),
                _ => {
                    println!("Invalid task number");
                }
            },
            Err(error) => eprintln!("Failed to parse int: {}", error),
        }
    }
}
