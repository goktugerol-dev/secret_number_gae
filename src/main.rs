use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main()
{

    println!("Adivina el numero");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("El nmumero secreto: {secret_number} "); // Quiero que aparezca despues al final.
    println!("Por favor entra el numero secreto que adivinas");
    //-snip--
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Por favor entra un numero");


    println!("Tu adivinación: {guess} ");
    
    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Menos!"),
        Ordering::Greater => println!("Más!"),
        Ordering::Equal => println!("Ganaste"),
        
    }
}