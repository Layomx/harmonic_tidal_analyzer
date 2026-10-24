fn main() {
    let language = "Rust";
    let numbers: Vec<u32> = (1..=5).collect();

    println!("Pruebas iniciales de {language}");
    println!("Numeros: {numbers:?}");

    let total: u32 = numbers.iter().sum();
    println!("Suma: {total}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_of_numbers_is_correct() {
        let numbers = [1, 2, 3, 4, 5];
        let total: u32 = numbers.iter().sum();

        assert_eq!(total, 15);
    }
}
