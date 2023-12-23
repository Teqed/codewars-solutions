fn format_money(amount: f64) -> String {
    format!("${:.2}", amount)
}

fn main() {
    println!("{}", format_money(3.99));
    println!("{}", format_money(39.99));
    println!("{}", format_money(399.99));
}