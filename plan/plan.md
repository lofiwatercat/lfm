# Plan for lovely file manager

## Overview of the data structures

The *Tab* is the primary structure which displays the directory contents. It contains the
  * dir_path
    - its a path::PathBuf
  * parent_path
    - its the PathBuf of the parent directory
  * parent_tab
    - Option because it isn't created until you go up a directory
  * child_tabs
    - Option because there may not be any child directories
  * entries
    - the directory contents, including files and directories
    - It's a vec of PathBuf
  * entries_str
    - vector of stringified entries
  * current_entry_index
    - index of the currently selected item, for persistence reasons
  * status
    - determines whether the tab should be the primary, secondary, or parent

## Overview of the files and directory structure
main.rs
  * Currently contains the main loop of the program
tabs.rs
  * Contains the tab data structure
cleanup.rs
  * Contains logic to clean up after the program

DONE - Implement tabs into the program.
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

DONE - Allow horizontal movement
  * Add a current_entry to show the currently selected entry.
    - allows for horizontal movement to remember previously seleected
  * Update primary tab, secondary tab, and parent tab
  * Need horizontal movement back

DONE - Refactor vertical movement
  * Update current_entry and highlight the line

DONE - Reorganize code
  * Break things up into modules

DONE - Refactor highlight (again)
  * Add it as an implementation for tab

DONE - Refactor entries into FileEntries and DirEntries

TODO - Fix horizontal movement
  * Moving up to the parent directory has a few steps
    1. clear the primary and child tab
    2. update primary's parent tab
    3. Make the primary tab equal to the primary tab's parent tab
    4. draw the parent and child tabs

TODO - Clean up main loop and break it up into functions
  * create functions that correspond with the movements, like move_down
    - should they be a function in main or a tab function?
    - made them a tab function

TODO - Fix going back too far breaking

TODO - REFACTOR tabs to be persistent. Like linked nodes
