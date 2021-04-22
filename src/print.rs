pub fn run() {
    println!("Hello, World!");

    // Basic formatting
    println!("{}", 3+4);
    println!("{} are {}", "Bananas", "Yellow");

    // Positional arguments
    println!("{0} are always the same color as {0}", "oranges");

    // Names arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Football");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));
}