fn ug_l_to_mg_l(value_ug_l: f64) -> f64 {
    value_ug_l / 1000.0
}

fn main() {
    println!("{:.2}", ug_l_to_mg_l(2500.0));
    println!("{:.2}", ug_l_to_mg_l(0.0));
}
