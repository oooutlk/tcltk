# Separator

A second approach to grouping widgets in one display is to place a thin
horizontal or vertical rule between groups of widgets; often, this can be more
space-efficient than using white space, which may be relevant for a tight
display. Tk provides a simple *separator* widget for this purpose.

|                  Separator widgets                  |
| :-------------------------------------------------: |
| ![Separator widgets.](./images/w_separator_all.png) |

Separators are created using the `add_ttk_separator` method:

```rust,no_run
parent.add_ttk_separator( "s" -orient("horizontal") )?;
```

The `orient` option may be specified as either `horizontal` or `vertical`.
