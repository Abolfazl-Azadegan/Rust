
struct Point<X1, Y1>{
    x: X1,
    y: Y1
}


impl <X1, Y1> Point <X1, Y1>{
    fn mixup<X2, Y2>(self,other: Point<X2, Y2>) -> Point <X1, Y2>{
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main(){

    let p1 = Point {x:5 ,   y:10.4};
    let p2 = Point {x: "Hello",y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x,p3.y);


    /*
    This example is difficult because it combines three separate generic concepts at the same time:

    A struct with two generic types
    An impl with two generic types
    A method with two additional generic types

    Let's build it from the beginning and then execute your exact program step by step.

    1. First understand the struct

    Your code starts with:

    struct Point<X1, Y1> {
        x: X1,
        y: Y1
    }

    Don't think about impl or mixup yet.

    Just understand this struct.

    Normally, we could have:

    struct Point {
        x: i32,
        y: f64,
    }

    That would mean every Point has:

    x -> i32
    y -> f64

    But your struct says:

    struct Point<X1, Y1> {
        x: X1,
        y: Y1
    }

    This means:

    Point has two generic type parameters.

    X1 represents the type of x.

    Y1 represents the type of y.

    So Rust can create different versions of Point.

    For example:

    Point<i32, f64>

    means:

    x: i32
    y: f64

    And:

    Point<&str, char>

    means:

    x: &str
    y: char

    And:

    Point<String, bool>

    means:

    x: String
    y: bool

    So the names X1 and Y1 are just placeholders for types.

    2. Now look at p1

    You have:

    let p1 = Point { x: 5, y: 10.4 };

    Rust looks at the values:

    x: 5

    5 is an i32.

    And:

    y: 10.4

    10.4 is an f64.

    Therefore Rust determines:

    X1 = i32
    Y1 = f64

    So the type of p1 is:

    Point<i32, f64>

    Therefore, for this particular value, the struct is effectively:

    struct Point {
        x: i32,
        y: f64
    }

    You didn't write that second struct. Rust determines the concrete types from the generic parameters.

    3. Now look at p2

    You have:

    let p2 = Point { x: "Hello", y: 'c' };

    Now:

    "Hello"

    has type:

    &str

    and:

    'c'

    has type:

    char

    Therefore:

    X1 = &str
    Y1 = char

    for this particular Point.

    So:

    p2

    has type:

    Point<&str, char>

    At this point, we have:

    p1: Point<i32, f64>

    p2: Point<&str, char>

    That's already one important part of the program.

    4. Now we get to the difficult part

    You have:

    impl<X1, Y1> Point<X1, Y1> {

    You already learned that:

    impl<T> Point<T>

    means:

    "For any type T, implement these methods for Point<T>."

    Here we have two types instead of one:

    impl<X1, Y1> Point<X1, Y1>

    Read it as:

    "For any types X1 and Y1, implement these methods for Point<X1, Y1>."

    So this impl applies to:

    Point<i32, f64>

    and:

    Point<&str, char>

    and:

    Point<String, bool>

    and any other combination of two types.

    5. Why are X1 and Y1 needed after impl?

    Same principle as before.

    This:

    impl<X1, Y1>

    declares two generic type parameters for the impl.

    Then:

    Point<X1, Y1>

    says:

    "This implementation is for Point using those two generic types."

    So:

    impl<X1, Y1> Point<X1, Y1>

    means:

    "For every possible pair of types X1 and Y1, these methods are available on Point<X1, Y1>."

    6. Now look at the method

    This is the most difficult line:

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>

    Let's separate it conceptually.

    There are two different sets of generic types here:

    X1, Y1

    and:

    X2, Y2

    This is extremely important.

    X1 and Y1 belong to the impl.

    X2 and Y2 belong specifically to the mixup method.

    7. Where did X1 and Y1 come from?

    From here:

    impl<X1, Y1> Point<X1, Y1>

    So inside this impl, Rust knows:

    X1
    Y1

    They represent the types of the Point on which the method is being called.

    For example, when we eventually call:

    p1.mixup(...)

    p1 is:

    Point<i32, f64>

    Therefore, for that method call:

    X1 = i32
    Y1 = f64
    8. Where did X2 and Y2 come from?

    From here:

    fn mixup<X2, Y2>

    This method introduces two new generic type parameters.

    They are independent from X1 and Y1.

    This is why the method can accept another Point whose types are completely different.

    The parameter:

    other: Point<X2, Y2>

    means:

    "The other parameter is another Point, but its x and y types can be different."

    So we could have:

    self: Point<i32, f64>

    and:

    other: Point<&str, char>

    That is exactly what your program does.

    9. Now let's understand the method's input

    Look at:

    fn mixup<X2, Y2>(
        self,
        other: Point<X2, Y2>
    )

    There are two parameters.

    First:
    self

    This is the Point on which we call the method.

    You will call:

    p1.mixup(p2)

    So:

    self

    is p1.

    And:

    other

    is p2.

    Because p1 is:

    Point<i32, f64>

    we have:

    X1 = i32
    Y1 = f64

    Because p2 is:

    Point<&str, char>

    we have:

    X2 = &str
    Y2 = char

    Now we can substitute those actual types into the method.

    10. Let's substitute the types

    The generic method is:

    fn mixup<X2, Y2>(
        self,
        other: Point<X2, Y2>
    ) -> Point<X1, Y2>

    For your actual call:

    p1.mixup(p2)

    we know:

    X1 = i32
    Y1 = f64

    X2 = &str
    Y2 = char

    So the method becomes conceptually:

    fn mixup(
        self: Point<i32, f64>,
        other: Point<&str, char>
    ) -> Point<i32, char>

    That is the key to understanding the whole example.

    11. Now look at the body

    The method contains:

    Point {
        x: self.x,
        y: other.y,
    }

    This is the whole purpose of mixup.

    It takes:

    x

    from self.

    And it takes:

    y

    from other.

    So:

    self.x

    comes from p1.

    And:

    other.y

    comes from p2.

    Let's look at their actual types.

    p1 is:

    Point<i32, f64>

    so:

    p1.x

    is:

    i32

    And p2 is:

    Point<&str, char>

    so:

    p2.y

    is:

    char

    Therefore the new Point contains:

    x: i32
    y: char

    So its type is:

    Point<i32, char>

    And that's exactly what the return type says:

    -> Point<X1, Y2>
    12. Why is the return type Point<X1, Y2>?

    This is the most important line to understand:

    -> Point<X1, Y2>

    It says:

    The new Point will use the x type from the first Point and the y type from the second Point.

    Remember:

    X1

    is the x type of self.

    And:

    Y2

    is the y type of other.

    Therefore:

    Point<X1, Y2>

    means:

    "Create a new Point whose x has the type of the first Point's x, and whose y has the type of the second Point's y."

    That exactly matches the implementation:

    Point {
        x: self.x,
        y: other.y,
    }
    13. Notice that Y1 isn't used in the result

    This is intentional.

    The original self has:

    Point<X1, Y1>

    But the result is:

    Point<X1, Y2>

    So:

    self.x is kept
    self.y is discarded
    other.x is discarded
    other.y is kept

    Your method is essentially saying:

    "Give me two Points. I'll keep the x value from the first one and the y value from the second one."

    14. Let's execute your exact program

    You have:

    let p1 = Point { x: 5, y: 10.4 };

    Rust determines:

    p1: Point<i32, f64>

    So:

    p1.x

    is:

    5

    and:

    p1.y

    is:

    10.4

    Then:

    let p2 = Point { x: "Hello", y: 'c' };

    Rust determines:

    p2: Point<&str, char>

    So:

    p2.x

    is:

    "Hello"

    and:

    p2.y

    is:

    'c'

    Then:

    let p3 = p1.mixup(p2);

    The method is called on p1.

    Therefore:

    self = p1

    and:

    other = p2

    The types are:

    self: Point<i32, f64>

    other: Point<&str, char>

    Therefore:

    X1 = i32
    Y1 = f64

    X2 = &str
    Y2 = char

    The return type:

    Point<X1, Y2>

    becomes:

    Point<i32, char>

    The body:

    Point {
        x: self.x,
        y: other.y,
    }

    becomes effectively:

    Point {
        x: 5,
        y: 'c',
    }

    Therefore:

    p3

    has type:

    Point<i32, char>
    15. Finally the println!

    You have:

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    p3.x is:

    5

    and p3.y is:

    'c'

    So the output is:

    p3.x = 5, p3.y = c
    16. The biggest source of confusion: there are FOUR type parameters

    Your code has:

    struct Point<X1, Y1>

    and:

    impl<X1, Y1>

    and:

    fn mixup<X2, Y2>

    You should mentally separate them.

    X1 and Y1

    These describe the Point that owns the method.

    impl<X1, Y1> Point<X1, Y1>

    For p1:

    X1 = i32
    Y1 = f64
    X2 and Y2

    These describe the other Point passed to the method.

    other: Point<X2, Y2>

    For p2:

    X2 = &str
    Y2 = char

    Therefore:

    Point<X1, Y1>

    is:

    Point<i32, f64>

    and:

    Point<X2, Y2>

    is:

    Point<&str, char>

    and the result:

    Point<X1, Y2>

    is:

    Point<i32, char>
    17. Why couldn't we just write Point<X1, Y1> for the result?

    Because that would mean the result has the same types as the first Point.

    For example:

    -> Point<X1, Y1>

    would mean:

    Point<i32, f64>

    for your example.

    But your body is:

    Point {
        x: self.x,
        y: other.y,
    }

    The y is coming from other, and other.y is a char.

    So the result cannot be:

    Point<i32, f64>

    because you're putting a char into its y field.

    The correct result is:

    Point<i32, char>

    which is why Rust code says:

    -> Point<X1, Y2>
    18. One final simplified version

    If we temporarily forget generics, your program is basically doing this:

    struct Point1 {
        x: i32,
        y: f64,
    }

    struct Point2 {
        x: &str,
        y: char,
    }

    Then the operation is essentially:

    Point {
        x: first_point.x,
        y: second_point.y,
    }

    giving:

    Point {
        x: 5,
        y: 'c',
    }

    The power of generics is that Rust doesn't need separate structs for every combination.

    Instead, this one definition:

    struct Point<X1, Y1> {
        x: X1,
        y: Y1
    }

    can represent:

    Point<i32, f64>
    Point<&str, char>
    Point<String, bool>
    Point<f32, i64>

    and the mixup method can combine any two compatible Points by taking the x type from the first and the y type from the second.

    The single most important sentence for this code is:

    X1 and Y1 describe the Point on which mixup is called; X2 and Y2 describe the other Point passed to mixup; Point<X1, Y2> says 
    the result keeps the first Point's x type and the second Point's y type.
    */

}