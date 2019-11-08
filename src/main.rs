#![allow(dead_code)]

mod console
{
    /// Returns user input
    pub fn get_line() -> String
    {
        use std::io::{stdin, stdout, Write};
    
        let mut input = String::new();
        print!("> ");
        stdout().flush()
            .expect("Could not flush output!");
        stdin().read_line(&mut input)
            .expect("Could not read input!");
        return input;
    }
    
    /// Prints a divider
    pub fn print_divider() {
        println!("=====================");
    }

    /// Prints a message then returns user input
    pub fn prompt(message: &str) -> String 
    {
        println!("{}", message);
        return get_line();
    }
}

struct IP(u8, u8, u8);

use std::fmt;

impl IP
{
    pub fn print(&self, last_octet: &str, cidr: u8) {
        println!("{}{} /{}", self, last_octet, cidr);
    }
}

impl fmt::Display for IP
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.", self.0, self.1, self.2)
    }
}

fn parse_ip(s: &String) -> Option<IP> 
{
    let mut ip = IP(0, 0, 0);
    let ip_str: Vec<&str> = s.split(".").collect();

    if ip_str.len() >= 3 
    { 
        ip.0 = match ip_str[0].parse() { Ok(e) => e, Err(_) => return None };
        ip.1 = match ip_str[1].parse() { Ok(e) => e, Err(_) => return None };
        ip.2 = match ip_str[2].trim().parse() { 
            Ok(e) => e, Err(_) => return None 
        };

        Some(ip) 
    } 
    else { None }
}

fn main() 
{
    // Header
    console::print_divider();
    println!("TINFO 250 - Subnet Utility");
    console::print_divider();

    let ip: IP;
    let mut cidr: u8;

    // Parse IP Address
    loop
    {
        let input_ip = console::prompt(
            "Enter an IP address\n\
            Example: 192.168.10");
    
        match parse_ip(&input_ip)
        {
            Some(i) => 
            { 
                ip = i;
                break; 
            },
            None => { }
        }
    }

    // Parse Cidr Value
    loop
    {
        let input_cidr = console::prompt("Enter a \
            Cidr Value [24-31]:");
        let cidr_str = input_cidr.trim();
        
        match cidr_str.parse()
        {
            Ok(i) =>
            {
                cidr = i;
                if i < 24 || i >= 32 {
                    continue;
                }
                break;
            },
            Err(_) => { }
        }  
    }

    let mut cidr_size: u32 = 0;
    let bits_in_cidr = cidr - 24;
    let mut value = 128;

    for _ in 0..bits_in_cidr 
    {
        cidr_size += value;
        value /= 2;
    }

    let block_size = 256 - cidr_size;

    console::print_divider();
    ip.print("XXX", cidr);
    print!("Cidr in Binary: ");
    
    for _ in 0..bits_in_cidr {
        print!("1");
    }

    for _ in 0..(8 - bits_in_cidr) {
        print!("0");
    }

    print!("\n");
    println!("Block Size: [{}]", block_size);
    console::print_divider();
    println!("");

    for i in 0..(256 / block_size)
    {
        println!("Network #{}", i);
        println!("{}{} <= Net ID", ip, i * block_size);

        if block_size > 2 {
            println!("{}{} <= Default Gateway", ip, i * block_size + 1);
        }

        if block_size > 3 {
            println!("{}{} <= Switch", ip, i * block_size + 2);
        }

        if block_size > 4
        {
            println!("{}{} <= First PC", ip, i * block_size + 3);
            println!("{}{} <= Last PC", ip, i * block_size + block_size - 2);
        }

        println!("{}{} <= Broadcast", ip, i * block_size + block_size - 1);
        println!("");
    }

    console::print_divider();

    loop
    {
        let quit = console::prompt("Type 'Exit' to Exit...");
        if quit.trim().to_lowercase() == "exit" {
            break;
        }
    }
}
