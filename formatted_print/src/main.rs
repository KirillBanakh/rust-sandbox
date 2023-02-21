fn main() {
    println!("Hello, world!");

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );

    println!("Base  2: {:b}", 55555);
    println!("Base  8: {:o}", 55555);
    println!("Base 10: {:?}", 55555);
    println!("Base 16: {:x}", 55555);
    println!("Base 16: {:X}", 55555);

    println!("{number:-<7}", number = 0);
    println!("{number:-^7}", number = 0);
    println!("{number:->7}", number = 0);
}
