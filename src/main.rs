use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;
use std::fmt;

use std::fs::File;

/// Represents an IP Address
#[derive(Copy, Clone)]
struct IP(i32, i32, i32);

/// Make IP easy to print
impl fmt::Display for IP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

/// Read a line of user input
fn read_line(string: &mut String) {
	print!("> ");
	string.clear();
	stdout().flush().expect("Could not flush output!");
	stdin().read_line(string).expect("Could not read line!");
}

/// Parse a &str Vec into an IP
fn parse_ip(vec: &Vec<&str>) -> Result<IP, ParseIntError> {
	let mut result = IP(0, 0, 0);
	result.0 = vec[0].parse()?;
	result.2 = vec[2].trim().parse()?;
	result.1 = vec[1].parse()?;
	Ok(result)
}

/// Gets total bits in a byte from cidr
fn bits_in_cidr(value: i32) -> i32 {
	value - ((value / 8) * 8)
}

/// Creates an Excel File
fn generate_excel_file(ip: IP, block_size: i32) -> Result<(), std::io::Error>
{
	let mut file = File::create("Output.csv")?;
	file.write(b"NETWORK,NET ID,GATEWAY,SWITCH,FIRST HOST, LAST HOST,BROADCAST\n")?;
	for i in 0..(256 / block_size)
	{
		file.write_fmt(format_args!("{},", i))?;
		file.write_fmt(format_args!("{}.{},", ip, i * block_size))?;
		if block_size > 2 {
			file.write_fmt(format_args!("{}.{},", ip, i * block_size + 1))?;
		}
		if block_size > 3 {
			file.write_fmt(format_args!("{}.{},", ip, i * block_size + 2))?;
		}
		if block_size > 4 {
			file.write_fmt(format_args!("{}.{},", ip, i * block_size + 3))?;
			file.write_fmt(format_args!("{}.{},", ip, i * block_size + block_size - 2))?;
		}
		file.write_fmt(format_args!("{}.{}\n", ip, i * block_size + block_size - 1))?;
	}

	Ok(())
}

fn main() {
	// Application Variables
	let mut input = String::new();
	let mut user_ip: IP;
	let mut user_cidr: i32;

	// Main Application Loop
	'root: loop {
		println!("TINFO 250 - Subnetting Utility");
		println!("==============================");

		// Read an IP Address from the user
		
		'ip: loop {
			println!("Enter an IP Address.");
			println!("Example: [192.168.10]");

			read_line(&mut input);
			let split: Vec<&str> = input.split(".").collect();
			if split.len() >= 3 {
				match parse_ip(&split) {
					Ok(ip) => {
						user_ip = ip;
						break 'ip; 
					}, Err(_) => {}
				}			
			}
		}

		// Read a Cidr from the user

		'cidr: loop {
			println!("Enter a Cidr value.");
			println!("Range: [24-31]");

			read_line(&mut input);
			match input.trim().parse::<i32>() {
				Ok(cidr) => {
					if cidr >= 24 && cidr <= 31 {
						user_cidr = cidr;
						break 'cidr;
					}
				}, Err(_) => {}
			}
		}

		// Calculate Block Size
		let bits_in_cidr = bits_in_cidr(user_cidr);
		let mut addition = 128;
		let mut mask = 0;

		for _ in 0..bits_in_cidr {
			mask += addition;
			addition /= 2;
		}

		let block_size = 256 - mask;
		
		// Print General Network Properties

		println!("\nConfiguring...");
		println!("{}.{} /{}", user_ip, "XXX", user_cidr);
		println!("Subnet Mask: 255.255.255.{}", mask);
		println!("Block Size: {}", block_size);
		println!("");

		// Print Networks

		for i in 0..(256 / block_size) {
			println!("Network #{}", i);
			println!("{}.{} <= Network ID", user_ip, i * block_size);
			if block_size > 2 {
				println!("{}.{} <= Default Gateway", user_ip, i * block_size + 1);
			}
			if block_size > 3 {
				println!("{}.{} <= Switch", user_ip, i * block_size + 2);
			}
			if block_size > 4 {
				println!("{}.{} <= First PC", user_ip, i * block_size + 3);
				println!("{}.{} <= Last PC", user_ip, i * block_size + block_size - 2);
			}
			println!("{}.{} <= Broadcast", user_ip, i * block_size + block_size - 1);
			println!("");
		}

		println!("^ {} Networks Created! ^", 256 / block_size);
		println!("");

		// Ask if whether or not you want an excel file

		println!("Create excel File? [Yes / No]");
		read_line(&mut input);
		if input.to_lowercase().trim() != "no" {
			match generate_excel_file(user_ip, block_size) {
				Ok(_) => println!("Created Output.csv!"),
				Err(_) => println!("Could not create Excel File!")
			}
		}

		// Ask user if they want to quit the application
		
		println!("Would you like to Restart? [Yes / No]");
		
		read_line(&mut input);
		if input.to_lowercase().trim() == "no" {
			break 'root;
		}
	}
}
