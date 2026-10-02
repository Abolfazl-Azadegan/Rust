

//Traits as parameter in functions
/* 
pub fn notify (item:&impl Summary){
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T:Summary>(item:&T){
    println!("Breaking news! {}",item.summarize())
}


pub fn notify(item1:&impl Summary, item2: &impl Summary)

pub fn notify <T:Summary>(item:&T, item2:&T)
*/


/*

Yes. These examples are using a trait as a requirement for a function parameter. This is one of the most important uses 
of traits in Rust.

Let's start from absolutely zero and build up to your four examples.

1. First, remember what Summary is

Suppose we have:

pub trait Summary {
    fn summarize(&self) -> String;
}

This means:

Any type that implements Summary must provide a summarize() method.

For example:

struct Article {
    title: String,
}

struct Tweet {
    username: String,
}

We can implement the trait for both:

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {}", self.username)
    }
}

Now both Article and Tweet have the summarize() method because both implement Summary.

So we could do:

let article = Article {
    title: String::from("Rust is great"),
};

println!("{}", article.summarize());

and:

let tweet = Tweet {
    username: String::from("EFE"),
};

println!("{}", tweet.summarize());
2. Now the first function

You have:

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

This looks strange because of:

&impl Summary

Let's break that apart.

Normally, you already know how to write a function like:

fn print_number(number: &i32) {
    println!("{}", number);
}

The parameter says:

number: &i32

meaning:

number is a reference to an i32.

Now:

item: &impl Summary

means:

item is a reference to some type that implements Summary.

It does not mean that Summary itself is a value or that impl Summary is a type.

It means:

"I don't care exactly what the type is. I only require that the type implements Summary."

So this function can receive an Article:

notify(&article);

and it can also receive a Tweet:

notify(&tweet);

because both implement Summary.

3. Why can the function call summarize()?

This is the important part.

Look at:

item.summarize()

How does Rust know that item has a summarize() method?

Because you told Rust:

item: &impl Summary

You are essentially telling Rust:

"Whatever type item refers to, that type implements Summary."

And Summary requires:

fn summarize(&self) -> String;

Therefore Rust knows that summarize() exists.

So:

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

can be understood in plain English as:

Create a function called notify. It accepts a reference to any type that implements Summary. Inside the function, I am 
allowed to use the behavior guaranteed by the Summary trait.

4. Now the second version

You also have:

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

This looks much more complicated, but it means essentially the same thing as the first version.

Let's start with:

<T>

You already learned generic types.

For example:

fn print_value<T>(value: T) {
}

Here:

T

is a generic type parameter.

It means:

The function can work with some type, but we don't know the exact type yet.

Now we add:

T: Summary

The : means a trait bound.

So:

T: Summary

means:

Whatever type T is, it must implement Summary.

Therefore:

fn notify<T: Summary>(item: &T)

means:

Create a generic function with a type parameter T. T must implement Summary. The parameter item is a reference to a T.

5. These two are basically equivalent

You have:

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

and:

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

For this particular situation, they express essentially the same requirement.

The first is shorter:

&impl Summary

The second explicitly introduces a generic type parameter:

<T: Summary>

You can think of them as:

&impl Summary

and:

&T where T: Summary

conceptually expressing the same idea.

But don't treat impl Summary as a normal concrete type. It means "some concrete type that implements Summary."

6. Let's actually call the function

Suppose:

struct Article {
    title: String,
}

struct Tweet {
    username: String,
}

And:

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {}", self.username)
    }
}

Then:

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

We can do:

let article = Article {
    title: String::from("Rust"),
};

let tweet = Tweet {
    username: String::from("EFE"),
};

notify(&article);
notify(&tweet);

When we call:

notify(&article);

Rust sees that:

Article: Summary

so it's valid.

When we call:

notify(&tweet);

Rust sees:

Tweet: Summary

so it's also valid.

7. Now your third example

You wrote:

pub fn notify(item1: &impl &Summary, item2: &impl Summary)

There is a syntax error here:

&impl &Summary

is not valid Rust.

You probably meant:

pub fn notify(item1: &impl Summary, item2: &impl Summary)

Notice the difference.

You had:

&impl &Summary

It should be:

&impl Summary

So the corrected function is:

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

Now something very important happens.

This function says:

item1 can be a reference to some type implementing Summary, and item2 can be a reference to some type implementing Summary.

They do not have to be the same type.

For example:

let article = Article {
    title: String::from("Rust"),
};

let tweet = Tweet {
    username: String::from("EFE"),
};

notify(&article, &tweet);

This can work.

Here:

item1

could be an Article.

And:

item2

could be a Tweet.

Both satisfy the requirement:

Summary
8. Now your fourth example is different

You wrote:

pub fn notify<T: Summary>(item: &T, item2: &T)

This is very important.

Here we have one generic type parameter:

T

and both parameters use that same T:

item: &T
item2: &T

Therefore:

item and item2 must be the same concrete type.

For example, if you call:

notify(&article, &another_article);

where both are Article, that's fine.

But:

notify(&article, &tweet);

is not allowed because:

item  = Article
item2 = Tweet

They are different types.

9. This is the key difference

Compare these two:

fn notify(item1: &impl Summary, item2: &impl Summary)

versus:

fn notify<T: Summary>(item1: &T, item2: &T)

The first:

fn notify(item1: &impl Summary, item2: &impl Summary)

allows:

Article + Article
Article + Tweet
Tweet + Article
Tweet + Tweet

as long as both types implement Summary.

The second:

fn notify<T: Summary>(item1: &T, item2: &T)

requires the same type for both:

Article + Article

or:

Tweet + Tweet

but not:

Article + Tweet
10. Why does impl Summary allow different types?

Because you wrote impl separately:

item1: &impl Summary
item2: &impl Summary

There are effectively two independent "some type" requirements.

You didn't give those two types a shared generic name.

But here:

<T: Summary>

you created one specific generic type parameter called T.

Then both parameters say:

&T

So they must refer to the same type T.

11. A concrete example

Let's make this very explicit.

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

struct Tweet {
    username: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {}", self.username)
    }
}

Now version A:

fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

We can call:

let article = Article {
    title: String::from("Rust"),
};

let tweet = Tweet {
    username: String::from("EFE"),
};

notify(&article, &tweet);

This is valid.

Why?

Because:

Article implements Summary
Tweet implements Summary

That's all the function requires.

Now version B:

fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

This is valid:

notify(&article, &article);

because both are Article.

This is valid:

notify(&tweet, &tweet);

because both are Tweet.

But this:

notify(&article, &tweet);

doesn't satisfy the function's requirement because one argument is Article and the other is Tweet.

12. So why would we ever use the generic version?

Because sometimes we want to express a relationship between parameters.

For example:

fn compare<T: PartialOrd>(first: &T, second: &T) -> bool {
    first > second
}

Here we want:

Compare two values of the same type.

So:

compare(&10, &20);

works.

But you wouldn't want:

compare(&10, &some_string);

because comparing an i32 with a String doesn't make sense.

The generic parameter T lets us express:

Both parameters must have the same type, and that type must implement PartialOrd.

13. One more important point: impl here does NOT mean impl block

You have already learned this:

impl Summary for Article {
    fn summarize(&self) -> String {
        ...
    }
}

Here impl means:

I am implementing the Summary trait for Article.

But here:

fn notify(item: &impl Summary)

impl has a different role.

It means:

Accept some concrete type that implements Summary.

So you should distinguish:

impl Summary for Article

from:

&impl Summary

The first implements a trait.

The second uses a trait as a requirement on a function parameter.

The four examples in one place
One parameter
fn notify(item: &impl Summary)

Means:

Accept a reference to any type that implements Summary.

Equivalent in this context to:

fn notify<T: Summary>(item: &T)
Two parameters, potentially different types
fn notify(item1: &impl Summary, item2: &impl Summary)

Means:

Both arguments must implement Summary, but they can be different types.

For example:

notify(&article, &tweet);
Two parameters, same type
fn notify<T: Summary>(item1: &T, item2: &T)

Means:

Both arguments must have the same type, and that type must implement Summary.

For example:

notify(&article, &another_article);

but not:

notify(&article, &tweet);

And your version:

&impl &Summary

is simply a syntax error; it should be:

&impl Summary
*/




/*
pub fn notify (item: &(impl Summary + Display)){
}

pub fn notify <T: Summary + Display>(item: &T){
}

*/


/*

Yes. This is the same idea you just learned, but now the function requires the type to implement two traits instead of one.

Let's build it from the beginning.

1. First, imagine we have two traits

Suppose we have:

trait Summary {
    fn summarize(&self) -> String;
}

and Rust's standard library gives us the Display trait:

use std::fmt::Display;

Display means, roughly:

This type knows how to be formatted for normal user-facing output with {}.

For example:

let number = 100;

println!("{}", number);

works because i32 implements Display.

Similarly:

let text = String::from("Hello");

println!("{}", text);

works because String implements Display.

2. What does Summary + Display mean?

Now look at:

impl Summary + Display

The + here means:

The type must satisfy both trait requirements.

So:

impl Summary + Display

means:

Some type that implements Summary and implements Display.

It does not mean addition.

This:

Summary + Display

is not adding two things together.

It is saying:

Requirement 1: Summary
Requirement 2: Display

Both are required.

3. Now look at your first function

You have:

pub fn notify(item: &(impl Summary + Display)) {

}

Let's break the parameter apart:

item: &(impl Summary + Display)

Start from the outside:

item:

This is the parameter name.

Then:

&

means:

item is a reference.

Then:

impl Summary + Display

means:

The reference points to some type that implements both Summary and Display.

So the whole thing means:

item is a reference to some type that implements both Summary and Display.

4. Why do we need both traits?

Suppose our function is:

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news!");
    println!("Summary: {}", item.summarize());
    println!("Item: {}", item);
}

We are doing two things with item:

item.summarize()

and:

println!("{}", item)

The first requires:

Summary

because summarize() comes from the Summary trait.

The second requires:

Display

because {} uses the Display formatting trait.

Therefore the function says:

impl Summary + Display

because it needs both capabilities.

5. Think about what would happen with only Summary

Suppose we wrote:

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
    println!("{}", item);
}

The first line is fine:

item.summarize()

because we told Rust:

impl Summary

But the second line:

println!("{}", item);

has a problem.

Why?

Because we only promised Rust:

item implements Summary.

We did not promise:

item implements Display.

Therefore Rust cannot assume that {} formatting is available.

So we add:

+ Display

Now Rust knows:

item: some type implementing Summary

and:

item: some type implementing Display
6. Now look at the second version

You have:

pub fn notify<T: Summary + Display>(item: &T) {

}

This is the generic version.

You already learned:

fn notify<T: Summary>(item: &T)

means:

T can be some type, but that type must implement Summary.

Now:

<T: Summary + Display>

simply adds another requirement.

It means:

T must implement Summary AND Display.

So:

pub fn notify<T: Summary + Display>(item: &T)

means:

Create a generic function with a type T. T must implement both Summary and Display. The function receives a reference to 
that T.

7. Compare the two versions

You have:

pub fn notify(item: &(impl Summary + Display)) {
}

and:

pub fn notify<T: Summary + Display>(item: &T) {
}

For one parameter, these express essentially the same requirement.

Both mean:

Give me a reference to some type that implements both Summary and Display.

The first version is shorter:

&(impl Summary + Display)

The second explicitly creates a generic type parameter:

<T: Summary + Display>
8. Let's make a complete example

Let's create our own type:

use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

Now implement Summary:

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

And implement Display:

impl Display for Article {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.title)
    }
}

Now Article satisfies both:

Article implements Summary
Article implements Display

Therefore this works:

fn notify(item: &(impl Summary + Display)) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}

And:

let article = Article {
    title: String::from("Rust is interesting"),
};

notify(&article);

Inside notify, Rust knows that item has both behaviors.

So:

item.summarize()

is allowed because of Summary.

And:

println!("{}", item);

is allowed because of Display.

9. What if a type implements only one?

Suppose we have:

struct Article {
    title: String,
}

and we only implement:

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

but we don't implement Display.

Then:

notify(&article);

will not work for:

fn notify(item: &(impl Summary + Display))

because Article satisfies:

Summary

but does not satisfy:

Display

The function requires both.

10. The + can be used with more than two traits

You could theoretically write:

fn process<T: Summary + Display + Clone>(item: &T) {
    // ...
}

Now T must implement all three:

Summary
Display
Clone

So the + means:

AND

Conceptually:

T: Summary + Display + Clone

means:

T implements Summary AND Display AND Clone.

11. One subtle but important point

Don't think that:

Summary + Display

creates a new trait.

It doesn't.

You still have two separate traits:

Summary
Display

The + simply combines their requirements for this particular generic parameter.

For example:

<T: Summary + Display>

doesn't create a new trait called:

SummaryDisplay

Instead, it says:

T has to satisfy both existing traits.

12. Compare what you've learned so far

You started with:

fn notify(item: &impl Summary)

Meaning:

item is a reference to some type that implements Summary.

Then:

fn notify<T: Summary>(item: &T)

Meaning:

T is a type that implements Summary, and item is a reference to T.

Now:

fn notify(item: &(impl Summary + Display))

Meaning:

item is a reference to some type that implements both Summary and Display.

And:

fn notify<T: Summary + Display>(item: &T)

Meaning:

T is a type that implements both Summary and Display, and item is a reference to T.

So the key new idea is simply:

Summary + Display

means:

the type must implement Summary AND Display.

One final detail: when you later see something like where T: Summary + Display, that's another way Rust lets you write the 
same kind of trait bounds, especially when the bounds become long.
*/



/* 
fn some_function1<T: Display + Clone , U:Clone + Debug>(t:&T , u:&U){

}

fn some_function2<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    10
}
*/

/*

Yes. These two functions are introducing a new syntax for writing trait bounds. The important thing is that they mean 
essentially the same thing; the second one just moves the trait requirements to a where clause.

Let's start from zero.

1. First, forget the complicated syntax

Look at this simple generic function:

fn some_function<T>(t: &T) {
}

We have:

<T>

This declares a generic type parameter called T.

So Rust is being told:

This function can work with some type T. I don't know what T is yet.

And:

t: &T

means:

t is a reference to a value of type T.

For example, T could become i32:

let number = 10;
some_function(&number);

Then conceptually:

T = i32
t = &i32

Or T could be String:

let text = String::from("hello");
some_function(&text);

Then:

T = String
t = &String
2. Now we add a trait requirement

Suppose we write:

fn some_function<T: Display>(t: &T) {
}

The part:

T: Display

is called a trait bound.

It means:

Whatever type T is, it must implement Display.

So we have two pieces:

<T: Display>

T = some generic type.

Display = requirement that this type must implement.

Therefore:

fn some_function<T: Display>(t: &T)

means:

Give me a reference to some type T, and T must implement Display.

3. Now your first function

You have:

fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {

}

There are actually two generic types here:

T
U

So let's separate the function into pieces:

fn some_function1<
    T: Display + Clone,
    U: Clone + Debug
>(
    t: &T,
    u: &U
) {

}

Now it's much easier to read.

4. What does T mean?

This:

T

is one generic type.

The function doesn't know yet whether T will be:

i32
String
MyStruct
...

But we put requirements on it:

T: Display + Clone

That means:

Whatever T is, it must implement Display AND Clone.

So T has two requirements.

5. What does U mean?

We have another generic type:

U

And:

U: Clone + Debug

means:

Whatever U is, it must implement Clone AND Debug.

So U has two requirements.

6. Put the whole thing into English

This:

fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
}

means:

Create a function with two generic types, T and U.

T must implement Display and Clone.

U must implement Clone and Debug.

t is a reference to a T.

u is a reference to a U.

That's the entire meaning.

7. Notice that T and U are independent

This is very important.

We have:

T: Display + Clone

and:

U: Clone + Debug

There is no requirement that T and U are the same type.

For example, imagine:

T = String
U = SomeStruct

That could be valid if:

String implements Display
String implements Clone

SomeStruct implements Clone
SomeStruct implements Debug

So:

t: &String
u: &SomeStruct

could satisfy the function.

8. Why is Clone written for both?

Look carefully:

T: Display + Clone
U: Clone + Debug

Clone appears in both places.

That means:

T must be Clone.

and separately:

U must also be Clone.

It does not mean that T and U have to be the same type.

For example:

T = String
U = MyStruct

is perfectly possible if both implement Clone.

9. What does Display mean?

Display is a standard Rust trait.

It is related to normal {} formatting:

println!("{}", value);

For example:

let number = 100;
println!("{}", number);

i32 implements Display.

So if:

T = i32

then:

T: Display

is satisfied.

10. What does Debug mean?

Debug is another standard Rust trait.

It is used with:

println!("{:?}", value);

For example:

let numbers = vec![10, 20, 30];

println!("{:?}", numbers);

Vec<i32> implements Debug.

So:

U: Debug

means:

Whatever U is, Rust must be able to format it using the Debug formatting system.

11. What does Clone mean?

You already learned this one.

Clone provides the .clone() operation.

For example:

let first = String::from("hello");
let second = first.clone();

So:

T: Clone

means:

Code working with T is allowed to use the Clone behavior on a T.

And:

U: Clone

means the same thing for U.

12. Now let's look at the parameters

This part:

(t: &T, u: &U)

means:

t: &T

T is the first generic type, and t is a reference to it.

And:

u: &U

U is the second generic type, and u is a reference to it.

For example, suppose Rust determines:

T = i32
U = String

Then the function parameters become conceptually:

t: &i32
u: &String

Of course, you don't write that yourself. Rust determines the generic types when the function is used.

13. Now your second function

You have:

fn some_function2<T, U>(t: &T, u: &U) -> i32

    where T: Display + Clone,
          U: Clone + Debug

{
}

At first this looks very different.

But it is basically the same requirements.

The first function puts the requirements here:

<T: Display + Clone, U: Clone + Debug>

The second function first declares the generic types:

<T, U>

and then puts the requirements later:

where T: Display + Clone,
      U: Clone + Debug
14. What is where?

where is another way to write trait bounds.

Look at:

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
{
}

Read it as:

This function has generic types T and U.

Then:

where

means:

Here are the requirements those generic types must satisfy.

Then:

T: Display + Clone

means:

T must implement Display and Clone.

And:

U: Clone + Debug

means:

U must implement Clone and Debug.

15. Therefore these two are essentially equivalent

First:

fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
}

Second:

fn some_function2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug
{
}

The important difference is where you write the trait bounds.

First style:

<T: Display + Clone, U: Clone + Debug>

Second style:

<T, U>
where
    T: Display + Clone,
    U: Clone + Debug

The second style becomes much easier to read when there are many generic types or many trait requirements.

16. There is one more thing in your second function

You have:

-> i32

This means:

The function returns an i32.

So:

fn some_function2<T, U>(t: &T, u: &U) -> i32

says:

This function receives references to generic types T and U, and returns an i32.

But your function body is empty:

{
}

Therefore the code as written will not compile, because the function promises to return an i32, but it doesn't return one.

For example, this would compile:

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

The final expression:

10

is the return value.

17. Why would we use where?

Imagine the function becomes more complicated:

fn some_function<T, U, V, W>(
    first: &T,
    second: &U,
    third: &V,
    fourth: &W,
)
where
    T: Display + Clone,
    U: Clone + Debug,
    V: Debug + PartialOrd,
    W: Clone + PartialOrd + Display,
{
}

This is much easier to read than putting everything inside < >:

fn some_function<
    T: Display + Clone,
    U: Clone + Debug,
    V: Debug + PartialOrd,
    W: Clone + PartialOrd + Display,
>(
    first: &T,
    second: &U,
    third: &V,
    fourth: &W,
) {
}

Both styles express trait bounds, but where separates:

What are the generic types?

<T, U, V, W>

from:

What requirements do they have?

where
    T: Display + Clone,
    U: Clone + Debug,
    ...

That's the main reason you will see where in real Rust code.

The mental model I want you to keep

When you see:

<T: Display + Clone, U: Clone + Debug>

read it as:

There are two unknown types, T and U.

T must have the Display and Clone behaviors.

U must have the Clone and Debug behaviors.

When you see:

<T, U>
where
    T: Display + Clone,
    U: Clone + Debug

read it exactly the same way.

The only difference is that the second form moves the requirements into a separate where section.
*/




fn main(){




}