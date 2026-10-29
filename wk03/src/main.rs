fn append_suffix(mut id: String) -> String{
    id.push_str("_checked");
    id
}

fn main() {
    let id = String::from("S01");
    let checked_id = append_suffix(id);
    println!("{checked_id}")
}