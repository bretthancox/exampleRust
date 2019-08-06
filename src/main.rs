fn main() {
    println!("Hello, world!");

    //Simple let that declares an integer, defaulting to i32
    let x = 42; //type is i32 by default
    let y: u32 = 42; //type is an unsigned 32 bit integer
    println!("default int: {}, unsigned 32 bit: {}", x, y);

    //floating point numbers default to 64 bits, where possible, to support greater precision
    let a = 42.42; //type is f64
    let b: f32 = 42.42; //type is f32
    println!("default float: {}, 32 bit float: {}", a, b);

    //boolean is always a boolean value, not a number
    let c = true;
    let d: bool = false;
    println!("True: {}, False: {}", c, d);

    //character type, denoted by single quotes. double quotes arre strings
    let e = 'z';
    println!("Char: {}", e);

    //tuples are written in parentheses, with their types also written that way:
    let f: (i32, f64, u8) = (500, 6.475, 9);
    let (g, h, i) = f; //destructuring
    let i_thirty_two = f.0;
    let f_sixty_four = f.1;
    println!("The tuple: ({} {} {})", f.0, f.1, f.2); //note: destructuring is simple
    println!("Destructured: {}, {}, {}", g, h, i);
    println!(
        "Access first value: {}, Access second value: {}",
        i_thirty_two, f_sixty_four
    );

    //array elements must be of the same type. They are allocated on the stack. Arrays are a fixed size.
    let array_example = [1, 2, 3];
    let first = array_example[0];
    let last = array_example[2];
    println!(
        "The array is [{}, {}, {}]",
        array_example[0], array_example[1], array_example[2]
    );
    println!("First value in variable: {}. Last: {}", first, last);

    //example function for providing parameters to a function
    let j: (i32, i32) = (7, 24);
    let k: i32 = 14;
    example_function(k, j);

    //example function for returning a value
    let ex_fn = example_function_too();
    println!("Example return from function: {}", ex_fn);
}

fn example_function(x: i32, y: (i32, i32)) {
    println!(
        "Example function variable x: {}, variable y: ({}, {})",
        x, y.0, y.1
    );
}

fn example_function_too() -> i64 {  //the arrow shows what is being returned. If multiple values, use commas.
    let x = 97;
    x //ending a statement without a semicolon makes it the return value
}
