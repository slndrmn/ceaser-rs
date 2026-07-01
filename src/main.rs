use std::io::{self,Write};

fn flush() {
    io::stdout().flush().unwrap();
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
        let mut desicion = String::new();
        print!("> ");
        flush();
        io::stdin().read_line(&mut desicion).expect("erorororo");

        if desicion.trim() == "1" {
            let mut s = String::new();
            let mut step = String::new();

            print!("wats da message: ");
            flush();
            io::stdin().read_line(&mut s).expect("erorororo");
            print!("wats da step (only integers allowed): ");
            flush();
            io::stdin().read_line(&mut step).expect("erorororo");

            if let Ok(i) = step.trim().parse::<i32>() {
                println!("{}",encode(s.trim().to_string(), i))
            } else {
                println!("bradar only integers allowed in dat section :rage:")
            }
        } else if desicion.trim() == "2" {
            let mut s = String::new();

            print!("wats da msage to decrypt: ");
            flush();
            io::stdin().read_line(&mut s).expect("eorororoor");

            decode(s.trim().to_string());
        } else {
            println!("bradar ar yo stpid there is only 2 options amk")
        }
    }
}
