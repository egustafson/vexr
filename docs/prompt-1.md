'vexr' is an app for reminding and alerting users of important events.
The name is derived from "vex" and is an intentional misspelling of
vexer.  The applications tag line is "Vexr: proactively annoying for
your own good."

technology stack:
* Language: Rust
* Framework: Tauri
* Supports: Windows, Linux/Gnome, MacOS

# Description

vexr is an application that is out of sight in the OS's tray most of
the time.  If the user needs to proactively interact with vexr they
will start with the tray icon.  The tray icon can have a context menu
and it may open up dialog windows for user interaction.

The principal purpose of vexr will be to interrupt the user with a
dialog or toast notification.  Different events may dictate dialog vs
toast notification.  Some events may require user feedback.

The first iteration will simply act as a reminder app.  Users should
be able to create reminders that are either a single ad-hoc reminder
at a certain day and time or a recurring reminder.  A context menu
from the tray icon should contain an option of "set reminder" and when
chosen a dialog box should be displayed that allows the user to
reminder.

An additional context menu from the tray icon should allow listing and
editing the current reminders.

Reminders should be stored locally.

Future iterations will add the capability of receiving asynchronous
alerts from external components.

All notifications should be acknowledged by the user before closing
and being removed from the screen.
