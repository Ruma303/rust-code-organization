pub mod file1;  // Importa `file1.rs`
pub mod file2;  // Importa `file2.rs`
pub mod file3;  // Importa `file3.rs`

pub fn saluta() {
    println!("Ciao dal modulo util!");
}

fn funzione_comune_ma_non_pubblica() {
  println!("Funzionalità comune definita in utils/mod.rs");
}