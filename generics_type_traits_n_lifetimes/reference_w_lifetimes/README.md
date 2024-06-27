[Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

Lifetimes are another kind of generics ! Rather than ensuring that a type has the behavior we want, lifetimes ensures that references are valid as long as we need them to be.

In RUST, every references has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

One lifetime annotation by itself doesn't have much meaning, because the annotations are meant to tell RUST how generic lifetime parameters of multiple references realte to each other.
