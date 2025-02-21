use spinoff::{Spinner, spinners, Color};

fn main() {
    let mut spinner = Spinner::new(spinners::SoccerHeader, "Running...", Color::Blue); 
    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        // dbg!(&input);
        
        if input.trim() == "exit"{
            break;
        }
    }

    spinner.success("Program Stopped!");
}
