use std::collections::HashMap;
use std::io;
use std::io::Write;

struct Bill {
    name: String,
    amount: f64,
}

struct BillManager {
    bills: HashMap<u32, Bill>,
    next_id: u32,
}

fn main() {
    let mut manager = BillManager {
        bills: HashMap::new(),
        next_id: 1,
    };

    println!("Welcome to bill manager");

    loop {
        print_menu();
        match read_menu_choice() {
            MenuChoice::Add => add_bill(&mut manager),
            MenuChoice::ViewAll => view_bills(&manager),
            MenuChoice::Remove => remove_bill(&mut manager),
            MenuChoice::Edit => edit_bill(&mut manager),
            MenuChoice::Exit => break,
            MenuChoice::Invalid => println!("Invalid choice. Try again."),
        }
    }
}

enum MenuChoice {
    Add,
    ViewAll,
    Remove,
    Edit,
    Exit,
    Invalid,
}

fn print_menu() {
    println!("\n1. Add bill");
    println!("2. View all bills");
    println!("3. Remove bill");
    println!("4. Edit bill");
    println!("5. Exit");
}

fn read_menu_choice() -> MenuChoice {
    let input = read_line("Enter choice: ");
    match input.as_str() {
        "1" => MenuChoice::Add,
        "2" => MenuChoice::ViewAll,
        "3" => MenuChoice::Remove,
        "4" => MenuChoice::Edit,
        "5" => MenuChoice::Exit,
        _ => MenuChoice::Invalid,
    }
}

fn add_bill(manager: &mut BillManager) {
    let name = read_non_empty("Enter name: ");
    let amount = read_f64("Enter amount: ");

    let bill = Bill { name, amount };
    manager.bills.insert(manager.next_id, bill);
    println!("Bill added with ID: {}", manager.next_id);
    manager.next_id += 1;
}

fn view_bills(manager: &BillManager) {
    if manager.bills.is_empty() {
        println!("No bills");
        return;
    }

    for (id, bill) in &manager.bills {
        println!("ID: {} | Name: {} | Amount: ${:.2}", id, bill.name, bill.amount);
    }
}

fn remove_bill(manager: &mut BillManager) {
    let Some(id) = read_u32_or_back("Enter ID to remove (or 'b' to go back): ") else {
        println!("Canceled remove.");
        return;
    };

    match manager.bills.remove(&id) {
        Some(bill) => println!("Removed: {} (${:.2})", bill.name, bill.amount),
        None => println!("No bill with ID {}", id),
    }
}

fn edit_bill(manager: &mut BillManager) {
    let Some(id) = read_u32_or_back("Enter ID to edit (or 'b' to go back): ") else {
        println!("Canceled edit.");
        return;
    };

    let Some(bill) = manager.bills.get_mut(&id) else {
        println!("No bill with ID {}", id);
        return;
    };

    loop {
        println!("\nEditing ID: {} | Name: {} | Amount: ${:.2}", id, bill.name, bill.amount);
        println!("1. Edit name");
        println!("2. Edit amount");
        println!("3. Back");
        let input = read_line("Enter choice: ");
        match input.as_str() {
            "1" => {
                let new_name = read_non_empty_or_back("Enter new name (or 'b' to go back): ");
                if let Some(name) = new_name {
                    bill.name = name;
                    println!("Name updated.");
                } else {
                    println!("Canceled name edit.");
                }
            }
            "2" => {
                let new_amount = read_f64_or_back("Enter new amount (or 'b' to go back): ");
                if let Some(amount) = new_amount {
                    bill.amount = amount;
                    println!("Amount updated.");
                } else {
                    println!("Canceled amount edit.");
                }
            }
            "3" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).unwrap_or(0);
    input.trim().to_string()
}

fn read_non_empty(prompt: &str) -> String {
    loop {
        let input = read_line(prompt);
        if !input.is_empty() {
            return input;
        }
        println!("Input cannot be empty.");
    }
}

fn read_non_empty_or_back(prompt: &str) -> Option<String> {
    loop {
        let input = read_line(prompt);
        if input.eq_ignore_ascii_case("b") {
            return None;
        }
        if !input.is_empty() {
            return Some(input);
        }
        println!("Input cannot be empty.");
    }
}

fn read_u32_or_back(prompt: &str) -> Option<u32> {
    loop {
        let input = read_line(prompt);
        if input.eq_ignore_ascii_case("b") {
            return None;
        }
        match input.parse::<u32>() {
            Ok(value) => return Some(value),
            Err(_) => println!("Please enter a valid number or 'b' to go back."),
        }
    }
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_line(prompt);
        match input.parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid amount."),
        }
    }
}

fn read_f64_or_back(prompt: &str) -> Option<f64> {
    loop {
        let input = read_line(prompt);
        if input.eq_ignore_ascii_case("b") {
            return None;
        }
        match input.parse::<f64>() {
            Ok(value) => return Some(value),
            Err(_) => println!("Please enter a valid amount or 'b' to go back."),
        }
    }
}
