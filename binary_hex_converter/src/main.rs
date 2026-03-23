use std::io;

enum Conversion {
    DecToBin,
    DecToHex,
    BinToDec,
    HexToDec,
    Quit,
    Invalid,
}

fn get_choice() -> Conversion {
    println!("\n1. Decimal → Binary");
    println!("2. Decimal → Hex");
    println!("3. Binary  → Decimal");
    println!("4. Hex     → Decimal");
    println!("5. Quit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    match input.trim() {
        "1" => Conversion::DecToBin,
        "2" => Conversion::DecToHex,
        "3" => Conversion::BinToDec,
        "4" => Conversion::HexToDec,
        "5" => Conversion::Quit,
        _   => Conversion::Invalid,
    }
}

fn read_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn dec_to_bin(input: &str) -> Result<String, String> {
    let num: u64 = input.parse()
        .map_err(|_| format!("'{}' is not a valid decimal number!", input))?;
    Ok(format!("{num:b}"))
}

fn dec_to_hex(input: &str) -> Result<String, String> {
    let num: u64 = input.parse()
        .map_err(|_| format!("'{}' is not a valid decimal number!", input))?;
    Ok(format!("{num:X}"))
}

fn bin_to_dec(input: &str) -> Result<String, String> {
    let num = u64::from_str_radix(input, 2)
        .map_err(|_| format!("'{}' is not a valid binary number!", input))?;
    Ok(num.to_string())
}

fn hex_to_dec(input: &str) -> Result<String, String> {
    let num = u64::from_str_radix(input, 16)
        .map_err(|_| format!("'{}' is not a valid hex number!", input))?;
    Ok(num.to_string())
}

fn main() {
    println!("Binary/Hex Converter");

    loop {
        match get_choice() {
            Conversion::DecToBin => {
                let input = read_input("Enter decimal number:");
                match dec_to_bin(&input) {
                    Ok(result) => println!("Binary: {result}"),
                    Err(e)     => println!("Error: {e}"),
                }
            }
            Conversion::DecToHex => {
                let input = read_input("Enter decimal number:");
                match dec_to_hex(&input) {
                    Ok(result) => println!("Hex: {result}"),
                    Err(e)     => println!("Error: {e}"),
                }
            }
            Conversion::BinToDec => {
                let input = read_input("Enter binary number:");
                match bin_to_dec(&input) {
                    Ok(result) => println!("Decimal: {result}"),
                    Err(e)     => println!("Error: {e}"),
                }
            }
            Conversion::HexToDec => {
                let input = read_input("Enter hex number:");
                match hex_to_dec(&input) {
                    Ok(result) => println!("Decimal: {result}"),
                    Err(e)     => println!("Error: {e}"),
                }
            }
            Conversion::Quit    => { println!("Bye!"); break; }
            Conversion::Invalid => println!("Invalid choice!"),
        }
    }
}