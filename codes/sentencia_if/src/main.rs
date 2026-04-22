fn main() {
    let numero = 28;

    if numero % 5 == 0 && numero % 3 == 0 {
        println!("{} es un TriQunit", numero);
    } else if numero % 6 == 0 && numero % 4 == 0 {
        println!("{} es un HexaQuad", numero);
}   else {
    println!("{} es solo otro número", numero);
}
}
