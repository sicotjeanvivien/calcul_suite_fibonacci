use std::io;

fn main() {
    println!("Suite de fibonacci");

    let mut fibonacci_zero: f64 = 0.0;
    let mut fibonacci_un: f64 = 1.0;
    let mut rand = 0;
    let mut supposition = String::new();

    io::stdin()
        .read_line(&mut supposition)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let supposition: u32 = supposition.trim().parse().expect("Veuillez entrer un nombre !");

    while rand < supposition {
        let fibonacci_suivant = fibonacci_zero + fibonacci_un;
        println!(
            "Rand {} : nouveau {} / rapport : {} ",
            rand,
            fibonacci_zero,
            fibonacci_un / fibonacci_suivant
        );
        fibonacci_zero = fibonacci_un;
        fibonacci_un = fibonacci_suivant;
        rand += 1;
    }
}
