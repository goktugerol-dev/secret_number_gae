use std::io;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use std::cmp::Ordering;


fn main()
{
    fn set_timeout(callback: fn() -> (), time: u64 ) -> ()
    {
        sleep(Duration::from_secs(time));
        callback();
    }

    println!("Adivina el numero");

    set_timeout(|| {
        println!("Por favor entra un numero entre 1-5");
    }, 3);
    let secret_number = rand::thread_rng().gen_range(1..=5);
    //-snip--
    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Por favor entra un numero");


    println!("Tu adivinación: {guess} ");
    loop{
    if  guess == secret_number {
        println!("Genial! El numero secreto es {secret_number} ");
        break;
    }
    
        else
        {   match guess.cmp(&secret_number) 
            {
                Ordering::Less => println!("Más!"),
                Ordering::Greater => println!("Menos!"),
                Ordering::Equal => println!("Ganaste!"),
            }
        }
        let mut guess = String::new (); 
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read_line");
        let guess: u32 = guess.trim().parse().expect("Equivocaste. Entra otro numero");
    
        println!("Valor: {guess} "); //fix it
        if guess == secret_number {
            println!("Ganaste!");
            println!("El numero secreto es {secret_number} ");
        } else 
        {
            println!("El numero incorrecto");
            fn set_timeout(callback: fn() -> (), time: u64) ->()
            {sleep(Duration::from_secs(time));
            callback();}
            set_timeout(|| {
            println!("Perdiste");}, 2);
            println!("El numero secreto era {secret_number} ");
                
        }
        break;
    }
 }
