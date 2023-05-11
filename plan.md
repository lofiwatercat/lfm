# Plan for lovely file manager

1. Take an argument, or default to current directory

Implement tabs into the program.
  * Tabs have the path to the current directory and vector of entries as DirEntry
  * Need to draw them, add a draw impl
  * Need to be able to create them given a path, and take care of the case where it's not a directory
  * Need to be able to draw the child_tab to the right side of the primary tab

Done - Draw children as we scroll through the entries
  * If the current row is a directory, then we draw the secondary tab
  * Need to fix cursor positioning when drawing
    - Currently, drawing a secondary tab doesn't move it back, causing problems when moving down and selecting another entry
  * Need to clear the secondary tab as we scroll

DONE - Refactor highlight
  * Need to to highlight where the tab actually is

DONE - Printing entries
  * Need to put directories first at the top of the list
  * Need to hide hidden files by default. Add option later to view them. 

TODO - Allow horizontal movement
  * Add a current_entry to show the currently selected entry.
    - allows for horizontal movement to remember previously seleected
  * Update primary tab, secondary tab, and parent tab
  * Need horizontal movement back

DONE - Refactor vertical movement
  * Update current_entry and highlight the line

TODO - Reorganize code
  * Break things up into modules

DONE - Refactor highlight (again)
  * Add it as an implementation for tab

TODO - Refactor loop
  * Primary tab -> Current tab
  * Current tab will be updated with horizontal movement

TODO - Parent tab
  * Much smaller than primary and secondary tab

