


struct ImportantExcerpt<'a>{
    part: &'a str,
}


fn main(){

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //an instance of ImportantExcerpt can't outlive
    //the reference it holds in its part field
    //the data in first_sentence is owned by novel

    let i = ImportantExcerpt{
        part:first_sentence,
    };


    /*
    Absolutely. This is a very good example for understanding lifetimes in a struct, because it combines String, &str, 
    borrowing, split(), next(), Option, expect(), and a lifetime parameter.

    Let's go through it from the beginning, without assuming those concepts are obvious.

    1. The struct definition
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    Let's separate this into pieces.

    struct ImportantExcerpt

    This defines a new struct type called ImportantExcerpt.

    The struct has one field:

    part

    So an ImportantExcerpt value contains one piece of data called part.

    But the important part is:

    part: &'a str

    This means:

    part is a reference to a string slice, and that reference has a lifetime represented by 'a.

    Remember that &str means a borrowed string slice.

    For example:

    let text = String::from("Hello world");
    let part = &text[0..5];

    Here:

    text: String
    part: &str

    text owns the actual string data.

    part does not own "Hello".

    It is only borrowing part of the data owned by text.

    2. What is <'a> doing?

    This:

    ImportantExcerpt<'a>

    declares a lifetime parameter named 'a.

    You can think of 'a as a name that Rust uses to describe a lifetime relationship.

    It is similar in spirit to a generic type parameter:

    struct Container<T> {
        value: T,
    }

    Here T is a generic type parameter.

    With:

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    'a is a lifetime parameter.

    So:

    <'a>

    means:

    "This struct has a lifetime parameter called 'a."

    And:

    &'a str

    means:

    "The reference stored in part must be valid for the lifetime represented by 'a."

    The lifetime does not mean that Rust creates some timer called 'a.

    It is compile-time information used by Rust's borrow checker.

    3. Now let's look at main
    fn main() {

    Everything inside main executes when the program starts.

    Now:

    let novel = String::from("Call me Ishmael. Some years ago...");

    This creates an owned String.

    The type is:

    String

    So conceptually:

    novel

    owns the text:

    Call me Ishmael. Some years ago...

    The important point is that novel owns this data.

    4. The interesting line

    Now we have:

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    This line looks complicated because several operations are chained together.

    Let's split it apart.

    Instead of thinking about the whole line at once, imagine writing:

    let pieces = novel.split('.');
    let first_piece = pieces.next();
    let first_sentence = first_piece.expect("Could not find a '.'");

    This is much easier to understand.

    Now let's examine each part.

    5. What does split('.') do?

    We have:

    novel.split('.')

    novel is a String.

    split() looks through the string and separates it around the character you give it.

    Here we give it:

    '.'

    which is a char representing a period.

    Our text is:

    Call me Ishmael. Some years ago...

    Splitting around . gives pieces approximately like:

    "Call me Ishmael"
    " Some years ago"
    ""
    ""

    The important thing is that split() does not create new owned Strings for all these pieces.

    It produces string slices that refer to the original novel.

    So the result is an iterator whose elements are:

    &str

    This is extremely important.

    The pieces are borrowed from novel.

    They don't own the text.

    6. What does .next() do?

    After:

    novel.split('.')

    we have an iterator.

    Then:

    .next()

    asks the iterator:

    "Give me the next item."

    The first item is:

    "Call me Ishmael"

    But because it is a borrowed slice, its type is:

    &str

    However, next() has to deal with the possibility that there is no next item.

    Therefore, it returns:

    Option<&str>

    Remember Option:

    enum Option<T> {
        Some(T),
        None,
    }

    So this:

    novel.split('.').next()

    produces something like:

    Some("Call me Ishmael")

    More precisely, the "Call me Ishmael" part is a &str borrowing from novel.

    7. What does .expect() do?

    We now have:

    novel.split('.').next()

    which gives:

    Option<&str>

    Then we call:

    .expect("Could not find a '.'")

    expect() says:

    "I expect this Option to contain a value. If it contains Some, give me the value. If it contains None, stop the program 
    and print this message."

    So:

    Some("Call me Ishmael")

    becomes:

    "Call me Ishmael"

    and therefore:

    first_sentence

    has type:

    &str

    It is a borrowed string slice.

    8. Who owns first_sentence's data?

    This is the most important part.

    We have:

    let novel = String::from("Call me Ishmael. Some years ago...");

    novel owns the actual string.

    Then:

    let first_sentence = novel.split('.').next().expect(...);

    first_sentence does not own "Call me Ishmael".

    It borrows that portion of novel.

    So conceptually:

    novel: String
    first_sentence: &str

    The relationship is:

    novel owns the text, while first_sentence borrows part of that text.

    This is exactly why a lifetime becomes important.

    9. Now create the struct

    We have:

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    Let's look at the types involved:

    first_sentence: &str

    and the struct expects:

    part: &'a str

    So Rust stores the reference in the struct.

    The resulting value:

    i

    has a type conceptually like:

    ImportantExcerpt<'some_lifetime>

    where 'some_lifetime represents the lifetime relationship of the borrowed string slice.

    You don't normally write the actual lifetime name when creating the value.

    Rust determines the appropriate lifetime from the references involved.

    10. What does the lifetime actually guarantee?

    Consider:

    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence =
        novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    The struct contains a reference:

    i.part

    and that reference ultimately points into the data owned by:

    novel

    Rust must therefore ensure that novel remains alive for as long as the reference stored in i.part is needed.

    This is what the lifetime parameter expresses.

    It prevents something like this:

    let i;

    {
        let novel = String::from("Call me Ishmael. Some years ago...");

        let first_sentence =
            novel.split('.').next().expect("Could not find a '.'");

        i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    println!("{}", i.part);

    This is rejected.

    Why?

    Because novel is created inside the inner block:

    {
        let novel = ...
    }

    When that block ends, novel is dropped.

    But i would still exist outside the block.

    Therefore i.part would refer to data that no longer exists.

    Rust prevents this at compile time.

    11. Notice something subtle

    The comment in your code says:

    // an instance of ImportantExcerpt can't outlive
    // the reference it holds in its part field

    That's a useful way to think about it.

    But let's make it slightly more precise.

    The important rule is:

    The reference stored inside ImportantExcerpt cannot be used after the data it refers to has become invalid.

    The lifetime parameter 'a allows Rust to express and check that relationship.

    It does not mean that the struct and the referenced string must have exactly the same lifetime.

    For example:

    let novel = String::from("Call me Ishmael. Some years ago...");

    {
        let first_sentence =
            novel.split('.').next().expect("Could not find a '.'");

        let excerpt = ImportantExcerpt {
            part: first_sentence,
        };

        println!("{}", excerpt.part);
    }

    println!("{}", novel);

    This is perfectly valid.

    Why?

    Because novel continues to exist after excerpt is gone.

    The reference inside excerpt is only needed while excerpt exists.

    So the reference doesn't need to remain valid for the entire lifetime of novel.

    12. The complete code with explicit type information

    You can make the important types easier to see by writing:

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel: String =
            String::from("Call me Ishmael. Some years ago...");

        let first_sentence: &str =
            novel
                .split('.')
                .next()
                .expect("Could not find a '.'");

        let i: ImportantExcerpt<'_> = ImportantExcerpt {
            part: first_sentence,
        };

        println!("{}", i.part);
    }

    Here:

    novel: String

    means novel owns the text.

    Then:

    first_sentence: &str

    means first_sentence borrows a portion of that text.

    Then:

    i: ImportantExcerpt<'_>

    means i is an ImportantExcerpt containing a reference.

    The '_ means:

    "Rust, determine the appropriate lifetime here."

    You don't normally need to write it when creating the value:

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    Rust infers it.

    13. Why can't we just write this?

    You might wonder why Rust doesn't allow:

    struct ImportantExcerpt {
        part: &str,
    }

    This is not valid.

    The problem is that the struct contains a reference, and Rust needs a lifetime parameter to express how that reference 
    relates to the data it refers to.

    Therefore we write:

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    There are two separate things here:

    <'a>

    declares the lifetime parameter.

    And:

    &'a str

    uses that lifetime parameter for the reference.

    This is very similar to generic types:

    struct Container<T> {
        value: T,
    }

    Here:

    <T>

    declares T.

    Then:

    value: T

    uses T.

    Likewise:

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    <'a> declares the lifetime parameter, and &'a str uses it.

    14. One more important point: first_sentence doesn't own anything

    This is worth emphasizing because it is easy to accidentally think:

    let first_sentence = ...

    means a new string was created.

    It wasn't.

    If you had:

    let first_sentence = String::from("Call me Ishmael");

    then:

    first_sentence: String

    and first_sentence would own its own string.

    But here:

    let first_sentence =
        novel.split('.').next().expect(...);

    the result is:

    &str

    So first_sentence is only a reference to part of novel.

    The actual ownership relationship is:

    novel owns the string data
    first_sentence borrows part of novel
    i.part borrows that same string slice

    Since you prefer no diagrams, the same thing in Rust types is:

    novel: String

    first_sentence: &str

    i.part: &'a str

    The & is the key:

    String   // owns
    &str     // borrows
    15. What the entire program is demonstrating

    This small example is primarily demonstrating:

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    A struct can contain a reference, but Rust needs a lifetime parameter to describe the validity of that reference.

    Then:

    let novel = String::from(...);

    creates the owner.

    Then:

    let first_sentence = novel.split('.').next().expect(...);

    creates a borrowed &str pointing into novel.

    Finally:

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    stores that borrowed &str inside the struct.

    So the essential idea is:

    ImportantExcerpt does not own the text. It borrows text owned by novel, and 'a lets Rust express and verify the lifetime 
    relationship of that borrowed reference.

    And the most important safety rule is:

    i.part cannot remain usable after the String containing its actual data has been destroyed.

    That is the reason this lifetime parameter exists.
     */


}