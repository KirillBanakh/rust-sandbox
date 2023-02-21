fn main() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?}", Structure(3));
    println!("{:?}", Deep(Structure(9)));

    #[derive(Debug)]

    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let my_person = Person {
        name: "Zakhar",
        age: 27,
    };

    impl Person<'_> {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
    }

    println!("{:#?}", my_person);

    println!("Type: {:>3}", std::any::type_name::<Person>());
    println!("Type: {:>3}", my_person.get_type_name());
    println!("Name: {:>3}", my_person.name);
    println!("Age: {:>3}", my_person.age);
}
