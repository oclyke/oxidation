# test package
this package is meant for me to understand how a package (this package, created with ```cargo new test-package```) can hold multiple crates of various kinds:

* binary crate(s)
  * optionally the default binary crate, in ```src/main.rs```, named after the package
  * optionally other binary crates, in ```*.rs``` files within ```bin```
* library crate
  * at most one library crate, in ```src/lib.rs```, named after the package