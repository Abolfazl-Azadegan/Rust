

struct Square {
    width: u32,
    height: u32
}

//In below line we want to define a method for Square struct so we should specify the name of the Square struct after impl
impl Square{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn what_is_width(&self) -> u32 {
        self.width
    }

    fn change_height (&mut self, new_height:u32) -> u32{
        self.height = new_height;
        return self.height
    }

}


fn main(){

    /************************************************************************************************************
        1. We already know how to create a struct

    For example:

    struct Person {
        name: String,
        age: u32,
    }

    And we can create a Person:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        println!("{}", person.name);
        println!("{}", person.age);
    }

    So far, Person only contains data:

    Person
    ┌──────────────────┐
    │ name = "EFE"     │
    │ age  = 34        │
    └──────────────────┘

    But often we want the Person to do something.

    For example:

    print its information
    change its age
    return its name
    determine whether it is an adult
    etc.

    This is where methods come in.

    2. What is a method?

    A method is a function that is associated with a particular type.

    For example:

    struct Person {
        name: String,
        age: u32,
    }

    We could create a method called say_hello:

    impl Person {
        fn say_hello(&self) {
            println!("Hello, my name is {}", self.name);
        }
    }

    Then we can do:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        person.say_hello();
    }

    Output:

    Hello, my name is EFE

    Notice this:

    person.say_hello();

    It looks different from calling an ordinary function.

    An ordinary function might be:

    say_hello(&person);

    A method lets us write:

    person.say_hello();
    3. What is impl?

    Before understanding self, you need to understand impl.

    This:

    impl Person {
        // methods go here
    }

    means:

    "Here I am defining functionality associated with the Person type."

    For example:

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn say_hello(&self) {
            println!("Hello!");
        }
    }

    Think of it like:

    struct Person
        │
        │ defines
        ▼
    DATA

    impl Person
        │
        │ defines
        ▼
    BEHAVIOR / METHODS

    The struct says what a Person has.

    The impl says what a Person can do.

    4. Now the important part: self

    Look at:

    impl Person {
        fn say_hello(&self) {
            println!("Hello, my name is {}", self.name);
        }
    }

    What is:

    &self

    ?

    self represents the particular Person on which the method was called.

    Suppose:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
    };

    and then:

    person.say_hello();

    Inside say_hello, self refers to:

    person

    So:

    self.name

    means:

    the name field of this particular person

    And:

    self.age

    means:

    the age field of this particular person
    5. A concrete example
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn print_info(&self) {
            println!("Name: {}", self.name);
            println!("Age: {}", self.age);
        }
    }

    Then:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        person.print_info();
    }

    When Rust executes:

    person.print_info();

    you can mentally understand it approximately as:

    person
    │
    │ calls
    ▼
    print_info(&person)

    Inside the method:

    self

    refers to person.

    Therefore:

    self.name

    means:

    person.name

    and:

    self.age

    means:

    person.age
    6. Why do we write &self instead of just self?

    This is very important.

    There are three common forms:

    &self
    &mut self
    self

    They mean three different things.

    Let's start with the easiest:

    &self

    means:

    "I want to access the struct without taking ownership of it."

    For example:

    impl Person {
        fn print_info(&self) {
            println!("{}", self.name);
        }
    }

    Then:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        person.print_info();

        println!("{}", person.name);
    }

    This works.

    Why?

    Because the method only borrows person.

    Conceptually:

    person
    │
    │ borrowed
    ▼
    &self

    After the method finishes, person is still available.

    7. What happens if we use self instead?

    Now:

    impl Person {
        fn print_info(self) {
            println!("{}", self.name);
        }
    }

    Notice:

    self

    instead of:

    &self

    This means the method takes ownership of the Person.

    Example:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        person.print_info();

        println!("{}", person.name);
    }

    The last line is now a problem.

    Why?

    Because:

    person.print_info();

    moves person into the method.

    Conceptually:

    Before method:

    person
    │
    ▼
    Person

    Then:

    person.print_info();

    ownership moves:

    person ─────────► self
                    │
                    ▼
                    Person

    The original person can no longer be used after that.

    8. Why would we ever use self?

    Sometimes the method needs to consume the object.

    For example:

    struct Person {
        name: String,
    }

    impl Person {
        fn take_name(self) -> String {
            self.name
        }
    }

    Now:

    fn main() {
        let person = Person {
            name: String::from("EFE"),
        };

        let name = person.take_name();

        println!("{}", name);
    }

    The method takes ownership of person:

    take_name(self)

    and extracts its name.

    After that, person no longer exists as something you can use.

    This can be useful when an object is being transformed into something else.

    9. &mut self

    Now suppose we want a method to modify the struct.

    For example:

    struct Person {
        name: String,
        age: u32,
    }

    We want:

    Person is 34
        ↓
    increase age
        ↓
    Person is 35

    We can write:

    impl Person {
        fn birthday(&mut self) {
            self.age += 1;
        }
    }

    The important part:

    &mut self

    means:

    "Borrow this particular struct mutably, so I can modify it."

    Then:

    fn main() {
        let mut person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        person.birthday();

        println!("{}", person.age);
    }

    Output:

    35
    10. Why does person need mut?

    Look at:

    let mut person = Person {

    The mut is necessary because the method is going to modify the object.

    Without it:

    let person = Person {
        name: String::from("EFE"),
        age: 34,
    };

    person.birthday();

    Rust won't allow the mutable borrow.

    Think of it as:

    let person
        ↓
    cannot modify person

    let mut person
        ↓
    can modify person
    11. What exactly does self.age += 1 mean?

    Inside:

    impl Person {
        fn birthday(&mut self) {
            self.age += 1;
        }
    }

    self represents the current Person.

    So:

    self.age

    means:

    the age field of this Person

    And:

    self.age += 1;

    means:

    take this Person's age
    add 1
    store it back

    If:

    person.age = 34

    then:

    self.age += 1

    changes it to:

    person.age = 35
    12. One Person, different methods

    Now let's put everything together:

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn print_info(&self) {
            println!("Name: {}", self.name);
            println!("Age: {}", self.age);
        }

        fn birthday(&mut self) {
            self.age += 1;
        }

        fn take_name(self) -> String {
            self.name
        }
    }

    And:

    fn main() {
        let mut person = Person {
            name: String::from("EFE"),
            age: 34,
        };

        person.print_info();

        person.birthday();

        person.print_info();

        let name = person.take_name();

        println!("Name was: {}", name);
    }

    Let's understand the three methods.

    First:
    person.print_info();

    Method:

    fn print_info(&self)

    It only needs to look at the person.

    &self
    ↓
    borrow
    ↓
    read
    Second:
    person.birthday();

    Method:

    fn birthday(&mut self)

    It needs to change the person.

    &mut self
    ↓
    mutable borrow
    ↓
    modify
    Third:
    person.take_name();

    Method:

    fn take_name(self)

    It takes ownership of the person.

    self
    ↓
    take ownership
    ↓
    consume Person
    13. The easiest way to remember the three

    When you see:

    fn something(&self)

    think:

    "I only need to look at this object."

    When you see:

    fn something(&mut self)

    think:

    "I need to modify this object."

    When you see:

    fn something(self)

    think:

    "I am taking ownership of this object."

    So:

    &self
    ↓
    read/access

    &mut self
    ↓
    modify

    self
    ↓
    take ownership
    14. Why is it called self?

    You could think of self as meaning:

    "this particular object."

    For example:

    let person1 = Person {
        name: String::from("EFE"),
        age: 34,
    };

    let person2 = Person {
        name: String::from("John"),
        age: 25,
    };

    Both are Persons.

    When you do:

    person1.print_info();

    inside the method:

    self

    refers to person1.

    When you do:

    person2.print_info();

    inside the same method:

    self

    refers to person2.

    So the same method can operate on different instances.

    person1.print_info()
        │
        ▼
        self = person1


    person2.print_info()
        │
        ▼
        self = person2

    That's the main idea behind self.

    15. One more important syntax detail

    You'll sometimes see:

    self.name

    and sometimes:

    Self

    They are different.

    self

    Lowercase:

    self

    means:

    the current instance/object.

    For example:

    self.name

    means the current object's name.

    Self

    Uppercase:

    Self

    means:

    the type we're currently implementing.

    For example:

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Self {
                name,
                age,
            }
        }
    }

    Here Self means Person.

    So:

    self
    ↓
    this particular Person

    Self
    ↓
    the Person type itself

    We'll need Self when we discuss associated functions and constructors, which is the next important part of impl.

    16. Methods vs normal functions

    Finally, this distinction is worth seeing.

    A normal function:

    fn print_person(person: &Person) {
        println!("{}", person.name);
    }

    Call:

    print_person(&person);

    A method:

    impl Person {
        fn print_person(&self) {
            println!("{}", self.name);
        }
    }

    Call:

    person.print_person();

    The method syntax:

    person.print_person();

    automatically gives the method the object as self.

    Conceptually:

    person.print_person()
            ↓
    print_person(&person)
            ↓
    inside function:
    self = person

    That's why self exists.

    The core picture

    You can now think about a struct like this:

    struct Person
        │
        ├── DATA
        │    ├── name
        │    └── age
        │
        └── impl Person
            │
            ├── print_info(&self)
            │       ↓
            │     read
            │
            ├── birthday(&mut self)
            │       ↓
            │     modify
            │
            └── take_name(self)
                    ↓
                    consume

    The most important thing for now is not to memorize complicated syntax. Understand that self is the parameter representing 
    the particular instance that called the method, and the form you choose determines what the method is allowed to do with 
    that instance:

    &self       // borrow it and read/access it
    &mut self   // mutably borrow it and modify it
    self        // take ownership of it
     */



    /************************************************************************************************************
    The three forms are:

    &self
    &mut self
    self

    The important thing is that self refers to the specific object that called the method.

    I’ll use Person as the struct name and employee as the variable name so they are clearly different.

    1. First: what does self mean?

    Start with a simple struct:

    struct Person {
        name: String,
        age: u32,
    }

    This defines a type called Person.

    Now we create an actual value of that type:

    let employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    Now we have:

    Person type
        |
        | creates
        ↓
    employee
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+

    Now suppose we give Person a method:

    impl Person {
        fn introduce(&self) {
            println!("My name is {}", self.name);
        }
    }

    And call it:

    employee.introduce();

    The important question is:

    Inside introduce(), what does self mean?

    It means:

    the particular Person value that called this method.

    Here:

    employee.introduce();

    so inside the method:

    self
    ↓
    employee

    Therefore:

    self.name

    means:

    employee.name

    So this:

    fn introduce(&self) {
        println!("My name is {}", self.name);
    }

    is basically saying:

    "Give me access to the object that called me, and I will read its name."

    2. Why are there three different forms?

    Because sometimes a method needs to:

    look at the object without taking it
    modify the object without taking it
    take the object itself

    That's what these mean:

    &self       → I want to look at/use the object temporarily
    &mut self   → I want to modify the object temporarily
    self        → I want to take ownership of the object

    Let's understand each one separately.

    3. &self — "Let me use your object, but I won't take it"

    Consider this:

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn introduce(&self) {
            println!("My name is {}", self.name);
            println!("I am {} years old", self.age);
        }
    }

    fn main() {
        let employee = Person {
            name: String::from("EFE"),
            age: 34,
        };

        employee.introduce();

        println!("{}", employee.name);
    }
    What is the purpose of this code?

    We want a method called introduce() that can read information from a Person.

    It doesn't need to change anything.

    It also shouldn't take ownership of the Person.

    That's why we use:

    &self
    What happens when this runs?

    First:

    let employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    We create:

    employee
    |
    ↓
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+

    Then:

    employee.introduce();

    Rust gives the method a reference to employee.

    Inside the method:

    fn introduce(&self)

    self refers to employee.

    So:

    self.name

    means:

    employee.name

    and:

    self.age

    means:

    employee.age

    The method only reads them.

    After the method finishes, employee is still available:

    println!("{}", employee.name);

    because the method didn't take ownership.

    Think of &self as:

    "Can I temporarily look at your object?"

    The object says:

    "Yes. You can look at it, but it's still mine."

    4. &mut self — "Let me modify your object"

    Now imagine we want a method that changes the person's age.

    For example:

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn have_birthday(&mut self) {
            self.age = self.age + 1;
        }
    }

    fn main() {
        let mut employee = Person {
            name: String::from("EFE"),
            age: 34,
        };

        employee.have_birthday();

        println!("{}", employee.age);
    }

    The output is:

    35
    What is the purpose of this code?

    We have a method:

    have_birthday()

    Its job is to change the age field.

    Before:

    employee
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+

    After:

    employee.have_birthday();

    we want:

    employee
    +----------------+
    | name = "EFE"   |
    | age  = 35      |
    +----------------+

    Because the method needs to modify the object, we use:

    &mut self
    What does this line mean?
    fn have_birthday(&mut self)

    Break it apart:

    &

    means we're using a reference rather than taking ownership.

    mut

    means that reference is allowed to modify the object.

    self

    means the object that called the method.

    So:

    &mut self

    means:

    "Give me temporary mutable access to the object that called this method."

    Why do we need mut here?

    Look at:

    let mut employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    We wrote:

    mut employee

    because we are going to modify the employee value.

    Then:

    employee.have_birthday();

    calls:

    fn have_birthday(&mut self)

    Inside:

    self.age = self.age + 1;

    Here self refers to employee.

    So conceptually:

    self.age = self.age + 1;

    means:

    employee.age = employee.age + 1;

    Therefore:

    34 → 35
    5. Difference between &self and &mut self

    This is the most important distinction:

    &self

    The method can access the object, but normally cannot directly modify its ordinary fields.

    impl Person {
        fn show_age(&self) {
            println!("{}", self.age);
        }
    }

    Purpose:

    Read information.

    &mut self

    The method gets mutable access and can modify the object.

    impl Person {
        fn increase_age(&mut self) {
            self.age += 1;
        }

    Purpose:

    Change information.

    So:

    &self
    ↓
    "I want to READ your object."

    &mut self
    ↓
    "I want to MODIFY your object."

    But in both cases, the method does not take ownership of the object.

    6. Now the confusing one: self

    Now let's look at:

    self

    without &.

    This is fundamentally different.

    Consider:

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn consume(self) {
            println!("Goodbye {}", self.name);
        }
    }

    fn main() {
        let employee = Person {
            name: String::from("EFE"),
            age: 34,
        };

        employee.consume();
    }
    What is the purpose of this code?

    The method is called:

    consume()

    Its purpose is to take ownership of the entire Person value.

    That's why we wrote:

    fn consume(self)

    not:

    fn consume(&self)
    7. What happens here?

    We have:

    let employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    employee owns the Person.

    Then:

    employee.consume();

    The ownership of the Person is moved into the method.

    Conceptually:

    Before calling method:

    employee
    |
    ↓
    Person
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+


    After calling:

    employee
    |
    X
    |
    ↓
    consume()
    |
    ↓
    owns Person

    Inside the method:

    fn consume(self)

    self now owns that Person value.

    Therefore the method can use it:

    println!("{}", self.name);

    And when the method finishes, that Person is dropped.

    8. Why would we ever want to take ownership?

    Sometimes a method represents an operation where the object is finished and shouldn't be used afterward.

    For example:

    struct BankAccount {
        owner: String,
        balance: u32,
    }

    impl BankAccount {
        fn close(self) {
            println!("Closing account belonging to {}", self.owner);
        }
    }

    The purpose of close() is to say:

    "This account is being closed. After this operation, we don't want this object anymore."

    So:

    let account = BankAccount {
        owner: String::from("EFE"),
        balance: 5000,
    };

    account.close();

    The method takes ownership.

    After:

    account.close();

    you cannot use:

    account

    again.

    Why?

    Because the method received ownership.

    9. Compare all three with one example

    Let's put all three methods into the same struct.

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {

        // 1. Read
        fn show_information(&self) {
            println!("Name: {}", self.name);
            println!("Age: {}", self.age);
        }

        // 2. Modify
        fn have_birthday(&mut self) {
            self.age += 1;
        }

        // 3. Take ownership
        fn consume(self) {
            println!("Goodbye {}", self.name);
        }
    }

    Now:

    fn main() {
        let mut employee = Person {
            name: String::from("EFE"),
            age: 34,
        };

        employee.show_information();

        employee.have_birthday();

        employee.show_information();

        employee.consume();
    }

    Let's follow it.

    Step 1
    let mut employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    We have:

    employee
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+
    Step 2
    employee.show_information();

    The method is:

    fn show_information(&self)

    So it receives a reference.

    employee
    ↑
    |
    &self

    It reads the data.

    Nothing is changed.

    After the method:

    employee
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+

    employee is still completely usable.

    Step 3
    employee.have_birthday();

    The method is:

    fn have_birthday(&mut self)

    So it receives mutable access.

    It executes:

    self.age += 1;

    which means conceptually:

    employee.age += 1;

    So:

    34 → 35

    Now:

    employee
    +----------------+
    | name = "EFE"   |
    | age  = 35      |
    +----------------+

    employee is still usable.

    Step 4
    employee.show_information();

    Again we use:

    &self

    It reads the updated value:

    Name: EFE
    Age: 35
    Step 5
    employee.consume();

    This time:

    fn consume(self)

    There is no &.

    The method takes ownership of employee.

    So after this call, employee cannot be used anymore.

    10. The easiest way to remember it

    Don't initially think of these as complicated Rust syntax.

    Think about what the method wants to do with the object.

    &self
    fn show(&self)

    Means:

    "Let me use your object without taking it."

    Usually used for reading.

    &mut self
    fn change(&mut self)

    Means:

    "Let me temporarily modify your object without taking ownership."

    Used when the method changes the object.

    self
    fn consume(self)

    Means:

    "Give me ownership of your object."

    Used when the method consumes/takes ownership of the object.

    11. One very important detail: self is NOT the same as Self

    You'll see both later.

    They are completely different.

    self

    lowercase:

    the current instance/value

    For example:

    employee.show_information();

    Inside the method:

    self

    refers to that employee.

    Self uppercase:

    the type being implemented

    For example:

    impl Person {
        fn create() -> Self {
            Self {
                name: String::from("EFE"),
                age: 34,
            }
        }
    }

    Here:

    Self

    means:

    Person

    So remember:

    self
    ↓
    current object/value

    Self
    ↓
    current type
 */


    /************************************************************************************************************
    Yes — the confusion is coming from mixing up the name self with the ownership of the value. Your question is exactly the right one.

    The key correction is:

    When self is used as a method parameter, self does not mean "give ownership back to employee."

    self is a new parameter inside the method, and ownership moves from employee to that parameter.

    Let's go very slowly.

    1. Start with the object

    We have:

    struct Person {
        name: String,
        age: u32,
    }

    And:

    let employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    At this point:

    employee
    |
    | OWNS
    ↓
    Person
    +----------------+
    | name = "EFE"   |
    | age  = 34      |
    +----------------+

    So you are completely correct:

    employee is the owner of the Person value.

    2. Now we define a method using self

    Suppose we write:

    impl Person {
        fn consume(self) {
            println!("{}", self.name);
        }
    }

    The important part is:

    fn consume(self)

    Here self is a parameter.

    You can think of it similarly to an ordinary function parameter.

    For example, imagine:

    fn take_value(value: Person) {
        println!("{}", value.name);
    }

    Here value is a parameter that receives ownership of a Person.

    The method version:

    fn consume(self)

    does essentially the same kind of thing, except Rust gives the parameter the special name:

    self

    because it represents the object on which the method was called.

    3. Now call the method

    We have:

    let employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    employee.consume();

    What happens?

    Before the call:

    employee
    |
    | owns
    ↓
    Person

    When we call:

    employee.consume();

    because the method requires:

    self

    ownership of the Person moves into the method parameter self.

    So conceptually:

    BEFORE METHOD CALL

    employee
    |
    | owns
    ↓
    Person

    Then:

    employee.consume()
            |
            | ownership moves
            ↓
        self
            |
            | owns
            ↓
        Person

    Now the important part:

    employee
    X
    |
    | no longer owns Person
    4. But why does self refer to employee then?

    This is the subtle part.

    When I previously said:

    "self refers to employee"

    I meant:

    The value received by self is the value that was previously stored in employee.

    I did not mean:

    "self is another name for the variable employee."

    Those are different ideas.

    Let's use a simpler ordinary function first.

    Ordinary function

    Imagine:

    fn take_value(value: Person) {
        println!("{}", value.name);
    }

    And:

    let employee = Person {
        name: String::from("EFE"),
        age: 34,
    };

    take_value(employee);

    Before calling:

    employee
    |
    ↓
    Person

    When we call:

    take_value(employee);

    ownership moves:

    employee                 value
    |                       ↑
    | owns                  |
    +-----------------------+
                |
                | MOVE
                ↓
            value
                |
                ↓
            Person

    After the move:

    value
    |
    ↓
    Person

    and:

    employee
        X

    You already know this from ownership.

    5. self is basically the method's special parameter

    Now compare:

    Ordinary function
    fn take_value(value: Person) {
        println!("{}", value.name);
    }

    Call:

    take_value(employee);

    The parameter is:

    value
    Method
    impl Person {
        fn consume(self) {
            println!("{}", self.name);
        }
    }

    Call:

    employee.consume();

    The parameter is:

    self

    So conceptually you can think:

    ordinary function:

    employee
    |
    | move
    ↓
    value


    method:

    employee
    |
    | move
    ↓
    self

    The difference is that Rust's method syntax lets you write:

    employee.consume();

    instead of manually passing employee as an argument.

    6. This explains your exact question

    You said:

    "self refers to the employee which is the one who owns the Person."

    Almost, but this is where the confusion occurs.

    self refers to the Person value, not to the variable name employee.

    These are different:

    employee

    is the variable/binding.

    Person { name: "EFE", age: 34 }

    is the value.

    Initially:

    employee ───────────→ Person value
    owner

    After calling:

    employee.consume();

    the value moves:

    employee                    self
        X                        |
                                ↓
                        Person value

    So self now owns the value that employee used to own.

    7. What does "self refers to employee" really mean?

    When we say:

    employee.consume();

    and inside the method:

    fn consume(self) {
        println!("{}", self.name);
    }

    we can say:

    self represents the employee object that called the method.

    But this is a convenient conceptual description, not saying that self and employee are two names for the same variable.

    More precisely:

    employee
    |
    | before call
    ↓
    Person value

        MOVE

    Person value
    ↑
    |
    self

    The value is the same value.

    The owner/binding changes.

    8. Think about your sentence "giving ownership back"

    You asked:

    "when we use self we are giving the ownership back to employee"

    No.

    It's actually the opposite.

    When you write:

    fn consume(self)

    you are saying:

    "When someone calls this method, move the object into this method."

    So:

    employee
        |
        | owns
        ↓
    Person

    becomes:

    employee        self
    X             |
                    | owns
                    ↓
                    Person

    Ownership moves from employee → self.

    It does not move:

    self → employee
    9. What happens when the method finishes?

    This is another important piece.

    Consider:

    impl Person {
        fn consume(self) {
            println!("{}", self.name);
        }
    }

    When we call:

    employee.consume();

    ownership moves into self.

    Then the method finishes.

    At the end of the method, self goes out of scope.

    Because self owns the Person, the Person is dropped.

    Conceptually:

    employee
    X

    self
    |
    ↓
    Person
    |
    |
    ↓
    method finishes
    |
    ↓
    self goes out of scope
    |
    ↓
    Person is dropped

    That's why employee cannot be used afterward.

    For example:

    fn main() {
        let employee = Person {
            name: String::from("EFE"),
            age: 34,
        };

        employee.consume();

        println!("{}", employee.name); // ERROR
    }

    The problem isn't that Rust forgot that employee existed.

    The problem is:

    employee gave its ownership away when calling consume().

    10. Compare this with &self

    This makes the difference much clearer.

    With:

    fn show(&self)

    the situation is:

    employee
    |
    | OWNS
    ↓
    Person
    ↑
    |
    &self

    self is accessing the object through a reference.

    Ownership remains with:

    employee

    After the method:

    employee
    |
    | still owns
    ↓
    Person

    But with:

    fn consume(self)

    the situation is:

    BEFORE:

    employee
    |
    | OWNS
    ↓
    Person


    METHOD CALL:

    employee
    X

    self
    |
    | OWNS
    ↓
    Person

    So the crucial difference is:

    &self
    ─────
    employee keeps ownership
    self only borrows access


    self
    ────
    employee gives ownership away
    self becomes the owner
    11. The best mental model

    Don't think:

    "self is another name for employee."

    Instead think:

    self is the method's parameter representing the object on which the method was called.

    And depending on how that parameter is written:

    &self

    means:

    self receives a reference to the object
    &mut self

    means:

    self receives a mutable reference to the object
    self

    means:

    self receives ownership of the object

    So:

    employee.consume()
        │
        │
        │ ownership moves
        ↓
        self

    That's the key idea.

    employee is the original owner. self is the new owner inside the method when you use self without &.
     */




    let sq = Square {
        width: 10,
        height: 20
    };

    
    println!("The width of sq is: {}", sq.width);
    println!("The height of sq is: {}", sq.height);
    println!("The area of the sq is: {}", sq.area());
    println!("The function returned width is: {}", sq.what_is_width());

    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");


    let mut sq2 = Square {
        width: 100,
        height: 200
    };


    println!("The width of sq2 is: {}", sq2.width);
    println!("The height of sq2 is: {}", sq2.height);
    println!("The area of the sq2 is: {}", sq2.area());
    println!("The function returned width is: {}", sq2.what_is_width());
    println!("The changed height is: {}", sq2.change_height(300));
    println!("The area of the sq2 after change is: {}", sq2.area());
    println!("-------------------------------------------------------------");
    println!("-------------------------------------------------------------");

   
    /************************************************************************************************************
    Yes. Let's go through this from the beginning, and especially connect impl, methods, self, &self, and &mut self to what 
    actually happens when your code runs.

    One small issue first: the code you posted needs to be inside a function such as main(). I'll explain the code exactly 
    as you wrote it conceptually, but a complete runnable version would have fn main() { ... } around the let and println! 
    statements.

    1. First, what are we trying to build?

    Your code creates a type called Square.

    The purpose is to represent a rectangle/square-like object that has:

    a width
    a height
    a way to calculate its area
    a way to get its width
    a way to change its height

    So we start with:

    struct Square {
        width: u32,
        height: u32
    }
    2. What does struct Square mean?
    struct Square {
        width: u32,
        height: u32
    }

    This defines a new type called Square.

    It says:

    Every Square value will contain a width and a height, and both will be u32.

    So we can later create an actual Square value:

    let sq = Square {
        width: 10,
        height: 20
    };

    Now we have an actual value:

    sq
    |
    ↓
    Square
    +----------------+
    | width  = 10    |
    | height = 20    |
    +----------------+

    It's important to distinguish:

    Square
    ↑
    the TYPE / blueprint

    sq
    ↑
    an actual VALUE of that type
    3. What is impl Square?

    Now we have:

    impl Square {

    impl means:

    I am going to define functionality associated with this type.

    So:

    impl Square {

    means:

    "Here are some methods/functions that belong to Square."

    Then you have:

    impl Square {

        fn area(&self) -> u32 {
            ...
        }

        fn what_is_width(&self) -> u32 {
            ...
        }

        fn change_height(&mut self, new_height: u32) -> u32 {
            ...
        }

    }

    You can think of it as:

    Square
    │
    ├── area()
    │
    ├── what_is_width()
    │
    └── change_height()

    This is why you write:

    impl Square

    and not:

    impl sq

    Because Square is the type whose functionality you're defining.

    sq doesn't exist yet when the impl block is defined.

    4. First method: area

    Your first method is:

    fn area(&self) -> u32 {
        self.width * self.height
    }

    Let's break this apart.

    fn area
    fn area

    This defines a function named:

    area

    But because it is inside:

    impl Square

    it is a method of Square.

    That's why later you can write:

    sq.area()

    instead of:

    area(sq)
    5. What does (&self) mean here?

    You have:

    fn area(&self) -> u32

    The:

    &self

    means:

    "This method wants to access the particular Square that called it, but it doesn't want to take ownership of it."

    Suppose:

    let sq = Square {
        width: 10,
        height: 20
    };

    Then:

    sq.area()

    means that self inside area() refers to the sq value.

    Conceptually:

    sq
    |
    ↓
    Square
    +----------------+
    | width  = 10    |
    | height = 20    |
    +----------------+
    ↑
    |
    &self

    So inside:

    self.width

    self refers to sq.

    Therefore:

    self.width

    is accessing:

    sq.width

    And:

    self.height

    is accessing:

    sq.height
    6. What does the body of area() do?

    You wrote:

    fn area(&self) -> u32 {
        self.width * self.height
    }

    The last expression is:

    self.width * self.height

    Rust automatically returns the value of the final expression because there is no semicolon.

    So if:

    self.width  = 10
    self.height = 20

    then:

    10 × 20 = 200

    Therefore:

    sq.area()

    returns:

    200
    7. What does -> u32 mean?

    This part:

    -> u32

    means:

    This method returns a u32.

    So:

    fn area(&self) -> u32

    means:

    "The area method receives access to a Square through &self and returns a u32."

    8. Why doesn't area() change sq?

    Because it uses:

    &self

    not:

    &mut self

    The purpose of area() is only to read:

    self.width
    self.height

    It doesn't modify anything.

    So:

    sq.area();

    doesn't destroy or change sq.

    After:

    sq.area();

    you can still do:

    println!("{}", sq.width);

    because sq still owns the Square.

    9. Second method: what_is_width

    Now:

    fn what_is_width(&self) -> u32 {
        self.width
    }

    This method's purpose is very simple:

    Give me the width of this particular Square.

    Again:

    &self

    means the method gets access to the object without taking ownership.

    And:

    self.width

    means:

    Get the width field from the current Square.

    So when you do:

    sq.what_is_width()

    Rust uses sq as the object represented by self.

    Therefore:

    self.width

    is effectively accessing:

    sq.width

    which is:

    10

    So:

    println!(
        "The function returned width is: {}",
        sq.what_is_width()
    );

    prints:

    The function returned width is: 10
    10. Why do we need what_is_width() if we already have sq.width?

    Excellent question to ask when learning.

    You don't actually need the method in this example.

    You can directly do:

    println!("{}", sq.width);

    because the fields are accessible.

    So:

    sq.width

    and:

    sq.what_is_width()

    both give you 10.

    The method is probably included in this learning example specifically to demonstrate how methods access struct fields 
    through self.

    In real programs, methods become much more useful when they perform some meaningful operation or enforce some rule.

    11. Now let's look at sq

    You create:

    let sq = Square {
        width: 10,
        height: 20
    };

    Let's follow the next four lines.

    Line 1
    println!("The width of sq is: {}", sq.width);

    This directly accesses the field.

    sq.width

    means:

    Go to the Square value owned by sq and get its width.

    Result:

    10

    Output:

    The width of sq is: 10
    Line 2
    println!("The height of sq is: {}", sq.height);

    Same idea:

    sq.height

    gives:

    20

    Output:

    The height of sq is: 20
    Line 3
    println!("The area of the sq is: {}", sq.area());

    Now we're using the method.

    sq.area()

    The method receives &self.

    Inside the method:

    self.width * self.height

    becomes conceptually:

    10 × 20

    so the method returns:

    200

    Output:

    The area of the sq is: 200
    Line 4
    println!("The function returned width is: {}", sq.what_is_width());

    The method:

    fn what_is_width(&self) -> u32 {
        self.width
    }

    gets the width.

    So:

    sq.what_is_width()
            ↓
        10

    Output:

    The function returned width is: 10
    12. Now we reach the important part: sq2

    You then write:

    let mut sq2 = Square {
        width: 100,
        height: 200
    };

    Now we create another, completely separate Square.

    sq
    |
    ↓
    Square
    +----------------+
    | width  = 10    |
    | height = 20    |
    +----------------+


    sq2
    |
    ↓
    Square
    +----------------+
    | width  = 100   |
    | height = 200   |
    +----------------+

    These are two different values.

    The methods defined in:

    impl Square

    can be used by both of them.

    That's one of the main benefits of methods.

    13. Why does the same area() method work for both?

    You have only written:

    fn area(&self) -> u32 {
        self.width * self.height
    }

    once.

    But you can call:

    sq.area()

    and:

    sq2.area()

    Why?

    Because self changes depending on which object called the method.

    When:

    sq.area()

    is called:

    self → sq

    So:

    self.width

    means:

    sq.width = 10

    and:

    self.height

    means:

    sq.height = 20

    Therefore:

    10 × 20 = 200

    When:

    sq2.area()

    is called:

    self → sq2

    So:

    self.width

    means:

    sq2.width = 100

    and:

    self.height

    means:

    sq2.height = 200

    Therefore:

    100 × 200 = 20,000

    So self is what allows the same method to work with different objects.

    14. Now the most important method: change_height

    You wrote:

    fn change_height(&mut self, new_height: u32) -> u32 {
        self.height = new_height;
        return self.height
    }

    This method has a different purpose.

    Its purpose is:

    Take the Square that called this method, change its height to the value supplied by the caller, and return the new height.

    Let's look at the pieces.

    15. &mut self

    You wrote:

    &mut self

    This means:

    "I want temporary mutable access to the object that called this method."

    Why do we need mutable access?

    Because this line changes the object:

    self.height = new_height;

    We're changing:

    height

    from one value to another.

    For example:

    BEFORE:

    sq2
    +----------------+
    | width  = 100   |
    | height = 200   |
    +----------------+

    AFTER:

    sq2
    +----------------+
    | width  = 100   |
    | height = 300   |
    +----------------+

    That's why we need:

    &mut self

    instead of:

    &self
    16. Why is sq2 declared with mut?

    Look at:

    let mut sq2 = Square {
        width: 100,
        height: 200
    };

    The mut is necessary because we're going to modify the value.

    Without it:

    let sq2 = Square {
        width: 100,
        height: 200
    };

    then trying to call a method requiring:

    &mut self

    would cause an error.

    You can think of it as:

    let mut sq2
        ↑
    "sq2 is allowed to be modified"
    17. What is new_height?

    The method has:

    fn change_height(&mut self, new_height: u32) -> u32

    There are actually two parameters here:

    &mut self
        ↑
    special method receiver


    new_height: u32
        ↑
    ordinary parameter

    When you call:

    sq2.change_height(300)

    Rust gives:

    self        → sq2
    new_height  → 300

    So inside the method:

    self.height = new_height;

    means:

    sq2.height = 300;
    18. Follow change_height(300) step by step

    Before the call:

    sq2
    +----------------+
    | width  = 100   |
    | height = 200   |
    +----------------+

    You call:

    sq2.change_height(300)

    The method receives:

    self       → mutable access to sq2
    new_height → 300

    Then it executes:

    self.height = new_height;

    which means:

    sq2.height = 300

    Now:

    sq2
    +----------------+
    | width  = 100   |
    | height = 300   |
    +----------------+
    19. What does return self.height do?

    After changing the height, your method has:

    return self.height

    At this moment:

    self.height = 300

    So:

    return self.height

    returns:

    300

    Therefore:

    sq2.change_height(300)

    returns:

    300

    This is why:

    println!("The changed height is: {}", sq2.change_height(300));

    prints:

    The changed height is: 300

    But there is something important happening here:

    the method does two things at once.

    It:

    changes sq2.height
    returns the new height
    20. After change_height, sq2 has really changed

    This is very important.

    When you write:

    println!("The changed height is: {}", sq2.change_height(300));

    the 300 printed isn't just some temporary result.

    The actual sq2 has changed.

    Before:

    sq2
    +----------------+
    | width  = 100   |
    | height = 200   |
    +----------------+

    After:

    sq2.change_height(300);

    the actual object is:

    sq2
    +----------------+
    | width  = 100   |
    | height = 300   |
    +----------------+

    So when you subsequently execute:

    sq2.area()

    it uses:

    width  = 100
    height = 300

    Therefore:

    100 × 300 = 30,000
    21. So your last line
    println!("The area of the sq2 after change is: {}", sq2.area());

    calls:

    sq2.area()

    The area method is:

    fn area(&self) -> u32 {
        self.width * self.height
    }

    This time:

    self → sq2

    and sq2 now contains:

    width  = 100
    height = 300

    Therefore:

    self.width * self.height
        ↓           ↓
        100 ×       300
                ↓
            30,000

    Output:

    The area of the sq2 after change is: 30000
    22. One complete execution of sq2

    Let's put the entire process together.

    You create:

    let mut sq2 = Square {
        width: 100,
        height: 200
    };

    State:

    sq2
    +----------------+
    | width  = 100   |
    | height = 200   |
    +----------------+

    Then:

    sq2.width

    gives:

    100

    Then:

    sq2.height

    gives:

    200

    Then:

    sq2.area()

    Inside area():

    self → sq2

    self.width  → 100
    self.height → 200

    100 × 200 = 20,000

    Then:

    sq2.what_is_width()

    Inside:

    self → sq2

    self.width → 100

    returns:

    100

    Then:

    sq2.change_height(300)

    Inside:

    self       → sq2
    new_height → 300

    Execute:

    self.height = new_height;

    Therefore:

    sq2.height = 300

    Then:

    return self.height

    returns:

    300

    Now sq2 is:

    sq2
    +----------------+
    | width  = 100   |
    | height = 300   |
    +----------------+

    Finally:

    sq2.area()

    does:

    100 × 300 = 30,000
    23. Now look at the three self forms in YOUR code

    Your code gives a very good example of the three forms we've been discussing.

    First:
    fn area(&self) -> u32

    Purpose:

    I only need to read the Square to calculate its area.

    Therefore:

    &self
    ↓
    borrow/read
    Second:
    fn what_is_width(&self) -> u32

    Purpose:

    I only need to read the width.

    Therefore:

    &self
    ↓
    borrow/read
    Third:
    fn change_height(&mut self, new_height: u32) -> u32

    Purpose:

    I need to modify the Square's height.

    Therefore:

    &mut self
    ↓
    borrow + modify

    Notice that your code doesn't have self by itself.

    It has:

    &self

    and:

    &mut self

    So your example demonstrates the first two forms, but not the ownership-taking form:

    self

    That third form would look like:

    impl Square {
        fn destroy(self) {
            println!("The square had width {}", self.width);
        }
    }

    Then:

    sq2.destroy();

    would move ownership of the Square into self.

    That's fundamentally different from your area() and change_height() methods.

    24. The whole impl block in one picture

    Your code can now be understood like this:

                        Square TYPE
                            │
                ┌─────────┴─────────┐
                │                   │
            struct definition      impl Square
                │                   │
                │          ┌────────┼─────────┐
                │          │        │         │
                │        area()   what...   change...
                │          │        │         │
                │        &self    &self    &mut self
                │          │        │         │
                │          ↓        ↓         ↓
                │        READ     READ      MODIFY
                │
                ↓
            actual values
            │         │
            ↓         ↓
            sq        sq2
            │         │
            │         │
        width=10   width=100
        height=20  height=200

    And the most important thing to understand about self from this example is:

    sq.area()
        ↓
    self = sq


    sq2.area()
        ↓
    self = sq2


    sq2.change_height(300)
        ↓
    self = sq2
    new_height = 300

    So self is what makes a method operate on the particular object that called it.

    And the & or &mut determines how the method gets access to that object:

    &self
    ↓
    access without taking ownership

    &mut self
    ↓
    mutable access without taking ownership

    self
    ↓
    take ownership

    That is the central idea behind the code you posted.
    
     */





}