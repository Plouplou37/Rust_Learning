[Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)

he concept of Generic enables to abstract stand-ins for concrete type or others queries.
It enable to redunce redundance in the code.
For example two function ca have the same behavior but takes != data type in parameters. We can group the behavior under
one function (instead of two) using generics. It adds an layer of abstaction.

Some example alreayd seen untill now are : - Option<T> - Vec<T> - HasMap<V, K> - Result<T, E>

The type of the value passed in the function is not known in advance !
