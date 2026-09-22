
//In this struct we used the generic as our type. As you can see in the definition the x and y both should have the same
//type which is denoted as T. so for example if the x is int and y should be int as well. we can not let them have different
// types as one of them be int and the otherone be float.
struct Point <T>{
    x: T,
    y: T,
}

// In this struct we have used two different generic variable in struct and then we can have different types for x and y
struct Point2 <T, U>{
    x: T,
    y: U,
}

impl <T> Point <T>{
    fn return_x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_center (&self) -> f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}

    /*
    Yes. The key is that there are two different places where types appear, and they do different jobs.

    Suppose we have:

    struct Point<T> {
        x: T,
        y: T,
    }
    1. What is <T> in struct Point<T>?

    Here:

    struct Point<T>

    T is a generic type parameter.

    It means:

    "When someone creates a Point, they will specify what type T is."

    Then you use that T for the fields:

    struct Point<T> {
        x: T,
        y: T,
    }

    So you're saying:

    x and y must have the same type, whatever that type happens to be.

    For example, if T becomes i32:

    Point<i32>

    then the definition effectively becomes:

    struct Point {
        x: i32,
        y: i32,
    }

    If T becomes f32:

    Point<f32>

    then it effectively becomes:

    struct Point {
        x: f32,
        y: f32,
    }

    If T becomes String:

    Point<String>

    then it effectively becomes:

    struct Point {
        x: String,
        y: String,
    }

    So:

    struct Point<T> {
        x: T,
        y: T,
    }

    means:

    "Point is a generic struct. T is a placeholder for a type, and both x and y use that type."

    2. Then what does Point<i32> mean?

    Now we're no longer defining the generic struct.

    We're saying:

    Point<i32>

    means:

    "Point where T is i32."

    So:

    Point<i32>

    has:

    x: i32
    y: i32

    And:

    Point<f32>

    has:

    x: f32
    y: f32

    And:

    Point<f64>

    has:

    x: f64
    y: f64

    The <...> tells Rust what type is being substituted for the generic parameter.

    3. How is this different from x: T?

    Look at:

    struct Point<T> {
        x: T,
        y: T,
    }

    This part:

    x: T

    means:

    The field named x has the type represented by T.

    This part:

    Point<T>

    means:

    Point has a generic type parameter called T.

    And later:

    Point<i32>

    means:

    For this particular Point, replace T with i32.

    So:

    struct Point<T> {
        x: T,
        y: T,
    }

    is the template/definition.

    Then:

    Point<i32>

    is a particular use of that definition.

    4. Now look at an actual variable

    You can explicitly write:

    let p: Point<i32> = Point {
        x: 3,
        y: 4,
    };

    There are actually two type-related things here:

    let p: Point<i32>

    This says:

    The variable p has type Point<i32>.

    And:

    x: 3,
    y: 4,

    are the actual values being put into the fields.

    Because p is a Point<i32>, Rust expects:

    x: i32
    y: i32

    You can also write:

    let p: Point<f32> = Point {
        x: 3.0,
        y: 4.0,
    };

    Now p is:

    Point<f32>

    so:

    x: f32
    y: f32
    5. You don't always need to write Point<i32> when creating it

    Rust can often infer the type:

    let p = Point {
        x: 3,
        y: 4,
    };

    Rust sees the integer values and determines that this is:

    Point<i32>

    Similarly:

    let p = Point {
        x: 3.0,
        y: 4.0,
    };

    is inferred as:

    Point<f64>

    by default for floating-point literals.

    You could explicitly force f32:

    let p: Point<f32> = Point {
        x: 3.0,
        y: 4.0,
    };
    6. Why did we use impl Point<f32>?

    Now this should make more sense:

    impl Point<f32> {
        fn distance_from_center(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    This says:

    "Define this method specifically for Point<f32>."

    So:

    Point<i32>

    doesn't get this method.

    Point<f32>

    does get this method.

    Point<f64>

    doesn't get this particular implementation either.

    That's because you specifically wrote:

    impl Point<f32>
    The whole picture
    struct Point<T> {
        x: T,
        y: T,
    }

    T is a generic type parameter.

    Then:

    Point<i32>

    means:

    Point with T = i32.

    Therefore its fields are:

    x: i32
    y: i32

    And:

    Point<f32>

    means:

    Point with T = f32.

    Therefore its fields are:

    x: f32
    y: f32

    So the <T> in the struct definition is essentially saying "this struct has a type parameter."

    The <i32> / <f32> when you use Point says "this is the actual type that I'm putting into that type parameter."
     */


fn main(){

    let integer_value = Point{ x: 5 , y: 10};
    println!("The x value of integer_value is: {:?}", integer_value.x);
    println!("The y value of integer_value is: {:?}", integer_value.y);

    let float_value = Point {x: 4.2 , y: 5.6};
    println!("The x value of float_value is: {:?}", float_value.x);
    println!("The y value of float_value is: {:?}", float_value.y);

    /* This will generate error because x and y have different types opposite to their definition
    let mixed_value = Point {x:30, y:4.68};
    println!("The x value of mixed_value is: {:?}", float_value.x);
    println!("The y value of mixed_value is: {:?}", float_value.y);
    */


    let mixed_value = Point2 {x:30, y:4.68};
    println!("The x value of mixed_value is: {:?}", mixed_value.x);
    println!("The y value of mixed_value is: {:?}", mixed_value.y);
    

    println!("----------------------------------------------------------------------------");


    let p1 = Point {x:3, y:4};
    println!("The x is: {}", p1.return_x());
    // This will generate an error
    //println!("Distance from the center is: {}", p.distance_from_center(4.5));

    let p2 = Point {x:3.2, y:4.6};
    println!("The x is: {}", p2.return_x());
    // This will generate an error
    println!("Distance from the center is: {}", p2.distance_from_center());

    /*
    1. What does impl Point<f32> mean?
    impl Point<f32> {

    Because Point is generic:

    struct Point<T> {
        x: T,
        y: T,
    }

    Point<T> can be a point containing different types:

    Point<i32>
    Point<f32>
    Point<f64>

    When you write:

    impl Point<f32> {

    you are saying:

    "The methods I'm defining inside this impl are specifically for Point<f32>."

    So this method:

    fn distance_from_center(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    can be called on a:

    Point<f32>

    but not on a:

    Point<i32>
    2. Your p is currently probably a Point<i32>

    You wrote:

    let p = Point { x: 3, y: 4 };

    Because 3 and 4 are integer literals and you haven't told Rust otherwise, Rust will normally infer:

    p: Point<i32>

    But your method belongs specifically to:

    Point<f32>

    So you need to create a Point<f32>:

    let p = Point {
        x: 3.0,
        y: 4.0,
    };

    Now Rust knows:

    p: Point<f32>

    because 3.0 and 4.0 are floating-point literals.

    3. Your method takes only &self

    Your method is:

    fn distance_from_center(&self) -> f32

    There is no additional parameter.

    &self means:

    "Use the particular Point on which the method is called."

    So:

    p.distance_from_center()

    is correct.

    But you wrote:

    p.distance_from_center(4.5)

    That is incorrect because you're giving the method an argument:

    4.5

    even though the method doesn't have a parameter for it.

    You should write:

    p.distance_from_center()
     */

    

}