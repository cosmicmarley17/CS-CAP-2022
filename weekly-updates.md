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

## Week 11 - Apr 05-Apr 11 2022:
* Took a break from glider code and learned how to add a sprite to the game
    - From here I might try adding a unique model for it, making it collectible and creating an item for it with its own properties and functions.

## Week 12 - Apr 12-Apr 18 2022:
* Explored sprites and items, figured out how to make the sprite I added give an item when you interact with it. Up next is figuring out how to make my own item that corresponds to the sprite.
* Added an item that corresponds to the flower sprite I added to the game. Now you receive an item when you harvest the flower.
* I also created a crafting recipe for a potion you can craft using the flower as an ingredient.
	- Currently it has a health potion effect, but I'm going to experiment with creating my own custom potion effect that is applied when you consume the potion.
* Spent some time working on a virtual mancala terminal game (WIP) to practice Rust coding outside of Veloren. [Link to my work here](https://github.com/cosmicmarley17/mancala "mancala repository on GitHub")
	- I took inspiration from Veloren's frontend-agnostic game design in the making of my long-term plans for my project.
	- My mancala project made me a lot more familiar with Rust's `Result` and `Option<T>` types, and working with organizing data and functionality with structs and struct methods.

## Week 13 - Apr 19-Apr 25 2022:
* Searched the Veloren codebase to find where potion effects are defined, so I can add my own.
* Followed a trail in the code leading from potions to `common/systems/src/buff.rs` and `common/src/comp/buff.rs` and the enum `BuffKind` to the enum `BuffEffect` to the `Stats` struct in `common/src/comp/stats.rs`. 
* If I want to get really deep into creating effects, this is where I'll go. I think I'll try to create a jump height boosting potion later, which will require creating a new value on the `Stats` struct. 
* For now, I'll focus on creating a new effect for my potion that simply combines existing effects.
* Added a new `BuffKind` variant that increases player attack speed and reduces damage taken by the player. I also applied this buff to the custom potion I added.
* The familiarity with structs, struct methods, and match statements that I gained working on my [mancala project](https://github.com/cosmicmarley17/mancala "my mancala repository on GitHub") was very transferable to the Veloren codebase and made it easier to understand the program organization and add my own content to the game.
* Learned and practiced squashing commits in git

## Week 14 - Apr 26-Apr 29 2022:
* Fixed compiler errors from adding my potion `BuffKind`. (I needed to handle my `BuffKind` on `match` statements)
* The potion works! I had to adjust the potion strength because I initially set it absurdly high.
* I added a [diff file](./code/veloren-new-potion-diff.txt "Changes made for custom potion and sprite") generated by git to the code folder in this repository that outlines the code I changed or added to create the custom potion and sprite. 
* Someone on the Veloren dev team saw the work I was doing on my branch, and told me they're looking to add a potion that temporarily increases health, and I could tackle that if I want. Now that I've had some practice creating my experiment potion, I'll look into implementing this feature! 

