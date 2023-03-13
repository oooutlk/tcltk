# Installing Tk On Linux

Pretty much all Linux distributions have Tcl/Tk packages available via their
package managers, e.g., apt. Usually there are a variety of packages, providing
libraries, command-line tools, development options if you're building
extensions, and many more. On Ubuntu and many other distributions,
`apt install tk8.6` should be enough.

By the way, you need to install `pkg-config` to compile the tcl/tk crates.
