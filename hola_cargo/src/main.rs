fn main() {
    let tuple = ("primer elemento", 3, true, 4.5);
    struct Persona{name: String, age: u8, twitter: String, remote: bool}
    struct Hobbies(String, String, String);

    let persona=Persona {
        name: String::from("Miguel"),
        age: 25,
        twitter: String::from("@wawa"),
        remote: true,
    };

    println!("Persona: {}", persona.name, persona.age, persona.twitter, persona.remote);
}
