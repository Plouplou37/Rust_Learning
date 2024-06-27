use std::fmt::Display;

//We want the signature to express the following constraint:
//the returned reference will be valid as long as both the parameters are valid.
// This is the relationship between lifetimes of the parameters and the return value
//the generic lifetime 'a will get the concrete lifetime
// that is equal to the smaller of the lifetimes of s1 and s2
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

//This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
struct ImportantExcerpt<'a> {
    // define the maximum lifetime of a ImportantExcerpt instance to 'a ! It can't be longer.
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[rustfmt::skip]
fn main() {

  //static lifetime which is valid for all the duration of the program
  //let jj: &'static str = "static reference lifetime";

  let novel = String::from("Call me ee, here it starts...");
  let first_sentence = novel.split(',').next().expect("No sentence found");

  let i = ImportantExcerpt {
    part: first_sentence,
  };

  i.announce_and_return_part("here is the annoucement");
  println!("the part is {}", i.part);

  let x = "x stringhere";
  let y = "y stirnghere";
  let res = longest_with_an_annoucement(x, y, i.part);
  println!("res: {}", res);


  /*let string1 = String::from("abcd");
  let result;
  let string2 = String::from("xyz");
  {
    //let string2 = String::from("xyz");
    //Here it desnt compile because result will have the lifetime of the smallest lifetime reference in the paramaters
    //which is the lifetime of string2.
    //string2 lifetime reside within the inner scope so result will only reside in this scope also !
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {result}");*/

  /*
  let string1 = String::from("abcd");
  
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {result}");
  }*/


  /*let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);

  println!("The longest string is {result}");*/


  /*let r;                  // ---------+-- 'a
                          //          |
  {                       //          |
      let x = 5;          // -+-- 'b  |
      r = &x;             //  |       |
  }                       // -+       |
                          //          |
  println!("r: {r}");     //          |
                          // ---------+*/
  /*
  let g = 5;            // ----------+-- 'b
                        //           |
  let f = &g;           // --+-- 'a  |
                        //   |       |
  println!("f: {f}");   //   |       |
                        // --+       |
                        // ----------+*/

}
