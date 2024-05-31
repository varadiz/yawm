# yawm

yet another window manager

Jump between windows in Ubuntu.

Ulauncher plugins for window switching are deprecated and/or do not work,
Kupfer did not properly find my terminal and kept opening a new window,
so decided to roll my own.

It uses `wmctrl` under the hood. You should have it installed.

## Usage

`cargo run`

## Dependencies

### wmctrl

`sudo apt install wmctrl` or similar.

* `wmctrl -l` List windows
* `wmctrl -R {applicationname}` Activate an application window

### gtk4

`sudo apt install libgtk-4-dev build-essential`

We use GTK4 rust bindings for the GUI.

