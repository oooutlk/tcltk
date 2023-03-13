# Installing Tk On FreeBSD

Tcl 8.6/Tk 8.6 are available both in ports tree and package repository. To
install Tk 8.6 by downloading binaries from repository, just run
`pkg install -y tk86` in the shell. To install from source, run
`make -C /usr/ports/x11-toolkits/tk86 install`.

By the way, you need to install `pkg-config` to compile the tcl/tk crates.
