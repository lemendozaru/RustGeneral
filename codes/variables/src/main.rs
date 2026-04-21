fn main() {
    // Declaramos una variable
    let valor = 9;

    // Declaramos un arreglo
    let numeros = [1, 2, 3, 4, 5];

    // Declaramos una tupla
    let persona = ("Alicia", 30, 5.4);

    // Declaramos una constante
    const PI: f32 = 3.14;

    // Imprimimos el número
    println!("El valor es {}", valor);

    // Imprimimos solo un elemento del arreglo
    println!("Elemento en el índice 0: {}", numeros[0]);

    // Imprimimos la tupla a conveniencia
    println!("Nombre: {}", persona.0);
    println!("Edad: {}", persona.1);

    // Imprimimos la constante
    println!("Valor de PI: {}", PI)
}
