# Plan for lovely file manager

1. Take an argument, or default to current directory

Implement tabs into the program.
  * Tabs have the path to the current directory and vector of entries as DirEntry
  * Need to draw them, add a draw impl
  * Need to be able to create them given a path, and take care of the case where it's not a directory
  * Need to be able to draw the child_tab to the right side of the primary tab

Draw children as we scroll through the entries
  * If the current row is a directory, then we draw the secondary tab
  * Need to fix cursor positioning when drawing
    - Currently, drawing a secondary tab doesn't move it back, causing problems when moving down and selecting another entry
  * Need to clear the secondary tab as we scroll

Refactor highlight
  * Need to to highlight where the tab actually is

When printing, put directories first. Use a sort by function?
