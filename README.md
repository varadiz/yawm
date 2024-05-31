# yawm

yet another window manager

Jump between windows in Ubuntu.

Ulauncher plugins for window switching are deprecated and/or do not work,
Kupfer did not properly find my terminal and kept opening a new window,
so decided to roll my own.

It uses `wmctrl` under the hood. You should have it installed.

`sudo apt install wmctrl` or similar.

## about wmctrl

commands we use

* `wmctrl -l` List windows
* `wmctrl -R {applicationname}` Activate an application window

## GUI

We use GTK4 rust bindings for the GUI.
