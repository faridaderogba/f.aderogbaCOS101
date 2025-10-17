fn main() {
    let p = 510_000; 
    let r = 5;      
    let n = 3;      

    // Value of the TV after 3 years
    let a = p * (1 - r / 100)^(n);

    println!("Value of the TV after 3 years = {:.2}", a);

}
