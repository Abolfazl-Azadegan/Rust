fn main() {

    println!("Hello World form section2");
    println!("-------------------------------------------------------------");

    let _var1 = 10; //This will store in stack memory
    let mut string1 = "Hello".to_string(); //This will store in Heap memory
    string1.push_str(" World!!");


    /*
    Ownership is one of the most important ideas in Rust. It is the system Rust uses to answer a fundamental question:
    Who is responsible for a piece of data in memory, and when should that memory be cleaned up?
    
    1. First: why does Rust need ownership?

    Imagine you create some data:

    let name = String::from("EFE");

    We already learned that a String owns its string data.

    Conceptually:

    name
    |
    | owns
    v
    Heap
    +----+----+----+
    | E  | F  | E  |
    +----+----+----+

    Now imagine your program eventually finishes using name.

    The memory containing "EFE" needs to be released.

    Someone has to be responsible for saying:

    "This memory is no longer needed. Release it."

    In languages such as C, the programmer may explicitly manage memory:

    malloc(...)
    free(...)

    If the programmer forgets free(), you can get a memory leak.

    If the programmer calls free() incorrectly, you can get things such as use-after-free or double-free errors.

    Languages with a garbage collector, such as Java or Go, take another approach. The runtime periodically determines which objects are no longer reachable and cleans them up.

    Rust takes a different approach.

    Rust uses ownership.


    2. The basic idea

    Rust gives every piece of owned data an owner.

    For example:

    let name = String::from("EFE");

    Here:

    name
    |
    └──── owns ────> "EFE"

    You can think of name as being responsible for that String.

    When name goes out of scope, Rust automatically frees the memory belonging to that String.

    For example:

    fn main() {
        let name = String::from("EFE");

        println!("{}", name);
    }

    When main() finishes, name goes out of scope.

    Rust knows:

    name owns this String, so I need to clean up its resources.

    You don't write:

    free(name);

    Rust handles it automatically.



    3. What does "goes out of scope" mean?

    This is important.

    Consider:

    fn main() {
        let name = String::from("EFE");

        println!("{}", name);
    }

    The variable name exists between:

    let name = ...

    and the end of the surrounding { }.

    Conceptually:

    fn main() {
        ┌─────────────────────────────
        │
        │ let name = String::from("EFE");
        │
        │ println!("{}", name);
        │
        └─────────────────────────────
    }

    When Rust reaches the closing }:

    }

    name goes out of scope.

    Because name owns the String, Rust cleans up the String.

    This automatic cleanup is one of the major benefits of ownership.


    4. The first ownership rule

    Rust's ownership system can initially be summarized with three rules.

    Rule 1

    Each value in Rust has an owner.

    For example:

    let name = String::from("EFE");

    The value represented by that String is owned by name.

    Rule 2

    There can only be one owner of a particular value at a time.

    This becomes extremely important when you assign one variable to another.

    For example:

    let string1 = String::from("EFE");
    let string2 = string1;

    At first you might think:

    string1 ──> "EFE"

    then:

    string1 ──> "EFE"
    string2 ──> "EFE"

    You might imagine that Rust simply copied the string.

    But that's not what happens.

    5. String makes ownership easier to see

    Remember our previous discussion about String.

    A String conceptually contains something like:

    String
    +------------------+
    | pointer          | ──────┐
    | length           |       |
    | capacity         |       |
    +------------------+       |
                            ↓
                            Heap
                        +----+----+----+
                        | E  | F  | E  |
                        +----+----+----+

    Suppose:

    let string1 = String::from("EFE");

    Conceptually:

    string1
    +------------------+
    | pointer ─────────|─────> [E][F][E]
    | length = 3       |
    | capacity = ...   |
    +------------------+

    Now:

    let string2 = string1;

    Rust moves ownership.

    Conceptually:

    Before:

    string1 ───────> [E][F][E]


    After:

    string1       string2
    X              |
                    |
                    v
                [E][F][E]

    string1 is no longer allowed to be used.

    string2 is now the owner.

    6. Why doesn't Rust simply copy it?

    Because copying the three bytes isn't always cheap.

    Imagine:

    let string1 = String::from("a huge amount of data ........");

    The string could contain millions of bytes.

    If:

    let string2 = string1;

    automatically copied all of that data, that could be expensive.

    Instead, Rust can transfer ownership.

    The String descriptor itself is small:

    pointer
    length
    capacity

    Rust can transfer that ownership without copying all the heap data.

    7. This is called a "move"

    This operation:

    let string2 = string1;

    is called a move.

    Ownership moves from:

    string1

    to:

    string2

    After:

    let string2 = string1;

    you can use:

    println!("{}", string2);

    But not:

    println!("{}", string1);

    Rust will give you an error.

    Why?

    Because string1 no longer owns the data.

    8. Why is this necessary?

    Imagine Rust allowed both variables to believe they owned the same heap allocation:

    string1 ─────┐
                │
                ↓
            [E][F][E]
                ↑
                │
    string2 ─────┘

    Then eventually both variables could reach the end of their scopes.

    Rust might have:

    string1 → "I should free this memory."

    string2 → "I should also free this memory."

    Now you have a double free problem.

    Rust's ownership rules prevent this.

    Instead:

    string1 = owner

    becomes:

    string2 = owner

    There is only one owner.

    9. But what about integers?

    You might wonder:

    "If ownership works this way, what happens with i32?"

    For example:

    let x = 10;
    let y = x;

    println!("{}", x);
    println!("{}", y);

    This works.

    Why?

    Because simple types such as integers can be cheaply copied.

    i32 is just a small fixed-size value.

    Conceptually:

    x = 10

            copy
            ↓
    y = 10

    Both variables can continue to be used.

    Rust calls these types Copy types.

    Examples include many simple scalar types:

    i32
    u64
    f64
    bool
    char

    assuming their values satisfy the relevant Copy rules.

    10. String is different

    Compare:

    let x = 10;
    let y = x;

    println!("{}", x);  // works

    with:

    let string1 = String::from("EFE");
    let string2 = string1;

    println!("{}", string1); // ERROR

    Why?

    Because:

    i32

    is a small value that can simply be copied.

    But:

    String

    owns dynamically allocated data.

    Rust therefore treats:

    let string2 = string1;

    as an ownership move.

    11. "Move" does NOT mean the bytes physically move

    This is an important detail.

    When we say:

    "Ownership moves from string1 to string2"

    we don't necessarily mean:

    [E][F][E]

    physically moves to another location in memory.

    The heap data can remain exactly where it is.

    Conceptually:

    Before:

    string1
    |
    v
    heap: [E][F][E]

    After:

    string1       string2
    X              |
                    v
    heap:         [E][F][E]

    The ownership relationship changed.

    The data doesn't necessarily physically move.

    That's why the term ownership move can initially be confusing.

    12. Ownership and borrowing

    This leads directly to another extremely important Rust concept:

    borrowing.

    Suppose:

    let string1 = String::from("EFE");

    and you want another variable to look at the data without taking ownership.

    You can write:

    let string2 = &string1;

    The & means:

    Borrow this value rather than taking ownership of it.

    Conceptually:

    string1
    |
    | owns
    v
    [E][F][E]
    ^
    |
    | borrows
    |
    string2

    Now string1 remains the owner.

    string2 merely has a reference to the data.

    This is the beginning of Rust's borrowing system.

    13. Connecting this to what you already learned

    Earlier you had:

    let string1 = "EFE";
    let string2 = string1.to_string();
    let string3 = &string2[..];

    Now you can understand these at a deeper level.

    string1
    let string1 = "EFE";

    string1 is:

    &str

    It is a borrowed view/reference to the string literal.

    The literal's data is embedded in the program.

    string2
    let string2 = string1.to_string();

    This creates a new:

    String

    and string2 owns that string data.

    string3
    let string3 = &string2[..];

    This creates a:

    &str

    that borrows the data owned by string2.

    So:

                    owns
    string2 ─────────────────> [E][F][E]
                                ↑
                                │
                            borrows
                                │
                            string3 (&str)

    This is exactly where ownership and borrowing start to connect.

    14. The three ideas you should have in your head

    For now, don't try to memorize all of Rust's ownership rules. Get these three ideas firmly established:

    Owned data
    let s = String::from("EFE");
    s
    │
    └──── owns ────> [E][F][E]
    Move
    let s1 = String::from("EFE");
    let s2 = s1;
    s1 ──X

    s2 ───────────> [E][F][E]
                    owns

    Ownership transferred from s1 to s2.

    Borrow
    let s1 = String::from("EFE");
    let s2 = &s1;
    s1 ───────────> [E][F][E]
    owns             ↑
                    │
                    │ borrows
                    │
                    s2

    s1 still owns the data.

    The simplest definition

    If you encounter the question "What is ownership in Rust?", a good answer is:

    Ownership is Rust's system for determining which variable is responsible for a value and therefore responsible for its cleanup. Each owned value has one owner, ownership can be transferred through moves, and when the owner goes out of scope, Rust automatically cleans up the value.

    And this is the key reason Rust can provide memory safety without requiring a garbage collector.

    */



    let string10 = "EFE".to_string();
    let string20= string10;

    println!("String20 is equal to: {}", string20);
    // Below line will generate an error becaues string10 moved its control over the variable value to string20
    //println!("String10 is equal to: {}", string10);


    let var10 = 10;
    let var20 = var10;
    // As explained above, variables type i and u and f can copy to other variable
    println!("var10 is equal to: {}", var10);
    println!("var20 is equal to: {}", var20);

    let vector1 = vec!["EFESHENEM".to_string()];
    let vector2 = vector1;
    // Below line will generate an error because the onwnership of the value in vector1 is moved to vector2 and it has no control
    // over the value and vector2 has access to the value.
    //println!("The vector1 is equal to: {:?}", vector1);
    println!("The vector2 is equal to: {:?}", vector2);


}

// var1 will drop after the main function terminate.
// string1 will drop after main function terminate.




