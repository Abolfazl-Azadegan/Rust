

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




fn main(){




}