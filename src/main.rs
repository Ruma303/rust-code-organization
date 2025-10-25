mod utils;

fn main() {

  //# Elementi importati dai sottomoduli di utils
  utils::file1::funzione_da_file1();
  utils::file2::funzione_da_file2();
  utils::file3::modulo3::funzione_da_file3();

  //# Funzione definita nel modulo utils
  utils::saluta();
}
