
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




    /*This will generate warning for use because the unsed_variable is defined but is not used.*/
    //let unsed_variable:i8 = 1000;
    /*We can solve this issue by adding _ to the variable name like below code.*/
    let _unsed_variable:i8 = 120;

    /*This will create a character variable*/
    let _character = 'A';

    let _byte = b'A';
    /*when you put b before the character: b'A'you are telling Rust:
    Give me the ASCII/byte value of this character as a u8.
    Therefore:
        let byte = b'A';
    has:
    byte
    │
    └── type: u8
        value: 65
    */
    /*Compare A and b'A'
    This distinction is extremely important:
    'A'is char
    while:
    b'A'is u8
    */

    /*Because byte is a u8. You can also print it as hexadecimal:*/

    let byte = b'A';
    println!("The Hex format of the varibale byte is: {:x}", byte);
    println!("The Decimal format of the varibale byte is: {}", byte);

    let _var_float_64 = 2.5; //The default variable size is 64 bit
    let _var_float_32:f32 = 2.5; //This is 32 bit floating point variable 





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

    /*Tuples have a fixed size in Rust.

    That means once you create a tuple, the number of elements cannot change.
    For example:

    let person = ("EFE", 34, true);

    This tuple has exactly 3 elements:
    person
    │
    ├── 0 → "EFE"
    ├── 1 → 34
    └── 2 → true
    You cannot add a fourth element to it later.
    */

    /* The type of a tuple includes its size

    This is important.

    let x = (10, 20, 30);

    has the type:
    (i32, i32, i32)

    Notice that the type itself describes three elements.
    Compare:

    let x = (10, 20);

    whose type is:
    (i32, i32)

    These are different types.
    So unlike something such as a dynamically growing collection, a tuple's size is part of its type.
    */

    /*Tuples can have different types for each element, For example:

    let data = (10, 3.14, true, 'A');

    Its type is approximately:

    (i32, f64, bool, char)

    and it has exactly 4 elements.
    This is one of the main differences between a tuple and an array:

    let tuple = (10, 3.14, true);

    Each element can have a different type.

    But:

    let array = [10, 20, 30];

    All elements must have the same type.

    Tuple:
    (10, 3.14, true)
    │    │      │
    i32  f64   bool
    └──── different types ────┘

    Array:
    [10, 20, 30]
    │   │   │
    i32 i32 i32
    └── same type ──┘

    And both have fixed size.

    So the key idea is:

    A Rust tuple has a fixed number of elements, and that number is part of the tuple's type.
    */

    let new_tup = ("EFE", "Hi", 34, 2.5);
    println!("The username is: {}", new_tup.0);
    println!("The user message is: {}", new_tup.1);
    println!("The user age is: {}", new_tup.2);
    println!("The user salary annually is: {}", new_tup.3);

    /*Below method will assign tuple members to variables:*/
    let (x1, x2, x3, x4) = new_tup;

    println!("x1 is equal to: {}", x1);
    println!("x2 is equal to: {}", x2);
    println!("x3 is equal to: {}", x3);
    println!("x4 is equl to: P{}", x4);


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

    /*Arrays in Rust also have a fixed size. For example:

    let numbers = [10, 20, 30, 40];

    This array has exactly 4 elements.
    Its type is:

    [i32; 4]

    The type means:

    [i32; 4]
    │    │
    │    └── 4 elements
    └─────── each element is i32
    You cannot change its size

    You cannot do something like:

    let numbers = [10, 20, 30];

    numbers.push(40); // ❌

    An array's size is fixed once it is created.
    */

    let array_1 = [1,2,3,4];
    println!("The first element of array_1 with index 0 is: {}",array_1[0]);
    println!("The first element of array_1 with index 1 is: {}",array_1[1]);
    println!("The first element of array_1 with index 2 is: {}",array_1[2]);
    println!("The first element of array_1 with index 3 is: {}",array_1[3]);

    let array_2: [i32; 4] = [10,20,30,40];
    println!("The first element of array_2 with index 0 is: {}",array_2[0]);
    println!("The first element of array_2 with index 1 is: {}",array_2[1]);
    println!("The first element of array_2 with index 2 is: {}",array_2[2]);
    println!("The first element of array_2 with index 3 is: {}",array_2[3]);

    /*This is wrong because the way we have defined the arrays made them immutable. To make the mutable we should use mut 
    before the variable definition.

    let array_2[0] = 100;
    */

    let mut array_3 = [100,200,300,400];
    println!("The first element of array_3 with index 0 is: {}",array_3[0]);
    println!("The first element of array_3 with index 1 is: {}",array_3[1]);
    println!("The first element of array_3 with index 2 is: {}",array_3[2]);
    println!("The first element of array_3 with index 3 is: {}",array_3[3]);
    println!("-------------------------------------------------------------");

    /* Here we changed the value of the first element of the array*/
    array_3[0] = 1000;
    println!("The first element of array_3 with index 0 is: {}",array_3[0]);
    println!("The first element of array_3 with index 1 is: {}",array_3[1]);
    println!("The first element of array_3 with index 2 is: {}",array_3[2]);
    println!("The first element of array_3 with index 3 is: {}",array_3[3]);


    /*Suppose you have:

    fn main() {
        let numbers = [10, 20, 30];

        println!("{}", numbers[5]);
    }

    The array has indices:
    numbers
    ┌────┬────┬────┐
    │ 10 │ 20 │ 30 │
    └────┴────┴────┘
      0    1    2

    But you're asking for:

    numbers[5]

    There is no index 5.
    Rust detects this at runtime and panics.
    What does "panic" mean?
    In Rust, a panic means:
    The program encountered an unrecoverable error and stops the current execution.
    You can think of it as Rust saying:
    "Something happened that shouldn't happen, and I can't safely continue."
    For example, running the above program might produce:
    thread 'main' panicked at src/main.rs:4:20:
    index out of bounds: the len is 3 but the index is 5
    Then the program terminates. So:

    numbers[5]
        │
        ▼
    Rust checks the index
        │
        ▼
    Is 5 < array length?
        │
        ├── YES → return the element
        │
        └── NO  → panic!
                    │
                    ▼
                program stops
    */

    /*Why does Rust panic instead of just accessing memory?

    This is particularly important if you're coming from C/C++.
    In C, you might do:

    int numbers[3] = {10, 20, 30};

    printf("%d\n", numbers[5]);

    C generally doesn't perform the same bounds check for you.
    You might accidentally read some memory that doesn't belong to the array.
    This can result in undefined behavior.
    Rust is much stricter:

    numbers[5]

    Rust checks the index and says:
    "No. This index doesn't exist."
    Then it panics rather than allowing an invalid array access.
    This is one of the mechanisms Rust uses to provide memory safety.
    Panic doesn't mean the computer crashed
    This is another important distinction.
    When Rust says:
    thread 'main' panicked
    it doesn't mean:
        Windows crashed
        Linux crashed
        your computer crashed
        the CPU crashed

    It means your Rust program terminated because of a panic. For example:

    fn main() {
        println!("Before");

        panic!("Something went wrong!");

        println!("After");
    }

    Output will be something like:
    Before
    thread 'main' panicked at ...
    Something went wrong!
    After is never printed because execution stopped at:
    panic!("Something went wrong!");
    */

    /*You can explicitly cause a panic

    Rust provides a macro called panic!:

    panic!("Something went wrong!");

    For example:

    fn main() {
        let age = 10;

        if age < 18 {
            panic!("You must be 18 or older");
        }
    }

    When the condition is true, the program panics.
    So an out-of-bounds array access is essentially a situation where Rust internally triggers a panic.
    */


}
