fn main() {
    let mut a = String::new();
    let mut b = String::new();
    std::io::stdin().read_line(&mut a).expect("Error read line");
    std::io::stdin().read_line(&mut b).expect("Error read line");

    let num_a: i32 = a.trim().to_string().parse::<i32>().unwrap();
    let num_b: i32 = b.trim().to_string().parse::<i32>().unwrap();

    println!("SOMA = {}", num_a + num_b);
}
