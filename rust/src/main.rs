
fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("The value of X is {}", x);



    /************************************************************************************************************
    This will generate error, because the x variable in immutable which is defined by let and we can not change its value
    later in the code.
    
    // x = 6;
    // println!("The value of X is {}", x);
    */



    /************************************************************************************************************
    to solve this issue we can make the x mutable by adding mut to its definition with let. like below code.

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



    /************************************************************************************************************
    This part will generate error because we are setting signed numbers (Negative) to unsigned numbers.
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



    /************************************************************************************************************
    Note: Decimal and hexadecimal are not different data types in Rust. They are different ways of writing integer values 
    (number literals). 
    For example, these all represent the same integer value:

    let a = 42;       // decimal
    let b = 0x2A;     // hexadecimal
    let c = 0b101010; // binary
    let d = 0o52;     // octal

    All four have the value 42.
    */



    /************************************************************************************************************
    Notes: 1. Rust's main data types
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




    /************************************************************************************************************
    Integer types

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



    /************************************************************************************************************
    Unsigned integers

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



    /************************************************************************************************************
    usize and isize

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



    /************************************************************************************************************
    Floating-point types

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



    /************************************************************************************************************
    Boolean

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



    /************************************************************************************************************
    Character

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




    /************************************************************************************************************
    This will generate warning for use because the unsed_variable is defined but is not used.*/
    //let unsed_variable:i8 = 1000;


    /************************************************************************************************************
    We can solve this issue by adding _ to the variable name like below code.*/
    let _unsed_variable:i8 = 120;


    /************************************************************************************************************
    This will create a character variable*/
    let _character = 'A';

    let _byte = b'A';


    /************************************************************************************************************
    when you put b before the character: b'A'you are telling Rust:
    Give me the ASCII/byte value of this character as a u8.
    Therefore:
        let byte = b'A';
    has:
    byte
    │
    └── type: u8
        value: 65
    */

    /************************************************************************************************************
    Compare A and b'A'
    This distinction is extremely important:
    'A'is char
    while:
    b'A'is u8
    */

    /************************************************************************************************************
    Because byte is a u8. You can also print it as hexadecimal:*/

    let byte = b'A';
    println!("The Hex format of the varibale byte is: {:x}", byte);
    println!("The Decimal format of the varibale byte is: {}", byte);

    let _var_float_64 = 2.5; //The default variable size is 64 bit
    let _var_float_32:f32 = 2.5; //This is 32 bit floating point variable 



    //-----------------------------------------------------------------------------------------------



    /************************************************************************************************************
    Tuple

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



    /************************************************************************************************************
    Tuples have a fixed size in Rust.

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



    /************************************************************************************************************
    The type of a tuple includes its size

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



    /************************************************************************************************************
    Tuples can have different types for each element, For example:

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


    //-----------------------------------------------------------------------------------------------

    

    /************************************************************************************************************
    Array
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



    /************************************************************************************************************
    Arrays in Rust also have a fixed size. For example:

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




    /************************************************************************************************************
    This is wrong because the way we have defined the arrays made them immutable. To make the mutable we should use mut 
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




    /************************************************************************************************************
    Suppose you have:

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


    /************************************************************************************************************
    Why does Rust panic instead of just accessing memory?

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


    /************************************************************************************************************
    You can explicitly cause a panic

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




    /************************************************************************************************************
    But sometimes you don't want a panic

    Suppose you're not sure whether an index is valid.
    You can check it yourself:

    fn main() {
        let numbers = [10, 20, 30];
        let index = 5;

        if index < numbers.len() {
            println!("{}", numbers[index]);
        } else {
            println!("Invalid index");
        }
    }

    Now the program doesn't panic.
    Another very important method is .get():

    let numbers = [10, 20, 30];

    let value = numbers.get(5);

    Instead of panicking, .get() returns an Option.
    For an invalid index:

    numbers.get(5)
        │
        ▼
       None

    For a valid index:

    numbers.get(1)
        │
        ▼
     Some(20)

    You'll eventually learn that Option<T> is one of Rust's most important types.
    */



    /************************************************************************************************************
    The big picture

    There are two different approaches:

    Direct indexing
    numbers[index]

    You are telling Rust:

    "I believe this index is valid."

    If it isn't:

    invalid index
        ↓
    panic!
        ↓
    program terminates
    Safe lookup
    numbers.get(index)

    You're saying:

    "I'm not sure whether this index exists. Tell me whether you found it."

                get(index)
                    │
            ┌───────┴───────┐
            ▼               ▼
        Some(value)        None
          found           not found
    */


    //-----------------------------------------------------------------------------------------------


    
    /************************************************************************************************************
    Vector
    A vector in Rust is a collection that stores multiple values of the same type, but unlike an array, 
    its size can change at runtime.

    Vector vs Array

    This is the easiest way to understand a vector.

    Array
    let numbers = [10, 20, 30];

    The array has a fixed size:

    [i32; 3]

    You cannot make it contain 4 elements.

    Vector
    let mut numbers = vec![10, 20, 30];

    The vector can grow:

    numbers.push(40);

    Now:

    numbers
    ┌────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │
    └────┴────┴────┴────┘

    So:

    Array                 Vector
    ─────                 ──────
    [i32; 3]              Vec<i32>
    fixed size            dynamic size
    same type             same type    

    */


    // How do you create a vector? There are several ways. Using vec!
    // The most convenient:

    let numbers_1 = vec![10, 20, 30];

    //Rust infers: Vec<i32>
   
    println!("-------------------------------------------------------------");
    println!("The numbers_1 vector is : {:?}", numbers_1);
    println!("The first element of the number_1 vector is: {}", numbers_1[0]);
    println!("The second element of the number_1 vector is: {}", numbers_1[1]);
    println!("The third element of the number_1 vector is: {}", numbers_1[2]);
    println!("-------------------------------------------------------------");


    // You can also specify the type:
    let mut numbers_2: Vec<i32> = vec![100, 200, 300];


    // Creating an empty vector
    let mut numbers_3: Vec<i32> = Vec::new();

    //Now: numbers = []
    println!("This is the numbers_3 vector: {:#?}", numbers_3);
    // Then:

    numbers_3.push(10);
    numbers_3.push(20);

    //gives: [10, 20]
    println!("This is the numbers_3 vector: {:#?}", numbers_3);
    println!("This is first member of the numbers_3 vector: {}", numbers_3[0]);
    println!("This is second member of the numbers_3 vector: {}", numbers_3[1]);
    println!("-------------------------------------------------------------");

    println!("This is numbers_2 vector: {:?}", numbers_2);
    numbers_2.pop();
    println!("This is numbers_2 vector after first pop: {:?}", numbers_2);
    numbers_2.pop();
    println!("This is numbers_2 vector after second pop: {:?}", numbers_2);
    println!("-------------------------------------------------------------");


    // Accessing elements
    // Just like arrays, you can use an index:

    let numbers = vec![10, 20, 30];

    println!("This is first element of numbers: {}", numbers[1]);

    //Output: 20



    /************************************************************************************************************ 
    The indices are:

    ┌────┬────┬────┐
    │ 10 │ 20 │ 30 │
    └────┴────┴────┘
       0    1    2

    Just like arrays, this:

    numbers[10]

    will panic if the index is outside the vector.
    */

    // .get() works with vectors too

    //You can avoid a panic with:

    let numbers_4 = vec![10, 20, 30];
    let value = numbers_4.get(10);
    println!("This is value with get(10): {:?}", value);




    /************************************************************************************************************
    This returns: None
    rather than panicking.
    For a valid index: */

    let value = numbers_4.get(1);
    println!("This is value with get(1): {:?}", value);
    // you get: Some(20)
    //This is where the Option type we discussed earlier becomes very useful.

    //-----------------------------------------------------------------------------------------------




    /************************************************************************************************************
    How does a vector actually work in memory?

    This is particularly important for you because you're interested in low-level/system programming.

    A Vec<T> is not simply a fixed block of memory like an array.

    Conceptually, a vector contains three important pieces of information:

    Vec<T>
    ┌─────────────────────────┐
    │ pointer                 │ ──────┐
    │ length                  │       │
    │ capacity                │       ▼
    └─────────────────────────┘   heap memory
                                    ┌────┬────┬────┬──────┐
                                    │ 10 │ 20 │ 30 │unused│
                                    └────┴────┴────┴──────┘
    Pointer

    The vector needs to know where its elements are stored in memory.

    Length

    How many elements currently exist.

    For:

    let numbers = vec![10, 20, 30];

    the length is:

    3
    Capacity

    How many elements the currently allocated memory can hold before another allocation is required.

    For example, conceptually:

    length   = 3
    capacity = 4

    means:

    ┌────┬────┬────┬──────┐
    │ 10 │ 20 │ 30 │ free │
    └────┴────┴────┴──────┘
    ↑              ↑
    used           available

    The exact capacity chosen by Rust is an implementation detail; don't assume it will always be 4.

    What happens when the vector becomes full?

    Suppose conceptually:

    length   = 4
    capacity = 4

    and you execute:

    numbers.push(50);

    There isn't enough allocated space for another element.

    Rust's vector implementation can allocate a larger memory region, move/copy the existing elements there, and then add the new element.

    Conceptually:

    Before:

    heap
    ┌────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │
    └────┴────┴────┴────┘
    capacity = 4


            push(50)
                │
                ▼

    allocate larger region

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘

    The important idea is:

    A vector manages a dynamically allocated region of memory for you.

    You don't manually call malloc() and free() like you might in C.
    */
    //-----------------------------------------------------------------------------------------------




    /************************************************************************************************************
    Vector elements must have the same type

    This is just like an array.
    This works:

    let numbers = vec![10, 20, 30];

    because all are integers. This doesn't:

    let values = vec![10, true, 3.14]; // ❌

    because these have different types:

    10    → integer
    true  → bool
    3.14  → floating point

    A Vec<T> has one T.

    For example:

    Vec<i32>
    Vec<u8>
    Vec<String>
    Vec<bool>
    */



    /************************************************************************************************************
    Vector vs tuple vs array

    At this point you can think about the three like this:

    Tuple
    let x = (10, true, 3.14);
    fixed size
    different types allowed

    Array
    let x = [10, 20, 30];
    fixed size
    same type required

    Vector
    let x = vec![10, 20, 30];
    dynamic size
    same type required
    */


    // This is another way to create a vector
    // Vec should start with a capital V
    // We should specify the type of the elements in vector when we define them with Vec, so the rust knows the type of the 
    // elements which are going to store in the vector
    let mut new_vec:Vec<i32> = Vec::new();
    new_vec.push(10);
    new_vec.push(20);
    new_vec.push(30);
    println!("-------------------------------------------------------------");
    println!("This is the whole vector on new_vec: {:?}", new_vec);
    println!("This is the first element of new_vec: {}", new_vec[0]);
    println!("This is the second element of new_vec: {}", new_vec[1]);
    println!("This is the third element of new_vec: {}", new_vec[2]);
    println!("-------------------------------------------------------------");


    let mut new_vec2:Vec<i32> = Vec::new();
    new_vec2.push(100);
    new_vec2.push(200);
    new_vec2.push(300);
    new_vec2.push(400);
    println!("This is new_vec2 whole member: {:?}", new_vec2);


    //This will reverse the new_vec2 vector
    new_vec2.reverse();
    println!("This is reversed new_vec2: {:?}", new_vec2);

    println!("-------------------------------------------------------------");

    let mut new_vec3 = Vec::<i32>::with_capacity(10);
    println!("This is the length of the new_vec3: {}", new_vec3.capacity());
    new_vec3.push(999);
    println!("-------------------------------------------------------------");

    // Another way to specify the type of elements in the vector.
    // Here, i8 is specified directly in Vec::<i8>.
    //One way is after the name of the variable.
    //One way is after the Vec which we define the vector with it.
    let mut new_vec4 = Vec::<i8>::new();
    println!("This is the length of the new_vec4: {}", new_vec4.capacity());
    new_vec4.push(1);
    println!("This is the length of the new_vec4 after first push: {}", new_vec4.capacity());
    new_vec4.push(2);
    println!("This is the length of the new_vec4 after second push: {}", new_vec4.capacity());
    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");



    /************************************************************************************************************
    Yes — your first part is correct, but there is one important mistake in your comments, and the second 
    part is about the difference between length and capacity.

    Your code:

    let mut new_vec4 = Vec::<i8>::new();

    println!("This is the length of the new_vec4: {}", new_vec4.capacity());

    new_vec4.push(1);

    println!("This is the length of the new_vec4 after first push: {}", new_vec4.capacity());

    new_vec4.push(2);

    println!("This is the length of the new_vec4 after second push: {}", new_vec4.capacity());
    1. Vec::<i8>::new() is correct

    This:

    Vec::<i8>::new()

    means:

    Create a new empty Vec, and explicitly tell Rust that the elements inside it will be i8.

    The syntax:

    Vec::<i8>::new()
        ^^^
        type of elements

    The ::<i8> part is called turbofish syntax.

    So these are two different ways of telling Rust the element type:

    let mut new_vec: Vec<i8> = Vec::new();

    and:

    let mut new_vec = Vec::<i8>::new();

    Both mean essentially:

    new_vec is a vector whose elements are i8.

    2. The important mistake: you're printing capacity(), not length()

    You wrote:

    new_vec4.capacity()

    But capacity() does not tell you how many elements are currently inside the vector.

    For the number of elements, use:

    new_vec4.len()

    So:

    println!("Length: {}", new_vec4.len());

    And for the allocated capacity:

    println!("Capacity: {}", new_vec4.capacity());

    These are two different things.

    3. What is the difference?

    Imagine your vector looks like this:

    new_vec4
    │
    ▼
    ┌──────────────┐
    │ pointer      │ ──────────────┐
    │ length       │               │
    │ capacity     │               │
    └──────────────┘               │
                                ▼
                            ┌────┬────┬────┬────┬────┐
                            │ 1  │ 2  │ ?  │ ?  │ ?  │
                            └────┴────┴────┴────┴────┘

    Suppose:

    length   = 2
    capacity = 5

    This means:

    There are 2 actual elements in the vector, but the allocated memory has room for 5 elements.

    So:

    length
    ↓
    ┌────┬────┐
    │ 1  │ 2  │
    └────┴────┘
    ↑
    2 elements actually exist


    capacity
    ↓
    ┌────┬────┬────┬────┬────┐
    │ 1  │ 2  │    │    │    │
    └────┴────┴────┴────┴────┘
    <--------->
    room for 5 i8s

    The empty spaces are allocated memory, but they don't represent elements that you've pushed.

    4. Let's fix your program

    Try this:

    let mut new_vec4 = Vec::<i8>::new();

    println!("Length: {}", new_vec4.len());
    println!("Capacity: {}", new_vec4.capacity());

    new_vec4.push(1);

    println!("Length: {}", new_vec4.len());
    println!("Capacity: {}", new_vec4.capacity());

    new_vec4.push(2);

    println!("Length: {}", new_vec4.len());
    println!("Capacity: {}", new_vec4.capacity());

    You might see something like:

    Length: 0
    Capacity: 0

    Length: 1
    Capacity: 4

    Length: 2
    Capacity: 4

    The exact capacity is not guaranteed to be 4. Your Rust version/allocator/implementation may produce different values.

    But the important thing is:

    After creating:

    length = 0
    capacity = 0


    After push(1):

    length = 1
    capacity = maybe 4


    After push(2):

    length = 2
    capacity = maybe 4
    5. Why doesn't capacity increase when you push 2?

    This is actually the key idea behind Vec.

    Suppose Rust initially allocates space for 4 i8 elements:

    capacity = 4
    length   = 0

    ┌────┬────┬────┬────┐
    │    │    │    │    │
    └────┴────┴────┴────┘

    Then:

    new_vec4.push(1);

    Now:

    length   = 1
    capacity = 4

    ┌────┬────┬────┬────┐
    │ 1  │    │    │    │
    └────┴────┴────┴────┘

    You have one element, but memory for four.

    Then:

    new_vec4.push(2);

    Rust doesn't need to allocate anything new because there is already available space:

    length   = 2
    capacity = 4

    ┌────┬────┬────┬────┐
    │ 1  │ 2  │    │    │
    └────┴────┴────┴────┘

    So:

    length changed:
    0 → 1 → 2

    capacity didn't need to change:
    0 → 4 → 4

    That's exactly what you are observing.

    6. Why does Rust allocate extra space?

    Because allocating memory every time you call push() would be inefficient.

    Imagine Rust did this:

    let mut v = Vec::<i8>::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    If every push() required a new allocation, it would conceptually have to do:

    push(1)
    allocate memory for 1

    push(2)
    allocate new memory for 2
    copy 1
    free old memory

    push(3)
    allocate new memory for 3
    copy 1,2
    free old memory

    push(4)
    allocate new memory for 4
    copy 1,2,3
    free old memory

    ...

    That would be expensive.

    Instead, Vec generally allocates more space than immediately necessary.

    For example:

                        allocated memory
                            │
                            ▼
                    ┌────┬────┬────┬────┐
                    │ 1  │ 2  │    │    │
                    └────┴────┴────┴────┘
                    └──────┘
                    actual elements

                    └───────────────┘
                        capacity

    Then the next push() can use the already allocated space.

    7. What happens when capacity is finally full?

    Suppose:

    length   = 4
    capacity = 4

    Memory:

    ┌────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │
    └────┴────┴────┴────┘

    Now you do:

    v.push(50);

    There is no free space.

    Rust has to obtain a larger allocation.

    Conceptually:

    OLD:

    ┌────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │
    └────┴────┴────┴────┘
    capacity = 4

    It obtains a larger memory area:

    NEW:

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘
    capacity = 8   ← example only

    The old elements are moved/copied into the new allocation, and the old allocation is released.

    Important: Rust's exact capacity-growth strategy is an implementation detail. Don't write code assuming that capacity always doubles.

    8. So there are three different concepts you should keep separate

    For a Vec, think about:

    len()

    How many elements are currently in the vector?

    v.len()

    Example:

    ┌────┬────┬────┬────┐
    │ 10 │ 20 │    │    │
    └────┴────┴────┴────┘
    ↑──────↑
    len = 2
    capacity()

    How many elements can the current allocation hold before another allocation may be necessary?

    v.capacity()

    Example:

    ┌────┬────┬────┬────┐
    │ 10 │ 20 │    │    │
    └────┴────┴────┴────┘
    <------------>
    capacity = 4
    push()

    Adds one element:

    v.push(30);

    So normally:

    len increases by 1

    but:

    capacity only increases when necessary
    One more correction to your comment

    You wrote:

    "This is other wat for specifying the type of variables inside the vector."

    I'd phrase it more accurately as:

    // Another way to specify the type of elements in the vector.
    // Here, i8 is specified directly in Vec::<i8>.
    let mut new_vec4 = Vec::<i8>::new();

    Because i8 is the type of the elements, not really the "type of variables inside the vector."

    And this:

    println!("This is the length: {}", new_vec4.capacity());

    should be:

    println!("This is the capacity: {}", new_vec4.capacity());

    If you want length:

    println!("This is the length: {}", new_vec4.len());

    The fact that you saw the number not change after push(1) and push(2) is actually a good observation—it means you're 
    starting to see the difference between the vector's length and its allocated memory capacity.

    */



    let mut new_vec4 = Vec::<i8>::new();

    println!("Length: {}", new_vec4.len());
    println!("Capacity: {}", new_vec4.capacity());

    new_vec4.push(1);

    println!("Length: {}", new_vec4.len());
    println!("Capacity: {}", new_vec4.capacity());

    new_vec4.push(2);

    println!("Length: {}", new_vec4.len());
    println!("Capacity: {}", new_vec4.capacity());

    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");


    let mut v = Vec::<i8>::new();

    for i in 0..100 {
        v.push(i);

        println!(
            "After push {:3}: length = {:3}, capacity = {:3}",
            i + 1,
            v.len(),
            v.capacity()
        );
    }



    /************************************************************************************************************
    Yes, your observation is correct, but the conclusion needs one important adjustment:

    Rust does not guarantee how much extra capacity a Vec gets when it grows.

    Your version happened to do this:

    Vec::new()

    Length     Capacity
    0           0

    push(1)

    Length     Capacity
    1           8

    push(2)

    Length     Capacity
    2           8

    That's perfectly valid Rust behavior.

    Why 8?

    Your vector is:

    let mut v = Vec::<i8>::new();

    An i8 occupies 1 byte.

    So when Rust decides it needs to allocate space for the first element, your current Rust implementation/allocator 
    chose enough memory for 8 i8 elements:

    8 × 1 byte = 8 bytes

    Conceptually:

    capacity = 8

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │    │    │    │    │    │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘
    ↑
    8 i8 elements

    After:

    v.push(1);

    you have:

    length = 1
    capacity = 8

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │  1 │    │    │    │    │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘

    After:

    v.push(2);

    you have:

    length = 2
    capacity = 8

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │  1 │  2 │    │    │    │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘

    Nothing needs to be allocated for 2, because there is already room.

    How can you see how YOUR Rust version grows it?

    The easiest way is to repeatedly push() elements and print both len() and capacity().

    Try this:

    fn main() {
        let mut v = Vec::<i8>::new();

        for i in 0..100 {
            v.push(i);

            println!(
                "After push {:3}: length = {:3}, capacity = {:3}",
                i + 1,
                v.len(),
                v.capacity()
            );
        }
    }

    You may get something similar to:

    After push   1: length =   1, capacity =   8
    After push   2: length =   2, capacity =   8
    After push   3: length =   3, capacity =   8
    After push   4: length =   4, capacity =   8
    After push   5: length =   5, capacity =   8
    After push   6: length =   6, capacity =   8
    After push   7: length =   7, capacity =   8
    After push   8: length =   8, capacity =   8
    After push   9: length =   9, capacity =  16
    ...

    Then you'll probably see another jump:

    capacity = 8
        ↓
    capacity = 16
        ↓
    capacity = 32
        ↓
    capacity = 64
        ↓
    ...

    But do not assume those exact numbers are guaranteed.

    Now to your more important question

    You said:

    "I think it is weird that in every system rust will allocate different space for vectors and it will make it 
    uncertain in different situations and systems?"

    This is a very good systems-programming question.

    And the answer is:

    Yes, the exact capacity can be different. But no, this does not make Rust programs uncertain or unreliable.

    The important distinction is:

    Rust guarantees the behavior you can depend on.

    For example, Rust guarantees that:

    let mut v = Vec::<i8>::new();

    v.push(10);
    v.push(20);

    will give you a vector containing:

    10
    20

    and:

    v.len()

    will return:

    2

    But Rust does not promise:

    capacity == 8

    after the first push.

    So your program should not depend on that.

    Think about capacity as an implementation detail

    Suppose you write:

    let mut v = Vec::<i8>::new();

    v.push(10);
    v.push(20);
    v.push(30);

    Your program cares about:

    Elements:
    10
    20
    30

    Length:
    3

    It generally doesn't care whether Rust internally allocated:

    capacity = 4

    or:

    capacity = 8

    or:

    capacity = 16

    All three can correctly represent your vector.

    Why doesn't Rust specify the growth strategy?

    Because Rust wants the implementation to have freedom to improve its memory-management strategy.

    Imagine Rust developers discover a better strategy for a particular platform.

    If Rust had promised:

    "Every Vec always doubles its capacity."

    then changing that behavior could break programs that incorrectly relied on it.

    Instead, Rust essentially says:

    "I guarantee what Vec does as a collection, but I don't promise exactly how much spare memory it allocates."

    That gives the implementation freedom.

    This is extremely common in systems programming.

    But you CAN control capacity yourself

    This is where it gets interesting.

    Suppose you know beforehand that you're going to put approximately 1,000 elements into a vector.

    You can write:

    let mut v = Vec::<i8>::with_capacity(1000);

    Now you're explicitly asking Rust to allocate capacity for at least 1,000 elements.

    You can check it:

    println!("Length: {}", v.len());
    println!("Capacity: {}", v.capacity());

    You might get:

    Length: 0
    Capacity: 1000

    Then:

    v.push(10);

    gives:

    Length: 1
    Capacity: 1000

    Then:

    v.push(20);

    gives:

    Length: 2
    Capacity: 1000

    So you're telling Rust:

    "I expect to need this much space. Please prepare it."

    This can be useful for performance optimization, because it can avoid repeated reallocations as the vector grows.

    There is another very important concept here

    Don't confuse:

    Vec::<i8>::new()

    with:

    Vec::<i8>::with_capacity(1000)

    The first says:

    "Give me an empty vector. I don't currently need any elements."

    The second says:

    "Give me an empty vector, but prepare storage for at least 1,000 elements."

    So:

    new()

    Length:   0
    Capacity: 0 initially

    versus:

    with_capacity(1000)

    Length:   0
    Capacity: 1000

    Notice that both vectors have zero elements.

    The difference is the amount of memory prepared for future elements.

    And one final subtle point

    You said:

    "in every system Rust will allocate different space"

    It's better to say:

    The exact capacity growth is not guaranteed to be the same across all Rust versions, platforms, allocators, or 
    implementation details.

    That doesn't mean every computer necessarily gives you a different result.

    For example, many systems might give:

    8 → 16 → 32 → 64

    But Rust deliberately does not make that sequence something your program should rely upon.

    So for normal programming:

    len()       ← information you can depend on
    capacity()  ← useful information, but exact growth is implementation-dependent

    And this distinction is going to become very important when we get to memory allocation, pointers, ownership, 
    borrowing, and reallocation, because when a Vec runs out of capacity, its underlying memory may have to move to a 
    different location.
    */




    /************************************************************************************************************
    The reason is that an empty Vec initially has no allocated element buffer at all. When you perform the first push(), 
    Rust has to allocate memory, and your current implementation chooses a capacity of 8.

    Let's go through your code one line at a time.

    1. When you create the vector
    let mut new_vec4 = Vec::<i8>::new();

    At this moment:

    Length    = 0
    Capacity  = 0

    Why?

    Because you said:

    Create a new vector, and I don't need any elements yet.

    Rust doesn't need to allocate memory for elements yet.

    Conceptually:

    new_vec4
    ┌──────────────┐
    │ pointer      │ ──────> nothing allocated
    │ length = 0   │
    │ capacity = 0 │
    └──────────────┘

    There is no reason to allocate memory for 8 i8s when you haven't put anything into the vector.

    So:

    println!("Capacity: {}", new_vec4.capacity());

    prints:

    Capacity: 0
    2. Then you execute the first push()
    new_vec4.push(1);

    Now Rust has a problem:

    "The vector currently has capacity 0, but the user wants to store one element."

    It therefore needs to allocate memory.

    Your Rust implementation decides to allocate enough space for 8 i8 elements.

    Remember:

    i8 = 1 byte

    Therefore:

    8 i8s = 8 bytes

    Conceptually, memory now looks like:

                        allocated memory
                            ↓
    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │  1 │    │    │    │    │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘
    ↑
    actual element

    length   = 1
    capacity = 8

    So your output becomes:

    Length: 1
    Capacity: 8

    This is the important distinction:

    Length = how many elements you actually have.

    Capacity = how many elements the current allocation can hold.

    3. Then you execute the second push()
    new_vec4.push(2);

    Rust checks:

    Current length:    1
    Current capacity:  8

    You're asking it to add one more element.

    After adding it:

    Length would become 2

    Is:

    2 <= 8

    Yes.

    So Rust doesn't need to allocate anything.

    It simply puts 2 into the already-allocated space:

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │  1 │  2 │    │    │    │    │    │    │
    └────┴────┴────┴────┴────┴────┴────┴────┘
    <──────>
    length = 2

    <──────────────────────────────────────>
                capacity = 8

    Therefore:

    Length: 2
    Capacity: 8
    So the whole process is
    Vec::<i8>::new()
            │
            ▼
    ┌─────────────────────────┐
    │ length   = 0            │
    │ capacity = 0            │
    │ no element allocation   │
    └─────────────────────────┘
            │
            │ push(1)
            ▼
    ┌─────────────────────────┐
    │ length   = 1            │
    │ capacity = 8            │
    │                         │
    │ [1][ ][ ][ ][ ][ ][ ][ ]│
    └─────────────────────────┘
            │
            │ push(2)
            ▼
    ┌─────────────────────────┐
    │ length   = 2            │
    │ capacity = 8            │
    │                         │
    │ [1][2][ ][ ][ ][ ][ ][ ]│
    └─────────────────────────┘

    The important event is the first push().

    That's when your vector goes from:

    capacity 0

    to:

    capacity 8

    After that, Rust can keep using that allocated memory until it fills up.

    What happens at the 9th push()?

    This is where the behavior you've been asking about becomes really interesting.

    If your vector has:

    length   = 8
    capacity = 8

    then:

    new_vec4.push(9);

    cannot simply put 9 into the existing allocation.

    There is no room:

    ┌────┬────┬────┬────┬────┬────┬────┬────┐
    │ 1  │ 2  │ 3  │ 4  │ 5  │ 6  │ 7  │ 8  │
    └────┴────┴────┴────┴────┴────┴────┴────┘
                                        ↑
                                capacity full

    Rust then has to grow the allocation.

    Your system might do something like:

    capacity 8
        ↓
    capacity 16

    and move the existing elements into the new allocation.

    That's why Vec is closely connected to heap memory, pointers, allocation, reallocation, and eventually ownership and borrowing.

    One very useful experiment now is to change your program to push 20 or 30 elements and print len() and capacity() 
    after every push. That will let you watch your particular Rust implementation grow the vector in real time.
    */


    println!("-------------------------------------------------------------");
    let vector_from_iterator:Vec<i32>= (0..5).collect();
    println!("The value of vector_from_iterator is: {:?}", vector_from_iterator);



    //-----------------------------------------------------------------------------------------------

    // This is how we define slices in rust

    let sv: &[i32] = &vector_from_iterator;
    println!("This is sv slice which points to the whole of vector_from_iterator: {:?}", sv);

    let sv2: &[i32] = &vector_from_iterator[2..4];
    println!("This is sv2 slice which points to the index 2 to 4 of vector_from_iterator: {:?}", sv2);



    /************************************************************************************************************
    A slice in Rust is a way to refer to part of a collection without owning that collection.

    Since you're learning Vec right now, the easiest way to understand slices is to start with a vector.

    Suppose we have:

    let numbers = vec![10, 20, 30, 40, 50];

    The vector contains five elements:

    index:    0    1    2    3    4
            ↓    ↓    ↓    ↓    ↓
            ┌────┬────┬────┬────┬────┐
            │ 10 │ 20 │ 30 │ 40 │ 50 │
            └────┴────┴────┴────┴────┘

    Now imagine that we only want to work with:

    20, 30, 40

    We can create a slice:

    let part = &numbers[1..4];

    Now:

    numbers:
    ┌────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │
    └────┴────┴────┴────┴────┘
        └──────────────┘
            slice

    part refers to elements at indexes:

    1, 2, 3

    so:

    println!("{:?}", part);

    prints:

    [20, 30, 40]
    The important part: what does & mean?

    This is where slices become connected to ownership and borrowing.

    We wrote:

    let part = &numbers[1..4];

    There are actually two concepts here:

    [1..4]

    means:

    Select the range from index 1 up to, but NOT including, index 4.

    And:

    &

    means:

    Borrow/refer to this data rather than taking ownership of it.

    So:

    &numbers[1..4]

    means roughly:

    "Give me a borrowed view of this portion of numbers."

    The slice does not create a new vector.

    That's extremely important.

    Slice vs Vec

    Consider:

    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    You now have:

    numbers
    │
    ▼
    ┌────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │
    └────┴────┴────┴────┴────┘
        ▲         ▲
        │         │
        └─────────┘
            part

    part doesn't contain another copy of:

    20, 30, 40

    Instead, it refers to the existing memory.

    Conceptually:

    numbers
    │
    │ owns
    ▼
    ┌────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │
    └────┴────┴────┴────┴────┘
        ▲         ▲
        │         │
        └─────────┘
            borrowed
            slice

    This is one of the reasons slices are very useful.

    What is the type of a slice?

    For a vector of i32:

    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    the type of part is:

    &i32

    No — careful! That's not correct.

    The type is:

    &[i32]

    Read this as:

    a reference (&) to a slice ([i32]) of i32 values.

    So:

    &i32
    │
    └── reference to ONE i32


    &[i32]
    │
    └── reference to a SLICE of i32 values

    For example:

    let x = &numbers[2];

    is a reference to one element:

    &i32

    while:

    let x = &numbers[1..4];

    is a reference to multiple elements:

    &[i32]
    Why does [1..4] contain 3 elements?

    Rust uses the convention:

    start..end

    where start is included and end is excluded.

    So:

    numbers[1..4]

    means:

    index 1
    index 2
    index 3

    but not index 4.

    Therefore:

    numbers:

    index       0    1    2    3    4
                ↓    ↓    ↓    ↓    ↓
            10   20   30   40   50
                    └───────┘
                    1..4

    The slice contains:

    20, 30, 40
    You can also omit the beginning or end

    For example:

    &numbers[..3]

    means:

    From the beginning through index 2.

    So:

    [10, 20, 30]

    Similarly:

    &numbers[2..]

    means:

    From index 2 until the end.

    So:

    [30, 40, 50]

    And:

    &numbers[..]

    means:

    The entire vector as a slice.

    So:

    let all = &numbers[..];

    gives you a &[i32] referring to the entire vector.

    Why not just use the Vec directly?

    This is one of the biggest reasons slices exist.

    Imagine you write a function that needs to process some numbers.

    You could write:

    fn print_numbers(numbers: &Vec<i32>) {
        println!("{:?}", numbers);
    }

    But this function specifically expects a Vec.

    A slice is more general:

    fn print_numbers(numbers: &[i32]) {
        println!("{:?}", numbers);
    }

    Now the function doesn't care whether the data came from a Vec, an array, or another slice.

    For example:

    let numbers = vec![10, 20, 30, 40];

    print_numbers(&numbers);

    You can pass the vector as a slice.

    You can also have an array:

    let numbers = [10, 20, 30, 40];

    print_numbers(&numbers);

    Both can work because both can be viewed as a slice of i32.

    This is the deeper idea

    A slice is basically a view into a contiguous sequence of elements.

    It doesn't own those elements.

    Think of a book.

    Suppose you own a 500-page book.

    The book is like the Vec:

    Vec
    ┌───────────────────────────┐
    │ Page 1 ... Page 500       │
    └───────────────────────────┘

    You ask someone:

    "Give me pages 100–150."

    They don't need to photocopy those 50 pages.

    They can simply give you a reference saying:

    "Look at pages 100 through 150 of that book."

    That's conceptually what a slice does.

    The original collection owns the data.

    The slice borrows a portion of it.

    And this connects directly to the memory discussion we just had

    Remember that we said a Vec conceptually contains:

    Vec
    ┌────────────────┐
    │ pointer        │ ──────┐
    │ length         │       │
    │ capacity       │       │
    └────────────────┘       │
                            ▼
                        ┌────┬────┬────┬────┬────┐
                        │ 10 │ 20 │ 30 │ 40 │ 50 │
                        └────┴────┴────┴────┴────┘

    A slice doesn't need to own this allocation.

    It can essentially say:

    "I start here"
        +
    "and contain this many elements"

    Conceptually:

    Vec
                        ┌────┬────┬────┬────┬────┐
                        │ 10 │ 20 │ 30 │ 40 │ 50 │
                        └────┴────┴────┴────┴────┘
                            ▲              ▲
                            │              │
                            │              │
                        start          length
                            │
                            └──────────────┘
                                slice

    A slice does not have a capacity in the same sense that a Vec does.

    That's because the slice doesn't manage the allocation.

    The Vec owns and manages the underlying memory.

    The slice just gives you a view into part of it.

    One important distinction to remember

    You will encounter these three things constantly:

    Vec<i32>

    A vector that owns its data and can grow/shrink.

    [i32; 5]

    An array containing exactly 5 i32s.

    And:

    &[i32]

    A borrowed slice of i32s.

    A very useful mental model is:

    Vec<i32>
    │
    │ can create a view of
    ▼
    &[i32]

    For example:

    let numbers = vec![10, 20, 30, 40, 50];

    let slice = &numbers[1..4];

    Here:

    numbers
    ↓
    Vec<i32>
    owns the data
        │
        │ borrow
        ▼
    slice
    ↓
    &[i32]
    doesn't own the data

    This concept becomes very important when we get to ownership and borrowing, because slices are one of the best 
    examples of why Rust needs its ownership system in the first place.
    */

    //-----------------------------------------------------------------------------------------------

    //This is how we define a string in rust
    let name  = String::from("Tylor");
    let course = "Rust".to_string();
    let new_name = name.replace("Tylor", "EFE");

    println!("-------------------------------------------------------------");
    println!("This is name String variable: {}", name);
    println!("This is course String variable: {}", course);
    println!("This is new_name String variable: {}", new_name);




    /************************************************************************************************************
    A slice in Rust is a way to refer to part of a collection without owning that collection.

    Since you're learning Vec right now, the easiest way to understand slices is to start with a vector.

    Suppose we have:

    let numbers = vec![10, 20, 30, 40, 50];

    The vector contains five elements:

    index:    0    1    2    3    4
            ↓    ↓    ↓    ↓    ↓
            ┌────┬────┬────┬────┬────┐
            │ 10 │ 20 │ 30 │ 40 │ 50 │
            └────┴────┴────┴────┴────┘

    Now imagine that we only want to work with:

    20, 30, 40

    We can create a slice:

    let part = &numbers[1..4];

    Now:

    numbers:
    ┌────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │
    └────┴────┴────┴────┴────┘
        └──────────────┘
            slice

    part refers to elements at indexes:

    1, 2, 3

    so:

    println!("{:?}", part);

    prints:

    [20, 30, 40]
    The important part: what does & mean?

    This is where slices become connected to ownership and borrowing.

    We wrote:

    let part = &numbers[1..4];

    There are actually two concepts here:

    [1..4]

    means:

    Select the range from index 1 up to, but NOT including, index 4.

    And:

    &

    means:

    Borrow/refer to this data rather than taking ownership of it.

    So:

    &numbers[1..4]

    means roughly:

    "Give me a borrowed view of this portion of numbers."

    The slice does not create a new vector.

    That's extremely important.

    Slice vs Vec

    Consider:

    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    You now have:

    numbers
    │
    ▼
    ┌────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │
    └────┴────┴────┴────┴────┘
        ▲         ▲
        │         │
        └─────────┘
            part

    part doesn't contain another copy of:

    20, 30, 40

    Instead, it refers to the existing memory.

    Conceptually:

    numbers
    │
    │ owns
    ▼
    ┌────┬────┬────┬────┬────┐
    │ 10 │ 20 │ 30 │ 40 │ 50 │
    └────┴────┴────┴────┴────┘
        ▲         ▲
        │         │
        └─────────┘
            borrowed
            slice

    This is one of the reasons slices are very useful.

    What is the type of a slice?

    For a vector of i32:

    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    the type of part is:

    &i32

    No — careful! That's not correct.

    The type is:

    &[i32]

    Read this as:

    a reference (&) to a slice ([i32]) of i32 values.

    So:

    &i32
    │
    └── reference to ONE i32


    &[i32]
    │
    └── reference to a SLICE of i32 values

    For example:

    let x = &numbers[2];

    is a reference to one element:

    &i32

    while:

    let x = &numbers[1..4];

    is a reference to multiple elements:

    &[i32]
    Why does [1..4] contain 3 elements?

    Rust uses the convention:

    start..end

    where start is included and end is excluded.

    So:

    numbers[1..4]

    means:

    index 1
    index 2
    index 3

    but not index 4.

    Therefore:

    numbers:

    index       0    1    2    3    4
                ↓    ↓    ↓    ↓    ↓
            10   20   30   40   50
                    └───────┘
                    1..4

    The slice contains:

    20, 30, 40
    You can also omit the beginning or end

    For example:

    &numbers[..3]

    means:

    From the beginning through index 2.

    So:

    [10, 20, 30]

    Similarly:

    &numbers[2..]

    means:

    From index 2 until the end.

    So:

    [30, 40, 50]

    And:

    &numbers[..]

    means:

    The entire vector as a slice.

    So:

    let all = &numbers[..];

    gives you a &[i32] referring to the entire vector.

    Why not just use the Vec directly?

    This is one of the biggest reasons slices exist.

    Imagine you write a function that needs to process some numbers.

    You could write:

    fn print_numbers(numbers: &Vec<i32>) {
        println!("{:?}", numbers);
    }

    But this function specifically expects a Vec.

    A slice is more general:

    fn print_numbers(numbers: &[i32]) {
        println!("{:?}", numbers);
    }

    Now the function doesn't care whether the data came from a Vec, an array, or another slice.

    For example:

    let numbers = vec![10, 20, 30, 40];

    print_numbers(&numbers);

    You can pass the vector as a slice.

    You can also have an array:

    let numbers = [10, 20, 30, 40];

    print_numbers(&numbers);

    Both can work because both can be viewed as a slice of i32.

    This is the deeper idea

    A slice is basically a view into a contiguous sequence of elements.

    It doesn't own those elements.

    Think of a book.

    Suppose you own a 500-page book.

    The book is like the Vec:

    Vec
    ┌───────────────────────────┐
    │ Page 1 ... Page 500       │
    └───────────────────────────┘

    You ask someone:

    "Give me pages 100–150."

    They don't need to photocopy those 50 pages.

    They can simply give you a reference saying:

    "Look at pages 100 through 150 of that book."

    That's conceptually what a slice does.

    The original collection owns the data.

    The slice borrows a portion of it.

    And this connects directly to the memory discussion we just had

    Remember that we said a Vec conceptually contains:

    Vec
    ┌────────────────┐
    │ pointer        │ ──────┐
    │ length         │       │
    │ capacity       │       │
    └────────────────┘       │
                            ▼
                        ┌────┬────┬────┬────┬────┐
                        │ 10 │ 20 │ 30 │ 40 │ 50 │
                        └────┴────┴────┴────┴────┘

    A slice doesn't need to own this allocation.

    It can essentially say:

    "I start here"
        +
    "and contain this many elements"

    Conceptually:

    Vec
                        ┌────┬────┬────┬────┬────┐
                        │ 10 │ 20 │ 30 │ 40 │ 50 │
                        └────┴────┴────┴────┴────┘
                            ▲              ▲
                            │              │
                            │              │
                        start          length
                            │
                            └──────────────┘
                                slice

    A slice does not have a capacity in the same sense that a Vec does.

    That's because the slice doesn't manage the allocation.

    The Vec owns and manages the underlying memory.

    The slice just gives you a view into part of it.

    One important distinction to remember

    You will encounter these three things constantly:

    Vec<i32>

    A vector that owns its data and can grow/shrink.

    [i32; 5]

    An array containing exactly 5 i32s.

    And:

    &[i32]

    A borrowed slice of i32s.

    A very useful mental model is:

    Vec<i32>
    │
    │ can create a view of
    ▼
    &[i32]

    For example:

    let numbers = vec![10, 20, 30, 40, 50];

    let slice = &numbers[1..4];

    Here:

    numbers
    ↓
    Vec<i32>
    owns the data
        │
        │ borrow
        ▼
    slice
    ↓
    &[i32]
    doesn't own the data

    This concept becomes very important when we get to ownership and borrowing, because slices are one of the best examples 
    of why Rust needs its ownership system in the first place.
    */



    /************************************************************************************************************
    The problem is simply capitalization.

    Your string is:

    let name = String::from("Tylor");

    Notice that the actual value is:

    Tylor
    ^
    uppercase T

    But in replace() you are searching for:

    "tylor"

    which is:

    tylor
    ^
    lowercase t

    Rust's string replacement is case-sensitive.

    So:

    let new_name = name.replace("tylor", "EFE");

    means:

    Find the exact sequence of characters t y l o r inside name and replace it with EFE.

    But your string contains:

    T y l o r

    The first character is different:

    "T" != "t"

    Therefore Rust finds nothing to replace.

    Correct version
    let name = String::from("Tylor");
    let course = "Rust".to_string();

    let new_name = name.replace("Tylor", "EFE");

    println!("{}", new_name);

    Output:

    EFE
    An important thing about replace()

    replace() does not modify the original String.

    Instead, it creates and returns a new String.

    So:

    let name = String::from("Tylor");

    let new_name = name.replace("Tylor", "EFE");

    conceptually gives you:

    name
    ↓
    "Tylor"

    new_name
    ↓
    "EFE"

    The original name is still "Tylor".

    You can verify:

    println!("name = {}", name);
    println!("new_name = {}", new_name);

    Output:

    name = Tylor
    new_name = EFE

    This is different from something like modifying an element of a Vec.

    Also notice that your two ways of creating a String are both valid:

    let name = String::from("Tylor");

    and:

    let course = "Rust".to_string();

    Both produce a String.

    The next concept worth learning here is why Rust has both String and &str. That distinction is fundamental and 
    will connect directly to the slices we just discussed. 
    */



    /************************************************************************************************************
    This is a very important distinction in Rust, and it connects directly to what we just learned about slices.

    Rust has two commonly used string types:

    String

    and

    &str

    The easiest way to understand them is:

    String owns string data. &str is a borrowed view (slice) of string data.

    Let's build this from the beginning.

    1. First, what is a string?

    Suppose we have:

    let name = String::from("Tylor");

    The text:

    Tylor

    is stored as bytes in memory.

    Conceptually:

    Tylor

    T   y   l   o   r
    ↓   ↓   ↓   ↓   ↓
    84  121 108 111 114

    These are UTF-8 bytes.

    The String type owns those bytes.

    So:

    let name = String::from("Tylor");

    creates a String that owns its memory.

    2. What is &str?

    Now consider:

    let name = String::from("Tylor");

    let part = &name[0..2];

    part is not another String.

    It is a string slice:

    &str

    It refers to a portion of the existing string.

    Conceptually:

    name: String
            │
            │ owns
            ▼
    ┌────┬────┬────┬────┬────┐
    │ T  │ y  │ l  │ o  │ r  │
    └────┴────┴────┴────┴────┘
    ▲    ▲
    └────┘
    &str

    The &str doesn't own those bytes.

    It says:

    "I want to look at this part of the existing string."

    3. You have already been using &str without realizing it

    Look at your previous code:

    let name = String::from("Tylor");

    Here:

    "Tylor"

    is a string literal.

    A string literal has type:

    &str

    So:

    let course = "Rust";

    means roughly:

    course
    ↓
    &str

    while:

    let course = "Rust".to_string();

    creates:

    course
    ↓
    String

    This is one of the most important things to understand.

    4. Why is a string literal &str?

    Consider:

    let name = "Tylor";

    Where does "Tylor" live?

    It is part of the program itself. The compiler knows this text when your program is compiled.

    Conceptually:

    Your executable
    ┌───────────────────────────────┐
    │                               │
    │ machine code                  │
    │                               │
    │ "Tylor"                       │
    │                               │
    │ other program data            │
    │                               │
    └───────────────────────────────┘
                ▲
                │
                │
            name

    name doesn't need to own and dynamically allocate "Tylor".

    It can simply borrow/view that string data.

    That's why:

    let name = "Tylor";

    has type:

    &str
    5. Compare these two
    String
    let name = String::from("Tylor");

    This creates an owned, growable string.

    &str
    let name = "Tylor";

    This creates a reference to a string slice.

    So:

    String
    │
    ├── owns the data
    ├── heap allocated in the usual case
    ├── growable
    └── can be modified


    &str
    │
    ├── doesn't own the data
    ├── borrowed view
    ├── not responsible for allocation
    └── cannot be used to grow the underlying string
    6. Why is String growable?

    You can do:

    let mut name = String::from("Tylor");

    name.push('!');

    Now:

    Tylor!

    You can also:

    name.push_str(" Smith");

    Now:

    Tylor Smith

    Because String owns its data, it can manage its memory and potentially allocate more space when necessary.

    7. Can you modify an &str?

    Consider:

    let name = "Tylor";

    You can't do:

    name.push('!');

    because name is an &str.

    It is a borrowed view, not an owned growable string.

    You can create a new String:

    let mut name = "Tylor".to_string();

    name.push('!');

    Now it works because name is a String.

    8. Why does &str have &?

    Remember what we learned about references:

    let x = 10;
    let y = &x;

    y doesn't own x.

    It refers to x.

    Same idea:

    let name = String::from("Tylor");

    let slice = &name[..];

    Here:

    name
    ↓
    String
    owns the data

    slice
    ↓
    &str
    borrows the data

    So you can think:

    &str
    │
    ├── &
    │   reference / borrow
    │
    └── str
        string slice
    9. str vs &str

    You may wonder:

    Why don't we just say str instead of &str?

    This is an important Rust concept.

    str is the slice type itself.

    But a str has a size that isn't known at compile time.

    For example:

    "Hi"

    has 2 UTF-8 bytes.

    But:

    "Hello"

    has 5.

    And:

    "Hello world"

    has 11.

    So a bare str doesn't have a fixed size.

    That's why you normally work with:

    &str

    which is a reference to a string slice.

    You don't normally write:

    let x: str;

    Instead:

    let x: &str;
    10. A very useful memory picture

    Consider:

    let name = String::from("Tylor");
    let slice = &name[1..4];

    Conceptually:

    Stack
    ────────────────────────────

    name
    ┌─────────────────┐
    │ pointer ──────────────┐
    │ length = 5            │
    │ capacity = ...        │
    └─────────────────┘     │
                            │
    slice                   │
    ┌─────────────────┐     │
    │ pointer ──────────────┼───┐
    │ length = 3       │   │   │
    └─────────────────┘     │   │
                            │   │
                            ▼   ▼

    Heap
    ────────────────────────────

    ┌────┬────┬────┬────┬────┐
    │ T  │ y  │ l  │ o  │ r  │
    └────┴────┴────┴────┴────┘
        └────────────┘
            slice

    The String owns the heap allocation.

    The &str simply points into that existing allocation.

    It does not create another copy of "ylo".

    11. This is why slices are efficient

    Suppose you have a huge string:

    "ABCDEFGHIJKLMNOPQRSTUVWXYZ.............."

    and you want only part of it.

    You could make another String, which would involve copying the relevant data.

    Or you can borrow a slice:

    let part = &text[10..20];

    Now you're simply looking at part of the existing data.

    That's cheap because you're not creating another owned string.

    12. A very important function example

    You'll often see Rust functions written like this:

    fn print_name(name: &str) {
        println!("{}", name);
    }

    Why not:

    fn print_name(name: String)

    ?

    Because the function doesn't need to own the string.

    It only needs to read it.

    So &str is a better interface.

    Then you can call it with a string literal:

    print_name("Tylor");

    or with a String:

    let name = String::from("Tylor");

    print_name(&name);

    Both work.

    Conceptually:

    String ──borrow──> &str
                        │
                        ▼
                    print_name()

    This is a major reason you'll see &str everywhere in Rust.

    13. One subtle point: String itself can be sliced

    For example:

    let name = String::from("Tylor");

    let first_two = &name[0..2];

    The types are:

    name
    ↓
    String

    first_two
    ↓
    &str

    So:

    String
    │
    │ borrow a portion
    ▼
    &str

    And if you borrow the whole thing:

    let whole = &name[..];

    you get an &str representing the entire string.

    This is very similar to what we learned with:

    Vec<i32>

    and:

    &[i32]

    In fact, you can see a very useful pattern:

    Vector:
    Vec<T>       → owns the elements
    &[T]         → borrowed slice of elements


    String:
    String       → owns the string data
    &str         → borrowed slice of string data

    That parallel is worth remembering.

    One warning about string slices

    There is one extra complication compared with slices of i32.

    Rust strings use UTF-8, where one character can occupy multiple bytes.

    For example:

    A     → 1 byte
    é     → 2 bytes
    ب     → 2 bytes
    🚀    → 4 bytes

    Therefore, this can be invalid:

    let text = "سلام";

    let part = &text[0..1];

    because byte index 1 may cut through the middle of a UTF-8 character.

    Rust prevents that and will panic rather than giving you invalid UTF-8.

    We'll get into this when we discuss UTF-8, bytes, char, String, and &str, because that's an important part of understanding strings properly.

    The core idea to remember
    String
    ↓
    "I OWN this string data."

    &str
    ↓
    "I BORROW/VIEW some string data."

    And:

    String ───────────── owns ─────────────> data
    │
    │ borrow
    ▼
    &str ───────────── views ─────────────> part/all of data

    That distinction—ownership versus borrowing—is one of the central ideas of Rust.
    */



    /************************************************************************************************************
    Yes, exactly. You have the right idea. The important thing is to distinguish characters from the bytes that represent those characters in UTF-8.

    For your example:

    let name = String::from("Tylor");

    The text "Tylor" is stored as UTF-8 encoded bytes in the string's underlying memory.

    "Tylor" in memory

    The characters are:

    T    y    l    o    r

    In UTF-8, each of these particular characters takes 1 byte, because they are ASCII characters.

    Their byte values are:

    T → 84
    y → 121
    l → 108
    o → 111
    r → 114

    So conceptually the memory contains:

    Memory:

    ┌────────┬────────┬────────┬────────┬────────┐
    │   84   │  121   │  108   │  111   │  114   │
    │   T    │   y    │   l    │   o    │   r    │
    └────────┴────────┴────────┴────────┴────────┘
        1 B      1 B      1 B      1 B      1 B

    So yes, the T is represented by the byte 84 in decimal.

    However, one small correction:

    Memory doesn't really store the number "84" as the characters 8 and 4.

    It stores the 8 bits representing the value 84:

    84 decimal
        ↓
    01010100 binary

    So you can think of the memory as:

    T
    ↓
    UTF-8 encoding
    ↓
    01010100
    ↓
    84 decimal
    ↓
    0x54 hexadecimal

    All of these describe the same byte:

    01010100 = 84 = 0x54 = UTF-8 byte for 'T'
    But this becomes more interesting with non-English characters

    Consider:

    let name = String::from("EFE");

    You get:

    E → 69
    F → 70
    E → 69

    So:

    ┌────┬────┬────┐
    │ 69 │ 70 │ 69 │
    └────┴────┴────┘

    But now consider:

    let name = String::from("ب");

    The Arabic/Persian character ب does not fit into one byte in UTF-8.

    It is represented by multiple bytes:

    ب
    ↓
    UTF-8
    ↓
    two bytes

    So the memory might conceptually look like:

    ┌────────┬────────┐
    │  216   │  168   │
    └────────┴────────┘

    Those two bytes together represent the character ب in UTF-8.

    And this is why I told you earlier that:

    &text[0..1]

    can be problematic with strings.

    Because the indexes of a Rust String are byte positions, not character positions.

    For example:

    "ب"

    takes 2 bytes, not 1:

    byte index:
        0       1
        ↓       ↓
        ┌───────┬───────┐
        │ byte1 │ byte2 │
        └───────┴───────┘

    You cannot take only the first byte and call it a valid UTF-8 string.

    And this connects to String vs &str

    When we say:

    let name = String::from("Tylor");

    the String owns those UTF-8 bytes.

    Conceptually:

    String
    ┌─────────────────────────┐
    │ pointer ────────────────┼──────┐
    │ length                  │      │
    │ capacity                │      │
    └─────────────────────────┘      │
                                    ▼
                            heap memory
                        ┌────┬────┬────┬────┬────┐
                        │ 84 │121 │108 │111 │114 │
                        └────┴────┴────┴────┴────┘

    And when you create:

    let slice = &name[1..4];

    the slice doesn't copy those bytes. It simply refers to:

                y     l     o
                ↓     ↓     ↓
            ┌────┬────┬────┐
            │121 │108 │111 │
            └────┴────┴────┘
            ↑           ↑
            └── &str ───┘

    So you're now seeing the connection between strings → UTF-8 → bytes → slices → memory, which is fundamental to 
    understanding Rust strings.
    */



    /************************************************************************************************************
    Yes — this is exactly the part that often causes confusion. The key is that "EFE" and the variable sample_string are two different things.

    When you write:

    let sample_string = "EFE";

    the text "EFE" does exist somewhere in memory. It is not that the text disappears because sample_string doesn't own it.

    Let's separate the two things.

    1. Where is "EFE"?

    Because "EFE" is a string literal, it is embedded into your compiled program.

    Conceptually, imagine your executable contains an area like this:

    Your program's memory

    ┌─────────────────────────────────┐
    │                                 │
    │ machine code                    │
    │                                 │
    │ "EFE"                           │
    │  ↓                              │
    │ ┌────┬────┬────┐                │
    │ │ 69 │ 70 │ 69 │                │
    │ └────┴────┴────┘                │
    │                                 │
    │ other program data              │
    │                                 │
    └─────────────────────────────────┘

    Remember:

    E = 69 decimal = 0x45
    F = 70 decimal = 0x46
    E = 69 decimal = 0x45

    So the bytes representing "EFE" are somewhere in the program's memory.

    2. Then what is sample_string?

    Now look at:

    let sample_string = "EFE";

    sample_string doesn't need to own those bytes.

    It essentially says:

    "I want to refer to the "EFE" text that already exists."

    Conceptually:

    sample_string
        │
        │ refers to
        ▼
    ┌────┬────┬────┐
    │ 69 │ 70 │ 69 │
    │ E  │ F  │ E  │
    └────┴────┴────┘

    That's why its type is:

    &str

    You can think of &str as:

    a reference to some UTF-8 string data + information about how much of that data belongs to the slice.

    3. But where exactly is "EFE" stored?

    For a string literal, such as:

    let sample_string = "EFE";

    the literal is typically stored in a read-only portion of the program's memory (often called the read-only data or rodata section).

    A simplified picture is:

    Process memory
    ────────────────────────────────

    Code
    ┌─────────────────────────────┐
    │ compiled Rust instructions  │
    └─────────────────────────────┘

    Read-only data
    ┌─────────────────────────────┐
    │ "EFE"                       │
    │                             │
    │ 69  70  69                  │
    └─────────────────────────────┘
            ▲
            │
            │ points to
            │
        sample_string
        ┌──────────────┐
        │ pointer      │
        │ length = 3   │
        └──────────────┘

    This is a simplified conceptual model, but it's the right mental model for learning.

    4. So why do we call it a "slice"?

    This is the subtle part.

    The type:

    &str

    doesn't mean:

    "The text itself is stored inside the variable."

    It means:

    "This variable contains a reference to some UTF-8 string data."

    For example:

    let sample_string = "EFE";

    Here, "EFE" happens to be the entire string literal.

    So sample_string is a &str referring to all three bytes:

            E     F     E
            ↓     ↓     ↓
            ┌─────┬─────┬─────┐
            │ 69  │ 70  │ 69  │
            └─────┴─────┴─────┘
            <──────────────>
                    ↑
                    &str

    But a slice doesn't have to represent the entire data.

    Consider:

    let name = String::from("ABCDE");
    let sample = &name[1..4];

    Now:

    String owns:

    ┌────┬────┬────┬────┬────┐
    │ A  │ B  │ C  │ D  │ E  │
    └────┴────┴────┴────┴────┘
        └────────────┘
            sample

    sample refers only to:

    BCD

    So:

    String
    │
    │ owns
    ▼
    ABCDE
    │
    │ borrowed
    ▼
    &str
    BCD

    That's why it's called a slice.

    It can represent a portion of a larger string.

    5. So "EFE" is a special case

    This is an important distinction:

    let sample_string = "EFE";

    Here:

    "E​​FE"

    is a string literal, and:

    sample_string

    is a &str referring to that literal.

    The literal itself is not a slice.

    The reference to it has the type &str.

    This is a much more precise way to think about it:

    "EFE"
    │
    │ is string data
    │
    ▼
    UTF-8 bytes
    69 70 69


    sample_string
    │
    │ is a &str
    │
    ▼
    references those bytes
    6. Compare it with String

    Now compare:

    let a = "EFE";

    with:

    let b = String::from("EFE");

    They both let you work with the text "EFE", but their ownership is different.

    First:
    let a = "EFE";

    Conceptually:

    Program's read-only data
    ┌────┬────┬────┐
    │ E  │ F  │ E  │
    └────┴────┴────┘
        ▲
        │
        │ borrowed/viewed by
        │
        a: &str

    a doesn't own the bytes.

    Second:
    let b = String::from("EFE");

    Conceptually:

    String b
    ┌─────────────────┐
    │ pointer ──────────────┐
    │ length = 3       │    │
    │ capacity = ...   │    │
    └─────────────────┘    │
                        ▼
                        heap
                    ┌────┬────┬────┐
                    │ E  │ F  │ E  │
                    └────┴────┴────┘

    b owns its string data.

    That's why b can grow:

    let mut b = String::from("EFE");

    b.push('X');

    Now:

    EFEX

    But you can't do the equivalent to:

    let a = "EFE";

    because a is only a borrowed view of the string literal.

    7. One more thing: &str isn't literally just a pointer

    This will become important when you study Rust memory more deeply.

    A &str is conceptually a fat pointer containing two pieces of information:

    &str
    ┌────────────────┐
    │ pointer        │ ────────> first byte
    │ length         │
    └────────────────┘

    For:

    let sample_string = "EFE";

    conceptually:

    sample_string
    ┌────────────────┐
    │ pointer ──────────────┐
    │ length = 3       │    │
    └────────────────┘    │
                        ▼
                    ┌────┬────┬────┐
                    │ 69 │ 70 │ 69 │
                    │ E  │ F  │ E  │
                    └────┴────┴────┘

    The pointer tells it where the string data starts.

    The length tells it how many bytes belong to the slice.

    And that's why a slice can point to just part of some larger data.

    So the most important thing to fix in your mental model is:

    "EFE" is the actual string data. sample_string is a &str that refers to that data.

    It is not:

    "sample_string contains the string but doesn't own it."

    It is closer to:

    "sample_string contains a borrowed reference/view describing where the string data is and how long it is."

    That distinction will make ownership and borrowing much easier to understand when we get there.
    */



    let string1  = "EFE"; // string1 is a &str that refers to the string literal "EFE",
                                // whose data is embedded in the program's read-only memory.
    let string2 = string1.to_string(); //This turns the string1 into string 
                                               // string2 owns its string data.
    let string3 = &string2[..]; // string3 is a &str that refers to all of the string data owned by string2.

    println!("This is string1: {}", string1);
    println!("This is string2: {}", string2);
    println!("This is string3: {}", string3);
    println!("This is the result of comparision: {}", "One".to_lowercase() == "one");


    //-----------------------------------------------------------------------------------------------

    //We can use below notation to enter string characters with their Hexadecimal codes in UTF-8.
    //We can use these Hexadecimal values to enter special characters like € or 🚀;
    let literal_string = "\x52\x75\x73\x74\u{20AC}\u{1F680}";
    println!("This is literal_string value: {}", literal_string);


    //-----------------------------------------------------------------------------------------------

    //For functions name Rust uses snake casing which as an example we can see one in below line:
    //In snake casing all the letters are in lowercase and we separate the words with _
    //this_is_snake_casing








    

}
