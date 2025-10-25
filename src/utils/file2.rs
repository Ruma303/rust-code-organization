pub fn funzione_da_file2() {
    println!("Attivata funzione da file2!");

    // Richiamare funzione modulo3::funzione_da_file3
    super::file3::modulo3::funzione_da_file3(); // Singola invocazione

    // Importazione locale con use.
    use super::file3::modulo3::funzione_da_file3;

    // Consente di utilizzare l'elemento importato più volte
    funzione_da_file3();
    funzione_da_file3();
    funzione_da_file3();
}