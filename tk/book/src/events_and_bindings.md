# Events and Bindings

One very cool thing we can do is define event bindings on tags. That allows us
to easily do things like recognize mouse clicks on particular ranges of text and
popup a menu or dialog in response. Different tags can have different bindings.
This saves the hassle of sorting out questions like "what does a click at this
location mean?". Bindings on tags are implemented using the `tag_bind` method:

```rust,no_run
txt.tag_bind( "important", event::button_press_1(), popup_important_menu )?;
```

Widget-wide event bindings continue to work as they do for every other widget,
e.g., to capture a mouse click anywhere in the text. Besides the normal
low-level events, the text widget generates a `Modified` virtual event whenever
a change is made to the content of the widget, and a `Selection` virtual event
whenever there is a change made to which text is selected.
