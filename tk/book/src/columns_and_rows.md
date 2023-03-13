# Columns and Rows

In grid, widgets are assigned a `column` number and a `row` number. These
indicate each widget's position relative to other widgets. All widgets in the
same column will be above or below each other. Those in the same row will be to
the left or right of each other.

Column and row numbers must be positive integers (i.e., 0, 1, 2, ...). You don't
have to start at 0 and can leave gaps in column and row numbers (e.g., column 1,
2, 10, 11, 12, 20, 21). This is useful if you plan to add more widgets in the
middle of the user interface later.

The width of each column will vary depending on the width of the widgets
contained within the column. Ditto for the height of each row. This means when
sketching out your user interface and dividing it into rows and columns, you
don't need to worry about each column or row being equal width.
