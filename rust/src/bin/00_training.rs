


fn largest_value_i32 (list: &[i32]) -> i32{
    let mut largest = list[0];
    for &item in list { // Here by using the & we are telling rust that we want to value which the varibale is refering to
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_value_char (list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


    /*
    Let's start from only these two function signatures and understand exactly what they mean.

    fn largest_value_i32(list: &[i32]) -> i32 {

    and

    fn largest_value_char(list: &[char]) -> char {

    The important thing is to understand three separate things:

    What list is.
    What &[i32] / &[char] means.
    What -> i32 / -> char means.
    1. fn largest_value_i32(list: &[i32]) -> i32

    Let's separate it:

    fn largest_value_i32(list: &[i32]) -> i32
    fn

    This says:

    "I am defining a function."

    largest_value_i32

    This is simply the name of the function.

    You choose this name.

    list

    This is the parameter name.

    It is the name that the function will use to access whatever you pass to it.

    For example:

    fn largest_value_i32(list: &[i32]) -> i32 {
        // ...
    }

    The function has one parameter called list.

    :

    This separates the parameter name from its type.

    list: &[i32]

    means:

    The parameter called list has type &[i32].

    This is the most important part.

    2. What is &[i32]?

    Let's first look at:

    [i32]

    This means:

    a slice of i32 values.

    For example, these are i32 values:

    10
    20
    30
    40

    So a slice could contain:

    10, 20, 30, 40

    But [i32] by itself is the slice type.

    The & means we are giving the function a reference to that slice.

    Therefore:

    &[i32]

    means:

    a reference to a slice containing i32 values.

    So when you call:

    largest_value_i32(&numbers)

    you are not giving the function ownership of numbers.

    You are giving it a reference to the numbers.

    For example:

    fn largest_value_i32(list: &[i32]) -> i32 {
        list[0]
    }

    fn main() {
        let numbers = vec![10, 20, 30];

        let result = largest_value_i32(&numbers);

        println!("{}", result);
    }

    Here:

    let numbers = vec![10, 20, 30];

    numbers has type:

    Vec<i32>

    Then:

    &numbers

    has type:

    &Vec<i32>

    But Rust can automatically convert the &Vec<i32> into the &[i32] that the function expects.

    So the function receives:

    list: &[i32]

    and list refers to the elements:

    10
    20
    30
    3. What values can I give to largest_value_i32?

    You can give it a Vec<i32> by borrowing it:

    let numbers = vec![10, 20, 30];

    largest_value_i32(&numbers);

    You can also give it an array:

    let numbers = [10, 20, 30];

    largest_value_i32(&numbers);

    Notice:

    numbers

    is an array:

    [i32; 3]

    and:

    &numbers

    can be used where:

    &[i32]

    is expected.

    You can also give it only part of a vector:

    let numbers = vec![10, 20, 30, 40, 50];

    largest_value_i32(&numbers[1..4]);

    Here:

    &numbers[1..4]

    refers to:

    20, 30, 40

    So the function receives a slice containing those three i32 values.

    You can also directly create an array:

    largest_value_i32(&[10, 20, 30]);

    That's perfectly valid.

    4. What does -> i32 mean?

    This part:

    -> i32

    does not describe what you pass into the function.

    It describes what the function returns.

    For example:

    fn largest_value_i32(list: &[i32]) -> i32 {
        100
    }

    This function receives:

    &i32 slice

    and returns:

    i32

    So:

    let result = largest_value_i32(&numbers);

    means result is an:

    i32

    For example:

    fn largest_value_i32(list: &[i32]) -> i32 {
        list[0]
    }

    fn main() {
        let numbers = vec![10, 20, 30];

        let result = largest_value_i32(&numbers);

        println!("{}", result);
    }

    result is:

    i32

    and contains:

    10
    Now the second function
    fn largest_value_char(list: &[char]) -> char {

    It has exactly the same structure.

    The difference is that instead of i32, it works with char.

    list: &[char]

    means:

    list is a reference to a slice of char values.

    For example:

    'a'
    'b'
    'c'
    'Z'

    are char values.

    So you could write:

    fn largest_value_char(list: &[char]) -> char {
        list[0]
    }

    fn main() {
        let letters = vec!['a', 'b', 'c'];

        let result = largest_value_char(&letters);

        println!("{}", result);
    }

    Here:

    letters

    has type:

    Vec<char>

    and:

    &letters

    is borrowed and can be used as:

    &[char]

    inside the function.

    The function returns:

    char

    So:

    result

    has type:

    char
    What can I pass to the char version?

    For example, a vector:

    let letters = vec!['a', 'b', 'c'];

    largest_value_char(&letters);

    An array:

    let letters = ['a', 'b', 'c'];

    largest_value_char(&letters);

    A slice of an array:

    let letters = ['a', 'b', 'c', 'd'];

    largest_value_char(&letters[1..3]);

    This passes the slice containing:

    'b', 'c'

    Or directly:

    largest_value_char(&['a', 'b', 'c']);
    The most important thing to understand

    Compare these two:

    fn largest_value_i32(list: &[i32]) -> i32

    and:

    fn largest_value_char(list: &[char]) -> char

    For the first one:

    list: &[i32]

    means:

    "I expect a reference to a sequence/slice whose elements are i32."

    and:

    -> i32

    means:

    "I will return one i32."

    For the second:

    list: &[char]

    means:

    "I expect a reference to a sequence/slice whose elements are char."

    and:

    -> char

    means:

    "I will return one char."

    So these are valid:

    let numbers = vec![10, 20, 30];
    largest_value_i32(&numbers);

    because:

    numbers: Vec<i32>
    &numbers: &[i32]   // usable as this parameter

    And:

    let letters = vec!['a', 'b', 'c'];
    largest_value_char(&letters);

    because:

    letters: Vec<char>
    &letters: &[char]  // usable as this parameter

    But this is not valid:

    let letters = vec!['a', 'b', 'c'];

    largest_value_i32(&letters);

    because the function requires:

    &[i32]

    but you're giving it:

    &[char]

    And this is not valid either:

    let numbers = vec![10, 20, 30];

    largest_value_char(&numbers);

    because the function requires:

    &[char]

    but you're giving it:

    &[i32]

    The key idea is therefore:

    &[i32]

    doesn't mean "a vector of i32".

    It means:

    a reference to a slice whose elements are i32.

    A Vec<i32> and an array of i32 can both be borrowed as a slice, which is why both can be passed to this function.
     */


    /*
    A slice is neither a Vec nor an array.

    A slice is a view into a sequence of elements that already exists somewhere else.

    For your current example, the easiest way to think about it is:

    A slice tells Rust: "Here are these elements from this existing collection; I don't own them."

    For example:

    let numbers = vec![10, 20, 30, 40, 50];

    numbers is a Vector:

    Vec<i32>

    Now:

    &numbers[1..4]

    is a slice:

    &[i32]

    It refers to:

    20, 30, 40

    It does not create another vector containing 20, 30, 40.

    A slice can come from a Vec
    let numbers = vec![10, 20, 30, 40];

    let part: &[i32] = &numbers[1..3];

    Here:

    numbers

    is:

    Vec<i32>

    while:

    part

    is:

    &[i32]

    So the original vector owns the elements. part just borrows some of them.

    A slice can also come from an array
    let numbers = [10, 20, 30, 40];

    let part: &[i32] = &numbers[1..3];

    Here:

    numbers

    is an array:

    [i32; 4]

    and:

    part

    is:

    &[i32]

    Again, the slice doesn't own the elements.

    This is why your function uses &[i32]

    When you write:

    fn largest_value_i32(list: &[i32]) -> i32 {

    you're saying:

    "I don't care whether you give me a vector or an array. Just give me a reference to a sequence of i32 values."

    Therefore both work:

    let numbers = vec![10, 20, 30];
    largest_value_i32(&numbers);

    and:

    let numbers = [10, 20, 30];
    largest_value_i32(&numbers);

    And you can even give it only part of either one:

    let numbers = vec![10, 20, 30, 40, 50];

    largest_value_i32(&numbers[1..4]);

    The function receives a slice containing 20, 30, 40.

    So remember these three as different things:

    Vec<i32>       // Vector: owns its elements
    [i32; 5]       // Array: owns its elements
    &[i32]         // Slice: borrows some/all elements

    The slice is not a third kind of container like a vector or array. It is a borrowed view of 
    elements stored in something else.
     */



    //For each data type we cannot have a function definition. So we need to use generics to define a uniform function

    fn largest_value_generics<T: Copy + PartialOrd> (list: &[T]) -> T{
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    /*
    There are two separate problems with this function.

    fn largestvaluegenerics<T>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    1. T might not be Copy

    This line:

    let mut largest = list[0];

    is a problem.

    Remember that:

    list: &[T]

    means list[0] gives you a reference to a T conceptually. You are trying to take the T out of that 
    reference and put it into largest.

    But Rust cannot assume that every possible T can simply be copied.

    For example, T could be String.

    You can copy an i32:

    let x = 10;
    let y = x;

    because i32 implements Copy.

    But you cannot simply copy a String this way:

    let x = String::from("hello");
    let y = x;

    because that moves the String.

    So Rust needs you to tell it:

    "The generic type T must be a type that can be copied."

    You do that with a trait bound:

    T: Copy
    2. T might not support >

    This line is another problem:

    if item > largest

    Rust doesn't automatically know that an arbitrary type T can be compared using >.

    For example, this makes sense:

    10 > 5

    because i32 supports comparison.

    But what does this mean?

    String::from("hello") > String::from("apple")

    Rust cannot assume that every possible T supports >.

    The trait that provides this kind of ordering comparison is PartialOrd.

    So you need:

    T: PartialOrd
    Therefore

    You can write:

    fn largestvaluegenerics<T: Copy + PartialOrd>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    Now you have told Rust two things about T:

    T: Copy

    means:

    T can be copied.

    and:

    T: PartialOrd

    means:

    T can be compared using ordering operations such as <, >, <=, and >=.

    So now Rust can understand:

    let mut largest = list[0];

    because it can copy the T out of the reference, and:

    if item > largest

    because T supports >.

    For example:

    fn largestvaluegenerics<T: Copy + PartialOrd>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let numbers = vec![10, 50, 20, 40];

        let result = largestvaluegenerics(&numbers);

        println!("{}", result);
    }

    This prints:

    50

    And the important part is that generic T means "any type" only if that type satisfies the requirements you actually need.

    Here you need two requirements:

    T: Copy + PartialOrd

    because your algorithm needs to copy values and compare values.
     */







fn main(){

    let number_list = vec![34, 50, 20, 80, 100, 65];
    let result_i32 = largest_value_i32(&number_list);
    println!("The returned value of the function with i32 is: {}", result_i32);

    let char_list = vec!['E', 'S', 'H'];
    let result_char = largest_value_char(&char_list);
    println!("The returned value of the function with char is: {}", result_char);


    let number_list_2 = vec![20,30,56,78,42,60];
    let result_generic_i32 = largest_value_generics(&number_list_2);
    println!("The returned value of the function with generic i32 is: {}", result_generic_i32);


    let char_list_2 = vec!['A', 'B', 'C'];
    let result_generic_char = largest_value_generics(&char_list_2);
    println!("The returned value of the function with generic char is: {}", result_generic_char)


    /*
    Yes. char implements Copy, so your generic function with T: Copy + PartialOrd can work with char.

    For example:

    fn largestvaluegenerics<T: Copy + PartialOrd>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    You can give it:

    let letters = vec!['a', 'z', 'c', 'm'];

    let result = largestvaluegenerics(&letters);

    println!("{}", result);

    The result is:

    z
    Why does char work?

    Because Rust's char is a simple value type, similar to integer types.

    For example:

    let character1: char = 'A';
    let character2 = character1;

    After this, both character1 and character2 are still usable:

    println!("{}", character1);
    println!("{}", character2);

    This works because char implements Copy.

    So when your function does:

    let mut largest = list[0];

    Rust can copy the char out of the reference.

    And when you have:

    for &item in list

    the &item pattern can copy the char value from the reference because char is Copy.

    Compare char and String
    char       → Copy
    String     → NOT Copy

    For example:

    let a = 'A';
    let b = a;       // Copy
    println!("{}", a); // still works

    But:

    let a = String::from("hello");
    let b = a;       // Move
    println!("{}", a); // ERROR

    String is not Copy because it owns dynamically allocated data, and automatically copying that ownership would not be the behavior Rust wants.

    One more important point: char is not an integer like i32, but it is still a small, fixed-size value type and implements Copy.

    So your generic function:

    T: Copy + PartialOrd

    works for char because char satisfies both requirements:

    char
    ├── Copy       ✓
    └── PartialOrd ✓

    It also works for types such as i32, u32, f64, and bool where the required traits are available.
     */

    /* 
    Exactly. All four can print the same output, but they do not do the same thing internally.

    The key is that there are actually two separate questions:

    Does the loop get the actual i32 or a reference &i32?
    Does the loop move the vector's elements or borrow them?

    Let's examine your four examples one by one.

    First, the most important distinction

    You have:

    let numbers = vec![10, 20, 30];

    numbers owns the Vec<i32>.

    Now compare:

    for item in numbers

    with:

    for item in &numbers

    These are fundamentally different.

    in numbers
    for item in numbers

    means:

    Iterate through numbers by value.

    For a Vec<i32>, this means the vector's elements are moved out of the vector and given to the loop.

    Because the vector is being consumed, you cannot use numbers afterward.

    For example:

    let numbers = vec![10, 20, 30];

    for item in numbers {
        println!("{}", item);
    }

    println!("{:?}", numbers); // ERROR

    Why?

    Because:

    for item in numbers

    consumed numbers.

    in &numbers
    for item in &numbers

    means:

    Borrow numbers and iterate through references to its elements.

    The vector remains owned by numbers.

    Therefore:

    let numbers = vec![10, 20, 30];

    for item in &numbers {
        println!("{}", item);
    }

    println!("{:?}", numbers); // OK

    works.

    This distinction is much more important than the &item issue.

    Now your four examples
    1. Your first code
    let numbers = vec![10, 20, 30];

    for item in numbers {
        println!("{}", item);
    }

    Here:

    for item in numbers

    numbers is being consumed.

    The loop receives the actual values:

    item: i32

    on each iteration.

    So:

    first iteration → item = 10
    second iteration → item = 20
    third iteration → item = 30

    No reference is involved.

    This is why you can simply write:

    println!("{}", item);

    And because i32 is Copy, the individual integers themselves are cheap values. But the important ownership point is that the Vec itself has been moved/consumed by the loop.

    2. Your second code
    let numbers = vec![10, 20, 30];

    for item in &numbers {
        println!("{}", item);
    }

    Now we have:

    &numbers

    So we're borrowing the vector.

    The loop receives references to its elements.

    Therefore:

    item

    has type:

    &i32

    So conceptually:

    first iteration → item is &10
    second iteration → item is &20
    third iteration → item is &30

    But println! can automatically handle the reference for {} in this situation, so:

    println!("{}", item);

    prints:

    10
    20
    30

    The important thing is that the vector has not been consumed.

    3. Your third code
    let numbers = vec![10, 20, 30];

    for item in &numbers {
        let reference_to_number: &i32 = item;

        println!("{}", reference_to_number);
    }

    This is basically the same as #2.

    You already have:

    item: &i32

    Then you do:

    let reference_to_number: &i32 = item;

    You're simply giving the same reference another variable name.

    So:

    item

    is:

    &i32

    and:

    reference_to_number

    is also:

    &i32

    Nothing special happens.

    You could think of this as:

    let reference_to_number = item;

    The explicit : &i32 just tells Rust what type you expect.

    So #3 is basically #2 with an unnecessary extra variable.

    4. Your fourth code
    let numbers = vec![10, 20, 30];

    for item in &numbers {
        println!("{}", *item);
    }

    Here:

    item

    is:

    &i32

    Then:

    *item

    dereferences it.

    That means:

    Give me the actual i32 value that this reference refers to.

    So:

    item   = reference
    *item  = actual i32

    Therefore:

    println!("{}", *item);

    prints the actual integer.

    So why do all four print the same thing?

    Because println! is only concerned with what you eventually display.

    All four eventually display:

    10
    20
    30

    But the ownership and types are different.

    The easiest way to see this is to add code after the loop.

    Example 1 after the loop
    let numbers = vec![10, 20, 30];

    for item in numbers {
        println!("{}", item);
    }

    println!("{:?}", numbers);

    This gives an error.

    Why?

    Because:

    for item in numbers

    consumed numbers.

    Example 2 after the loop
    let numbers = vec![10, 20, 30];

    for item in &numbers {
        println!("{}", item);
    }

    println!("{:?}", numbers);

    This works.

    Why?

    Because:

    &numbers

    only borrowed the vector.

    So this is one major reason we use &numbers.

    But your actual question is: "Why &item?"

    Now let's finally get to this:

    for &item in list

    This is different from:

    for item in &numbers

    Don't mix these two & positions.

    These:

    for item in &numbers

    and:

    for &item in list

    have completely different purposes.

    Let's start with this
    let numbers = vec![10, 20, 30];

    for item in &numbers {
        println!("{}", item);
    }

    We established:

    item: &i32

    So the loop variable is a reference.

    Now suppose we want the actual integer.

    We can write:

    for item in &numbers {
        println!("{}", *item);
    }

    Because:

    *item

    gets the actual i32.

    Now Rust provides another syntax for saying essentially:

    "When I receive &i32, immediately extract the i32 from it."

    That's:

    for &item in &numbers {
        println!("{}", item);
    }

    Now the important part is:

    &item

    Here the & is being used in a pattern.

    It says:

    "The thing coming from the iterator is a reference. I want to match that reference and bind the value inside it to item."

    Therefore:

    for item in &numbers

    gives:

    item: &i32

    while:

    for &item in &numbers

    gives:

    item: i32
    Compare these two very carefully
    Version A
    for item in &numbers {
        println!("{}", *item);
    }

    The type of item is:

    &i32

    So we need:

    *item

    to get:

    i32
    Version B
    for &item in &numbers {
        println!("{}", item);
    }

    The type of item is:

    i32

    The & in:

    &item

    already tells Rust to destructure the reference and give us the value.

    So we don't need:

    *item
    Why would someone prefer for &item in list?

    Because they want to work with the values directly.

    Your original function:

    fn largest_value(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    The author wants:

    item

    to be an i32.

    Why?

    Because then this is straightforward:

    if item > largest {
        largest = item;
    }

    They could have written:

    fn largest_value(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for item in list {
            if *item > largest {
                largest = *item;
            }
        }

        largest
    }

    This is also correct.

    Here item is:

    &i32

    and they explicitly use:

    *item

    to get the integer.

    The original author chose:

    for &item in list

    because they wanted the loop variable itself to be the integer.

    One very important correction

    You said:

    "all these types of code give the same output and all are correct. so why we use & before item?"

    You're absolutely right that the output is the same.

    But &item isn't necessary because the program needs it to print the number.

    It's used because the programmer wants a particular type for the loop variable.

    Without it:

    for item in list

    you get:

    item: &i32

    With it:

    for &item in list

    you get:

    item: i32

    So the choice is essentially:

    for item in list {
        // item is &i32
    }

    versus:

    for &item in list {
        // item is i32
    }

    And if you want the actual integer in the first version, you write:

    *item
    The three versions you should remember

    When iterating over a slice:

    for item in list

    means:

    item is &i32

    Then:

    *item

    means:

    give me the i32

    Or you can write:

    for &item in list

    which means:

    give me the i32 directly as item

    So these two are effectively equivalent for your i32 example:

    for item in list {
        println!("{}", *item);
    }

    and:

    for &item in list {
        println!("{}", item);
    }

    The first keeps item as a reference and explicitly dereferences it.

    The second uses pattern matching to get the i32 directly.

    That's the entire reason for &item in your original code.
    */












    
}