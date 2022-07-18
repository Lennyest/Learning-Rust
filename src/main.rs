fn nextstep()
{
    println!("---------------------");
}

// Full credits to original doc.rust-lang.org, this is a simplified version and to be used as a reference.
fn main() {
    // VARIABLES

    // Mut key word allows a variable to be mutable, variables are immutable by default.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    nextstep();

    // Cant use mut with const, type must be annotated. Can be declared in any scope, even global.
    // Must be set to a constant expression, not a value that can only be computed at runtime. (Ex, An input variable will be the default.)
    const THREE_HOURS_IN_SEONDS: u32 = 60 * 60 * 3;

    // Shadowing
    /*
        Declare a new varaible with the same name as a previous variable, (Shadowing it)
        Variable is now set to the new value.
        Once scope has ended, the value returns to the original.

        If this is a brand new concept, consider reading about it on the docs.
        https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    */
    let x = 5;
    let x = x + 1;

    // This is an inner scope. Once it has finished, 
    // it returns to the original value of 5.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 5

    nextstep();

    // Shadowing 2:
    /*
        Shadowing spares us from creating new names for variables.
        We can also change the type of the value.
    */

    
    // Type: &str
    let spaces = "   ";
    // Type: usize
    let spaces = spaces.len();
    //Works!

    // We use a string literal to show the value because it is a number type.
    println!("{spaces}");

    // This is not possible because of the type:
    /*
        let mut spaces = "   ";
        spaces = spaces.len();
     */

     nextstep();

    // DATA TYPES
    /*

        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize

        n = bits
        -(2n - 1) to 2n - 1 - 1

        isize and usize depend on architecture of your computer, 64 or 32 bit.

        **Number literals**

        Decimal	        98_222
        Hex	            0xff
        Octal	        0o77
        Binary	        0b1111_0000
        Byte(u8 only)	b'A'

        You can also use _ as a visual seperator, ex:
            1_000 == 1000


        **Integer Overflow**
        Rust will panic at runtime (Exiting with a error) in development mode.

        In release mode, rust performs two's complement wrapping. 
        Values greater than max value the type can hold wrap around to the minimum.
        256 + 1 = 1, etc.
        You should not rely on wrapping.

        **Handling overflow**
        Methods from standard library for primitive numeric types:
            wrapping_* (wrapping_add, etc).
            Return the non value if there is an overflow with the checked_* methods.
            Return the val and bool if there was an overflow with the overflowing_* methods.
            Saturate at the value's min or max with saturating_* methods.
        

        **Floating-point Types**
        All floating point types are signed.
        f32 & f64, f64 is default because it's roughly the same speed as 32 but more precision.
    */

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // IEEE-754 standard
    //The f32 type is a single-precision float and f64 has double precision.

    /*
        **Numeric Operations**

        Supports basic mathemathical operations.
        Integer division rounds down to the nearest integer.    
    */

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;
    println!("{sum} | {difference} | {product} | {quotient} | {floored} | {remainder}");
    nextstep();

    /*
        **Boolean Type**
        General boolean behavior, 1 byte, two values (true/false).
        Type is specified with "bool".
    */

    let t = true;

    let f: bool = false; // explicit type annotation.

    println!("{t} - {f}");

    nextstep();

    /*
        **Character Type**
        Defined with single quotes.
        Four bytes in size and represents a unicode scalar value.
            Accented, Chinese, Japanese, emojis, zero-width spaces.
        Unicode scalar values range from 
            U+0000 to U+D7FF
            and
            U+E000 to U+10FFFF inclusive.
        A character in Unicode is not the same as human intuition, may not match with what a char is in Rust.
        
        More on this in chapter 8.
    */

    let char_example = 'Z';
    let emoji: char = '🤣';
    let chinese_character = '富';
    println!("{char_example} - {emoji} - {chinese_character}");

    nextstep();
    /*
        **Compound Types**
        Group multiple values into one type.
        Tuples and Arrays

        **The Tuple Type**
        Fixed size, once declared it can not grow.
        Each position in the tuple has a type and the values don't have to be the same.
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // The variable binds to the entire tuple, to get the individual values:
    let (x, y, z) = tup;
    println!("The value of each element: X:{x} Y:{y} Z:{z}");

    // We can access each element by indexing:
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} - {six_point_four} - {one}");
    // The tuple without any values is called an unit. Type and value is written as () and represent an empty value/return type.
    // Expressions return the unit value if they don't have any other value.


    /*
        **The Array Type**
        The array values must all have the same type.
        Arrays have a fixed length.  
        Arrays are allocated on the stack rather than heap. (Chapter 4)
        
        Vectors are similar to arrays, but can grow in size.
        Chapter 8 talks about vectors.
    */
    let array = [1, 2, 3, 4, 5, 6];

    // You can dictate the number of elements by doing:
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // [i32; **5**] is the size.

    // You can initiate an array with the same value like this:
    let array = [3; 5];
    // Initial value and then length. 3 = value, 5 = length.

    // Accessing elements:
    let first = array[0];
    let second = array[1];
    let third = array[3];
    // Indexing an array with a value out of bounds will result in panic (Panic = crash).

    println!("{first} - {second} - {third}");

    nextstep();

    // **Functions**
    fn test()
    {
        println!("This is from a function.");
    }
    test();

    // **Parameters**

    // In parameters you must declare the type. Separated by commas.
    fn test_param(x: i32, other_param: bool) {
        println!("Value : {x} - {other_param}");
    }
    test_param(10, true);


    // **Statements and Expressions**

    // let x = (let y = 6) is not possible because the variable doesnt return a value.

    let test_statement =
    {
        let x = 3;
        x + 1 // This line does not end with a ; because it is the return value and ending.
    };

    println!("{test_statement}");

    // **Functions with return values**
    /*
        Return values are not named.
        Type must be declared using ->
        You can return early from a block by using the return keyword with a value.
    */

    fn five() -> i32 {
        5
    }
    let test_statement = five(); // We can do this because the function returns a value.
    println!("{test_statement}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    let test_statement = plus_one(5); // We can do this because the function returns a value.
    println!("{test_statement}");

    // Conditional Statement
    let number = 3;

    // Single if
    if number < 5 {
        println!("True!");
    } else {
        println!("False.");
    }

    // Else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Conditions in a statement, remember the lack of semi colon !
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");
    // You have to use types that are compatible for both return types.

    // **Loops**
    /*
        Types: loop, while and for
        This will run until we explicitly stop it. You can break out of it with the 'break' keyword as well as 'continue' to continue.
        This loop will run once then stop.
        
        Break and continue applies to the innermost loop at the current point. You can assign a loop label to specifically break or continue
        a specific loop.

        Labels must begin with a single quote. Ex:
                    'example_loop
    */
    loop {
        println!("ran!");
        break;
    }

    // Returning values from a loop using break.
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");

    // Loops with labels:
    let mut counter = 0;
    'loop_label: loop {
        println!("{counter}");

        let mut remaining = 10;

        loop {
            println!("remaining - {remaining}");

            if remaining == 0 {
                break;
            }

            if counter == 2 
            {
                // Breaking the outer loop using the label.
                break 'loop_label;
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("End count: {counter}");


    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    
    //Looping through a collection with for

    let array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("value: {}", array[index]);
        index += 1;
    }

    // item iterator for loop
    for element in array {
        println!("element: {element}");
    }

    // Reversing
    for number in (1..4).rev() {
        println!("{number}");
    }

    // Chapter 3 complete!
}