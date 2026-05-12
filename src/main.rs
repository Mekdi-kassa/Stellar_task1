use std::io;
use std:: collections::HashMap;
struct Bill{
    name:String,
    amount:f64,
}
struct BillManager{
    bills:HashMap<u32,Bill>,
    next_id:u32,
}
fn main() {
    
    let mut manager = BillManager{
        bills:HashMap::new(),
        next_id:1,
    };
    println!(" Welcome to bill calculator");
    loop{
        println!("1. Add bill");
        println!("2. View all bills");
        println!("3. Remove bill");
        println!("4. View one bill by ID");
        println!("5. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();
        if choice == 1{
            let mut name = String::new();
            println!("Enter name:");
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            let mut amount = String::new();
            println!("Enter amount:");
            io::stdin().read_line(&mut amount).unwrap();
            let amount: f64= amount.trim().parse().unwrap();
            let bill = Bill{name , amount};
            manager.bills.insert(manager.next_id, bill);
            println!("Bill added with ID: {}", manager.next_id);
            manager.next_id += 1;
        }
        else if choice == 2{
            if manager.bills.is_empty(){
                println!("No bills");
            }
            else{
                for (id , bill) in &manager.bills{
                    println!("ID: {} | Name: {} | Amount: ${:.2}", id, bill.name, bill.amount);
                }
            }
        }
        else if choice == 3 {
            let mut id_str = String::new();
            println!("Enter ID to remove:");
            io::stdin().read_line(&mut id_str).unwrap();
            let id: u32 = id_str.trim().parse().unwrap();

            match manager.bills.remove(&id) {
                Some(bill) => println!("Removed: {} (${:.2})", bill.name, bill.amount),
                None => println!("No bill with ID {}", id),
            }
        }
        else if choice == 4 {
            let mut id_str = String::new();
            println!("Enter bill ID to view:");
            io::stdin().read_line(&mut id_str).unwrap();
            let id: u32 = id_str.trim().parse().unwrap();

            match manager.bills.get(&id) {
                Some(bill) => println!("ID: {} | Name: {} | Amount: ${:.2}", id, bill.name, bill.amount),
                None => println!("No bill found with ID {}", id),
            }
        }
        else if choice == 5{
            break;
        }
        else{
            println!("invalid number");
        }
        

    }
    
}
