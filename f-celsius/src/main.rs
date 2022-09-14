use std::io;
//funcion ppal que invoca f_celsius
fn main() {

    //Ingreso de valor por user
    println!("Ingresa tº en Fahrenheit: ");
    let mut var1 = String::new();
    io::stdin().read_line(&mut var1).expect("No puedo leer datos");

    //convertimos string a integer
    let a: i32 = var1.trim().parse().ok().expect("Programa solo procesa numeros");

    let x = f_celsius(a);

    println!("{x}º Celsius")
}

//funcion que retorna un valor
fn f_celsius(x: i32) -> i32 {
    (x - 32) * 5/9
}


