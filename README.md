# binunit

A simple testing framework for c.

### use

binunit recursively scans the current directory for c functions
that are marked with a `///[test]` comment, and generates
an executable to call each of these functions. By default,
binunit will recursively search the current directory
and attempt to link to any object files that contain
your test functions. If the definition of your test
function cannot be found by the linker, that test
fails and informs you that your test could not
be found.  

binunit assumes that object/library files for your tests
are available, so it depends on the project's build being
up to date. Suggested use is to call binunit from
a makefile target that depends on your build.

### build

binunit uses cargo, Rust's build tool.

`cargo build`  
`cargo run`  
`cargo test`