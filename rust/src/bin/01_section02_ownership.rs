fn main() {

    println!("Hello World form section2");
    println!("-------------------------------------------------------------");

    let _var1 = 10; //This will store in stack memory
    let mut string1 = "Hello".to_string(); //This will store in Heap memory
    string1.push_str(" World!!");


    /************************************************************************************************************
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

    println!("-------------------------------------------------------------");


    let cloned_var1 = "EFE".to_string();
    let cloned_var2 = cloned_var1.clone();
    let cloned_var3 = cloned_var2.clone();

    println!("This is cloned_var1 value: {:?}", cloned_var1);
    println!("This is cloned_var2 value: {:?}", cloned_var2);
    println!("This is cloned_var3 value: {:?}", cloned_var3);
    
    /************************************************************************************************************
    1. What is .clone()?

    When you write:

    cloned_var1.clone()

    you are telling Rust:

    Create another independent copy of this value.

    So:

    let cloned_var1 = 10;
    let cloned_var2 = cloned_var1.clone();

    means conceptually:

    cloned_var1 = 10

                clone()
                ↓

    cloned_var2 = 10

    You now have two independent values:

    cloned_var1 → 10
    cloned_var2 → 10
    2. Why doesn't cloned_var1 become unusable?

    Remember what we learned about move.

    With something like String:

    let string1 = String::from("EFE");
    let string2 = string1;

    ownership moves from string1 to string2.

    After that:

    println!("{}", string1);

    would cause an error.

    But your code does:

    let cloned_var2 = cloned_var1.clone();

    This is different.

    clone() explicitly creates another value.

    Conceptually:

    Before:

    cloned_var1
        |
        ↓
        10


    After clone():

    cloned_var1          cloned_var2
        |                    |
        ↓                    ↓
        10                   10

    They are independent values.

    Therefore both can be used:

    println!("{}", cloned_var1);
    println!("{}", cloned_var2);
     */

    /************************************************************************************************************
    4. Copy and Clone are related, but not the same

    This is an important distinction.

    Copy

    Copy means Rust can automatically make a simple copy when you assign the value.

    For example:

    let a = 10;
    let b = a;

    Rust can effectively copy the integer.

    So both remain usable:

    a → 10
    b → 10
    Clone

    Clone means the type provides a .clone() method that explicitly creates another value.

    For example:

    let a = 10;
    let b = a.clone();

    You are explicitly asking for a clone.

    So:

    a → 10
    b → 10
    5. Why is clone() much more interesting with String?

    This is where you will really see why clone() exists.

    Consider:

    let string1 = String::from("EFE");
    let string2 = string1;

    This is a move.

    Conceptually:

    string1 ───────X

    string2 ─────────────> Heap
                            [E][F][E]

    string1 can no longer be used.

    But suppose you actually want two independent Strings.

    You can use:

    let string1 = String::from("EFE");
    let string2 = string1.clone();

    Now:

    string1 ─────────────> Heap A
                            [E][F][E]

    string2 ─────────────> Heap B
                            [E][F][E]

    There are two separate copies of the string data.

    Both variables own their own data.

    Therefore:

    println!("{}", string1);
    println!("{}", string2);

    both work.

    6. Why is this different from &str?

    Remember your previous example:

    let string1 = "EFE";
    let string2 = &string1[..];

    Here you're creating a reference/slice, not a new owned copy of the data.

    Conceptually:

    Read-only program memory:

    [E][F][E]
    ↑     ↑
    │     │
    │     └── string2 (&str)
    │
    └──────── string1 (&str)

    Both are referring to the same underlying data.

    But with:

    let string1 = String::from("EFE");
    let string2 = string1.clone();

    you get two separately owned copies:

    string1 ──> [E][F][E]

    string2 ──> [E][F][E]

    That's a major difference.


    8. Compare Move, Copy, and Clone

    This is worth keeping very clear in your mind.

    Move
    let a = String::from("EFE");
    let b = a;

    Conceptually:

    a ──X

    b ─────────> "EFE"
                owns it

    Ownership transfers.

    Copy
    let a = 10;
    let b = a;

    Conceptually:

    a ─────> 10

    b ─────> 10

    The value is automatically copied because i32 implements Copy.

    Clone
    let a = String::from("EFE");
    let b = a.clone();

    Conceptually:

    a ─────────> [E][F][E]
                OWNED DATA A

    b ─────────> [E][F][E]
                OWNED DATA B

    A new independent value is explicitly created.



    9. Why doesn't Rust automatically clone everything?

    This is extremely important.

    Imagine:

    let a = String::from("A very very very large string...");
    let b = a;

    If Rust automatically cloned a every time you assigned it to another variable, it might have to copy a huge amount of data.

    That could be expensive and unexpected.

    Instead, Rust does:

    let b = a;

    → move

    If you really want a full copy:

    let b = a.clone();

    → explicit clone

    So Rust makes you explicitly say:

    Yes, I know I'm asking for another copy of this data.

    This gives you more control over performance.



    One final point that is important for your Rust learning

    Don't learn the rule:

    "clone() means make a copy of the bytes."

    That is sometimes an oversimplification.

    A better rule is:

    Clone is a trait that defines how a type can explicitly create another value from itself.

    For an i32, cloning is trivial.

    For a String, cloning involves creating another owned string and copying its contents.

    For other types, clone() can have more complicated behavior.

    So when you see:

    some_value.clone()

    your first thought should be:

    "I'm explicitly asking this type to create another value from this one, rather than transferring ownership."

    And because cloning can involve allocating and copying potentially large amounts of data, you generally shouldn't 
    use .clone() just to make an ownership error disappear without understanding what is being copied.

     */

    println!("-------------------------------------------------------------");




     let string1 = String::from("EFESHENEM");
     // Here the string1 has the ownership of the string and it can print it But later we pass the variable to the fucntion
     // with the ownership of the variable and it will be moved to the funtion and the string1 will not have the ownership of the
     // variable and it will not be able to print the value of the variable. So we need to pass the reference of the variable to
     // the function to avoid the ownership transfer and the string1 will be able to print the value of the variable.
     println!("The text is: {}", string1);

    // This should work and print the value of the variable because we pass the reference of the variable to the function
    // and the function will not take the ownership of the variable and the string1 will be able to print the value of the 
    // variable.
    print_string_by_reference(&string1);

    // This will work as well because string1 still has the ownership of the variable and it can print the value of the variable.
    println!("The text is: {}", string1);

    // This will work and will print the value of the variable because we pass the value of the variable to the fucntion
    // and the function will take the ownership of the variable and the string1 will not be able to print the value of the 
    // variable. So from now on the fucntion will take the ownership of the variable.
    print_string_by_value(string1);

    //This will not work and will generate and error because the string1 has lost the ownership of the variable and it will 
    // not be able to print the value of the variable.
    //println!("The text is: {}", string1);



    /************************************************************************************************************
    1. First, what is actually inside string1?

    You have:

    let string1 = String::from("EFESHENEM");

    Because String owns dynamically allocated text, conceptually we can think of it like this:

    string1
    |
    | owns
    v
    +-------------------------+
    | String                  |
    |                         |
    | pointer ────────────────┼──────┐
    | length = 8              |      |
    | capacity = ...          |      |
    +-------------------------+      |
                                    v
                            +----------------+
                            | E F E S H E N E M |
                            +----------------+
                                heap

    There is an important distinction here.

    string1 itself is a variable containing the String structure.

    The actual characters are stored in the dynamically allocated buffer.

    So when we say:

    "string1 owns the string"

    we mean that the String value stored in string1 is responsible for that heap allocation.

    2. Before calling any function

    You have:

    let string1 = String::from("EFESHENEM");

    println!("The text is: {}", string1);

    At this point:

    string1
    |
    | OWNER
    v
    String
    |
    v
    "EFESHENEM"

    Therefore this works:

    println!("{}", string1);

    because string1 owns the String.

    3. Now let's look at the reference version

    You call:

    print_string_by_reference(&string1);

    Notice the &:

    &string1

    This means:

    "Do not give ownership of string1. Give the function a reference to it."

    Your function is:

    fn print_string_by_reference(s: &String) {
        let string_in_function = s;

        println!(
            "This is the text in the function: {}",
            string_in_function
        );
    }

    The parameter:

    s: &String

    means:

    s is a reference to a String.

    It does not mean:

    s owns the String.

    That's a very important distinction.

    Conceptually:

    string1
    |
    | OWNS
    v
    +----------+
    | String   |
    +----------+
        |
        v
    "EFESHENEM"


    s
    |
    | BORROWS / REFERENCES
    v
    String

    There are now two variables involved:

    string1 = owner
    s       = borrower/reference

    string1 still owns the String.

    4. What about this line?

    Inside the function you have:

    let string_in_function = s;

    This might look like we're creating another owner.

    But we're not.

    Remember:

    s: &String

    So s is a reference.

    Therefore:

    let string_in_function = s;

    creates another reference.

    Conceptually:

    string1
    |
    | OWNS
    v
    String
    |
    v
    "EFESHENEM"
    ^
    |
    | reference
    |
    s
    ^
    |
    | reference
    |
    string_in_function

    Neither s nor string_in_function owns the actual String.

    They only have access to it.

    5. What happens when the reference function ends?

    This is very important.

    Suppose:

    fn print_string_by_reference(s: &String) {
        let string_in_function = s;

        println!("{}", string_in_function);
    }

    When the function starts:

    string1
    |
    | OWNS
    v
    String
    |
    v
    "EFESHENEM"

    s
    |
    | borrows
    v
    String

    string_in_function
    |
    | borrows
    v
    String

    Then the function reaches:

    }

    The function's local variables disappear:

    s                    -> gone
    string_in_function   -> gone

    But:

    string1

    still exists.

    Therefore:

    println!("{}", string1);

    works.

    The String itself was not destroyed, because the function never owned it.

    6. Now compare this with passing by value

    You have:

    print_string_by_value(string1);

    There is no &.

    And the function is:

    fn print_string_by_value(s: String) {
        let string_in_function = s;

        println!(
            "This is the text in the function: {}",
            string_in_function
        );
    }

    The parameter is:

    s: String

    not:

    s: &String

    This means s is capable of owning a String.

    And because you pass:

    string1

    by value, ownership is moved.

    7. What exactly happens during the move?

    Before:

    let string1 = String::from("EFESHENEM");

    we have:

    string1
    |
    | OWNER
    v
    String
    |
    v
    "EFESHENEM"

    Then:

    print_string_by_value(string1);

    Ownership moves:

    BEFORE FUNCTION CALL

    string1
    |
    | OWNER
    v
    String
    |
    v
    "EFESHENEM"

    After the function receives it:

    string1
    |
    X  no longer owner
    

    s
    |
    | OWNER
    v
    String
    |
    v
    "EFESHENEM"

    This is the key idea:

    string1 does not become a second owner.

    There is only one owner.

    Ownership has moved from string1 to s.

    8. Important: the actual String data doesn't necessarily physically move

    This is another thing beginners often misunderstand.

    Suppose the heap contains:

    HEAP

    +---+---+---+---+---+---+---+---+---+
    | E | F | E | S | H | E | N | E | M |
    +---+---+---+---+---+---+---+---+---+

    When ownership moves:

    print_string_by_value(string1);

    Rust does not necessarily take all those bytes and physically move them somewhere else.

    Conceptually, what changes is which variable is responsible for the data.

    Before:

    string1
    |
    | owns
    v
    heap data

    After:

    s
    |
    | owns
    v
    heap data

    The ownership relationship changes.

    9. Now we reach the really important question you asked

    You asked:

    "If the variable in the function is its owner, what will happen to the variable after the function ends?"

    Excellent question.

    Consider:

    fn print_string_by_value(s: String) {
        println!("{}", s);
    }

    When the function starts:

    s
    |
    | OWNER
    v
    String
    |
    v
    "EFESHENEM"

    Then the function reaches:

    }

    Now s goes out of scope.

    Because s owns the String, Rust automatically drops the String.

    Conceptually:

    s
    |
    | OWNER
    v
    String
    |
    v
    heap data

    becomes:

    s -> destroyed
    String -> dropped
    heap allocation -> released

    So after the function finishes, the String is gone.

    10. This is why string1 cannot be used afterward

    You have:

    let string1 = String::from("EFESHENEM");

    print_string_by_value(string1);

    println!("{}", string1);

    The last line produces an error.

    Why?

    Because:

    print_string_by_value(string1);

    moved ownership away from string1.

    After the call:

    string1
    |
    X
    |
    v
    NO OWNERSHIP

    The function owns it:

    s
    |
    | owns
    v
    String

    And when the function finishes:

    s -> goes out of scope
    String -> dropped
    heap data -> freed

    Therefore string1 cannot magically get ownership back.

    11. What happens to string_in_function?

    You also have:

    fn print_string_by_value(s: String) {
        let string_in_function = s;

        println!("{}", string_in_function);
    }

    There is actually another ownership transfer happening here.

    Initially, when the function starts:

    s
    |
    | OWNER
    v
    String

    Then:

    let string_in_function = s;

    moves ownership from s to string_in_function.

    So now:

    s
    |
    X
    |
    v

    string_in_function
    |
    | OWNER
    v
    String
    |
    v
    "EFESHENEM"

    Then the function ends.

    string_in_function goes out of scope:

    string_in_function -> destroyed
    String             -> dropped
    heap allocation    -> released

    So in your exact code, string_in_function is actually the final owner, not s.

    This is a very useful detail to understand.

    12. Your two functions side-by-side
    Function 1: reference
    fn print_string_by_reference(s: &String) {
        let string_in_function = s;
        println!("{}", string_in_function);
    }

    Ownership:

    string1
    |
    | OWNS
    v
    String
    |
    v
    "EFESHENEM"

    s
    |
    | BORROWS
    v
    String

    string_in_function
    |
    | BORROWS
    v
    String

    Function ends:

    s                    -> gone
    string_in_function   -> gone

    string1              -> STILL OWNS STRING
    String               -> STILL ALIVE

    Therefore:

    println!("{}", string1);

    works.

    Function 2: value
    fn print_string_by_value(s: String) {
        let string_in_function = s;
        println!("{}", string_in_function);
    }

    Ownership:

    BEFORE CALL:

    string1
    |
    | OWNS
    v
    String

    After passing it:

    s
    |
    | OWNS
    v
    String

    After:

    let string_in_function = s;

    we have:

    string_in_function
    |
    | OWNS
    v
    String

    Then function ends:

    string_in_function -> gone
    String             -> dropped
    heap data          -> released

    And:

    string1

    cannot be used anymore.


     */


    let mut _string1 = "EFE".to_string();
    let mut _string2: String;

    // This loop will genrate an error because the ownership of the _string1 in the first itteration of the loop has given to
    // _string2 and for the second itteration _string1 has no ownership over the variable. So this will generate an error
    /************************************************************************************************************
    loop{
        _string2 = string1;
    }
    */




    /************************************************************************************************************
    it depends on whether your reference is mutable or immutable. This is one of the most important parts of Rust's 
    borrowing system.

    Let's start from the simplest case.

    1. Normal reference: &String

    Suppose:

    let mut string1 = String::from("EFE");

    let string2 = &string1;

    Here:

    string1
    |
    | OWNS
    v
    "EFE"

    string2
    |
    | BORROWS
    v
    "EFE"

    string2 is an immutable reference because we wrote:

    &string1

    not:

    &mut string1

    Therefore, you cannot modify the String through string2:

    let mut string1 = String::from("EFE");

    let string2 = &string1;

    string2.push_str(" SHENEM");  // ❌ ERROR

    Why?

    Because string2 is only allowed to look at the String, not modify it.

    Think:

    &String
    |
    +-- can READ
    |
    +-- cannot MODIFY
    2. Mutable reference: &mut String

    If you want the new variable to be able to modify the String, you need a mutable reference:

    let mut string1 = String::from("EFE");

    let string2 = &mut string1;

    string2.push_str(" SHENEM");

    println!("{}", string1);

    Output:

    EFE SHENEM

    Now the situation is:

    string1
    |
    | OWNS
    v
    "EFE"

    string2
    |
    | &mut
    v
    "EFE"

    string2 doesn't own the String.

    It borrows the String from string1, but it has permission to modify it.

    So:

    &String
        ↓
    read only

    &mut String
        ↓
    read + modify
    3. Why does string1 need mut?

    Notice:

    let mut string1 = String::from("EFE");

    The mut is necessary.

    Without it:

    let string1 = String::from("EFE");

    let string2 = &mut string1; // ❌

    Rust won't allow you to create a mutable reference to an immutable variable.

    You can think of it as:

    let string1
        ↓
    "I own this, but nobody is allowed to modify it"

    let mut string1
        ↓
    "I own this, and modification is allowed"
    4. Very important: mut on the reference and mut on the String are different things

    This can initially be confusing.

    Consider:

    let mut string1 = String::from("EFE");

    let string2 = &mut string1;

    There are two concepts:

    string1
    |
    | owns String
    |
    | mutable owner
    v
    "EFE"

    string2
    |
    | mutable reference
    v
    "EFE"

    mut on string1 means:

    The owner allows its String to be modified.

    &mut means:

    This particular borrower is allowed to modify the String.

    5. You can also pass a mutable reference to a function

    For example:

    fn change_string(s: &mut String) {
        s.push_str(" SHENEM");
    }

    fn main() {
        let mut string1 = String::from("EFE");

        change_string(&mut string1);

        println!("{}", string1);
    }

    Output:

    EFE SHENEM

    Let's follow ownership.

    Initially:

    string1
    |
    | OWNS
    v
    "EFE"

    Then:

    change_string(&mut string1);

    We temporarily give the function a mutable borrow:

    string1
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | mutable borrow
    |
    s

    The function can modify the data:

    s.push_str(" SHENEM");

    So the data becomes:

    "EFE SHENEM"

    But s never becomes the owner.

    When the function finishes:

    s → gone

    and:

    string1
    |
    | STILL OWNS
    v
    "EFE SHENEM"

    That's why this works:

    println!("{}", string1);
    6. But Rust has an important rule

    You cannot have a mutable reference and another reference to the same data being used at the same time.

    For example:

    let mut string1 = String::from("EFE");

    let string2 = &string1;
    let string3 = &mut string1;

    This is not allowed because:

    string2
    |
    | reading
    v
    string1

    string3
    |
    | modifying
    v
    string1

    Rust doesn't want one part of your program reading the data while another part might change it simultaneously.

    The simplified rule is:

    Either you can have multiple immutable references,
    or you can have one mutable reference,
    but you cannot have both at the same time.

    So this is fine:

    let string1 = String::from("EFE");

    let a = &string1;
    let b = &string1;
    let c = &string1;

    Multiple readers are okay.

    And this is fine:

    let mut string1 = String::from("EFE");

    let a = &mut string1;

    a.push_str("!");

    One mutable borrower is okay.

    But not simultaneously:

    let mut string1 = String::from("EFE");

    let a = &string1;
    let b = &mut string1; // ❌
    The key thing to remember

    For now, memorize this:

    &String

    means:

    "I can borrow the String and read it, but I cannot modify it through this reference."

    Whereas:

    &mut String

    means:

    "I can temporarily borrow the String and modify it, but I still don't own it."

    So:

    String
    │
    └── ownership
        ↓
        owner


    &String
    │
    └── immutable borrowing
        ↓
        read


    &mut String
    │
    └── mutable borrowing
        ↓
        read + modify

     */

    // In below example code we can not define the s as a variable which is immutable because later in the fucntion we will
    // modify the text and this will generate an error.
    //So we define it as muable and pass it with reference and mut to the fucntion and we can mpdify it.
    let mut s = String::from("Hello");
    println!("Before: {}", s);
    push_to_string(&mut s);
    println!("After: {}", s);


    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");

    let vector = vec![1,3,5,7];
    println!("The returned value is: {}", check_vector(vector));
    // below line will generate an error because the ownership of the function has moved to variable vec_val in function
    // and we passed the vector vector to the check_vector function with value not by reference so the ownership will move
    // to the variable in the function.
    // println!("The main vector is: {:?}", vector);


    let mut vector2 = vec![1,3,5,7];
    println!("The returned value is: {}", check_vector2(&vector2));
    vector2.push(15);
    println!("The main vector after running the function is: {:?}", vector2);


    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");


    //The i value types are simple types and we can run the below function without moving the ownership to the varibale inside
    // the function. the simple types will copy instead of move ownership to the variable.
    let var1 = 10;
    println!("var1 before the function is: {}", var1);
    println!("The returned value by the function is: {}", add_2 (var1));
    println!("var1 after the function is: {}", var1)


}


fn print_string_by_reference (s: &String) {
    let string_in_function = s;
    println!("This is the text in the fucntion when pass the stirng with reference: {}", string_in_function);
}

fn print_string_by_value (s: String) {
    let string_in_function = s;
    println!("This is the text in the fucntion when pass the stirng with value: {}", string_in_function);
}


fn push_to_string(receive_string: &mut String){
    receive_string.push_str(" World!!");
}

// var1 will drop after the main function terminate.
// string1 will drop after main function terminate.



fn check_vector (vec_val: Vec<i8>) -> bool{
    if vec_val[0] == 1{
        return true
    }else {
        return false
    }
}


fn check_vector2 (vec_val: &Vec<i8>) -> bool{
    if vec_val[0] == 1{
        return true
    }else {
        return false
    }
}


fn add_2 (v:i32) -> i32{
    return v+2;
}


