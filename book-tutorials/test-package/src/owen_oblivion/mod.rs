// module definition (declaration?) for OwenOblivion

// an inline submodule for this module
mod owen_inline {

}

// declare a submodule of 'owen_oblivion' named 'owen_submod_one' which would be found in either of:
// src/owen_oblivion/owen_submod_one.rs
// src/owen_oblivion/owen_submod_one/mod.rs
mod owen_submod_one;

// declare a submodule of 'owen_oblivion' named 'owen_submod_two' which would be found in either of:
// src/owen_oblivion/owen_submod_two.rs
// src/owen_oblivion/owen_submod_two/mod.rs
mod owen_submod_two;
