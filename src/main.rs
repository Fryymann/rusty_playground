

// fn greet(names: Vec<String>) {
//     println!("Welcome, {:?} to the Rust world!", names);
//     names.clear();
// }
// fn main() {
//     let mut names = vec!["Rusty".to_string(), "Marcel".to_string()];
//     greet(&mut names);
//     names.push("John".to_string());
//     greet(&mut names);
// }


// fn greet(mut names: Vec<String>) -> Vec<String> {
//     println!("Welcome, {:?} to the Rust world!", names);
//     names.clear();
//     names.push("John".to_string());
//     names
// }
// fn main() {
//     let names = vec!["Rusty".to_string(), "Marconious".to_string()];
//     let new_names = greet(names);
//     greet(new_names);
// }


fn greet(names: Vec<String>) {
    println!("Whaddup, {:?}.", names);
}

fn main() {

    let names = vec!["Kaleb".to_string(), "Ian".to_string()];
    greet( names );
}