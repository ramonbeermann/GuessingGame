use std::io; 

fn main(){
    println!("Rate eine Zahl:"); 
    
    println!("Bitte gibt uns eine Schätzung"); 

    let mut guess = String::new(); 
    
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim Lesen der Zeile"); 


println!("Du hast geschätzt:  {guess}"); 




}