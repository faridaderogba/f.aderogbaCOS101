// 1. Define the Laptop struct
struct Laptop {
    brand: String,
    unit_cost: u64, // Using u64 for cost in Naira (large integer)
    stock_quantity: u32,
}

// 2. Implement methods for the Laptop struct
impl Laptop {
    // A constructor-like method to create a new Laptop instance
    fn new(brand: &str, unit_cost: u64, stock_quantity: u32) -> Laptop {
        Laptop {
            brand: brand.to_string(),
            unit_cost,
            stock_quantity,
        }
    }

    // A method to calculate the cost for a specific number of units purchased
    fn calculate_purchase_cost(&self, quantity_purchased: u32) -> u64 {
        // We assume the purchase quantity is available in stock for this calculation
        if quantity_purchased > 0 {
            self.unit_cost * (quantity_purchased as u64)
        } else {
            0
        }
    }
}

fn main() {
    // 3. Initialize the consignment data
    let hp = Laptop::new("HP", 650_000, 10);
    let ibm = Laptop::new("IBM", 755_000, 6);
    let toshiba = Laptop::new("Toshiba", 550_000, 10);
    let dell = Laptop::new("Dell", 850_000, 4);

    // 4. Define the customer's purchase scenario
    let customer_purchase_quantity: u32 = 3;

    // 5. Calculate the total cost using the 'calculate_purchase_cost' method for each brand
    let hp_cost = hp.calculate_purchase_cost(customer_purchase_quantity);
    let ibm_cost = ibm.calculate_purchase_cost(customer_purchase_quantity);
    let toshiba_cost = toshiba.calculate_purchase_cost(customer_purchase_quantity);
    let dell_cost = dell.calculate_purchase_cost(customer_purchase_quantity);

    // 6. Sum up the individual costs to get the grand total
    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    // 7. Print the results
    println!("--- Customer Purchase Details (3 Laptops of each brand) ---");
    println!("HP Cost (3 @ ₦{}): ₦{}", hp.unit_cost, hp_cost);
    println!("IBM Cost (3 @ ₦{}): ₦{}", ibm.unit_cost, ibm_cost);
    println!("Toshiba Cost (3 @ ₦{}): ₦{}", toshiba.unit_cost, toshiba_cost);
    println!("Dell Cost (3 @ ₦{}): ₦{}", dell.unit_cost, dell_cost);
    println!("----------------------------------------------------------");
    println!("**TOTAL CUSTOMER PURCHASE COST: ₦{}**", total_cost);
}