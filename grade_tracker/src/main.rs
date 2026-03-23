use std::collections::HashMap;
use std::io;

enum MenuChoice { 
    AddStudent,
    ShowAverage,
    ShowAll,
    Quit,
    Invalid,
}

fn get_menu_choice() -> MenuChoice { 
    println!("\n1. Add student");
    println!("2. Show average");
    println!("3. Show all students");
    println!("4. Quit");
    println!("Choice"); 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    match input.trim() { 
        "1" => MenuChoice::AddStudent,
        "2" => MenuChoice::ShowAverage,
        "3" => MenuChoice::ShowAll,
        "4" => MenuChoice::Quit,
        _ => MenuChoice::Invalid,
    }
}

fn add_student(students: &mut HashMap<String, Vec<f64>>) {
    println!("Student name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read");
    let name = name.trim().to_string();

    println!("Grade: ");
    let mut grade_input = String::new();
    io::stdin().read_line(&mut grade_input).expect("Failed to read");

    let grade: f64 = match grade_input.trim().parse() {
        Ok(n)  => n,
        Err(_) => { println!("Invalid grade!"); return; }
    };

    students.entry(name).or_insert(Vec::new()).push(grade);
    println!("Student added!");
}

fn show_average(students: &HashMap<String, Vec <f64>> ) { 
    println!("Student name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read");
    let name = name.trim(); 

    match students.get(name) { 
        Some(grades) => { 
            let sum: f64 = grades.iter().sum();
            let avg = sum / grades.len() as f64;
            println!("Average for {name}: {avg:.2}");
        }
        None => println!("Student not found"),
    }
}

fn show_all(students: &HashMap<String, Vec<f64>>) { 
    if students.is_empty() { 
        println!("No students yet!");
        return;
    }

    for (name, grades) in students {
        let sum: f64 = grades.iter().sum();
        let avg = sum / grades.len() as f64;
        println!("{name}: grades={grades:?}, avg={avg:.2}");
    }
}

fn main() { 
    let mut students: HashMap<String, Vec<f64>> = HashMap::new();

    println!("Student Grade Tracker");

    loop { 
        match get_menu_choice() { 
            MenuChoice::AddStudent => add_student(&mut students),
            MenuChoice::ShowAverage => show_average(&students),
            MenuChoice::ShowAll => show_all(&students),
            MenuChoice::Quit => { println!("Bye!"); break; }
            MenuChoice::Invalid => println! ("Invalid choice! "),
        }

    }
}