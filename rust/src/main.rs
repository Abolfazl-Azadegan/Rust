
fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("The value of X is {}", x);

    /*This will generate error, because the x variable in immutable which is defined by let and we can not change its value
    later in the code.
    
    // x = 6;
    // println!("The value of X is {}", x);

    */

    /* to solve this issue we can make the x mutable by adding mut to its definition with let. like below code.

    // let mut x = 5;
    // println!("The value of X is {}", x);

    now we can have below code together with let mut x = 5;

    // x = 6;
    // println!("The value of X is {}", x);
    */

    const SECOND: i8 = 60; //This is how we declare a constant variable. i8 here is the data type which is integer 8bit.
    println!("The constant variable is equal to: {}", SECOND);

    let mut y:i8 = 10; //Signed 8 bit integer
    let mut z:i16 = 20; //Signed 16 bit integer
    let mut a:i32 = 30; //Signed 32 bit integer
    let mut b:i64 = 40; //Signed 64 bit integer
    let mut c:i128 = 50; //Signed 128 bit integer
    println!("i8= {}",y);
    println!("i16= {}",z);
    println!("i32= {}",a);
    println!("i64= {}",b);
    println!("i128= {}",c);

    y= -10; //Signed 8 bit integer
    z= -20; //Signed 16 bit integer
    a= -30; //Signed 32 bit integer
    b= -40; //Signed 64 bit integer
    c= -50; //Signed 128 bit integer
    println!("i8= {}",y);
    println!("i16= {}",z);
    println!("i32= {}",a);
    println!("i64= {}",b);
    println!("i128= {}",c);


    let y:u8 = 100; //Signed 8 bit integer
    let z:u16 = 200; //Signed 16 bit integer
    let a:u32 = 300; //Signed 32 bit integer
    let b:u64 = 400; //Signed 64 bit integer
    let c:u128 = 500; //Signed 128 bit integer
    println!("u8= {}",y);
    println!("u16= {}",z);
    println!("u32= {}",a);
    println!("u64= {}",b);
    println!("u128= {}",c);

    /*This part will generate error because we are setting signed numbers (Negative) to unsigned numbers.
    y= -100; //Signed 8 bit integer
    z= -200; //Signed 16 bit integer
    a= -300; //Signed 32 bit integer
    b= -400; //Signed 64 bit integer
    c= -500; //Signed 128 bit integer
    println!("i8= {}",y);
    println!("i16= {}",z);
    println!("i32= {}",a);
    println!("i64= {}",b);
    println!("i128= {}",c);
    */

    /* Note: Decimal and hexadecimal are not different data types in Rust. They are different ways of writing integer values 
    (number literals). 
    For example, these all represent the same integer value:

    let a = 42;       // decimal
    let b = 0x2A;     // hexadecimal
    let c = 0b101010; // binary
    let d = 0o52;     // octal

    All four have the value 42.
    */

    /* Notes: 1. Rust's main data types
    Rust has four major groups of types you'll encounter initially:
    Rust types
    │
    ├── Scalar types
    │   ├── Integers
    │   ├── Floating-point
    │   ├── Boolean
    │   └── Character
    │
    ├── Compound types
    │   ├── Tuple
    │   └── Array
    │
    ├── String-related types
    │   ├── String
    │   └── &str
    │
    └── Other important types
        ├── Struct
        ├── Enum
        ├── Reference
        └── ...
    */

    /*Integer types

    Integers are whole numbers:
    ... -3, -2, -1, 0, 1, 2, 3 ...
    Rust provides several integer types.
    Signed integers
    Signed means they can represent both positive and negative numbers.
            i8
            i16
            i32
            i64
            i128
    The number tells you how many bits are used.
    For example:
    let x: i8 = 10;
    means:
    x is an integer occupying 8 bits and it can be negative or positive.
    The ranges are:
            Type	Bits	Range
            i8	    8	    -128 → 127
            i16	    16	    -32,768 → 32,767
            i32	    32	    -2³¹ → 2³¹−1
            i64	    64	    -2⁶³ → 2⁶³−1
            i128	128	    -2¹²⁷ → 2¹²⁷−1
    */

    /*Unsigned integers

    Unsigned means:
    No negative values.
    Rust has:
            u8
            u16
            u32
            u64
            u128
            usize
    For example:
    let age: u8 = 34;
    u8 can represent:
    0 → 255
    because all 8 bits are available for the value.
    Compare:
    i8:
    -128 → 127
    u8:
    0 → 255
    Similarly:
            Type	Bits	Range
            u8	    8	    0 → 255
            u16	    16	    0 → 65,535
            u32	    32	    0 → 2³²−1
            u64	    64	    0 → 2⁶⁴−1
            u128	128	    0 → 2¹²⁸−1
    */

    /*usize and isize

    These are special.
    usize is an unsigned integer whose size depends on the architecture.
    On a 64-bit system:
    usize = 64 bits
    On a 32-bit system:
    usize = 32 bits
    You will encounter usize constantly when working with:
    arrays
    indexing
    memory
    pointers
    collections
    For example:
            let numbers = [10, 20, 30];
            let index: usize = 1;
            println!("{}", numbers[index]);
    The index is usize.
    */

    /*Floating-point types

    Rust has two floating-point types:
            f32
            f64
    For example:
    let x: f32 = 3.14;
    let y: f64 = 3.1415926535;
    They represent numbers with fractional parts.
            3.14
            10.5
            -2.75
    f64 is the usual default when Rust needs to infer a floating-point type.
    */

    /*Boolean

    Rust has:
            bool
    with only two possible values:
            true
            false
    Example:
        let is_running: bool = true;
    You can use it in conditions:
        if is_running {
            println!("The program is running");
        }
    */

    /*Character

    Rust has:
    char
    A char represents a Unicode scalar value.
    For example:
    let c: char = 'A';
    let x: char = 'ب';
    let emoji: char = '🚀';
    Notice the difference between:
    'A'
    and:
    "A"
    The first is a char.
    The second is a string slice (&str).
    Also, unlike C, Rust's char is 4 bytes, not 1 byte.
    */

    /*Tuple

    A tuple lets you put values of different types together.
    let person = ("EFE", 34, true);
    Conceptually:
    person
    │
    ├── "EFE" → string
    ├── 34    → integer
    └── true  → boolean
    You access elements using their position:
    println!("{}", person.0);
    println!("{}", person.1);
    println!("{}", person.2);
    Output:
    EFE
    34
    true
    */

    /*Array
    An array contains multiple values of the same type and has a fixed size.
    let numbers = [10, 20, 30, 40];
    This is essentially:
    numbers
    ┌────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │
    └────┴────┴────┴────┘
      0    1    2    3

    You can specify its type explicitly:
    let numbers: [i32; 4] = [10, 20, 30, 40];
    The [i32; 4] means:
    i32 → element type
    4   → number of elements
    */



}
