pub fn funzione_da_file1() {
    println!("Attivata funzione da file1!");

    // Funzioni dal modulo padre utils/mod.rs
    super::saluta();
    super::funzione_comune_ma_non_pubblica();
}