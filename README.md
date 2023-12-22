Find
----

This is a Rust implementation of the standard UNIX program `find`, a command
line tool for searching for files in a file system. It's far from complete,
but it has the basic infrastructure implemented (finding and printing files),
what's left is adding more filters and options that the standard `find` tool
supports. The interface resembles the original one, except only a limited
set of flags is supported at the moment.

Build
-----

Simply use cargo:

    $ cargo build
    
Usage
-----

    $ find /path/to/directory
    
The program then recursively lists all the files and directories
within given location. No filtering or formatting flags are
implemented yet.
