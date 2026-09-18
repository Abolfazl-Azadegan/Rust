


fn main(){


    let r;
    {
        let x = 10;
        r = &x;

        // For Rust's formatting system, you escape curly braces by doubling them:
        /************************************************************************************************************
        println!("I want to print {{}}");
        Output:
        I want to print {}
        So:

        {{  → {
        }}  → }

        This is not a backslash escape.
        It is a rule of Rust's formatting syntax.
         */
        
        println!("The variable x in the {{}} is: {}", x);
        println!("The variable r in the {{}} is: {}", r);
    }// The variable x is dropped here so it no longer is available

    //Below line will generate error.
    // println!("The variable r out of the {} is: ", r); // This is dangeling referencing. It means referencec to a variable which
    // Is no loner available.

}



