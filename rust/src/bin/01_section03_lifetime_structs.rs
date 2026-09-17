fn main(){

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









}