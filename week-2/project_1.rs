fn main() {
    let p = 520_000_000; 
    let r = 10;        
    let n = 5;         

    // Amount
    let a = p * (1 + r / 100)^(n);

    // Compound Interest
    let ci = a - p;

    println!("Amount = {}", a);
    println!("Compound Interest = {}", ci);
}
