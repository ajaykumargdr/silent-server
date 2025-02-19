fn main() {
    println!("Running...");

    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        dbg!(&input);
        
        if input.trim() == "exit"{
            break;
        }
    }
}
