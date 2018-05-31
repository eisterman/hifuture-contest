use std::io;
use std::process::exit;

fn main() {
    print!("Inserisci un numero e ti dirò se ci piace: ");
    let buffer = String::new();
    io::stdin().read_line(&mut buffer);
    if !buffer.chars().all(|x| x.is_ascii_digit()) || buffer.is_empty() {
        eprintln!("Input non numerico!");
        exit(1);
    }
    if buffer.len() < 2 {
        eprintln!("Il numero deve essere lungo almeno 2 cifre!");
        exit(1)
    }
    for i in 0..(len(buffer)-1) {
        if abs(buffer[i] as i64 - buffer[i+1] as i64) != 1 {
            println!("Il numero non ci piace :c");
            exit(0)
        }
    }
    println!("Il numero ci piace!");
}
