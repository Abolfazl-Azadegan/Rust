pub trait Summary {
    fn summarize_author(&self) -> String;

    //No change needs to Tweet struct.
    //Tweet has it's own implementation.

    fn summarize(&self) -> String{
        String::from("(Read More...)")
    }
}

//Here is trait we have defined the summarize method so when we want to assign the trait to the struct we do not need to define
//function in the struct because it has already defined when we define the trait.

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle{
    fn summarize_author(&self) -> String {
        //implementation here
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}





fn main(){



    let article1 = NewArticle{
        headline: String::from("Article 1"),
        location: String::from("Location 1"),
        author: String::from("Author 1"),
        content: String::from("Content 1"),
    };

    println!("{}", article1.summarize_author());
    println!("{}", article1.summarize());


}