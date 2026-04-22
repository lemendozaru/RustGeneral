fn main() {
    // Variable para el nombre

    let nombre = "Juan";

    // Variable para la edad
    let mut edad: i8 = 21;

    // Constante para los años
    const PLAZO: i8 = 6;

    // Calcula la edad
    edad += PLAZO;

    // Edad calculada
    println!("En {} años, {} tendrá {} años", PLAZO, nombre, edad);
}
