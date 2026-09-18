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















}