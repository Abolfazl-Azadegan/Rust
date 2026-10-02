

pub trait Summary {
    fn summarization(&self) -> String;
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



fn returns_summarization() -> impl Summary{

    Tweet{
        username:String::from("horse_ebooks"),
        content:String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }

}




pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}







/*
Yes. This is the return-value version of impl Trait.

You have already seen:

fn notify(item: &impl Summary)

which means:

The function accepts some type that implements Summary.

Now this:

fn returns_summarization() -> impl Summary

means:

The function returns some type that implements Summary.

That is the central idea.

1. Let's look at the complete code
fn returns_summarization() -> impl Summary {

    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

Suppose we have:

trait Summary {
    fn summarize(&self) -> String;
}

and:

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

and:

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

Now we have:

fn returns_summarization() -> impl Summary
2. What does -> mean?

You already know that:

fn add(a: i32, b: i32) -> i32

means:

This function returns an i32.

For example:

fn add(a: i32, b: i32) -> i32 {
    a + b
}

The -> i32 tells us the return type.

So:

fn returns_summarization() -> impl Summary

also specifies a return type.

But instead of saying:

-> Tweet

it says:

-> impl Summary
3. What does impl Summary mean here?

Here it means:

The function returns some concrete type that implements Summary.

In your code, that concrete type happens to be:

Tweet

because the function returns:

Tweet {
    username: ...,
    content: ...,
    reply: ...,
    retweet: ...,
}

And we have:

impl Summary for Tweet

Therefore Tweet satisfies the requirement.

4. So why not just write -> Tweet?

Excellent question.

You could write:

fn returns_summarization() -> Tweet {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

That means:

This function specifically returns a Tweet.

But:

fn returns_summarization() -> impl Summary

means:

This function returns some concrete type that implements Summary. I don't want the caller to depend on the exact concrete type.

This can be useful when the implementation wants to hide the exact type it returns.

5. Think about the difference

With:

fn returns_summarization() -> Tweet

the caller knows:

The return type is Tweet.

With:

fn returns_summarization() -> impl Summary

the caller knows:

The return value implements Summary.

But the function's public signature does not reveal the concrete type.

The caller can use the behavior guaranteed by Summary.

For example:

let result = returns_summarization();

println!("{}", result.summarize());

This works because Rust knows:

result: some type implementing Summary

and Summary guarantees that:

summarize()

exists.

6. But there is an important restriction

Look at this:

fn returns_summarization() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Hello"),
        reply: false,
        retweet: false,
    }
}

The function returns a Tweet.

You might think:

Since it says impl Summary, can I sometimes return Tweet and sometimes Article, as long as both implement Summary?

No.

This is one of the most important things to understand.

Suppose:

struct Article {
    title: String,
}

and:

impl Summary for Article {
    fn summarize(&self) -> String {
        self.title.clone()
    }
}

Both types implement Summary:

Tweet → Summary
Article → Summary

But this is not allowed:

fn returns_summarization(condition: bool) -> impl Summary {

    if condition {
        Tweet {
            username: String::from("EFE"),
            content: String::from("Hello"),
            reply: false,
            retweet: false,
        }
    } else {
        Article {
            title: String::from("Rust"),
        }
    }
}

Why?

Because impl Summary in a return position means:

One specific concrete type chosen by this function.

It does not mean:

Any different type implementing Summary can be returned at runtime.

So the function must consistently return one concrete type.

7. Why is this useful?

Imagine you have a complicated function:

fn create_something() -> impl Summary {
    // lots of complicated code...
    
    Tweet {
        username: String::from("EFE"),
        content: String::from("Hello"),
        reply: false,
        retweet: false,
    }
}

Maybe internally you have complicated logic, helper functions, generic types, or some type that you don't want users of 
your code to know about.

You can simply tell the caller:

Whatever I'm returning implements Summary. You can use the Summary functionality.

The caller doesn't need to know the exact concrete type to use the trait's interface.

8. Compare this with the impl you learned earlier

Earlier you saw:

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

Here:

&impl Summary

is an input parameter.

It means:

Give me a reference to some type implementing Summary.

Now:

fn returns_summarization() -> impl Summary

has:

impl Summary

in the return position.

It means:

I will give you some concrete type implementing Summary.

So:

fn notify(item: &impl Summary)

means:

I accept something implementing Summary.

While:

fn returns_summarization() -> impl Summary

means:

I return something implementing Summary.

9. A complete example
use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn returns_summarization() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let result = returns_summarization();

    println!("{}", result.summarize());
}

The important sequence is:

let result = returns_summarization();

The function says:

-> impl Summary

So Rust knows that result is some type implementing Summary.

The actual concrete type is Tweet.

Therefore:

result.summarize()

is allowed.

10. One thing I want you to notice about the name

The function is called:

returns_summarization

but it actually returns a Tweet value, not a String containing the summary.

This:

-> impl Summary

does not mean:

"I return the result of summarize()."

It means:

"I return an object/value whose type implements the Summary trait."

So these are completely different:

fn returns_summarization() -> String

means:

Return the actual summary text.

Whereas:

fn returns_summarization() -> impl Summary

means:

Return a value that has the Summary behavior.

For example:

let result = returns_summarization();

println!("{}", result.summarize());

The .summarize() call happens after the function returns the Tweet.

The key idea

You can remember the two forms like this:

fn notify(item: &impl Summary)

Input:
"Give me something that implements Summary."

fn returns_summarization() -> impl Summary

Output:
"I will give you something that implements Summary."

And in your exact example, that "something" is a Tweet.
*/


fn main(){



    /*
    
    You're right to question it. The confusing part is that impl Summary does not mean "any type that implements Summary 
    can be returned." It has a more specific meaning in a return position.

    Let's rebuild this carefully.

    1. First: your reasoning about if is understandable

    You are thinking:

    Tweet implements Summary.
    Article implements Summary.
    Therefore, if the if branch returns Tweet and the else branch returns Article, shouldn't impl Summary allow both?

    It would be reasonable to expect that.

    But Rust's impl Trait in a return position does not mean:

    "The function can return any type satisfying this trait."

    Instead, it means:

    "The function chooses one concrete return type, but I am hiding its concrete name and exposing only the trait it implements."

    That's the key.

    2. Let's first remove impl Summary

    Suppose we write:

    fn get_something(condition: bool) -> Tweet {
        if condition {
            Tweet {
                username: String::from("EFE"),
                content: String::from("Hello"),
                reply: false,
                retweet: false,
            }
        } else {
            Tweet {
                username: String::from("another"),
                content: String::from("Hi"),
                reply: false,
                retweet: false,
            }
        }
    }

    This works.

    Why?

    Because both branches return:

    Tweet

    The function promises:

    -> Tweet

    and both branches satisfy that promise.

    3. Now suppose we use Article in the second branch
    fn get_something(condition: bool) -> Tweet {
        if condition {
            Tweet {
                username: String::from("EFE"),
                content: String::from("Hello"),
                reply: false,
                retweet: false,
            }
        } else {
            Article {
                title: String::from("Rust"),
            }
        }
    }

    This obviously doesn't work.

    Why?

    Because the function says:

    -> Tweet

    but the else branch produces:

    Article

    The return type of the function is Tweet, not "Tweet or Article."

    4. Now you might think impl Summary solves this

    So we try:

    fn get_something(condition: bool) -> impl Summary {
        if condition {
            Tweet {
                username: String::from("EFE"),
                content: String::from("Hello"),
                reply: false,
                retweet: false,
            }
        } else {
            Article {
                title: String::from("Rust"),
            }
        }
    }

    Both types implement:

    Summary

    But Rust still rejects this.

    Why?

    Because the return type:

    impl Summary

    doesn't mean:

    Tweet OR Article OR SomeOtherType

    Instead, Rust interprets it as:

    There is one concrete type behind this impl Summary, and that type is hidden from the caller.

    In this function, Rust has to determine what that hidden concrete type is.

    If the first branch gives:

    Tweet

    and the second gives:

    Article

    there is no single concrete type that represents the return type.

    That's the problem.

    5. Think about what the compiler has to determine

    Suppose:

    fn get_something(condition: bool) -> impl Summary {
        if condition {
            Tweet { ... }
        } else {
            Article { ... }
        }
    }

    The compiler has to determine:

    What is the concrete type represented by `impl Summary`?

    If the first branch says:

    Tweet

    then the hidden concrete type would be Tweet.

    But the second branch says:

    Article

    Now the hidden concrete type would have to be Article.

    It cannot simultaneously be:

    Tweet

    and:

    Article

    Therefore the function doesn't satisfy its return type.

    6. So your question is actually pointing to an important distinction

    You are thinking of:

    impl Summary

    as:

    "a value whose type belongs to the set of all types implementing Summary."

    But Rust's return-position impl Trait is closer to:

    "there is one particular concrete type here that implements Summary, but I am not telling you which concrete type it is."

    For example:

    fn get_something() -> impl Summary {
        Tweet {
            username: String::from("EFE"),
            content: String::from("Hello"),
            reply: false,
            retweet: false,
        }
    }

    The compiler knows:

    hidden concrete type = Tweet

    The caller doesn't have to know that.

    That's the important benefit.

    7. And this answers your second question

    You asked:

    If I can only return one specific type, why don't I just write -> Tweet?

    This is an excellent question.

    Sometimes you should write:

    fn get_something() -> Tweet

    There is nothing wrong with that.

    impl Summary becomes useful when you want to expose behavior rather than the concrete implementation type.

    Here's a more realistic example.

    Suppose we have:

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    and:

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    Now imagine the caller only needs to know:

    "Whatever you give me has summarize()."

    They don't necessarily need to know:

    "The concrete type is Tweet."

    So:

    fn create_summary_source() -> impl Summary {
        Tweet {
            username: String::from("EFE"),
            content: String::from("Hello"),
            reply: false,
            retweet: false,
        }
    }

    The caller can do:

    let result = create_summary_source();

    println!("{}", result.summarize());

    The caller knows that result implements Summary.

    But the function signature doesn't expose Tweet.

    8. Why would hiding Tweet be useful?

    Imagine the function internally becomes more complicated.

    Maybe today you implement it with:

    Tweet

    But your public API only promises:

    "I return something that implements Summary."

    The caller writes:

    let result = create_summary_source();

    println!("{}", result.summarize());

    The caller depends on the Summary behavior, not on the concrete type.

    That's useful for abstraction.

    You're basically saying:

    "I guarantee this behavior. The particular type I use internally is my implementation detail."

    9. There is another very important reason: complex types

    This becomes especially useful with things like iterators.

    Imagine:

    fn numbers() -> impl Iterator<Item = i32> {
        (1..10).filter(|number| number % 2 == 0)
    }

    The actual concrete type returned by that expression is complicated.

    It involves iterator types created by Rust's standard library and the filter operation.

    You could theoretically write out the exact concrete type, but it would be ugly and unnecessary.

    Instead:

    -> impl Iterator<Item = i32>

    means:

    I return some concrete iterator that produces i32 values.

    The caller doesn't need to know the complicated concrete iterator type.

    That's one of the major practical uses of impl Trait in return position.

    10. But what if I really want Tweet OR Article?

    This is where an enum comes in.

    You already learned enums.

    If you genuinely want:

    The function can return either a Tweet or an Article.

    Then you can explicitly represent those alternatives:

    enum SummaryItem {
        Tweet(Tweet),
        Article(Article),
    }

    Then:

    fn get_something(condition: bool) -> SummaryItem {
        if condition {
            SummaryItem::Tweet(
                Tweet {
                    username: String::from("EFE"),
                    content: String::from("Hello"),
                    reply: false,
                    retweet: false,
                }
            )
        } else {
            SummaryItem::Article(
                Article {
                    title: String::from("Rust"),
                }
            )
        }
    }

    Now Rust knows:

    This function returns exactly one SummaryItem, and that SummaryItem can contain either a Tweet or an Article.

    That's different from impl Summary.

    11. This is the important comparison
    -> Tweet

    Means:

    The function returns a Tweet.

    -> impl Summary

    Means:

    The function returns one specific concrete type that implements Summary, but the concrete type is hidden from the caller.

    -> SummaryItem

    where:

    enum SummaryItem {
        Tweet(Tweet),
        Article(Article),
    }

    means:

    The function can return a value containing either a Tweet or an Article.

    These are three different ideas.

    12. Why can't Rust simply treat Tweet and Article as Summary?

    Because Summary is a trait, not a concrete type.

    For example:

    trait Summary {
        fn summarize(&self) -> String;
    }

    This doesn't define one particular data structure.

    It defines behavior.

    Tweet has one layout in memory.

    Article has another layout.

    For example:

    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    versus:

    struct Article {
        title: String,
    }

    They are completely different concrete types.

    If you want one variable to hold either one, Rust needs some mechanism to represent that possibility, such as an enum or a 
    trait object.

    13. This is where dyn Summary will eventually come in

    You will probably encounter this next:

    fn get_something(condition: bool) -> Box<dyn Summary>

    This is a different concept.

    A trait object such as:

    dyn Summary

    can represent different concrete types that implement Summary.

    So conceptually:

    impl Summary

    in a return position:

    one concrete type, hidden

    while:

    dyn Summary

    is used when you want:

    different possible concrete types behind a common trait interface

    There are additional details involving Box, because a trait object has dynamically sized type characteristics, but that's 
    the next concept rather than something you need to mix into impl Summary yet.

    So your original objection was useful

    You were correct about this part:

    "If the if branch returns Tweet and the else branch returns Article, and both implement Summary, why can't impl Summary 
    accept them?"

    Because impl Summary in a return position is not an "either/or" type.

    It says:

    There is ONE concrete return type.
    That type implements Summary.
    I am hiding the name of that concrete type.

    So this is valid:

    fn get_something() -> impl Summary {
        Tweet { ... }
    }

    because the hidden concrete type is Tweet.

    But this isn't:

    fn get_something(condition: bool) -> impl Summary {
        if condition {
            Tweet { ... }
        } else {
            Article { ... }
        }
    }

    because now there are two different concrete return types.

    And yes, if you don't need to hide the concrete type, there is absolutely nothing wrong with simply writing:

    fn get_something() -> Tweet

    The purpose of impl Summary is not to allow multiple return types. Its purpose is primarily to expose a trait-based 
    interface while hiding the concrete implementation type, and it is also extremely useful for returning complicated types 
    such as iterators.
     */


}