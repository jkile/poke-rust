use std::io::{ self, Read };
use console::{ Term };
use dialoguer::{ Select };

pub fn init() -> io::Result<()> {
    let term = Term::stdout();
    term.write_line("Welcome to Poke-Rust!")?;
    println!("Create your team: ");
    let input1 = get_input_string();

    Ok(())
}

fn get_input_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn selec

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
