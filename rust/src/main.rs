use std::io;
use std::process::exit;

fn main() {
    println!("Inserisci un numero e ti dirò se ci piace: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let data = buffer.trim();
    if !data.chars().all(|x| x.is_ascii_digit()) || data.is_empty() {
        eprintln!("Input non numerico!");
        exit(1);
    }
    if data.len() < 2 {
        eprintln!("Il numero deve essere lungo almeno 2 cifre!");
        exit(1)
    }
    for i in 0..(data.len()-1) {
        if (data.chars().nth(i).unwrap() as i64 - data.chars().nth(i+1).unwrap() as i64).abs() != 1 {
            println!("Il numero non ci piace :c");
            exit(0)
        }
    }
    println!("Il numero ci piace!");
}
