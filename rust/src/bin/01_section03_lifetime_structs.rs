
// We use Camel Case for creating a structure in rust. 
// In below struct the User is the name of the struct.
// This is named struct
struct User{
    name:String,
    age: i32,
    is_active:bool,
    score: f32
}

//This is tuple struct
struct Coordinates(f64,f64);



struct UnitStruct;


fn main(){


    let user1 = User{
        name : String::from("EFE"),
        age : 34,
        is_active : true,
        score : 18.83
    };

    println!("The user1 name is: {}", user1.name);
    println!("The user1 age is: {}", user1.age);
    println!("The user1 is active: {}", user1.is_active);
    println!("The user1 score is: {}", user1.score);

    /************************************************************************************************************
    1. Why do we need structs?

    Suppose you want to represent a person.

    A person might have:

    name
    age
    height
    is_student

    Without a struct, you could have separate variables:

    let name = String::from("EFE");
    let age = 34;
    let height = 180.5;
    let is_student = false;

    These variables are related to the same person, but Rust doesn't know that.

    A struct allows you to group them:

    struct Person {
        name: String,
        age: i32,
        height: f64,
        is_student: bool,
    }

    Now Person is a new type that you created.

    You can create a value of that type:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
        height: 180.5,
        is_student: false,
    };

    Conceptually:

    person
    |
    v
    +-----------------------------+
    | Person                      |
    |                             |
    | name       → "EFE"          |
    | age        → 34             |
    | height     → 180.5          |
    | is_student → false          |
    +-----------------------------+

    So a struct is essentially a custom container/type whose fields can have different types.

    2. What is a field?

    In:

    struct Person {
        name: String,
        age: i32,
        height: f64,
    }

    these are called fields:

    name
    age
    height

    Each field has a type:

    name   → String
    age    → i32
    height → f64

    The syntax is:

    field_name: type

    For example:

    age: i32,

    means:

    The field called age contains an i32.

    3. Creating a struct value

    First you define the type:

    struct Person {
        name: String,
        age: i32,
    }

    Then you create a value:

    let person1 = Person {
        name: String::from("EFE"),
        age: 34,
    };

    Notice the difference:

    struct Person { ... }

    defines the type.

    While:

    let person1 = Person { ... };

    creates an actual value of that type.

    Think about it like this:

    Person
    |
    | defines the structure
    |
    +----------------+
    | name: String   |
    | age: i32       |
    +----------------+


    person1
    |
    | actual value
    v
    +----------------+
    | name: "EFE"    |
    | age: 34        |
    +----------------+
    4. How do we access fields?

    Use the dot . operator:

    println!("{}", person1.name);
    println!("{}", person1.age);

    For example:

    let person1 = Person {
        name: String::from("EFE"),
        age: 34,
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);

    Output:

    Name: EFE
    Age: 34

    The syntax:

    person1.name

    means:

    Go to the value stored in person1 and access its name field.

    5. Structs can contain different types

    This is one of their biggest advantages.

    For example:

    struct Employee {
        name: String,
        age: i32,
        salary: f64,
        active: bool,
    }

    One struct contains:

    String
    i32
    f64
    bool

    This is different from a Vec.

    A Vec requires all elements to have the same type:

    let numbers = vec![10, 20, 30];

    All are integers.

    But a struct can combine different types:

    struct Person {
        name: String,
        age: i32,
        height: f64,
        active: bool,
    }
    6. Structs and ownership

    This connects directly to what you were just learning.

    Consider:

    struct Person {
        name: String,
        age: i32,
    }

    Then:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
    };

    The Person value owns its String field.

    Conceptually:

    person
    |
    | owns
    v
    +-------------------+
    | Person            |
    |                   |
    | name              |
    |   |               |
    |   | owns          |
    |   v               |
    | "EFE"             |
    |                   |
    | age = 34          |
    +-------------------+

    So the ownership relationship applies to fields too.

    If you move the entire struct:

    let person1 = Person {
        name: String::from("EFE"),
        age: 34,
    };

    let person2 = person1;

    then ownership of the entire Person moves:

    person1
    |
    X

    person2
    |
    | owns
    v
    Person
    |
    +---- name → "EFE"
    |
    +---- age → 34

    person1 can no longer be used.

    7. There are three main kinds of structs in Rust

    When people talk about "types of structs" in Rust, they usually mean these three forms:

    Named-field structs
    Tuple structs
    Unit-like structs

    Let's understand each.

    8. Type 1: Named-field struct

    This is the most common type.

    struct Person {
        name: String,
        age: i32,
    }

    The fields have names:

    name
    age

    You create it:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
    };

    And access fields:

    println!("{}", person.name);
    println!("{}", person.age);

    This is what you'll use most often when the data has meaningful names.

    9. Type 2: Tuple struct

    A tuple struct looks like this:

    struct Color(i32, i32, i32);

    There are no field names.

    Instead, fields are accessed by position:

    let red = Color(255, 0, 0);

    Then:

    println!("{}", red.0);
    println!("{}", red.1);
    println!("{}", red.2);

    Output:

    255
    0
    0

    Think of it as:

    red
    |
    v
    +----------------+
    | 255 | 0 | 0    |
    +----------------+
    ↑    ↑   ↑
    .0   .1  .2

    Compare this with a normal struct:

    struct Color {
        red: i32,
        green: i32,
        blue: i32,
    }

    You would use:

    color.red
    color.green
    color.blue

    With the tuple struct:

    Color(255, 0, 0)

    you use:

    color.0
    color.1
    color.2

    Tuple structs are useful when the positions are meaningful enough and naming every field would be unnecessary.

    10. Type 3: Unit-like struct

    A unit-like struct has no fields:

    struct User;

    That's it.

    You can create one:

    let user = User;

    There is no data inside it.

    So conceptually:

    User
    |
    v
    +------+
    |      |
    +------+

    You might wonder:

    Why would we ever need a struct that contains nothing?

    They are useful when the type itself represents something, even though it doesn't need to store data.

    For example, Rust programs can use such types as markers or for implementing particular behavior/traits.

    You don't need to worry about the advanced uses yet. Just remember:

    struct User;

    creates a type called User with no fields.

    11. So the three forms look like this
    Named-field struct
    struct Person {
        name: String,
        age: i32,
    }

    Use:

    person.name
    person.age
    Tuple struct
    struct Point(i32, i32);

    Use:

    point.0
    point.1
    Unit-like struct
    struct User;

    No fields.

    12. Structs can contain other structs

    This is very important in real programs.

    For example:

    struct Address {
        city: String,
        country: String,
    }

    struct Person {
        name: String,
        age: i32,
        address: Address,
    }

    Now:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
        address: Address {
            city: String::from("Baku"),
            country: String::from("Azerbaijan"),
        },
    };

    You can access:

    person.name

    and:

    person.address.city

    Conceptually:

    person
    |
    v
    +--------------------------------+
    | Person                         |
    |                                |
    | name → "EFE"                   |
    | age  → 34                      |
    |                                |
    | address                        |
    |    |                           |
    |    v                           |
    |  +--------------------------+  |
    |  | Address                  |  |
    |  | city    → "Baku"         |  |
    |  | country → "Azerbaijan"   |  |
    |  +--------------------------+  |
    +--------------------------------+

    So structs can be combined to create more complicated data structures.

    13. Structs can contain Vec

    For example:

    struct Student {
        name: String,
        grades: Vec<i32>,
    }

    Then:

    let student = Student {
        name: String::from("EFE"),
        grades: vec![18, 19, 20],
    };

    Conceptually:

    student
    |
    v
    +-------------------------+
    | Student                 |
    |                         |
    | name                    |
    |   ↓                     |
    | "EFE"                   |
    |                         |
    | grades                  |
    |   ↓                     |
    | Vec                     |
    |   ↓                     |
    | [18, 19, 20]            |
    +-------------------------+

    Again, the struct owns its fields.

    14. Structs can also contain references

    This is where your current ownership/borrowing lessons become important.

    You could have:

    struct Person<'a> {
        name: &'a str,
    }

    And:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Now:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | BORROWS
    |
    person.name

    The Person does not own "EFE".

    It merely has a reference to it.

    The 'a here is called a lifetime parameter. Don't worry about it yet. It's something you'll encounter when learning references in structs.

    15. Structs can have behavior too

    So far we've used structs only as data.

    But Rust allows us to attach functions to structs using impl.

    For example:

    struct Person {
        name: String,
        age: i32,
    }

    Then:

    impl Person {
        fn say_hello(&self) {
            println!("Hello, my name is {}", self.name);
        }
    }

    Now:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
    };

    person.say_hello();

    Output:

    Hello, my name is EFE

    This is how Rust implements something similar to what many languages call methods.

    Don't worry about impl yet. We'll get there after the basic struct concepts are clear.

    16. One very important distinction: struct vs object

    Since you may encounter object-oriented programming terminology:

    A Rust struct is not exactly the same thing as an object in languages such as Java or C++.

    A struct primarily defines a custom data type.

    For example:

    struct Person {
        name: String,
        age: i32,
    }

    Then:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
    };

    Person is the type.

    person is a value of that type.

    Then Rust lets you associate behavior with that type through impl.

    17. The big picture

    You've now seen several ways Rust can organize data:

    Primitive types
        ↓
    i32, f64, bool, char
        
    Compound types
        ↓
    Tuple
    Array
        
    Collections
        ↓
    Vec
    String
        
    Custom types
        ↓
    Struct
    Enum

    A struct is particularly useful when you want to say:

    "These pieces of data belong together and represent one thing."

    For example:

    struct NetworkDevice {
        hostname: String,
        ip_address: String,
        port: i32,
        active: bool,
    }

    Then:

    let server = NetworkDevice {
        hostname: String::from("server01"),
        ip_address: String::from("192.168.1.10"),
        port: 8080,
        active: true,
    };

    Now instead of managing four unrelated variables, you have one value:

    server
    |
    v
    NetworkDevice
    |
    +-- hostname   → "server01"
    |
    +-- ip_address → "192.168.1.10"
    |
    +-- port       → 8080
    |
    +-- active     → true

    And because hostname and ip_address are Strings, the struct owns those Strings unless you deliberately use references.
     */



    /************************************************************************************************************
    1. First, remember what ownership means

    Consider:

    let name = String::from("EFE");

    We have:

    name
    |
    | OWNS
    v
    String
    |
    v
    "EFE"

    name owns the String.

    When name goes out of scope, Rust drops the String.

    For example:

    fn main() {
        let name = String::from("EFE");

        println!("{}", name);
    }

    At the end of main:

    name → goes out of scope
        ↓
    String → dropped
        ↓
    heap memory → released

    So far, no lifetime parameter is necessary.

    2. Now let's say we don't want the struct to own the String

    Suppose we have:

    struct Person {
        name: String,
    }

    If we create:

    let name = String::from("EFE");

    let person = Person {
        name: name,
    };

    then the situation is:

    name
    |
    | ownership MOVES
    v
    person.name
    |
    | OWNS
    v
    "EFE"

    The Person now owns the String.

    After this:

    println!("{}", name);

    would not work because name was moved into the struct.

    3. What if we DON'T want the struct to own the String?

    Maybe we want:

    name
    |
    | OWNS
    v
    "EFE"

    person
    |
    | BORROWS
    v
    "EFE"

    In other words:

    name remains the owner, while Person just keeps a reference to it.

    We can write:

    struct Person<'a> {
        name: &'a String,
    }

    And:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Now we have:

    name
    |
    | OWNS
    v
    String
    |
    v
    "EFE"
    ^
    |
    | BORROWS
    |
    person.name

    This is the important difference.

    4. What does &String mean here?

    Look at:

    struct Person<'a> {
        name: &'a String,
    }

    The:

    &String

    means:

    name doesn't contain an owned String. It contains a reference to a String.

    So:

    let person = Person {
        name: &name,
    };

    means:

    "Person, here is a reference to my String. You can access it, but you don't own it."

    Conceptually:

    name
    |
    | OWNER
    v
    +---------+
    | String  |
    +---------+
        |
        v
    "EFE"
        ^
        |
        | reference
        |
    person.name
    5. Then what is 'a?

    This is the part that usually confuses beginners.

    You see:

    struct Person<'a> {
        name: &'a String,
    }

    and wonder:

    "What is 'a? Is it another variable?"

    No.

    'a is not a variable.

    It is a lifetime parameter.

    The easiest way to understand it initially is:

    'a represents the period of time during which the reference is guaranteed to remain valid.

    It does not mean:

    "Keep the String alive for some specific number of seconds."

    It is about the relationship between scopes.

    6. Why does Rust need to care about this?

    Imagine:

    let person;

    {
        let name = String::from("EFE");

        person = Person {
            name: &name,
        };
    }

    println!("{}", person.name);

    Something is wrong here.

    Let's look at the scopes.

    The outer scope:

    main
    ┌──────────────────────────────────────┐
    │                                      │
    │ let person;                          │
    │                                      │
    │   ┌──────────────────────────────┐   │
    │   │ let name = "EFE";             │   │
    │   │                              │   │
    │   │ person borrows name           │   │
    │   │                              │   │
    │   └──────────────────────────────┘   │
    │                                      │
    │ println!("{}", person.name);         │
    │                                      │
    └──────────────────────────────────────┘

    The problem is:

    name
    ↓
    created inside inner scope
    ↓
    inner scope ends
    ↓
    name is destroyed

    But:

    person
    ↓
    still exists
    ↓
    contains reference to name

    So we'd have:

    person.name
        |
        | reference
        v
    ????

    The thing that the reference points to no longer exists.

    That's a dangling reference.

    Rust prevents this.

    7. This is what the lifetime parameter helps describe

    When you write:

    struct Person<'a> {
        name: &'a String,
    }

    you're essentially saying:

    "A Person contains a reference, and that reference must remain valid for the lifetime represented by 'a."

    It doesn't tell Rust:

    "Keep this String alive."

    Instead, it tells Rust:

    "The reference inside this struct cannot outlive the thing it refers to."

    This distinction is extremely important.

    8. Lifetime does NOT control ownership

    This is another very important point.

    Suppose:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Who owns the String?

    name
    |
    | OWNS
    v
    "EFE"

    Who borrows it?

    person.name
    |
    | BORROWS
    v
    "EFE"

    The lifetime 'a doesn't change that.

    It doesn't make person the owner.

    It doesn't extend the life of name.

    It simply describes the validity relationship of the reference.

    9. Think about lifetime as a "validity period"

    For now, you can mentally think:

    'a
    ↓
    "How long is this reference valid?"

    Suppose:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    If name lives for this period:

    name:
    |-----------------------------|

    then the reference inside person must be valid within that period:

    person.name reference:
    |-----------------------------|

    It cannot continue after name is destroyed.

    10. A much simpler example

    Let's temporarily forget structs.

    Consider:

    let name = String::from("EFE");

    let reference = &name;

    We have:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | BORROWS
    |
    reference

    The reference reference cannot be valid after name is destroyed.

    Rust therefore tracks the lifetimes of these references.

    For example:

    let reference;

    {
        let name = String::from("EFE");

        reference = &name;
    }

    println!("{}", reference);

    Rust rejects this.

    Why?

    Because:

    name
    ↓
    destroyed here
    }

    but:

    reference
    ↓
    used here
    println!

    The reference would be pointing to something that no longer exists.

    11. Now put the same idea into a struct

    Without a reference:

    struct Person {
        name: String,
    }

    The struct owns the String.

    With a reference:

    struct Person<'a> {
        name: &'a String,
    }

    The struct borrows a String.

    So:

    OWNING VERSION
    ────────────────────────

    Person
    |
    +── name: String
        |
        └── OWNS "EFE"


    BORROWING VERSION
    ────────────────────────

    Person
    |
    +── name: &'a String
        |
        └── BORROWS "EFE"

    The 'a is necessary because Rust needs to track how long that borrowed reference can remain valid.

    12. Let's analyze this complete example
    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let name = String::from("EFE");

        let person = Person {
            name: &name,
        };

        println!("{}", person.name);
        println!("{}", name);
    }

    Let's go line by line.

    Step 1
    struct Person<'a>

    We're defining a struct called Person.

    'a says:

    This struct has a lifetime parameter because it will contain a reference.

    Step 2
    name: &'a String,

    This says:

    The name field is a reference to a String.

    And 'a describes the lifetime associated with that reference.

    Step 3
    let name = String::from("EFE");

    Now:

    name
    |
    | OWNS
    v
    "EFE"
    Step 4
    let person = Person {
        name: &name,
    };

    We create a Person.

    But instead of giving it:

    name

    we give it:

    &name

    So we're borrowing.

    Now:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | BORROWS
    |
    person.name
    Step 5
    println!("{}", person.name);

    The struct accesses the borrowed String.

    This is allowed.

    Step 6
    println!("{}", name);

    This is also allowed.

    Why?

    Because person never took ownership.

    name is still the owner.

    13. What happens when person disappears?

    Suppose:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Eventually person goes out of scope.

    Then:

    person → destroyed

    But:

    name
    |
    | STILL OWNS
    v
    "EFE"

    The String remains alive.

    Why?

    Because person was only borrowing it.

    14. Compare this with an owning struct
    Owning:
    struct Person {
        name: String,
    }

    let name = String::from("EFE");

    let person = Person {
        name: name,
    };

    Ownership:

    name
    |
    X  ownership moved

    person
    |
    | OWNS
    v
    "EFE"
    Borrowing:
    struct Person<'a> {
        name: &'a String,
    }

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Ownership:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | BORROWS
    |
    person.name

    That's the fundamental difference.

    15. One more important thing: &str

    In practice, you will often see:

    struct Person<'a> {
        name: &'a str,
    }

    rather than:

    struct Person<'a> {
        name: &'a String,
    }

    Why?

    Because &str is already a borrowed string slice.

    For example:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Here &name can be used as a string slice reference.

    Conceptually:

    name
    |
    | OWNS
    v
    String
    |
    v
    "EFE"
    ^
    |
    | &'a str
    |
    person.name

    This is often more flexible because the function/struct doesn't need to care whether the string came from a String or a string literal.

    For example:

    struct Person<'a> {
        name: &'a str,
    }

    can work with:

    let person1 = Person {
        name: "EFE",
    };

    and:

    let name = String::from("EFE");

    let person2 = Person {
        name: &name,
    };

    But don't worry too much about this distinction yet. We'll get deeper into String vs &str later.

    16. The most important mental model

    For now, I want you to separate these two concepts in your head:

    Ownership

    Answers:

    Who owns the data?

    String
    ↑
    |
    name
    OWNER
    Lifetime

    Answers:

    How long is a reference guaranteed to remain valid?

    name exists
    |-----------------------|

    reference must be valid
    |-----------------------|

    So:

    OWNERSHIP
        ↓
    Who is responsible for the data?

    LIFETIME
        ↓
    How long can this reference safely be used?

    They are related, but they are not the same thing.

    17. And one thing you should NOT conclude

    Don't think:

    "'a means the String lives for 'a."

    That's not correct.

    The String's lifetime is determined by the scope/ownership of the actual String.

    The lifetime parameter describes the reference's validity relationship.

    A useful beginner approximation is:

    struct Person<'a> {
        name: &'a String,
    }

    Read it as:

    "Person contains a reference to a String, and that reference has some lifetime 'a that must be long enough for every use of the Person that depends on it."

    You don't manually assign a value to 'a. Rust figures out the actual lifetime from how the value is used.

    Finally, connect this to what you've already learned

    You have now encountered three different situations:

    let string1 = String::from("EFE");

    Owner:

    string1
    |
    | OWNS
    v
    "EFE"

    Then:

    let string2 = string1;

    Ownership moves:

    string1 ──X

    string2 ──→ "EFE"
                OWNER

    Then:

    let string2 = &string1;

    Borrowing:

    string1 ──→ "EFE"
                OWNER
                ↑
                |
    string2 ───────┘
                BORROW

    And finally:

    struct Person<'a> {
        name: &'a String,
    }

    A struct stores that borrow:

    string1
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | BORROW
    |
    person.name

    The 'a exists because Rust must make sure that:

    person.name

    never remains usable after:

    string1

    has been destroyed.

    Once this is comfortable, then impl and functions/methods on structs become much easier, because you'll encounter things like:

    impl Person {
        fn print_name(&self) {
            println!("{}", self.name);
        }
    }
     */


    /************************************************************************************************************
    The most important thing first:

    A lifetime is not something that runs at runtime and does not keep a value alive. It is information Rust's 
    compiler uses to check that a reference can never be used after the thing it points to has been destroyed.

    Let's build this carefully.

    1. First forget structs completely

    Before talking about a reference inside a struct, let's understand a normal reference.

    Start with:

    fn main() {
        let name = String::from("EFE");

        let reference = &name;

        println!("{}", reference);
    }

    There are two variables:

    name
    |
    | OWNS
    v
    String
    |
    v
    "EFE"


    reference
    |
    | BORROWS
    v
    String

    So:

    name       = owner
    reference  = borrower

    reference does not own "EFE".

    It simply says:

    "I want to access the String that name owns."

    2. What is the danger with a reference?

    Imagine this:

    fn main() {
        let reference;

        {
            let name = String::from("EFE");

            reference = &name;
        }

        println!("{}", reference);
    }

    Let's understand the scopes.

    The outer scope is:

    main
    ┌─────────────────────────────────────────┐
    │                                         │
    │ let reference;                          │
    │                                         │
    │     ┌─────────────────────────────┐     │
    │     │                             │     │
    │     │ let name = String::from...  │     │
    │     │                             │     │
    │     │ reference = &name            │     │
    │     │                             │     │
    │     └─────────────────────────────┘     │
    │                                         │
    │ println!("{}", reference);              │
    │                                         │
    └─────────────────────────────────────────┘

    The important thing happens here:

    {
        let name = String::from("EFE");

        reference = &name;
    }

    When we reach:

    }

    name is destroyed.

    So:

    Before }:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | reference
    |
    reference

    After }:

    name → DESTROYED

    reference
        |
        | points to something that no longer exists
        v
    ????

    Then we try:

    println!("{}", reference);

    That would be using a dangling reference.

    A dangling reference means:

    A reference points to memory/data that is no longer valid.

    Rust's compiler prevents this.

    3. Now where does lifetime come into this?

    Rust needs to reason about something like:

    How long does `name` exist?

    How long does `reference` exist?

    Can `reference` outlive `name`?

    We can visualize the scopes as periods of time:

    name:
    |----------------------|
    created                destroyed


    reference:
    |----------------------------------|
    created                            used

    That's bad.

    The reference is being used after the thing it references is gone.

    Rust wants something like:

    name:
    |----------------------|
    created                destroyed

    reference:
    |------------------|
    created          last use

    The reference's useful lifetime must fit within the lifetime of the data it references.

    This is what the lifetime system is about.

    4. Very important: lifetime does NOT control anything at runtime

    This is probably where the previous explanation became confusing.

    Suppose you write:

    let name = String::from("EFE");
    let reference = &name;

    Rust does not create some runtime timer:

    'a = 10 seconds

    No.

    There is no:

    lifetime counter
    lifetime variable
    lifetime object

    running inside your program.

    Instead, during compilation, Rust analyzes your code and says:

    "Okay, this reference is used here. The thing it references exists until here. That's safe."

    Or:

    "This reference could be used after the thing it references is destroyed. That's not safe."

    So think:

    LIFETIME
    ↓
    compiler checking information
    ↓
    NOT runtime data

    This is extremely important.

    5. Now let's introduce the struct

    Suppose:

    struct Person {
        name: &String,
    }

    You might think:

    "Why can't I just do this?"

    Because Rust needs to know something about the reference stored inside the struct.

    When the compiler sees:

    name: &String

    it asks:

    "Okay, this Person contains a reference to a String. But what lifetime is associated with that reference?"

    Rust requires you to explicitly describe that relationship in a struct definition.

    So this:

    struct Person {
        name: &String,
    }

    produces an error.

    You need:

    struct Person<'a> {
        name: &'a String,
    }
    6. What does 'a actually mean here?

    Let's forget the letter a for a moment.

    Imagine we wrote:

    struct Person<'SOME_LIFETIME> {
        name: &'SOME_LIFETIME String,
    }

    That isn't valid Rust syntax, but it helps us understand the idea.

    The meaning is approximately:

    "This Person contains a reference to a String, and that reference has some lifetime that I will call 'SOME_LIFETIME."

    Rust uses a short name:

    'a

    So:

    struct Person<'a> {
        name: &'a String,
    }

    means:

    "Person has a reference field, and I call the lifetime associated with that reference 'a."

    That's all 'a means.

    It isn't a special magical lifetime.

    7. Does it have to be 'a?

    Absolutely not.

    You could write:

    struct Person<'a> {
        name: &'a String,
    }

    or:

    struct Person<'text> {
        name: &'text String,
    }

    or:

    struct Person<'whatever> {
        name: &'whatever String,
    }

    The name is chosen by the programmer.

    For example:

    struct Person<'text> {
        name: &'text String,
    }

    is perfectly valid.

    And this:

    struct Person<'banana> {
        name: &'banana String,
    }

    is syntactically possible too.

    But obviously, we normally choose meaningful names like:

    'a
    'text
    'name

    because humans have to read the code.

    So:

    'a
    'text
    'name

    are just names for lifetime parameters.

    They don't represent different kinds of lifetimes.

    8. Now let's see how Rust uses it

    Consider:

    struct Person<'a> {
        name: &'a String,
    }

    Then:

    fn main() {
        let name = String::from("EFE");

        let person = Person {
            name: &name,
        };

        println!("{}", person.name);
    }

    Let's go through it.

    First:

    let name = String::from("EFE");

    We have:

    name
    |
    | OWNS
    v
    "EFE"

    Then:

    let person = Person {
        name: &name,
    };

    We create:

    person
    |
    +---- name
            |
            | BORROWS
            v
        "EFE"

    The important point is:

    name        → owner
    person.name → reference

    The struct doesn't own the String.

    9. So what does 'a prevent?

    Here's the important example.

    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let person;

        {
            let name = String::from("EFE");

            person = Person {
                name: &name,
            };
        }

        println!("{}", person.name);
    }

    Let's follow it.

    Inside the inner scope:

    name
    |
    | OWNS
    v
    "EFE"

    person.name
    |
    | BORROWS
    v
    "EFE"

    Then:

    }

    name is destroyed.

    So:

    name → DESTROYED

    But person still exists:

    person
    |
    +---- name
            |
            | reference
            v
        ??????

    Then:

    println!("{}", person.name);

    would try to use that reference.

    Rust sees this situation during compilation and says:

    No. The reference stored inside person cannot remain valid that long.

    That's where the lifetime checking matters.

    10. 'a doesn't "fix" the problem

    This is another critical point.

    You might think:

    "If I write 'a, Rust now knows the lifetime and therefore keeps name alive."

    No!

    Writing:

    struct Person<'a> {
        name: &'a String,
    }

    does not extend the lifetime of name.

    It doesn't say:

    "Keep name alive."

    Instead it says:

    "The reference stored in Person has some lifetime 'a, and Rust must make sure the actual usage is valid."

    So if you create an invalid situation, 'a doesn't save you.

    Rust still rejects it.

    11. Think of 'a as a label Rust puts on the reference

    This is a useful mental model for now.

    Imagine:

    struct Person<'a> {
        name: &'a String,
    }

    as saying:

    Person
    |
    +---- name
            |
            | reference
            |
            +---- lifetime label: 'a

    The compiler then tracks the relationship.

    For example:

    String exists:
    |-------------------------|

    Person's reference:
    |--------------------|

    Good.

    But:

    String exists:
    |----------------|

    Person's reference:
    |--------------------------|

    Bad.

    The second reference could outlive the String.

    12. Now your question about THREE references

    You asked:

    "How can we pass three or more references to a struct?"

    Very easily.

    For example:

    struct Person<'a> {
        first_name: &'a String,
        last_name: &'a String,
        country: &'a String,
    }

    This struct has three references.

    Now:

    fn main() {
        let first_name = String::from("Abolfazl");
        let last_name = String::from("Azadegan");
        let country = String::from("Iran");

        let person = Person {
            first_name: &first_name,
            last_name: &last_name,
            country: &country,
        };

        println!("{}", person.first_name);
        println!("{}", person.last_name);
        println!("{}", person.country);
    }

    Now visualize the ownership:

    first_name
        |
        | OWNS
        v
    "Abolfazl"
        ^
        |
        | BORROWS
        |
    person.first_name


    last_name
        |
        | OWNS
        v
    "Azadegan"
        ^
        |
        | BORROWS
        |
    person.last_name


    country
        |
        | OWNS
        v
    "Iran"
        ^
        |
        | BORROWS
        |
    person.country

    The Person owns none of these strings.

    It has three references.

    13. Why can all three use the same 'a?

    This is an important question.

    We wrote:

    struct Person<'a> {
        first_name: &'a String,
        last_name: &'a String,
        country: &'a String,
    }

    You might ask:

    "Does 'a mean all three references have exactly the same lifetime?"

    For this simple example, we can think of 'a as saying:

    All three references must be valid for at least the lifetime required by this particular Person value.

    In our example, all three strings live long enough:

    first_name:
    |--------------------------------|

    last_name:
    |--------------------------------|

    country:
    |--------------------------------|

    person:
    |-------------------------|

    So everything is safe.

    But here's an important subtlety:

    A single lifetime parameter does not necessarily mean the three original variables are created and destroyed at exactly the same time.

    Rust can infer the actual lifetime required for the references based on how they're used.

    For now, think of 'a as a relationship/constraint, not a stopwatch.

    14. Can we give each reference a different lifetime name?

    Yes!

    For example:

    struct Person<'a, 'b, 'c> {
        first_name: &'a String,
        last_name: &'b String,
        country: &'c String,
    }

    Now there are three lifetime parameters:

    'a → lifetime associated with first_name
    'b → lifetime associated with last_name
    'c → lifetime associated with country

    This is legal Rust.

    But you don't automatically need three lifetime parameters just because you have three references.

    Often one lifetime parameter is sufficient.

    15. Why would we ever need different lifetimes?

    Suppose we have:

    first_name exists:
    |----------------------------|

    last_name exists:
    |-------------------|

    country exists:
    |-------------------------|

    They don't necessarily have exactly the same scope.

    You could represent the relationships separately:

    struct Person<'a, 'b, 'c> {
        first_name: &'a String,
        last_name: &'b String,
        country: &'c String,
    }

    Now Rust can reason about each reference separately.

    But don't make the mistake of thinking:

    'a = 10 seconds
    'b = 20 seconds
    'c = 30 seconds

    No.

    They're names representing lifetime relationships determined by the program.

    16. Let's make the different-lifetime example concrete

    Consider:

    struct Person<'a, 'b> {
        first_name: &'a String,
        last_name: &'b String,
    }

    Then:

    fn main() {
        let first_name = String::from("Abolfazl");

        let person;

        {
            let last_name = String::from("Azadegan");

            person = Person {
                first_name: &first_name,
                last_name: &last_name,
            };

            println!("{}", person.first_name);
            println!("{}", person.last_name);
        }
    }

    Inside the inner scope:

    first_name:
    |------------------------------------|

    last_name:
        |-------------------|

    person:
        |-------------------|

    Everything is okay inside the inner scope because both references are valid there.

    But if you tried:

    {
        let last_name = String::from("Azadegan");

        person = Person {
            first_name: &first_name,
            last_name: &last_name,
        };
    }

    println!("{}", person.first_name);
    println!("{}", person.last_name);

    the second reference would be invalid because:

    last_name:
        |----------------|
                        ↑
                    destroyed

    while:

    person.last_name:
        |----------------------|

    would need to remain usable longer.

    Rust rejects the program.

    17. This is why lifetime parameters exist in structs

    Now we can answer your original question:

    Why can't Rust just let me write &String in the struct?

    Because a struct value can potentially live for a different amount of time from the data it references.

    For example:

    Struct:
    |----------------------------|

    String:
    |----------------|

    That would create a possible dangling reference.

    Rust therefore makes you express the lifetime relationship:

    struct Person<'a> {
        name: &'a String,
    }

    And then the compiler checks how you actually use Person.

    18. The lifetime is checked at compile time

    This is perhaps the most important sentence in this whole explanation:

    Lifetimes are primarily a compile-time concept.

    Your compiled program doesn't normally contain a little object called 'a.

    For example:

    struct Person<'a> {
        name: &'a String,
    }

    doesn't mean the resulting Person contains:

    String pointer
    String length
    lifetime = 25 seconds

    No.

    The runtime object is essentially concerned with the reference itself.

    Conceptually:

    Person
    +------------------+
    | name reference   |
    +------------------+

    The 'a is used by the compiler when checking the source code.

    19. This is very similar to your earlier ownership example

    Remember:

    let vector = vec![1, 3, 5, 7];

    check_vector(vector);

    You passed the vector by value:

    vector
    |
    | OWNS
    v
    Vec

    Then:

    vector
    |
    X ownership moved

    vec_val
    |
    | OWNS
    v
    Vec

    With a reference:

    check_vector(&vector);

    you get:

    vector
    |
    | OWNS
    v
    Vec
    ^
    |
    | BORROW
    |
    function parameter

    The lifetime system is answering an additional question:

    How long is that borrow allowed to remain valid?

    20. Let's connect ownership, borrowing, and lifetime

    These three questions are different:

    Ownership
    Who owns the data?

    Example:

    let name = String::from("EFE");

    Answer:

    name owns the String
    Borrowing
    Who is temporarily allowed to access the data without owning it?

    Example:

    let reference = &name;

    Answer:

    reference borrows the String
    Lifetime
    How long can that reference safely remain valid?

    Example:

    name exists:
    |-----------------------|

    reference:
    |-------------------|

    The reference must not outlive the data it references.

    21. Now let's look at the simplest possible struct example

    Don't think about complicated lifetimes yet.

    Just understand this:

    struct Person<'a> {
        name: &'a String,
    }

    Read it from left to right:

    struct
    ↓
    I'm defining a struct

    Person
    ↓
    Its name is Person

    <'a>
    ↓
    This struct has a lifetime parameter named 'a

    name
    ↓
    The struct has a field called name

    : &'a String
    ↓
    That field contains a reference to a String
    and that reference is associated with lifetime 'a

    Then:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    means:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | BORROW
    |
    person.name

    And Rust makes sure that person.name cannot be used after name has been destroyed.

    22. One more thing: String vs &str

    You may have seen this:

    struct Person<'a> {
        name: &'a str,
    }

    instead of:

    struct Person<'a> {
        name: &'a String,
    }

    Don't let this confuse the lifetime concept.

    The lifetime idea is exactly the same.

    &'a str means:

    a reference to a string slice, valid according to lifetime 'a.

    For example:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    The ownership is:

    name
    |
    | OWNS
    v
    String
    |
    v
    "EFE"
    ^
    |
    | &str
    |
    person.name

    The lifetime is still about making sure that:

    person.name

    doesn't outlive:

    name
    23. And finally, an important correction to a common mental model

    Don't think:

    'a = lifetime of the String

    That's not quite right.

    Think:

    'a = lifetime parameter used to describe
        how long the reference is valid
        relative to the data it references

    The String itself has its own lifetime based on when its owner goes out of scope.

    For example:

    String's lifetime:
    |----------------------|

    The reference might have:

    Reference's lifetime:
    |------------------|

    The reference is allowed to be shorter.

    But not:

    String's lifetime:
    |------------------|

    Reference's lifetime:
    |--------------------------|

    because then the reference would outlive the data.

    24. So here is the complete picture

    Imagine:

    struct Person<'a> {
        name: &'a String,
    }

    and:

    fn main() {
        let name = String::from("EFE");

        let person = Person {
            name: &name,
        };

        println!("{}", person.name);
    }

    At runtime, conceptually:

                OWNERSHIP
                    │
                    ▼
    name ───────────────────► String
                                │
                                ▼
                            EFE
                                ▲
                                │
                                │ reference
                                │
                        person.name

    At compile time, Rust additionally checks:

    Does the String remain alive
    for as long as person.name
    could be used?

    If yes:

                SAFE ✓

    If no:

                ERROR ✗
                ↓
        possible dangling
            reference

    And 'a is the mechanism in the type definition that lets you express this lifetime relationship.

    One last answer to each of your questions

    "What happens if I don't put 'a?"

    If a struct contains a reference, you generally cannot define it simply as:

    struct Person {
        name: &String,
    }

    Rust will complain because the reference needs a lifetime parameter in the struct type. You would write:

    struct Person<'a> {
        name: &'a String,
    }

    "Does 'a control the lifetime?"

    Not in the sense of controlling it at runtime. It is a compile-time constraint/information that Rust uses to verify that references remain valid.

    "Can I have three references?"

    Yes:

    struct Person<'a> {
        first_name: &'a String,
        last_name: &'a String,
        country: &'a String,
    }

    You can have many.

    "Do all references have to use 'a?"

    No. 'a is just a name.

    You could write:

    struct Person<'text> {
        name: &'text String,
    }

    or:

    struct Person<'name> {
        name: &'name String,
    }

    "Can I have different lifetime parameters?"

    Yes:

    struct Person<'a, 'b, 'c> {
        first_name: &'a String,
        last_name: &'b String,
        country: &'c String,
    }

    But you don't necessarily need different ones. One lifetime parameter can be enough when the references can be 
    described with the same lifetime relationship.
     */





    /************************************************************************************************************
     the { } in that example are not a function. They create a scope, which is simply a region of code where variables 
     can exist.

    This is fundamental to understanding Rust ownership and lifetimes, so let's go very slowly.

    1. You've already seen { } before

    Look at a normal Rust function:

    fn main() {
        let name = String::from("EFE");

        println!("{}", name);
    }

    You have:

    fn main() {
        // code
    }

    The { } after main() define the body of the function.

    So here, the braces are part of the function syntax.

    But Rust also allows us to create { } by themselves:

    fn main() {

        {
            let name = String::from("EFE");
        }

    }

    These braces are not defining a function.

    They create a new scope.

    2. What is a scope?

    A scope is simply:

    A region of code where a variable exists and can be used.

    Let's start with:

    fn main() {
        let name = String::from("EFE");

        println!("{}", name);
    }

    The entire body of main is a scope:

    fn main() {
    │
    │   let name = ...
    │
    │   println!("{}", name);
    │
    }  ← end of main's scope

    name is created inside this scope.

    Therefore name exists until the end of this scope.

    3. We can create another scope inside main

    For example:

    fn main() {

        let name = String::from("EFE");

        {
            let age = 34;

            println!("{}", age);
        }

        println!("{}", name);
    }

    Now there are two scopes.

    The outer scope:

    fn main() {
    │
    │   let name = ...
    │
    │   ┌─────────────────────┐
    │   │                     │
    │   │   let age = 34;     │
    │   │                     │
    │   │   println!("{}",age)│
    │   │                     │
    │   └─────────────────────┘
    │
    │   println!("{}", name);
    │
    }

    The inner { } is called an inner scope.

    4. What happens to age?

    age is created here:

    {
        let age = 34;
    }

    So age belongs to this inner scope.

    When Rust reaches:

    }

    the inner scope ends.

    Therefore age is no longer available.

    This won't work:

    fn main() {

        {
            let age = 34;
        }

        println!("{}", age); // ❌
    }

    Why?

    Because age was created inside the inner scope:

    main scope
    ┌───────────────────────────────────┐
    │                                   │
    │   inner scope                     │
    │   ┌───────────────────────────┐   │
    │   │ let age = 34;             │   │
    │   │                           │   │
    │   └───────────────────────────┘   │
    │                                   │
    │   println!("{}", age); ❌         │
    │                                   │
    └───────────────────────────────────┘

    age only exists inside the inner { }.

    5. Think of { } as a room

    A simple mental model:

    {
        // You are inside a room
    }

    A variable created inside that room belongs to that room:

    {
        let name = String::from("EFE");

        println!("{}", name); // ✅
    }

    println!("{}", name); // ❌

    You can use name while you're inside the room.

    Once you leave:

    }

    name is no longer accessible.

    6. Now let's return to your original example

    You asked about:

    fn main() {
        let reference;

        {
            let name = String::from("EFE");

            reference = &name;
        }

        println!("{}", reference);
    }

    Now we can understand why I deliberately created that inner { }.

    There are two scopes:

    OUTER SCOPE: main
    ┌───────────────────────────────────────────┐
    │                                           │
    │ let reference;                            │
    │                                           │
    │   INNER SCOPE                             │
    │   ┌───────────────────────────────────┐   │
    │   │                                   │   │
    │   │ let name = String::from("EFE");   │   │
    │   │                                   │   │
    │   │ reference = &name;                │   │
    │   │                                   │   │
    │   └───────────────────────────────────┘   │
    │                                           │
    │ println!("{}", reference);                │
    │                                           │
    └───────────────────────────────────────────┘

    Why did I do this?

    Because I wanted to create a situation where:

    reference

    lives longer than:

    name

    And that is exactly where the lifetime problem becomes visible.

    7. Let's follow name

    Here:

    {
        let name = String::from("EFE");

    name is created.

    It exists inside this inner scope:

    name's scope
    ┌──────────────────────────┐
    │                          │
    │ let name = "EFE";        │
    │                          │
    │ reference = &name;       │
    │                          │
    └──────────────────────────┘
                ↑
            name dies
            here

    When Rust reaches:

    }

    the scope ends.

    Therefore name goes out of scope.

    Because name owns the String:

    name
    |
    | OWNS
    v
    "EFE"

    the String is dropped.

    8. But look at reference

    We created reference outside the inner scope:

    let reference;

    Therefore it belongs to the outer scope.

    Its scope is approximately:

    reference's scope
    ┌─────────────────────────────────────┐
    │                                     │
    │ let reference;                      │
    │                                     │
    │   ┌───────────────────────┐         │
    │   │ name                  │         │
    │   │                       │         │
    │   │ reference = &name     │         │
    │   └───────────────────────┘         │
    │                                     │
    │ println!("{}", reference);          │
    │                                     │
    └─────────────────────────────────────┘

    So reference potentially exists longer than name.

    9. Now look at the dangerous line

    Inside the inner scope:

    reference = &name;

    We're saying:

    "Make reference point to name."

    At that moment:

    name
    |
    | OWNS
    v
    "EFE"
    ^
    |
    | reference

    Everything is fine right now.

    But then:

    }

    The inner scope ends.

    name is destroyed.

    So now:

    name → DESTROYED

    But reference still exists:

    reference
        |
        | reference to...
        v
    ????

    And then:

    println!("{}", reference);

    tries to use it.

    That's why Rust rejects this program.

    10. Why did we need the inner { } to demonstrate this?

    If we wrote:

    fn main() {
        let name = String::from("EFE");

        let reference = &name;

        println!("{}", reference);
    }

    there is no problem.

    Both variables are in the same scope:

    main
    ┌─────────────────────────────┐
    │                             │
    │ name                       │
    │ reference                  │
    │                             │
    │ println!("{}", reference)  │
    │                             │
    └─────────────────────────────┘

    name remains alive when reference is used.

    So there is no lifetime problem.

    I deliberately added:

    {
        ...
    }

    to make name have a shorter scope.

    11. Scope and lifetime are related, but they are NOT exactly the same thing

    This is another important distinction.

    Scope is about where a variable is accessible in the source code.

    For example:

    {
        let name = String::from("EFE");
    }

    The scope of name is this block.

    Lifetime is about how long a value/reference is valid.

    For your beginner understanding, you can initially think:

    scope
    ↓
    where the variable is accessible

    lifetime
    ↓
    how long a reference is valid

    They are closely related, but Rust's actual lifetime analysis is more precise than simply saying "lifetime = braces."

    12. Here's an important example

    Consider:

    fn main() {
        let name = String::from("EFE");

        {
            let reference = &name;

            println!("{}", reference);
        }

        println!("{}", name);
    }

    This works.

    Why?

    Because:

    name
    ┌────────────────────────────────────────┐
    │                                        │
    │  reference                             │
    │  ┌──────────────────┐                  │
    │  │                  │                  │
    │  └──────────────────┘                  │
    │                                        │
    └────────────────────────────────────────┘

    name lives longer than reference.

    So:

    name lifetime:
    |--------------------------------------|

    reference lifetime:
        |------------------|

    That's safe.

    13. Reverse the situation

    Now:

    fn main() {
        let reference;

        {
            let name = String::from("EFE");

            reference = &name;
        }

        println!("{}", reference);
    }

    Now:

    name:
        |----------------|
                        ↑
                    destroyed


    reference:
    |--------------------------|

    The reference potentially lasts longer than the thing it points to.

    That's unsafe.

    So Rust says no.

    14. { } can also be used simply to organize code

    The braces aren't only for demonstrating lifetimes.

    You can deliberately create scopes:

    fn main() {

        {
            let x = 10;
            println!("{}", x);
        }

        {
            let y = 20;
            println!("{}", y);
        }

    }

    Here:

    Outer scope
    ┌────────────────────────────────────┐
    │                                    │
    │  Inner scope 1                     │
    │  ┌────────────────────┐            │
    │  │ x = 10             │            │
    │  └────────────────────┘            │
    │                                    │
    │  Inner scope 2                     │
    │  ┌────────────────────┐            │
    │  │ y = 20             │            │
    │  └────────────────────┘            │
    │                                    │
    └────────────────────────────────────┘

    x only exists in the first block.

    y only exists in the second block.

    15. There is another useful consequence

    Because ownership is connected to scopes, this:

    {
        let name = String::from("EFE");

        println!("{}", name);
    }

    means that when the block ends:

    }

    name goes out of scope.

    Since name owns the String:

    name
    |
    | OWNS
    v
    String
    |
    v
    "EFE"

    Rust drops the String at the end of that scope.

    So the relationship is:

    scope ends
        ↓
    owner goes out of scope
        ↓
    owned value is dropped

    This is one of the fundamental reasons scopes are so important for understanding Rust ownership.

    16. Now you can understand our lifetime example

    The entire purpose of this:

    fn main() {
        let reference;

        {
            let name = String::from("EFE");

            reference = &name;
        }

        println!("{}", reference);
    }

    was to create this situation:

    OUTER SCOPE
    ┌─────────────────────────────────────────────┐
    │                                             │
    │ reference                                   │
    │                                             │
    │    INNER SCOPE                              │
    │    ┌───────────────────────────────┐        │
    │    │                               │        │
    │    │ name                          │        │
    │    │   ↓                           │        │
    │    │ "EFE"                         │        │
    │    │   ↑                           │        │
    │    │ reference = &name             │        │
    │    │                               │        │
    │    └───────────────────────────────┘        │
    │          name destroyed                     │
    │                                             │
    │ println!("{}", reference);  ← BAD          │
    │                                             │
    └─────────────────────────────────────────────┘

    The compiler detects:

    reference could be used after name has gone away.

    And therefore the program doesn't compile.

    The one thing I want you to take away right now

    When you see:

    {
        // some code
    }

    by itself, the { } are creating a block/scope.

    They are not a function.

    A function looks like:

    fn function_name() {
        // function body
    }

    A standalone block looks like:

    {
        // block
    }

    And a block creates a smaller scope inside its surrounding scope.

    For ownership, this is crucial because:

    VARIABLE GOES OUT OF SCOPE
            ↓
    OWNER IS FINISHED
            ↓
    OWNED VALUE IS DROPPED

    And for references:

    REFERENCE
        ↓
    must not be used after
        ↓
    THE DATA IT REFERENCES
        ↓
    IS DESTROYED

    This is the problem that lifetime checking solves.
      */




    /************************************************************************************************************
      The most important thing is this:

    String owns text. &str is a borrowed view of text.

    But that sentence alone isn't enough, so let's build it from the beginning.

    Part 1 — What is a String?

    Suppose we write:

    fn main() {
        let name = String::from("EFE");
    }

    There are several things happening here.

    1. "EFE" by itself

    When you write:

    "EFE"

    this is not a String.

    It is a string literal, and its type is:

    &str

    We'll explain &str carefully in a moment.

    For now, just remember:

    "EFE"   →   &str
    2. String::from("EFE")

    Now:

    let name = String::from("EFE");

    creates an actual String.

    You can think of it as:

    String
    ┌─────────────────────────────┐
    │ owns the text "EFE"          │
    └─────────────────────────────┘

    The important word is owns.

    name is the owner of this String.

    let name = String::from("EFE");

    means approximately:

    name
    │
    │ owns
    ▼
    "EFE"

    Because String owns its text, it can be changed and grown.

    For example:

    let mut name = String::from("EFE");

    name.push_str(" SHENEM");

    println!("{}", name);

    Output:

    EFE SHENEM

    We can do this because String is an owned, growable string.

    Part 2 — Why does String need to own the text?

    Imagine:

    let mut name = String::from("EFE");

    You can add more text:

    name.push_str(" ABC");

    Now the String contains:

    EFE ABC

    And later:

    name.push_str(" XYZ");

    Now:

    EFE ABC XYZ

    So String needs storage that it can manage and potentially grow.

    Conceptually, you can imagine a String as having something like:

    name
    │
    ▼
    ┌──────────────────────────────┐
    │ pointer                      │
    │ length                       │
    │ capacity                     │
    └──────────────────────────────┘
                │
                ▼
            memory containing
            E F E

    You don't need to memorize the internal implementation yet.

    The important point is:

    String owns the memory containing its text.

    Part 3 — Then what is &str?

    Now let's look at:

    let name = "EFE";

    What is the type of name?

    It's:

    &str

    So:

    let name = "EFE";

    does not create an owned String.

    Instead, name is a reference to some text.

    Think of it like this:

    "E F E"
    ▲
    │
    │ reference
    │
    name

    name does not own the text.

    It is simply saying:

    "I want to access this text."

    That's why the & is important.

    You already encountered & with normal borrowing:

    let string1 = String::from("EFE");

    let reference = &string1;

    Here:

    string1
    │
    │ owns
    ▼
    String "EFE"
    ▲
    │
    │ borrows
    │
    reference

    Exactly the same basic idea applies to &str:

    &str

    means:

    a reference to a sequence of UTF-8 text.

    Part 4 — Why is "EFE" a &str?

    This is an important point.

    When Rust sees:

    "EFE"

    the text is already part of the compiled program.

    Conceptually:

    Your compiled program
    ┌─────────────────────────────┐
    │                             │
    │   "EFE"                     │
    │                             │
    └─────────────────────────────┘

    Then:

    let name = "EFE";

    creates a reference to that text.

    So:

    name
    │
    │ &str
    ▼
    "E F E"

    name doesn't own those bytes.

    The program itself contains the string literal.

    That's why this works:

    let name = "EFE";

    println!("{}", name);

    But you cannot do:

    let mut name = "EFE";

    name.push_str(" ABC");

    because name is an &str.

    An &str doesn't own a growable buffer.

    Part 5 — The most important comparison

    Look at these two:

    let name1 = String::from("EFE");

    and:

    let name2 = "EFE";

    They look similar, but they are fundamentally different.

    First:
    let name1 = String::from("EFE");

    Type:

    String

    Conceptually:

    name1
    │
    │ OWNS
    ▼
    ┌─────────┐
    │ E F E   │
    └─────────┘
    Second:
    let name2 = "EFE";

    Type:

    &str

    Conceptually:

    name2
    │
    │ BORROWS / REFERENCES
    ▼
    ┌─────────┐
    │ E F E   │
    └─────────┘

    The critical difference:

    String
    ↓
    OWNER

    &str
    ↓
    BORROWER / VIEW
    Part 6 — Why do we call &str a "slice"?

    This word can be confusing.

    Suppose we have:

    let name = String::from("ABCDEFG");

    The String owns:

    A B C D E F G

    Now suppose we want only:

    C D E

    We can write:

    let part = &name[2..5];

    Now:

    name
    │
    │ owns
    ▼
    A B C D E F G
        └─────┘
        CDE
        ▲
        │
        │ borrowed slice
        │
        part

    part is:

    &str

    So &str can refer to part of a String.

    That's why it's called a string slice.

    A slice is basically:

    "Give me access to this portion of some string."

    Part 7 — A very important distinction

    There are actually two situations where you can have an &str.

    Situation 1 — Entire string literal
    let name = "EFE";

    Here:

    name: &str

    It refers to the entire literal.

    Situation 2 — Part of a String
    let name = String::from("ABCDEFG");

    let part = &name[2..5];

    Here:

    name: String
    part: &str

    name owns all the text.

    part borrows only part of it.

    So:

    String
    ┌──────────────────────────┐
    │ A B C D E F G            │
    └──────────────────────────┘
        ↑     ↑
        │     │
        └─────┘
        &str
        "CDE"
    Part 8 — Why can't &str exist after the String disappears?

    This connects directly to the lifetime discussion you were asking about earlier.

    Look at:

    fn main() {
        let part;

        {
            let name = String::from("ABCDEFG");

            part = &name[2..5];
        }

        println!("{}", part);
    }

    The problem is:

    Outer scope
    ────────────────────────────────────────

    part
    │
    │ wants to borrow
    ▼

    Inner scope
        ┌─────────────────────────────┐
        │ name = "ABCDEFG"             │
        │                             │
        │ part → "CDE"                 │
        └─────────────────────────────┘
                ↓
        inner scope ends

        name is dropped
                ↓
        "ABCDEFG" is gone

    Then we try:

    println!("{}", part);

    But part would be referring to something that no longer exists.

    Rust therefore rejects the program.

    This is exactly where lifetimes become relevant.

    But don't worry about lifetimes yet. First make sure this is clear:

    String
        ↓
    owns the text

    &str
        ↓
    borrows/references text
    Part 9 — String vs &str with a very simple analogy

    Imagine a book.

    String = the person who owns the book

    &str = someone looking at some pages of the book

    If I own the book:

    String
    ↓
    I own the book

    Someone else can look at it:

    &str
    ↓
    I am looking at some text

    The person looking at the book doesn't own it.

    And if the owner destroys the book, the person can't continue looking at those pages.

    Again, the analogy is only to understand ownership vs borrowing.

    Part 10 — Now let's put String and &str into a struct

    Now we're ready for structs.

    Suppose we want a person:

    struct Person {
        name: String,
    }

    This means:

    Person
    ┌────────────────────┐
    │ name: String       │
    └────────────────────┘

    Let's create one:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
        };

        println!("{}", person.name);
    }

    Here person.name is a String.

    Therefore:

    person
    │
    │ owns
    ▼
    String "EFE"

    The Person struct owns the name.

    This is usually very straightforward.

    Part 11 — What if the struct doesn't want to own the String?

    Now imagine we already have:

    let name = String::from("EFE");

    and we want a Person that merely borrows that String.

    We might want:

    struct Person {
        name: &String,
    }

    But Rust says:

    No.

    This is where the lifetime syntax appears.

    We have to write:

    struct Person<'a> {
        name: &'a String,
    }

    Now let's understand every part.

    Part 12 — struct Person<'a>

    Start with:

    struct Person<'a> {

    The:

    'a

    is a lifetime parameter.

    You can think of it as a label that allows us to describe how long the reference inside the struct is valid.

    Then:

    name: &'a String,

    means:

    name is a reference to a String, and that reference has the lifetime represented by 'a.

    So conceptually:

    Person
    ┌─────────────────────────┐
    │ name                    │
    │   │                     │
    │   │ &'a String           │
    └───┼─────────────────────┘
        │
        │ borrows
        ▼
    String "EFE"

    The Person does not own the String.

    The String is owned somewhere else.

    Part 13 — Complete example
    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let name = String::from("EFE");

        let person = Person {
            name: &name,
        };

        println!("{}", person.name);
    }

    Let's execute this mentally.

    First:

    let name = String::from("EFE");

    We have:

    name
    │
    │ owns
    ▼
    String
    "EFE"

    Then:

    let person = Person {
        name: &name,
    };

    We create a reference:

    person.name
        │
        │ borrows
        ▼
    name
        │
        │ owns
        ▼
    String "EFE"

    So:

    Person
    ┌──────────────────┐
    │ name ────────────┼────┐
    └──────────────────┘    │
                            ▼
                    String "EFE"
                            ▲
                            │
                        owner
                        variable
                        `name`

    The important thing:

    person does not own "EFE".

    name still owns it.

    Part 14 — Why does Rust need 'a here?

    Because Rust needs to make sure this cannot happen:

    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let person;

        {
            let name = String::from("EFE");

            person = Person {
                name: &name,
            };
        }

        println!("{}", person.name);
    }

    Look at the scopes:

    main scope
    ─────────────────────────────────────
    person
    │
    │
    ▼
    inner scope
        ┌───────────────────────┐
        │ name                   │
        │   │                   │
        │   └── Person.name     │
        │                       │
        └───────────────────────┘
                ↓
            name is dropped
                ↓
            Person.name
            would point to
            destroyed data

    Rust's lifetime system prevents this.

    The lifetime parameter doesn't keep name alive.

    This is extremely important.

    It doesn't mean:

    'a = keep name alive

    Instead, it means:

    'a = describe/constraint the validity of this reference

    Rust checks that the reference inside Person cannot be used longer than the data it references is valid.

    Part 15 — What if the struct owns the String?

    Then we don't need a lifetime at all.

    Compare:

    Struct owns the String
    struct Person {
        name: String,
    }

    Usage:

    let person = Person {
        name: String::from("EFE"),
    };

    Ownership:

    person
    │
    │ owns
    ▼
    String "EFE"

    No lifetime parameter.

    Struct borrows the String
    struct Person<'a> {
        name: &'a String,
    }

    Usage:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Ownership:

    name
    │
    │ owns
    ▼
    String "EFE"
    ▲
    │
    │ borrows
    │
    person.name

    Because the struct contains a reference, we need to describe that reference's lifetime.

    Part 16 — You will also see &str in structs

    Instead of:

    struct Person<'a> {
        name: &'a String,
    }

    we can often write:

    struct Person<'a> {
        name: &'a str,
    }

    This is very common.

    Here:

    name: &'a str

    means:

    name is a borrowed string slice.

    For example:

    let name = String::from("EFE");

    let person = Person {
        name: &name,
    };

    Here:

    name: String
        owns "EFE"

    person.name: &str
                borrows "EFE"

    The &str can also borrow a portion:

    let name = String::from("ABCDEFG");

    let person = Person {
        name: &name[2..5],
    };

    Then:

    name
    ┌─────────────────────┐
    │ A B C D E F G       │
    └─────────────────────┘
        ▲
        │
        │ person.name
        │
        C D E

    person.name is an &str referring to "CDE".

    The picture I want you to remember

    Forget the complicated lifetime syntax for a moment.

    First remember these three things:

    String
    │
    └── OWNS text


    &String
    │
    └── BORROWS an entire String


    &str
    │
    └── BORROWS text (a string slice)

    And then:

    struct Person {
        name: String,
    }

    means:

    Person OWNS the name

    while:

    struct Person<'a> {
        name: &'a str,
    }

    means:

    Person BORROWS the name

    And 'a is there because Rust needs to describe the lifetime of that borrowed reference.

    One more important point: &str is generally more flexible than &String for function parameters and struct fields because it 
    can refer both to a whole String and to a string literal or a substring. That's why you'll see &str very frequently in Rust.

    If this distinction is clear, the next step should be to take one Person<'a> example and trace exactly what happens to name, 
    person.name, the reference, the scopes, and 'a line by line.
       */


    /************************************************************************************************************
        Sure. Let's explain only str in a struct, without going back over ownership or unrelated concepts.

    1. Look at this struct
    struct Person<'a> {
        name: &'a str,
    }

    The important part is:

    name: &'a str

    Break it into pieces:

    &'a str
    │  │
    │  └── str
    │
    └───── reference

    So name is a reference to a string slice.

    2. What is str?

    str is the Rust type representing a string slice.

    You normally don't use a bare str directly. You normally see:

    &str

    For example:

    let name: &str = "EFE";

    Here:

    name
    │
    │ &str
    ▼
    "E F E"

    name is an &str.

    The & means that we are working with a reference to a str.

    3. Why does the struct say &'a str?

    Because the struct contains a reference.

    struct Person<'a> {
        name: &'a str,
    }

    The type of name is:

    &'a str

    You can read this as:

    "name is a reference to a str, and 'a describes the lifetime of that reference."

    The 'a is a lifetime parameter.

    For now, don't think of 'a as a duration or timer. It is simply a name that Rust uses to describe the lifetime relationship 
    of the reference.

    4. Using the struct

    For example:

    struct Person<'a> {
        name: &'a str,
    }

    fn main() {
        let person = Person {
            name: "EFE",
        };

        println!("{}", person.name);
    }

    The value:

    "EFE"

    has type:

    &str

    So it can be placed into:

    name: &'a str

    The resulting structure is conceptually:

    Person
    ┌─────────────────┐
    │ name            │
    │   │             │
    │   ▼             │
    │  "EFE"          │
    └─────────────────┘

    person.name is an &str.

    5. &str can also refer to part of a String

    This is one of the most important reasons we call it a slice.

    struct Person<'a> {
        name: &'a str,
    }

    fn main() {
        let full_name = String::from("ABCDEFG");

        let person = Person {
            name: &full_name[2..5],
        };

        println!("{}", person.name);
    }

    full_name contains:

    A B C D E F G
    0 1 2 3 4 5 6

    This:

    &full_name[2..5]

    means:

    C D E

    because 2..5 means:

    start at 2
    stop before 5

    So:

    full_name
    ┌─────────────────┐
    │ A B C D E F G   │
    │     └─────┘     │
    │       CDE       │
    └─────────────────┘
            ▲
            │
    person.name
        &str

    Therefore &str doesn't necessarily mean "the entire string."

    It can mean:

    a reference to some string data, possibly only a portion of it.

    That's why it's called a string slice.

    6. Why can't we write str directly?

    You might wonder why we don't write:

    struct Person {
        name: str,
    }

    instead of:

    struct Person<'a> {
        name: &'a str,
    }

    The reason is that str itself has no known fixed size.

    For example, these are different sizes:

    "EFE"
    "Hello"
    "Hello, my name is EFE"

    A bare str could represent text of different lengths.

    Rust therefore normally uses a reference:

    &str

    The reference gives us a way to refer to the string slice.

    7. The exact meaning of the struct

    So when you see:

    struct Person<'a> {
        name: &'a str,
    }

    read it as:

    Define a Person struct. It has a field called name. That field is a reference to a string slice, and 'a is the lifetime 
    parameter describing that reference.

    The syntax:

    &'a str

    is simply:

    &       → reference
    'a      → lifetime parameter
    str     → string slice type

    And that's all you need to understand about str in this struct before moving deeper into lifetimes.
     */



    /************************************************************************************************************
    Yes. The first one is wrong, and the second one is also wrong, but for different reasons.

    1. First example

    You wrote:

    struct Person<'a> {
        name: &'a str,
    }

    fn main() {
        let string_var = String::from("EFE");
        let person = Person {
            name: string_var,
        };

        println!("{}", person.name);
    }

    The problem is here:

    name: string_var,

    Your struct says:

    name: &'a str

    So name must receive an &str.

    But:

    string_var

    is a String, not an &str.

    You would need a reference/slice:

    let person = Person {
        name: &string_var,
    };

    So this is valid:

    struct Person<'a> {
        name: &'a str,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: &string_var,
        };

        println!("{}", person.name);
    }

    The important difference is:

    name: string_var

    vs.

    name: &string_var

    The first gives the struct a String.

    The second gives the struct an &str.

    2. Your second example

    You wrote:

    struct Person<'a> {
        name: &'a str,
    }

    struct Person<'a> {
        name: &'a String,
    }

    This is wrong because you are defining Person twice.

    Rust doesn't allow:

    struct Person { ... }

    struct Person { ... }

    with the same name.

    You'll get a duplicate-definition error.

    If you want to compare the two types, give them different names:

    struct PersonStr<'a> {
        name: &'a str,
    }

    struct PersonString<'a> {
        name: &'a String,
    }

    Then:

    fn main() {
        let person1 = PersonStr {
            name: "EFE",
        };

        let string_var = String::from("EFE");

        let person2 = PersonString {
            name: &string_var,
        };

        println!("{}", person1.name);
        println!("{}", person2.name);
    }

    Both are valid.

    The difference is simply:

    PersonStr
        name: &str
            ↑
        string slice

    PersonString
        name: &String
            ↑
        reference to a String

    And notice this:

    name: "EFE"

    works for &str because "EFE" is an &str.

    But for:

    name: &'a String

    you need an actual String and a reference to it:

    let string_var = String::from("EFE");

    let person = PersonString {
        name: &string_var,
    };

    So the three cases are:

    name: &'a str

    → expects an &str

    name: &'a String

    → expects an &String

    name: String

    → expects a String

    That's the key distinction.
     */



    /************************************************************************************************************
     Yes — this one is wrong:

    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let person = Person {
            name: "EFE",
        };

        println!("{}", person.name);
    }

    The problem is this line:

    name: "EFE",

    Your struct requires:

    name: &'a String

    That means:

    name must be a reference to a String.

    But:

    "EFE"

    is an &str, not an &String.

    So the types don't match:

    Struct requires:    &'a String
                        ↑
                        reference to a String

    You provided:       "EFE"
                        ↑
                        &str
    To make it work

    You need an actual String:

    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: &string_var,
        };

        println!("{}", person.name);
    }

    Now the types match:

    string_var
        │
        │ String
        ▼
    "EFE"
        ▲
        │
        │ &String
        │
    person.name

    Compare the two:

    struct Person<'a> {
        name: &'a str,
    }

    can accept:

    name: "EFE"

    because "EFE" is an &str.

    But:

    struct Person<'a> {
        name: &'a String,
    }

    cannot accept:

    name: "EFE"

    because "EFE" is not an &String.
      */



    /************************************************************************************************************
    Yes, the code is correct. The confusion is between String and &String.

    You are not passing an &str in:

    name: &string_var

    You are passing an &String.

    Let's look at it carefully.

    1. string_var is a String
    let string_var = String::from("EFE");

    So:

    string_var
        ↓
    String

    Its type is:

    String
    2. What does &string_var mean?

    The & means "take a reference to this value."

    Therefore:

    &string_var

    has type:

    &String

    So:

    string_var       → String
    &string_var      → &String
    3. Now compare it with the struct

    Your struct says:

    struct Person<'a> {
        name: &'a String,
    }

    The type of name is:

    &'a String
    │    │
    │    └── the referenced value is a String
    │
    └────── reference

    And you're giving it:

    name: &string_var

    whose type is:

    &String

    So they match:

    Expected:   &'a String
    Given:      &String
                ────────
                match

    The 'a doesn't change the fact that the type is &String. It tells Rust about the lifetime of that reference.

    What about &str?

    This is a completely different type:

    &String

    versus:

    &str

    They are not the same type.

    For example:

    let string_var = String::from("EFE");

    let a = &string_var;

    Here:

    a → &String

    But:

    let b = "EFE";

    Here:

    b → &str

    So your original code:

    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: &string_var,
        };

        println!("{}", person.name);
    }

    is correct because:

    string_var      : String
    &string_var     : &String
    person.name     : &'a String

    The important correction is: &string_var is &String, not &str.
     */



    /************************************************************************************************************
     Yes, this one is correct except for the unnecessary <>.

    You should write:

    struct Person {
        name: String,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: string_var,
        };

        println!("{}", person.name);
    }
    Why?

    Your struct says:

    name: String

    So it expects an actual String.

    And:

    let string_var = String::from("EFE");

    creates a String.

    Therefore:

    name: string_var,

    matches perfectly:

    string_var        → String
                        ↓
    Person.name       → String

    There is no & here because you are giving the struct the String itself, not a reference to it.

    Compare the two cases:

    // Struct contains a String
    struct Person {
        name: String,
    }

    Then:

    name: string_var

    is correct.

    // Struct contains a reference to a String
    struct Person<'a> {
        name: &'a String,
    }

    Then:

    name: &string_var

    is correct.

    So the difference is:

    String
    ↓
    actual String

    &String
    ↓
    reference to a String

    And <>:

    struct Person<>

    is simply unnecessary here. Since the struct has no generic or lifetime parameters, write:

    struct Person
      */


    /************************************************************************************************************
    This one is wrong.

    struct Person {
        name: String,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: &string_var,
        };

        println!("{}", person.name);
    }

    The problem is here:

    name: &string_var,

    Your struct says:

    name: String

    So Rust expects:

    String

    But &string_var is:

    &String

    Therefore:

    Expected:   String
    Given:      &String

    They are different types.

    Correct version if the struct owns the String

    Remove &:

    struct Person {
        name: String,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: string_var,
        };

        println!("{}", person.name);
    }
    Correct version if the struct should contain a reference

    Then the struct must say &String and needs a lifetime:

    struct Person<'a> {
        name: &'a String,
    }

    fn main() {
        let string_var = String::from("EFE");

        let person = Person {
            name: &string_var,
        };

        println!("{}", person.name);
    }

    So the simple rule is:

    struct says String
            ↓
    give it String

    struct says &'a String
            ↓
    give it &String

    And specifically:

    name: string_var

    → String

    while:

    name: &string_var

    → &String
     */
    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");

    let user2 = create_user("Abolfazl".to_string());
    println!("The user2 name is: {}", user2.name);
    println!("The user2 age is: {}", user2.age);
    println!("The user2 is active is: {}", user2.is_active);
    println!("The user2 score is: {}", user2.score);

    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");


    let first_coord = Coordinates(25.8,26.8);

    println!("The latitude is: {}", first_coord.0);
    println!("The longitude is: {}", first_coord.1);


}


fn create_user (username:String) -> User{
    User {
        name:username,
        age: 20,
        is_active: false,
        score:19.5

    }

}




