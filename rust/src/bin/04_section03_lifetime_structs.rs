
//1st rule
//The compiler assigns a lifetime value to each parameter that is a reference ('&')



fn foo1<'a>(x: &'a i32) -> &'a i32{
    x
}


fn foo2<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32{
    x
}
fn foo3<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32{
    y
}


//This will make error because we have told the returned value should have lifetime 'a but we have returned variable with lifetime
//equal to 'a
/* 
fn foo4<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32{
    x
}
*/

//This is incorrect because Rust can not understand we have only one lifetime 'a and the returned value should use that lifetime
//We should tell the return lifetime manually of we should remove all the lifetimes 'a and Rust will undetstand that there is 
//just one lifetime and it will automatically assign the lifetime in the fucntion
/* 
fn foo5<'a>(x: &'a i32) -> & i32{
    println!("Hello");
    x
}
*/

fn foo6(x: & i32) -> & i32{
    println!("Hello");
    x
}








fn main(){




    /*
    Yes. This is referring to the first lifetime-elision rule in Rust. Let's start from zero and then connect it to your 
    examples.

    There are a couple of syntax errors in your second example, so I'll correct those while explaining them.

    1. What does "lifetime assigned to each reference parameter" mean?

    Consider:

    fn foo<'a>(x: &'a i32) {
    }

    There are three important parts:

    fn foo<'a>(x: &'a i32)
        ^^^

    'a is a lifetime parameter.

    Then:

    x: &'a i32

    means:

    x is a reference to an i32, and that reference has lifetime 'a.

    So we have:

    'a

    declared here:

    fn foo<'a>

    and used here:

    x: &'a i32
    2. But why does Rust say we don't always have to write 'a?

    Because Rust has lifetime elision rules.

    "Elision" basically means:

    Rust can omit some lifetime annotations because it can determine them automatically.

    For example, you can write:

    fn foo(x: &i32) {
    }

    instead of:

    fn foo<'a>(x: &'a i32) {
    }

    These are equivalent for this purpose.

    So when Rust sees:

    fn foo(x: &i32)

    it internally reasons about it as if you had written:

    fn foo<'a>(x: &'a i32)

    You don't have to write 'a yourself because the compiler can determine the necessary lifetime relationship.

    3. The first lifetime-elision rule

    The rule you quoted is:

    Each reference parameter gets its own lifetime parameter.

    Let's use:

    fn foo(x: &i32, y: &i32) {
    }

    There are two reference parameters:

    x: &i32
    y: &i32

    According to the first rule, Rust gives each reference its own lifetime.

    Conceptually, this:

    fn foo(x: &i32, y: &i32) {
    }

    is treated like:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {
    }

    Notice something very important:

    x: &'a i32
    y: &'b i32

    They have different lifetime parameters.

    That is because x and y are two independent references.

    4. Why do they get different lifetimes?

    Imagine:

    fn foo(x: &i32, y: &i32) {
        println!("{}", x);
        println!("{}", y);
    }

    Someone could call it with:

    let number1 = 10;
    let number2 = 20;

    foo(&number1, &number2);

    The two references are:

    &number1
    &number2

    There is no reason Rust should assume they have the same lifetime.

    number1 and number2 are two different values with potentially different lifetimes.

    So Rust conceptually gives us:

    x: &'a i32
    y: &'b i32

    where 'a and 'b represent potentially different lifetimes.

    5. Your first example

    You wrote:

    fn foo<'a>(x:&'a i32);

    This is basically explicitly writing what Rust could infer from:

    fn foo(x: &i32);

    The semicolon is important here.

    fn foo<'a>(x: &'a i32);

    is a function declaration without a body.

    For example, you might see this in a trait:

    trait Example {
        fn foo<'a>(x: &'a i32);
    }

    If you're just writing a normal function definition, you'd need a body:

    fn foo<'a>(x: &'a i32) {
        println!("{}", x);
    }
    6. Your second example

    You wrote:

    fn foo<'a, 'b>(x:&'a i32 , &'b y);

    There are two syntax problems.

    You probably mean:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {
    }

    Now let's examine it.

    First:

    <'a, 'b>

    declares two lifetime parameters:

    'a
    'b

    Then:

    x: &'a i32

    means:

    x is a reference to an i32 with lifetime 'a.

    And:

    y: &'b i32

    means:

    y is a reference to an i32 with lifetime 'b.

    So:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {
    }

    is the explicit version of:

    fn foo(x: &i32, y: &i32) {
    }

    because of the first lifetime-elision rule.

    7. Let's see this with actual variables

    Consider:

    fn foo(x: &i32, y: &i32) {
        println!("x = {}", x);
        println!("y = {}", y);
    }

    fn main() {
        let number1 = 10;
        let number2 = 20;

        foo(&number1, &number2);
    }

    The compiler can conceptually treat the function as:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x = {}", x);
        println!("y = {}", y);
    }

    So when we call:

    foo(&number1, &number2);

    we have:

    x: &'a i32

    borrowing number1, and:

    y: &'b i32

    borrowing number2.

    The important point is that:

    'a

    and:

    'b

    are not necessarily the same lifetime.

    8. This is the key idea of Rule 1

    Suppose we write:

    fn foo(x: &i32, y: &i32) {
    }

    Rust doesn't think:

    "There are two references, so I'll give them the same lifetime."

    Instead, the first lifetime-elision rule says conceptually:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {
    }

    Each reference parameter gets its own lifetime.

    So:

    x: &i32

    becomes:

    x: &'a i32

    and:

    y: &i32

    becomes:

    y: &'b i32
    9. But there is an important reason this rule matters

    Consider a function that returns a reference:

    fn get_x(x: &i32) -> &i32 {
        x
    }

    We didn't write any lifetime.

    Rust applies the rules.

    There is one reference parameter:

    x: &i32

    So Rule 1 gives it a lifetime:

    x: &'a i32

    Then another lifetime-elision rule says that when there is exactly one input reference lifetime, the output reference 
    gets that same lifetime.

    So conceptually:

    fn get_x<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    This makes sense:

    The returned reference comes from x, so it cannot be valid longer than the reference passed into the function.

    This is where lifetime elision becomes really useful.

    10. Compare one parameter and two parameters

    With one reference parameter:

    fn get_number(number: &i32) -> &i32 {
        number
    }

    Rust can infer:

    fn get_number<'a>(number: &'a i32) -> &'a i32 {
        number
    }

    With two reference parameters:

    fn get_number(x: &i32, y: &i32) -> &i32 {
        // Which one should the returned reference have?
    }

    Now there are two possible input lifetimes:

    'a
    'b

    Conceptually:

    fn get_number<'a, 'b>(x: &'a i32, y: &'b i32) -> &i32

    At this point, Rust cannot simply assume whether the returned reference should have 'a or 'b.

    That's why the lifetime rules become important.

    For example, if we return x:

    fn get_x(x: &i32, y: &i32) -> &i32 {
        x
    }

    this does not get a simple automatic lifetime solution from the elision rules.

    You would explicitly write:

    fn get_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
        x
    }

    Now we have clearly told Rust:

    The returned reference comes from x, so its lifetime is 'a.

    11. The three lifetime-elision rules

    Since you're currently studying the first rule, it helps to know where it fits.

    Rust's lifetime elision rules are essentially:

    Rule 1

    Each reference parameter gets its own lifetime.

    fn foo(x: &i32, y: &i32)

    conceptually becomes:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
    Rule 2

    If there is exactly one input lifetime, that lifetime is assigned to all output references.

    For example:

    fn foo(x: &i32) -> &i32

    becomes:

    fn foo<'a>(x: &'a i32) -> &'a i32
    Rule 3

    For methods, if one of the parameters is &self or &mut self, the lifetime of self is assigned to output references.

    For example:

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn get_part(&self) -> &str {
            self.part
        }
    }

    Rust can understand the returned &str as having the lifetime associated with self.

    The main thing to remember about Rule 1

    When you see:

    fn foo(x: &i32, y: &i32)

    don't think Rust gives both references one common lifetime.

    Think:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

    Each reference parameter gets its own lifetime.

    And this:

    <'a, 'b>

    doesn't create lifetimes or make the values live longer. It simply gives names to the lifetime relationships that Rust is 
    tracking.

    So your two examples are essentially:

    fn foo<'a>(x: &'a i32);

    and:

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32);

    The second one demonstrates the first elision rule more clearly because there are two reference parameters, therefore two 
    potentially different lifetimes.
    */





}