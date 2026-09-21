
//In this struct we used the generic as our type. As you can see in the definition the x and y both should have the same
//type which is denoted as T. so for example if the x is int and y should be int as well. we can not let them have different
// types as one of them be int and the otherone be float.
struct Point <T>{
    x: T,
    y: T,
}


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

}