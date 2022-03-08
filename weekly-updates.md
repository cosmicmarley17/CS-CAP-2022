# Weekly Updates

## Week 1 - Jan 18-24 2022:
* Set up a git repository on GitHub to log my progress
* Attended a virtual Veloren Reading Club meeting and learned about attacks and character states
* Set up a development environment for Veloren on my computer
* Read [Chapter 6 of The Rust Book](https://doc.rust-lang.org/stable/book/ch06-00-enums.html "Chapter 6 - The Rust Programming Language")
	* Reviewed enums and the `match` control flow operator
	* Learned about `if let` syntax and `Option<T>`

## Week 2 - Jan 25-31 2022:
* Read about the codebase structure of Veloren in the [Veloren Owner's Manual](https://book.veloren.net/contributors/index.html "Veloren Owner's Manual - For Contributors")
* Learned about the [Entity Component System (ECS)](https://book.veloren.net/contributors/developers/ecs.html "Veloren Owner's Manual - ECS") used in Veloren and did some research to understand it more
* Read [Chapter 7 of The Rust Book](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html "Chapter 7 - The Rust Programming Language")
	* Learned about organizing projects into packages, crates, and modules
* Attended a short virtual Veloren dev meeting focusing on reviewing some merge requests
* Started reading [Chapter 8 of The Rust Book](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html "Chapter 8 - The Rust Programming Language")

## Week 3 - Feb 01-07 2022:
* Finished reading [Chapter 8 of The Rust Book](https://doc.rust-lang.org/stable/book/ch08-00-common-collections.html "Chapter 8 - The Rust Programming Language")
	* Reviewed vectors, strings, and learned hash maps
	* Practiced coding in Rust by trying an example exercise at the end of the chapter: [Link to my solution here](./code/vector_stats_exercise/src/main.rs "vector_stats_exercise")
* Attended another virtual Veloren Reading Club presentation. 
	* The topic was getting started as a new contributor and making your first contributions and pull requests
	* Learned a lot more about the collaboration workflow
* Started writing a solution to [Project Euler problem #2](https://projecteuler.net/problem=2 "Project Euler - #2") to practice coding in Rust with what I've learned so far

## Week 4 - Feb 08-14 2022:
* Found and troubleshooted a bug in Veloren and submitted a detailed issue to their GitLab repository, following guidelines in the [Veloren handbook](https://book.veloren.net/players/reporting-bugs.html?highlight=issue#reporting-bugs "Veloren Owner's Manual - Reporting Bugs"). Here's a link to the issue: [Veloren GitLab: Issue #1464](https://gitlab.com/veloren/veloren/-/issues/1464 "Issue #1464")
	* I spent a significant amount of time reading/tracing code, trying to figure out what might be causing the issue.
		- I think it might be either an issue with the `grab_cursor` function in Veloren's `window` module, or an issue upstream with the external library `winit`.
* Read [Chapter 9 of The Rust Book](https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html "Chapter 9 - The Rust Programming Language")
	* Learned about error handling with `panic!`, `match`, `Result`, `unwrap`, and the `?` operator.

## Week 5 - Feb 15-21 2022:
* Did some more Veloren bug-hunting for [Issue #1464](https://gitlab.com/veloren/veloren/-/issues/1464 "Issue #1464").
* Started reading [Chapter 10 of The Rust Book](https://doc.rust-lang.org/stable/book/ch10-00-generics.html "Chapter 10 - The Rust Programming Language")

## Week 6 - Feb 22-28 2022:
* Read some of the [SPECS Book](https://specs.amethyst.rs/docs/tutorials/01_intro.html "SPECS Documentation") to learn about the Entity Component System that Veloren uses.
* Finished reading [Chapter 10 of The Rust Book](https://doc.rust-lang.org/stable/book/ch10-00-generics.html "Chapter 10 - The Rust Programming Language")
	- Learned about generic data types
	- Learned about traits (for checking for and implementing shared behavior)
	- Learned about lifetimes (for using references in functions without allowing dangling references) and lifetime annotations e.g. `'a`
* Spent some time exploring the Veloren codebase

## Week 7 - Mar 01-07 2022:
* Did some more troubleshooting for [Issue #1464](https://gitlab.com/veloren/veloren/-/issues/1464 "Issue #1464")
	- Discovered that the issue doesn't occur when running on the sway desktop. So it could possibly be an issue with or in conjunction with the GNOME desktop.
	- I asked for some help finding other Wayland-native games to troubleshoot this in the official Discord
* Browsed some of the dev discussion in the Discord
* Started looking at the glider-related code to figure out how an option to invert flight controls might be implemented

## Week 8 - Mar 08-14 2022:
* ...

