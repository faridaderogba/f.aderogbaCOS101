fn main() {
    let p = 520000000.0; 
    let r = 10.0;        
    let n = 5.0;         

    // Amount
    let a = p * (1.0 + r / 100.0).powf(n);

    // Compound Interest
    let ci = a - p;

    println!("Amount = {}", a);
    println!("Compound Interest = {}", ci);
}
