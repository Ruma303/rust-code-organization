pub mod network; // rende pubblico il modulo network
pub mod utils;
// Opzionale: re-export diretto delle funzioni per uso più ergonomico
pub use network::{connect, disconnect};

// Non valido!
/*
  println!("You are in lib.rs!");
  connect();
  disconnect();
*/

// Uso consentito: definizione costanti
const MAX_CONNECTIONS: usize = 10;

// Funzione pubblica per dimostrazione/uso/test
pub fn test_connection() {
    println!("You are in lib.rs!");
    connect();
    println!("Max connections number: {}", MAX_CONNECTIONS);
    disconnect();
    close_connection();
}

// Funzioni private per uso interno alla libreria
fn close_connection() {
    println!("Closing connection.");
}

// Uso consentito: test
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_connection() {
        connect();
    }

    #[test]
    fn test_disconnection() {
        disconnect();
    }
}
