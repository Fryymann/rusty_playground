use std::fmt;

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


// fn greet(mut names: Vec<String>) -> Vec<String> {
//     println!("Whaddup, {:?}.", names);
//     names.clear();
//     names
// }

// fn add_sol(mut names: Vec<String> ) -> Vec<String>{
//     names.push("Sol".to_string());
//     names
// }

// fn main() {
//     let names = vec!["Kaleb".to_string(), "Ian".to_string()];
//     let new_names = greet(names);
//     let new_new_names = add_sol( new_names );
//     greet( new_new_names );
// }


// struct Person {
//     name: String,
//     age: u32,
// }

// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Name: {} | Age: {}", &self.name, &self.age)
//     }
// }

// impl Person {
//     fn age(&mut self) {
//         self.age = &self.age + 1;
//     }
// }

// fn age_person(person: &mut Person) {
//     person.age = person.age + 1;
// }

// fn main() {
//     let mut ian = Person{name: "Ian".to_string(), age: 36};
//     let kaleb = Person{name: "Kaleb".to_string(), age: 19};

//     println!("{}, {}", ian, kaleb);

//     age_person( &mut ian );
//     ian.age();

//     println!("{}, {}", ian, kaleb);
// }



struct Person {
    name: String,
    age: u32
}

impl Person {
    fn greet(&self) {
        println!("Hi, my name is {}.", &self.name);
    }

    fn wave(&self) {
        println!("{name} Waves", name = &self.name);
    }

    fn slap(&self, target: &Person) {
        println!("{} rears back and slaps the shit out of {}!", &self.name, &target.name);
    }
}


impl fmt::Display for Person {
    fn fmt(&self, k: &mut fmt::Formatter) -> fmt::Result {
        write!(k, "name {}, age {}", &self.name, &self.age)
    }
}

fn main(){
    let kaleb =  Person{name: "Kaleb".to_string(),  age: 19};
    let ian: Person = Person{name: "Ian".to_string(), age: 36};

    println!("{}", kaleb);

    kaleb.greet();
    kaleb.wave();

    kaleb.slap(&ian);


}