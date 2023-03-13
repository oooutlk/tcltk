# Introduction

This tutorial will quickly get you up and running with Tk 8.6 from Rust. It
provides all the essentials about core Tk concepts, the various widgets,
layout, events and more that you need for your application.

It's not a reference guide. It's not going to cover everything, just the
essentials you need in 95% of applications. The rest you can find in
[reference documentation](https://tcl.tk/man/tcl8.6/TkCmd/contents.htm).

Tk has, for most of its lifetime, gotten a bad rap, to put it mildly. Some of
this has been well deserved, most of it not so much. Like any GUI tool, it can
be used to create absolutely terrible looking and outdated user interfaces.
Still, with the proper care and attention, it can also be used to develop
spectacularly good ones. Most people know about the crappy ones; most of the
good ones people don't even know are done in Tk. In this tutorial, we're going
to focus on what you need to build good user interfaces.