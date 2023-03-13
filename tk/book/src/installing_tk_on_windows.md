# Installing Tk On Windows

On Windows, the easiest way to get Tcl/Tk onto your machine is to install the
"ActiveTcl" distribution from ActiveState. In your web browser, go to
[activestate](http://www.activestate.com), and follow along the links to
download the Community Edition of ActiveTcl for Windows. Make sure you're
downloading an 8.6.x version.  Note that you will need to create an account with
ActiveState (no cost) to download it.

Run the installer, and follow along. You'll end up with a fresh install of
ActiveTcl, usually located in `C:\ActiveTcl`. From a command prompt, you should
then be able to run a Tcl/Tk 8.6 shell via:

```
% C:\ActiveTcl\bin\wish
```

This should pop up a small window titled "wish", which will contain your
application. A second, larger window titled "Console" is where you can type in
Tcl/Tk commands. To verify the exact version of Tcl/Tk that you are running,
type the following:

```
% info patchlevel
```

We want this to be returning something like '8.6.9'.

Type "exit" in the console window to exit. You may also want to add
`C:\ActiveTcl\bin` to your PATH environment variable.

Note: verified install using ActiveTcl 8.6.9.8609-2 on Windows 10.
