fn main() {
    const COSTANTE: i32 = 10;

    struct Local(i32);

    let x = {
        let a = 2;
        let b = 3;
        a + b // valore restituito: 5
    };
    println!("Somma: {}", x);

    impl Local {
        fn stampa(&self) {
            println!("Valore: {}", self.0);
        }
    }

    fn somma(a: i32, b: i32) -> i32 {
        a + b
    }

    let x = Local(somma(3, COSTANTE));
    x.stampa();

    let moltiplica = |x: i32| x * 2;
    println!("Doppio: {}", moltiplica(5));
}
