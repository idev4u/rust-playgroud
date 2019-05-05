fn main() {
    println!("Hello, world!");
    test();
}

fn test() {
    let name = "Bricktop";
    println!("Hello from test");
    // print formatter
    print!("{} mal hello \n", 27);
    //print some specific values
    println!("My name is {firstparam}!", firstparam = name);

    // disable the linter
    #[allow(dead_code)]
    struct Structure(i32);
    // add the debug formatter for print
    #[derive(Debug)]

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{:?}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
