use std::io::{self,Write};

fn flush() {
    io::stdout().flush().unwrap();
}

fn input(s:String) -> String {
    print!("{}",s);
    flush();
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("erorroroor");
    result
}

fn encode(msg:String,step:i32) -> String {
    let mut encoded_msg = String::new();
    for character in msg.chars() {
        let ascii = (character as u8) + ((step % 26) as u8);
        if ascii > 122 {
            encoded_msg += &((ascii - 26) as char).to_string()
        } else if character == ' ' {
            encoded_msg += " "
        } else {
            encoded_msg  += &(ascii as char).to_string()
        }
    }
    encoded_msg
}

fn decode(msg:String) {
    for step in 1..=26 {
        let mut decoded_msg = String::new();
        for character in msg.chars() {
            let ascii = (character as u8) + ((step % 26) as u8);
            if ascii > 122 {
                decoded_msg += &((ascii - 26) as char).to_string()
            } else if character == ' ' {
                decoded_msg += " "
            } else {
                decoded_msg += &(ascii as char).to_string()
            }
        }
        println!("{}",decoded_msg)
    }
}

fn main() {
    print!("1- encode a message\n2- decode a message\n");
    flush();
    loop {
        let desicion = input("> ".to_string());

        if desicion.trim() == "1" {
            let s = input("wats da msg: ".to_string());
            let step = input("wats da step (only integers allowed): ".to_string());

            if let Ok(i) = step.trim().parse::<i32>() {
                println!("{}",encode(s.trim().to_lowercase().to_string(), i))
            } else {
                println!("bradar only integers allowed in dat section :rage:")
            }
        } else if desicion.trim() == "2" {
            let s = input("wats da msage to decode: ".to_string());

            decode(s.trim().to_lowercase().to_string());
        } else {
            println!("bradar ar yo stpid there is only 2 options amk")
        }
    }
}
