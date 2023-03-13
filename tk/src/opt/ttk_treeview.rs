def_widget_opts! {
    TtkTreeviewOpt: (
        // standard
        TkClass,
        TkCursor,
        TkPadding,
        TkStyle,
        TkTakeFocus,
        TkXScrollCommand,
        TkYScrollCommand,

        // widget-specific
        TkColumns,
        TkDisplayColumns,
        TkHeight,
        TkSelectMode,
        TkShow,
    ),
    TtkTreeviewColumnOpt: (
        TkAnchor,
        TkId,
        TkMinWidth,
        TkStretch,
        TkWidth,
    ),
    TtkTreeviewHeadingOpt: (
        TkAnchor,
        TkCommand,
        TkImage,
        TkText,
    ),
    TtkTreeviewItemOpt: (
        TkText,
        TkImage,
        TkValues,
        TkOpen,
        TkTags,
    ),
    TtkTreeviewTagOpt: (
        TkForeground,
        TkBackground,
        TkFont,
        TkImage,
    ),
}
