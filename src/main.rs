mod displays;

fn main() {
    println!("Hello, world!");
    print_examples();
    debug_examples();
    display_example();
}

fn print_examples() {
    let name = "Bricktop";
    println!("Hello from test");
    // print formatter
    print!("{} mal hello \n", 27);
    //print some specific values
    println!("My name is {firstparam}!", firstparam = name);

    // disable the linter
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    // add the debug formatter for print

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{:?}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct DebugStructure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(DebugStructure);

fn debug_examples() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} ok some integer value", 21);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now is {:?} printable", DebugStructure(3));
    println!("Now is {:?} printable", Deep(DebugStructure(7)));
}

// display example

fn display_example() {
    let my_display = displays::DisplayStruct::new();
    println!("Display {} DisplayStructure", my_display);
    let my_custom_display = displays::DisplayStruct::new_with_value(7);
    println!("Display {} DisplayStructure", my_custom_display);
}
