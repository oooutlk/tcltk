# Customizing the Display

There are many aspects of how the treeview widget is displayed that we can
customize. We've already seen some of them, such as the text of items, fonts and
colors, names of column headings, and more. Here are a few additional ones.


* Specify the desired number of rows to show using the `height` widget configuration option.
* Control the width of each column using the column's `width` or `minwidth`
  options. The column holding the tree can be accessed with the symbolic name
  `#0`. The overall requested width for the widget is based on the sum of the
  column widths.
* Choose which columns to display and the order to display them in using the
  `displaycolumns` widget configuration option.
* You can optionally hide one or both of the column headings or the tree itself
  (leaving just the columns) using the `show` widget configuration option
  (default is "tree headings" to show both).
* You can specify whether a single item or multiple items can be selected by
  users via the `selectmode` widget configuration option, passing `browse`
  (single item), `extended` (multiple items, the default), or `none`. 
