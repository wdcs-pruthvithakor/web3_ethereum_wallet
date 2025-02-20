// src/interactive.rs

use crate::balance::fetch_balance;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use anyhow::Result;
use std::io::{self, Write};

/// Helper function to print a separator line
fn print_separator() {
    println!("\n------------------------------------------------------");
}

pub async fn interactive_loop() -> Result<()> {
    // Ask if the user wants to use the same node URL for all operations
    let mut use_same_node = String::new();
    print!("Do you want to use the same Ethereum node URL for all operations? (y/n): ");
    io::stdout().flush().unwrap(); // Ensure prompt is shown before reading input
    io::stdin().read_line(&mut use_same_node)?;
    
    let use_same_node = use_same_node.trim().to_lowercase() == "y"; // Check if the answer is 'y'

    let mut node_url = String::new();
    
    if use_same_node {
        // Ask for the node URL once and use it for all operations
        print!("Enter Ethereum node URL: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut node_url)?;
        node_url = node_url.trim().to_string();
    }

    loop {
        print_separator();
        
        println!("\nSelect an option:");
        println!("1. Create a new wallet");
        println!("2. Check Ethereum balance");
        println!("3. Send Ethereum transaction");
        println!("4. Exit");

        let mut input = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Flush to ensure prompt is shown before reading input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        print_separator();

        match input.trim() {
            "1" => {
                // Create a new wallet
                match Wallet::generate() {
                    Ok(wallet) => {
                        print_separator();
                        wallet.display();
                    }
                    Err(e) => {
                        print_separator();
                        println!("\nError creating wallet: {:?}", e);
                    }
                }
            }
            "2" => {
                // Fetch balance
                let mut address = String::new();
                
                if !use_same_node {
                    // If the user opted not to use the same node, ask for the node URL here
                    println!("Enter Ethereum node URL:");
                    io::stdin().read_line(&mut node_url)?;
                }

                println!("Enter Ethereum address:");
                io::stdin().read_line(&mut address)?;
                print_separator(); 
                match fetch_balance(address.trim(), node_url.trim()).await {
                    Ok(_) => {
                    },
                    Err(e) => {
                        println!("\nError fetching balance: {:?}", e);
                    }
                }
            }
            "3" => {
                // Send transaction
                let mut private_key = String::new();
                let mut to_address = String::new();
                let mut amount = String::new();

                if !use_same_node {
                    // If the user opted not to use the same node, ask for the node URL here
                    println!("Enter Ethereum node URL:");
                    io::stdin().read_line(&mut node_url)?;
                }

                println!("Enter your private key:");
                io::stdin().read_line(&mut private_key)?;
                println!("Enter recipient Ethereum address:");
                io::stdin().read_line(&mut to_address)?;
                println!("Enter amount in ETH:");
                io::stdin().read_line(&mut amount)?;

                let amount: f64 = amount.trim().parse().unwrap_or(0.0);

                let tx = Transaction {
                    from_private_key: private_key.trim(),
                    to_address: to_address.trim(),
                    amount,
                    node_url: node_url.trim(),
                };
                print_separator();
                match tx.send().await {
                    Ok(_) => {
                    },
                    Err(e) => {
                        println!("\nError sending transaction: {:?}", e);
                    }
                }
            }
            "4" => {
                print_separator();
                println!("\nExiting...");
                break;
            }
            _ => {
                print_separator();
                println!("\nInvalid option, please try again.");
            }
        }
    }

    Ok(())
}
