#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        let mut result = String::from("");
        match self {
            Command::Power(i_or_d, num) => {
                if *i_or_d { result.push_str("Power increased by ") }
                else { result.push_str("Power decreased by ") }
                result.push_str(num.to_string().as_str());
                result.push_str("%");
            },
            Command::Missiles(i_or_d, num) => {
                if *i_or_d { result.push_str("Missiles increased by ") }
                else { result.push_str("Missiles decreased by ") }
                result.push_str(num.to_string().as_str());
            },
            Command::Shield(on_off) => {
                if *on_off { result.push_str("Shield turned on") }
                else { result.push_str("Shield turned off") }
            },
            Command::Try => { result.push_str("Call attempt failed") },
            Command::Invalid => { result.push_str("Not a command") }
        }
        result
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    if s == "" { return Command::Invalid }
    else if s == "try calling Miss Potts" { return Command::Try }
    else if s == "shield on" { return Command::Shield(true) }
    else if s == "shield off" { return Command::Shield(false) }

    let string = String::from(s);
    let mut words = string.split_whitespace();
    let tup = (words.next(), words.next(), words.next(), words.next());
    let result = match tup {
        (Some("power"), Some(i_or_d), Some(num), None) => {
            match num.parse::<i32>() {
                Ok(n) => {
                    if i_or_d == "inc" { Command::Power(true, n)}
                    else if i_or_d == "dec" { Command::Power(false, n)}
                    else { Command::Invalid }
                }
                _ => Command::Invalid
            }
        }
        (Some(f_or_a), Some(num), Some("missiles"), None) => {
            match num.parse::<i32>() {
                Ok(n) => {
                    if f_or_a == "fire" { Command::Missiles(false, n) }
                    else if f_or_a == "add" { Command::Missiles(true, n) }
                    else { Command::Invalid }
                }
                _ => Command::Invalid 
            }
        }
        _ => Command::Invalid
    };
    result
}
