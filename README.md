# rusty_life
Yet another Conway Game of Life implementation, in Rust.
This is obviously mainly an attempt at getting familiar with Rust syntax and way of thinking.

However, instead of being satisfied with things just working, I also want to focus on a data-oriented architecture instead of OOP, and safe and performant way to do the minimum calculations necessary, while having it readable.

I've already looked at other implementations, and I am taking some inspirations. The goal for myself is to try a data-oriented design instead of OOP, and try some of that Rust fearless concurrency, first through CPU, then GPU. 

The `0.1` version focuses on rust basic syntax, and is implemented in 2 threads, one for generating the gid, another one for "drawing" it.
The generating part was just so fastm it became quite obvious the drawing thread was a bottleneck. It was, after all, just printing symbols to the console.
It was fine for the basics, and was implemented with only the rand crate as dependency, and some standard library crates. Sweet and simple.

The `0.2 ` version is a complete rewrite, this time using actual game rendering library `amthyst`. No more printing to the console, we're doing 2D animated sprites now. And god damn you bet it's still data-oriented.