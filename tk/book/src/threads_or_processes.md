# Threads or Processes

Sometimes it's either impossible or impractical to break up a long-running
computation into discrete pieces that each run quickly. Or you may be using a
library that doesn't support asynchronous operations. Or, like Python's asyncio,
it doesn't play nice with Tk's event loop. In cases like these, to keep your Tk
GUI responsive, you'll need to move those time-consuming operations or library
calls out of your event handlers and run them somewhere else. Threads, or even
other processes, can help with that.

Running tasks in threads, communicating with them, etc., is beyond the scope of
this tutorial. However, there are some restrictions on using Tk with threads
that you should be aware of. The main rule is that you must only make Tk calls
from the thread where you loaded Tk.

It can be even more complicated. The Tcl/Tk libraries can be built either with
or without thread support. If you have more than one thread in your application,
make sure you're running in a threaded build. If you're unsure, check the Tcl
variable `tcl_platform(threaded)`; it should be 1, not 0.

> Most everyone should be running threaded builds. The ability to create
non-threaded builds in Tcl/Tk is likely to go away in future. If you're using a
non-threaded build with threaded code, consider this a bug in your application,
not a challenge to make it work.
