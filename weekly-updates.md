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
* Browsed some of Veloren's world-simulation code
	- encountered a `Box<T>` and looked it up in the Rust Book
* Looked further into inverting glider controls.
	- Maybe a way to do this is to make use of the already-existing game setting that inverts the mouse y-axis. 
	- Asked for help in the Veloren dev Discord
		+ A couple developers got back to me (including a core dev) and recommended starting with `voxygen/src/session/mod.rs` to look for where the regular mouse invert happens and create a check that inverts the mouse when gliding.
		
## Week 9 - Mar 22-28 2022:
* Officially requested (and received!) access to the Veloren official dev workgroup on GitLab as well as the dev repository on GitLab. This will allow me to create a branch for my work so community members can review it and provide suggestions.
* Created a feature branch on the Veloren dev repository and pushed a commit with my glider invert progress there.
	- Here is a link to the first commit: [Commit c0fada91](https://gitlab.com/veloren/dev/veloren/-/commit/c0fada9174cd1ae1724c12ceaeeacfb9d20f9a7d?view=inline "GitLab - Experimenting with glider y-axis invert (Commit c0fada91)")

## Week 10 - Mar 29-Apr 04 2022:
* Asked the Veloren dev Discord community for help figuring out an error with my glider code, providing a link to the commit I made on my feature branch on the dev repository.
* Got a response from a core developer and have been looking into the advice they provided.
* Have been tracing existing code to figure out how to get mine to work.
	- I might need to add a mouse invert toggle function to `client/src/lib.rs`
* ...
