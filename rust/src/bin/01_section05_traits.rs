
pub trait Summary {
    fn summarization(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle{
    fn summarization(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarization(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


fn main(){


    let article1 = NewArticle{
        headline: String::from("Article 1"),
        location: String::from("Location 1"),
        author: String::from("Author 1"),
        content: String::from("Content 1"),
    };

    println!("{}", article1.summarization());

    let tweet1 = Tweet{
        username: String::from("User 1"),
        content: String::from("Content 1"),
        reply: true,
        retweet: false,
    };


    println!("{}", tweet1.summarization());

    /*
    
    Absolutely. Since you just learned generics and monomorphization, traits are the next important Rust concept. 
    I’ll build them from zero and connect them to what you already know.

    The most important sentence to understand first is:

    A trait defines behavior that a type promises to provide.

    It does not define a new concrete type like a struct does.

    1. First, why do we need traits?

    Suppose you have two structs:

    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    They are completely different types.

    Now imagine you want both of them to have a function called speak().

    You could write:

    impl Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    impl Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }

    Now:

    let my_dog = Dog {
        name: String::from("Max"),
    };

    let my_cat = Cat {
        name: String::from("Luna"),
    };

    my_dog.speak();
    my_cat.speak();

    This works.

    But now imagine you want to write one function that can accept anything that can speak.

    You don't want to say:

    fn make_dog_speak(dog: &Dog)

    because then it only accepts Dog.

    And you don't want:

    fn make_cat_speak(cat: &Cat)

    because then it only accepts Cat.

    You want to express this idea:

    "I don't care whether you are a Dog or a Cat. I only care that you provide a speak() behavior."

    This is where a trait becomes useful.

    2. What is a trait?

    Let's create one:

    trait Speak {
        fn speak(&self);
    }

    Let's read this very slowly.

    trait

    This keyword tells Rust:

    "I am defining a trait."

    Speak

    This is the name of our trait.

    We could call it:

    trait AnimalBehavior

    or:

    trait Printable

    or:

    trait NetworkDevice

    The name is up to us.

    { ... }

    The body contains the behavior that types implementing this trait must provide.

    Inside we have:

    fn speak(&self);

    Notice something unusual.

    There is no function body.

    Normally you write:

    fn speak(&self) {
        println!("Woof!");
    }

    But here we only write:

    fn speak(&self);

    The semicolon means:

    "The trait requires a method called speak, but the trait itself is not providing the implementation here."

    The individual types will provide their own implementations.

    3. Implementing the trait

    Now we tell Dog:

    "Dog implements the Speak trait."

    We do this:

    impl Speak for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    Let's understand the syntax:

    impl Speak for Dog

    means:

    "I am implementing the Speak trait for the Dog type."

    Then we must provide the method required by the trait:

    fn speak(&self) {
        println!("Woof!");
    }

    Now do the same for Cat:

    impl Speak for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }

    So we have:

    struct Dog {
        name: String,
    }

    struct Cat {
        name: String,
    }

    trait Speak {
        fn speak(&self);
    }

    impl Speak for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    impl Speak for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }
    4. What did the trait actually accomplish?

    The trait did not create a new object.

    It did not create a Dog.

    It did not create a Cat.

    Instead, it established a common behavior.

    We can now say:

    Dog implements Speak
    Cat implements Speak

    Therefore both types satisfy the requirement:

    "I have a speak() method with the required signature."

    This is the central purpose of traits.

    5. Using the trait

    Now we can call:

    fn main() {
        let my_dog = Dog {
            name: String::from("Max"),
        };

        let my_cat = Cat {
            name: String::from("Luna"),
        };

        my_dog.speak();
        my_cat.speak();
    }

    Output:

    Woof!
    Meow!

    You might now say:

    "But we could already do this with normal impl blocks. Why do we need a trait?"

    Excellent question.

    The important part comes next.

    6. Traits become really useful with generics

    Suppose we want a function that accepts anything that implements Speak.

    We can write:

    fn make_speak<T: Speak>(animal: &T) {
        animal.speak();
    }

    This combines two things you've already learned:

    generics
    traits

    Let's understand it carefully.

    T

    T is a generic type parameter.

    We learned this earlier:

    fn something<T>(value: T)

    means:

    "The exact type will be determined later."

    But now we have:

    T: Speak

    The : means that T has a trait bound.

    So:

    T: Speak

    means:

    "T can be any type, but it must implement the Speak trait."

    Therefore:

    fn make_speak<T: Speak>(animal: &T)

    means:

    "Give me a reference to some type T, as long as that type implements Speak."

    7. Now we can pass both types
    fn main() {
        let my_dog = Dog {
            name: String::from("Max"),
        };

        let my_cat = Cat {
            name: String::from("Luna"),
        };

        make_speak(&my_dog);
        make_speak(&my_cat);
    }

    For the first call:

    make_speak(&my_dog);

    Rust determines:

    T = Dog

    Then Rust checks:

    Does Dog implement Speak?

    Yes.

    So the call is valid.

    For:

    make_speak(&my_cat);

    Rust determines:

    T = Cat

    Then:

    Does Cat implement Speak?

    Yes.

    So that is valid too.

    8. What if we create another type?

    Suppose:

    struct Robot {
        id: u32,
    }

    We haven't implemented Speak for Robot.

    Now:

    let robot = Robot { id: 10 };

    make_speak(&robot);

    Rust rejects it.

    Why?

    Because our function says:

    fn make_speak<T: Speak>(animal: &T)

    which means:

    "T must implement Speak."

    But Robot doesn't.

    So the trait gives us a way to tell Rust exactly what capabilities a type must have.

    9. This is the big difference between a generic and a trait

    Consider:

    fn process<T>(value: T)

    This says:

    "Give me any type."

    But:

    fn process<T: Speak>(value: T)

    says:

    "Give me any type that implements Speak."

    So a trait gives meaning to the generic parameter.

    Without a trait:

    T

    is basically:

    "Some type."

    With a trait bound:

    T: Speak

    it becomes:

    "Some type that provides the behavior required by Speak."

    10. A more practical example

    Let's move away from animals.

    Suppose you're writing a network program.

    You might have:

    struct Router {
        ip_address: String,
    }

    struct Server {
        ip_address: String,
    }

    Both can have a method that returns their IP address.

    We can define:

    trait NetworkDevice {
        fn ip_address(&self) -> &str;
    }

    Then:

    impl NetworkDevice for Router {
        fn ip_address(&self) -> &str {
            &self.ip_address
        }
    }

    and:

    impl NetworkDevice for Server {
        fn ip_address(&self) -> &str {
            &self.ip_address
        }
    }

    Now we can write:

    fn show_ip<T: NetworkDevice>(device: &T) {
        println!("IP address: {}", device.ip_address());
    }

    And use it:

    let router = Router {
        ip_address: String::from("192.168.1.1"),
    };

    let server = Server {
        ip_address: String::from("192.168.1.100"),
    };

    show_ip(&router);
    show_ip(&server);

    The important thing is that show_ip() doesn't care whether the object is a Router or a Server.

    It only cares:

    "Does this type implement NetworkDevice?"

    That is a very powerful idea.

    11. A trait can require multiple methods

    A trait isn't limited to one method.

    For example:

    trait NetworkDevice {
        fn ip_address(&self) -> &str;
        fn restart(&mut self);
    }

    Now any type implementing NetworkDevice must provide both methods.

    For example:

    impl NetworkDevice for Router {
        fn ip_address(&self) -> &str {
            &self.ip_address
        }

        fn restart(&mut self) {
            println!("Router is restarting");
        }
    }

    If you forget restart():

    impl NetworkDevice for Router {
        fn ip_address(&self) -> &str {
            &self.ip_address
        }
    }

    Rust gives you a compiler error because Router has not fulfilled the complete NetworkDevice contract.

    12. Traits can also contain default implementations

    This is another important feature.

    You can write:

    trait NetworkDevice {
        fn ip_address(&self) -> &str;

        fn show_status(&self) {
            println!("Device IP: {}", self.ip_address());
        }
    }

    Here:

    fn ip_address(&self) -> &str;

    has no implementation.

    The type must implement it.

    But:

    fn show_status(&self) {
        println!("Device IP: {}", self.ip_address());
    }

    already has an implementation.

    Therefore a type implementing NetworkDevice doesn't have to implement show_status() unless it wants to provide 
    different behavior.

    For example:

    impl NetworkDevice for Router {
        fn ip_address(&self) -> &str {
            &self.ip_address
        }
    }

    Then:

    router.show_status();

    works even though we never wrote show_status() inside impl NetworkDevice for Router.

    The trait supplied the default implementation.

    13. Traits are not inheritance

    This is important because if you know languages like Java or C++, you might initially think:

    "Is a trait like a parent class?"

    Not exactly.

    Rust does not use traditional class inheritance.

    A trait is primarily about shared behavior/interface requirements.

    For example:

    trait Speak {
        fn speak(&self);
    }

    You can implement it for:

    Dog
    Cat
    Robot
    Human
    Server
    NetworkDevice

    They don't have to belong to the same class hierarchy.

    The trait simply says:

    "These types provide this behavior."

    14. Traits and your largest function

    This connects directly to what you were learning before.

    You had:

    fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    You might have wondered:

    "Why do we write T: Copy + PartialOrd?"

    Because Copy and PartialOrd are traits.

    Copy tells Rust:

    "This type has the Copy behavior."

    PartialOrd tells Rust:

    "This type supports partial ordering comparisons such as <, >, <=, >=."

    So:

    T: Copy + PartialOrd

    means:

    "T must implement both the Copy trait and the PartialOrd trait."

    That's why the function is allowed to do things like:

    let mut largest = list[0];

    and:

    if item > largest

    The compiler knows that T has the required behaviors.

    15. + between traits

    This:

    T: Copy + PartialOrd

    doesn't mean addition.

    The + means:

    "T must satisfy both trait requirements."

    So:

    T: Copy + PartialOrd

    is equivalent conceptually to:

    T implements Copy AND T implements PartialOrd.

    You could also write:

    fn largest<T>(list: &[T]) -> T
    where
        T: Copy + PartialOrd,
    {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    This is just another way to write the same trait bounds.

    You don't need to learn the where version yet; just recognize it when you see it.

    16. Traits can be implemented by types you create

    You can create your own trait:

    trait Describe {
        fn describe(&self);
    }

    Then implement it for your own structs:

    struct Server {
        hostname: String,
    }

    struct Router {
        hostname: String,
    }

    Then:

    impl Describe for Server {
        fn describe(&self) {
            println!("This is a server: {}", self.hostname);
        }
    }

    and:

    impl Describe for Router {
        fn describe(&self) {
            println!("This is a router: {}", self.hostname);
        }
    }

    Now both types have the same trait-defined capability, but they can implement it differently.

    That's one of the most important reasons traits are useful.

    17. The really important distinction: type vs behavior

    This is probably the best way to organize the concepts you've learned.

    A struct defines what data a type contains:

    struct Server {
        hostname: String,
        port: u16,
    }

    It answers:

    "What data does a Server have?"

    An enum defines which possible variants a value can have:

    enum ConnectionStatus {
        Connected,
        Disconnected,
    }

    It answers:

    "What possible forms can this value have?"

    A trait defines what behavior a type provides:

    trait Restartable {
        fn restart(&mut self);
    }

    It answers:

    "What can this type do?"

    So:

    struct → data
    enum   → alternatives
    trait  → behavior

    That's a very useful mental model.

    18. Traits become even more powerful with generic functions

    Suppose:

    trait Speak {
        fn speak(&self);
    }

    and:

    impl Speak for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    impl Speak for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }

    Then:

    fn make_speak<T: Speak>(animal: &T) {
        animal.speak();
    }

    The generic function doesn't need to know the exact type.

    It only requires:

    T implements Speak

    This is a major pattern in Rust:

    fn some_function<T: SomeTrait>(value: &T)

    means:

    "I don't care what the exact type is. I only care that it provides the behavior described by SomeTrait."

    19. And now monomorphization makes sense

    This connects directly to your previous question.

    Suppose:

    fn make_speak<T: Speak>(animal: &T) {
        animal.speak();
    }

    and you call:

    make_speak(&my_dog);
    make_speak(&my_cat);

    Rust knows:

    first call:  T = Dog
    second call: T = Cat

    Because this is a generic function, Rust can monomorphize it.

    Conceptually, you can think of it as generating:

    fn make_speak_for_dog(animal: &Dog) {
        animal.speak();
    }

    and:

    fn make_speak_for_cat(animal: &Cat) {
        animal.speak();
    }

    Again, you don't write these functions.

    Rust's compiler handles the specialization.

    So now you can see the relationship:

    Trait

    trait Speak {
        fn speak(&self);
    }

    defines the required behavior.

    Generic

    fn make_speak<T: Speak>(animal: &T)

    says:

    "Accept any type having that behavior."

    Monomorphization

    happens during compilation and specializes the generic function for the concrete types being used.

    20. One more very important syntax

    You will frequently see:

    impl Speak for Dog

    and:

    impl Dog

    They look similar, but they mean different things.

    impl Dog
    impl Dog {
        fn new() -> Self {
            // ...
        }

        fn bark(&self) {
            // ...
        }
    }

    This means:

    "Define methods associated with Dog."

    impl Speak for Dog
    impl Speak for Dog {
        fn speak(&self) {
            // ...
        }
    }

    This means:

    "Make Dog implement the Speak trait."

    So:

    impl Dog

    is about methods belonging to Dog.

    Whereas:

    impl Speak for Dog

    is about fulfilling the behavior defined by Speak.

    21. How should YOU use traits?

    When you're writing Rust, don't start by thinking:

    "I need a trait."

    Instead, ask yourself:

    "Do I have different types that should provide the same kind of behavior?"

    For example, imagine you have:

    struct HuaweiRouter {
        ip: String,
    }

    struct CiscoRouter {
        ip: String,
    }

    Both can:

    show IP
    restart
    send packet

    You might define:

    trait NetworkDevice {
        fn restart(&mut self);
        fn ip_address(&self) -> &str;
    }

    Then both types implement it.

    Now functions can work with any network device, without needing to know whether they're dealing with Huawei or Cisco.

    That's where traits become extremely useful in real software.

    22. The three concepts you should now connect

    You have now learned three things that are closely related:

    Generics
    fn process<T>(value: T)

    means:

    "The exact type is not fixed here."

    Trait bounds
    fn process<T: NetworkDevice>(value: T)

    means:

    "The exact type isn't fixed, but it must provide the behavior required by NetworkDevice."

    Monomorphization

    Rust's compiler can take:

    fn process<T: NetworkDevice>(value: T)

    and, when your program uses it with concrete types, generate specialized code during compilation.

    The simplest definition to remember

    If someone asks you:

    "What is a trait in Rust?"

    A good answer is:

    A trait defines a set of behaviors that a type can implement. It allows us to write code that works with different 
    types based on what they can do, rather than based only on their concrete type.

    And if someone asks:

    "What is T: SomeTrait?"

    It means:

    T can be any type, but that type must implement SomeTrait.

    And:

    "What is monomorphization?"

    It is the compiler's process of specializing generic code for the concrete types used by the program.

    The next concept that naturally follows from traits is dyn Trait and dynamic dispatch. That is where you'll see why 
    Rust has both T: Trait and &dyn Trait, and the difference between static dispatch and dynamic dispatch becomes much 
    clearer.

     */

    /*
    
    Yes — your main idea is correct, but there is one very important detail to fix.

    You are describing one of the main purposes of traits:

    Different types can have completely different fields and other methods, but they can implement the same trait method, 
    with the same method name and same required signature, while having different implementations (different bodies).

    Let's use exactly your addition vs multiplication example.

    1. First, define the trait

    Suppose we create a trait called Calculate:

    trait Calculate {
        fn calculate(&self, a: i32, b: i32) -> i32;
    }

    This does not say how calculate() should work.

    It only says:

    Any type that implements Calculate must provide a method named calculate that:

    receives &self
    receives an i32 called a
    receives an i32 called b
    returns an i32

    So the trait is defining a required interface/behavior.

    Notice this:

    fn calculate(&self, a: i32, b: i32) -> i32;

    There is no { } body.

    2. Now create two completely different types

    For example:

    struct Adder {
        name: String,
    }

    struct Multiplier {
        name: String,
    }

    These are two different types.

    They could have completely different fields:

    struct Adder {
        name: String,
        maximum_value: i32,
    }

    and:

    struct Multiplier {
        id: u32,
        description: String,
    }

    That doesn't matter.

    The trait only cares about whether they provide the required behavior.

    3. Implement the trait for Adder
    impl Calculate for Adder {
        fn calculate(&self, a: i32, b: i32) -> i32 {
            a + b
        }
    }

    Here we are saying:

    Adder implements Calculate.

    And inside Adder's implementation, calculate() means:

    a + b

    For example:

    let adder = Adder {
        name: String::from("My adder"),
    };

    let result = adder.calculate(10, 5);

    println!("{}", result);

    The result is:

    15
    4. Now implement the SAME trait for Multiplier
    impl Calculate for Multiplier {
        fn calculate(&self, a: i32, b: i32) -> i32 {
            a * b
        }
    }

    This is the important part.

    The method has the same name:

    calculate

    and the same signature:

    fn calculate(&self, a: i32, b: i32) -> i32

    But its body is completely different:

    For Adder:

    a + b

    For Multiplier:

    a * b

    So:

    let adder = Adder {
        name: String::from("Adder"),
    };

    let multiplier = Multiplier {
        name: String::from("Multiplier"),
    };

    println!("{}", adder.calculate(10, 5));
    println!("{}", multiplier.calculate(10, 5));

    produces:

    15
    50

    So yes, this part of your understanding is exactly right.

    5. But there is an important correction

    You said:

    "the function name is the same and we call this function or in another word method a trait"

    Almost.

    calculate() is not the trait.

    The trait is:

    trait Calculate {
        fn calculate(&self, a: i32, b: i32) -> i32;
    }

    The trait is called:

    Calculate

    The method required by that trait is:

    calculate()

    So you should say:

    "Calculate is the trait, and calculate() is a method defined by that trait."

    Then:

    impl Calculate for Adder

    provides the calculate() implementation for Adder.

    And:

    impl Calculate for Multiplier

    provides another implementation of the same trait method for Multiplier.

    6. Now why is this useful?

    This is where traits become really powerful.

    Suppose I write:

    fn run_calculation<T: Calculate>(calculator: &T) -> i32 {
        calculator.calculate(10, 5)
    }

    Let's understand this:

    <T: Calculate>

    means:

    T can be any type, as long as it implements Calculate.

    So I can do:

    let adder = Adder {
        name: String::from("Adder"),
    };

    let multiplier = Multiplier {
        name: String::from("Multiplier"),
    };

    let result1 = run_calculation(&adder);
    let result2 = run_calculation(&multiplier);

    println!("{}", result1);
    println!("{}", result2);

    The results are:

    15
    50

    The function:

    run_calculation()

    doesn't need to know:

    "Am I receiving an Adder?"

    or:

    "Am I receiving a Multiplier?"

    It only knows:

    "Whatever T is, it implements Calculate, so I know it has a calculate() method with the required signature."

    That's the real power of traits.

    7. But there is one rule you cannot break

    The implementations can have different bodies, but they must satisfy the trait's required method signature.

    Our trait says:

    trait Calculate {
        fn calculate(&self, a: i32, b: i32) -> i32;
    }

    Therefore this is valid:

    impl Calculate for Adder {
        fn calculate(&self, a: i32, b: i32) -> i32 {
            a + b
        }
    }

    And this is valid:

    impl Calculate for Multiplier {
        fn calculate(&self, a: i32, b: i32) -> i32 {
            a * b
        }
    }

    Different body: yes.

    Different method signature: no.

    For example, this would not satisfy the trait:

    impl Calculate for Multiplier {
        fn calculate(&self, a: f64, b: f64) -> f64 {
            a * b
        }
    }

    Why?

    Because the trait requires:

    fn calculate(&self, a: i32, b: i32) -> i32;

    but Multiplier is trying to provide:

    fn calculate(&self, a: f64, b: f64) -> f64;

    Those are different method signatures.

    8. So your understanding can be stated like this

    Your idea:

    "I have many different types. They can have completely different fields and other methods. But they have a common method. 
    I can define that common method in a trait. Each type can implement that method differently, as long as it follows the 
    method signature required by the trait."

    Yes. Exactly.

    For your specific example:

    trait Calculate {
        fn calculate(&self, a: i32, b: i32) -> i32;
    }

    Adder can implement:

    a + b

    while Multiplier can implement:

    a * b

    The method name is the same:

    calculate()

    The signature is the same:

    (&self, i32, i32) -> i32

    But the implementation/body is different.

    And that's one of the fundamental purposes of traits in Rust.
    
     */




     /*
     
     pub is a very important next topic because it introduces visibility in Rust.

    You already understand the trait part of this code. Now let's focus on what pub means and why it appears in 
    several different places.

    1. What does pub mean?

    pub is short for public.

    It controls who is allowed to access something.

    By default, many things in Rust are private.

    So when you write:

    pub struct NewArticle {

    you are saying:

    "This NewArticle type can be accessed from outside the module where it is defined."

    And when you write:

    pub trait Summary {

    you are saying:

    "The Summary trait can be accessed from outside the module where it is defined."

    And:

    pub headline: String,

    means:

    "The headline field can be accessed from outside the module where NewArticle is defined."

    So pub is basically Rust's way of saying:

    "This item is publicly accessible."

    2. Why does Rust need this?

    Imagine your program becomes large.

    You might have:

    main.rs
    network.rs
    database.rs
    user.rs

    Each file/module contains many things.

    You probably don't want everything inside network.rs to automatically be accessible everywhere.

    For example, maybe you have:

    struct NetworkConnection {
        ip_address: String,
        password: String,
    }

    You might want other parts of the program to use NetworkConnection, but you don't want them to directly access the password.

    Rust lets you control this.

    You could write:

    pub struct NetworkConnection {
        pub ip_address: String,
        password: String,
    }

    Now:

    NetworkConnection itself is public.
    ip_address is public.
    password is private.

    So another module can access:

    connection.ip_address

    but cannot directly access:

    connection.password

    This is called visibility.

    3. Let's look at your code

    You have:

    pub trait Summary {
        fn summarization(&self) -> String;
    }

    There are two separate concepts here:

    pub trait Summary

    and:

    fn summarization(&self) -> String;

    pub applies to the trait itself.

    So:

    pub trait Summary

    means:

    The Summary trait is publicly accessible.

    It does not mean that the method automatically becomes public.

    This distinction is important.

    4. What about the method inside the trait?

    You have:

    pub trait Summary {
        fn summarization(&self) -> String;
    }

    Notice that you did not write:

    pub fn summarization

    You wrote:

    fn summarization

    This is intentional.

    A trait method is part of the trait's interface.

    When another type implements the trait:

    impl Summary for NewArticle {
        fn summarization(&self) -> String {
            format!(
                "{}, by {} ({})",
                self.headline,
                self.author,
                self.location
            )
        }
    }

    that method is available as part of the trait implementation according to the trait's visibility.

    So you don't normally write:

    impl Summary for NewArticle {
        pub fn summarization(...)
    }

    In fact, you should not add pub there.

    5. Now look at this
    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    There are actually two different levels of visibility here.

    First:

    pub struct NewArticle

    makes the struct itself public.

    Second:

    pub headline
    pub location
    pub author
    pub content

    makes each field public.

    These are separate decisions.

    6. This is very important: public struct does NOT mean public fields

    You could write:

    pub struct NewArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    Now NewArticle is public, but all of its fields are private.

    That means another module can know that NewArticle exists and can potentially use it through its public API, but it 
    cannot directly do:

    article.headline

    from outside the module where the field is private.

    So:

    pub struct NewArticle

    and:

    pub headline: String

    control different things.

    The first controls the visibility of the type.

    The second controls the visibility of the field.

    7. Let's make this concrete with modules

    This is where pub becomes much easier to understand.

    Imagine we have:

    mod news {
        pub struct NewArticle {
            pub headline: String,
            author: String,
        }
    }

    And then:

    fn main() {
        let article1 = news::NewArticle {
            headline: String::from("Article 1"),
            author: String::from("Author 1"),
        };
    }

    There are two problems here.

    NewArticle is public:

    pub struct NewArticle

    so main is allowed to use the type.

    headline is public:

    pub headline: String

    so main is allowed to provide a value for it.

    But author is private:

    author: String

    so main cannot directly access that field from outside the news module.

    8. Why would we want private fields?

    This is extremely common in real software.

    Imagine:

    pub struct BankAccount {
        pub owner: String,
        balance: f64,
    }

    We might want people to know who owns the account:

    account.owner

    But we don't necessarily want arbitrary code to directly change:

    account.balance

    Instead, we might provide methods:

    impl BankAccount {
        pub fn deposit(&mut self, amount: f64) {
            self.balance += amount;
        }

        pub fn balance(&self) -> f64 {
            self.balance
        }
    }

    Now other code can do:

    account.deposit(100.0);

    but cannot directly do:

    account.balance = -999999.0;

    This is one of the major purposes of visibility.

    You control how other parts of the program interact with your data.

    9. Your Tweet has the same idea

    You wrote:

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    This means:

    The type is public
    pub struct Tweet

    Other modules can use Tweet.

    The fields are public
    pub username
    pub content
    pub reply
    pub retweet

    Other modules can directly access those fields.

    For example:

    tweet1.username

    is allowed because username is public.

    10. What if we remove pub?

    Suppose we change:

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    to:

    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    Now the Tweet type itself is private.

    Even though the fields say pub, another module can't simply use Tweet because the type itself isn't publicly accessible.

    This gives you an important rule:

    The visibility of the outer item matters too.

    11. pub doesn't mean "global"

    This is another common misunderstanding.

    pub does not mean:

    "Everyone everywhere can access this."

    Rust has a module system and visibility rules.

    pub means that the item is public according to Rust's module visibility system.

    For example:

    mod news {
        pub struct NewArticle {
            pub headline: String,
        }
    }

    Then from outside that module:

    news::NewArticle

    can be accessed because NewArticle is public.

    Without pub:

    mod news {
        struct NewArticle {
            pub headline: String,
        }
    }

    the type is private to the news module.

    12. What does pub have to do with your trait?

    Your example has:

    pub trait Summary {
        fn summarization(&self) -> String;
    }

    Why make the trait public?

    Imagine your trait is in a library/module:

    mod articles {
        pub trait Summary {
            fn summarization(&self) -> String;
        }
    }

    If Summary were private, code outside that module couldn't use the trait as part of its public interface.

    By writing:

    pub trait Summary

    you're saying:

    "Summary is part of the public API of this module."

    This becomes particularly important when you're writing libraries/crates that other Rust programs will use.

    13. Your entire example

    Let's annotate your code:

    pub trait Summary {
        fn summarization(&self) -> String;
    }

    This means:

    Create a public trait called Summary.

    The trait requires:

    fn summarization(&self) -> String;

    which means:

    Any type implementing Summary must provide a method named summarization that takes &self and returns a String.

    Then:

    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    means:

    Create a public struct called NewArticle, and make all four fields public.

    Then:

    impl Summary for NewArticle {

    means:

    Make NewArticle implement the Summary trait.

    Then:

    fn summarization(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    provides the actual implementation of the trait method for NewArticle.

    So calling:

    article1.summarization()

    uses the NewArticle implementation.

    Then:

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    creates another public type with public fields.

    And:

    impl Summary for Tweet {
        fn summarization(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    says:

    Tweet also implements Summary, but its implementation of summarization() is different.

    Therefore:

    article1.summarization()

    and:

    tweet1.summarization()

    have the same method name, because the trait requires that name, but they execute different implementations.

    14. The key idea you should remember

    You now have three closely related concepts:

    struct

    Defines a type and its data:

    struct Tweet {
        username: String,
        content: String,
    }
    trait

    Defines behavior that types can implement:

    trait Summary {
        fn summarization(&self) -> String;
    }
    pub

    Controls whether an item is publicly accessible outside its module:

    pub struct Tweet
    pub trait Summary
    pub username: String

    And these are independent decisions.

    For example:

    pub struct Tweet {
        username: String,
    }

    means:

    The Tweet type is public, but its username field is private.

    Whereas:

    struct Tweet {
        pub username: String,
    }

    means:

    The Tweet type itself is private, so the public field doesn't make the type publicly usable from outside its module.
      */


}