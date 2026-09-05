
fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("The value of X is {}", x);

    /*This will generate error, because the x variable in immutable which is defined by let and we can not change its value
    later in the code.
    
    // x = 6;
    // println!("The value of X is {}", x);

    */

    /* to solve this issue we can make the x mutable by adding mut to its definition with let. like below code.

    // let mut x = 5;
    // println!("The value of X is {}", x);

    now we can have below code together with let mut x = 5;

    // x = 6;
    // println!("The value of X is {}", x);
    */

    const SECOND: i8 = 60; //This is how we declare a constant variable. i8 here is the data type which is integer 8bit.
    println!("The constant variable is equal to: {}", SECOND);

    let mut y:i8 = 10; //Signed 8 bit integer
    let mut z:i16 = 20; //Signed 16 bit integer
    let mut a:i32 = 30; //Signed 32 bit integer
    let mut b:i64 = 40; //Signed 64 bit integer
    let mut c:i128 = 50; //Signed 128 bit integer
    println!("i8= {}",y);
    println!("i16= {}",z);
    println!("i32= {}",a);
    println!("i64= {}",b);
    println!("i128= {}",c);

    y= -10; //Signed 8 bit integer
    z= -20; //Signed 16 bit integer
    a= -30; //Signed 32 bit integer
    b= -40; //Signed 64 bit integer
    c= -50; //Signed 128 bit integer
    println!("i8= {}",y);
    println!("i16= {}",z);
    println!("i32= {}",a);
    println!("i64= {}",b);
    println!("i128= {}",c);


    let y:u8 = 100; //Signed 8 bit integer
    let z:u16 = 200; //Signed 16 bit integer
    let a:u32 = 300; //Signed 32 bit integer
    let b:u64 = 400; //Signed 64 bit integer
    let c:u128 = 500; //Signed 128 bit integer
    println!("u8= {}",y);
    println!("u16= {}",z);
    println!("u32= {}",a);
    println!("u64= {}",b);
    println!("u128= {}",c);

    /*This part will generate error because we are setting signed numbers (Negative) to unsigned numbers.
    y= -100; //Signed 8 bit integer
    z= -200; //Signed 16 bit integer
    a= -300; //Signed 32 bit integer
    b= -400; //Signed 64 bit integer
    c= -500; //Signed 128 bit integer
    println!("i8= {}",y);
    println!("i16= {}",z);
    println!("i32= {}",a);
    println!("i64= {}",b);
    println!("i128= {}",c);
    */

    
}
