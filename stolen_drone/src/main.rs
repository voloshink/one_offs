fn main() {
    let ids = vec![10, 20, 33, 20, 11, 56, 33, 11, 60, 60, 1, 78, 1, 78, 10];

    let mut unique_id = 0;
    for id in &ids {
        unique_id = unique_id ^ id;
    }

    println!("The id of the stolen drone is: {}", unique_id);
}
