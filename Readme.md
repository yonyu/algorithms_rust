# Implement some algorithms

## Directories

- arrays/ - Algorithms for arrays

- dynamic_programming/ - Algorithms for dynamic programming

- grids/ - Algorithms for 2D arrays

- linked_lists/ - Algorithms for linked lists

- lists/ - Algorithms for lists

- strings/ - Algorithms for strings

A crate attribute is an attribute (#[...]) that applies to the enclosing context (#![...]). This attribute must be added to the top of your crate root, thus the context is the crate itself:

        #![attribute_name]
        #![attribute_name(arg1, ...)]

If you are creating

    a library — the crate root will be a file called lib.rs
    an application — the crate root would be the primary .rs file you build. In many cases, this will be called main.rs
    an integration test - the crate root is each file in tests/
    an example - the crate root is each file in examples/
