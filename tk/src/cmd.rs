use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::*;
use crate::error::*;
use crate::{
    opt,
    types::{TkCaret, TkWindowingSystem},
    query::CreatedWidgets,
};

use heredom::{DomForest, Visit};

use tcl::{
    CodeToResult,
    error::{
        DeError,
        InterpError,
        NotList,
    },
    from_obj,
};

use tuplex::{
    Convert,
    ConvertTuple,
    HomoTuple,
    IntoHomoTuple,
    NonZeroLen,
    PushFront,
    PushBack,
    PopFront,
};

use std::{
    collections::HashMap,
    os::raw::{c_double, c_longlong},
    path::PathBuf,
    ptr::null_mut,
};

pub(crate) const WIDGET_NAMES : [&'static str; 36] = [
    "button",
    "canvas",
    "checkbutton",
    "entry",
    "frame",
    "label",
    "labelframe",
    "listbox",
    "menu",
    "menubutton",
    "message",
    "panedwindow",
    "radiobutton",
    "scale",
    "scrollbar",
    "spinbox",
    "text",
    "toplevel",
    "ttk::button",
    "ttk::checkbutton",
    "ttk::combobox",
    "ttk::entry",
    "ttk::frame",
    "ttk::label",
    "ttk::labelframe",
    "ttk::menubutton",
    "ttk::notebook",
    "ttk::panedwindow",
    "ttk::progressbar",
    "ttk::radiobutton",
    "ttk::scale",
    "ttk::scrollbar",
    "ttk::separator",
    "ttk::sizegrip",
    "ttk::spinbox",
    "ttk::treeview",
];

pub(crate) fn find_widget_name( name: &str ) -> Option<&'static str> {
    WIDGET_NAMES.iter().find( |&widget_name| widget_name == &name ).copied()
}

/// The "base class" to which all Tk widgets `deref()`.
#[derive( Copy, Clone )]
pub struct Widget<Inst:TkInstance> {
    pub(crate) path : &'static str,
    pub(crate) inst : Inst,
    pub(crate) mark : NotSendSync,
}

impl<Inst:TkInstance> From<Widget<Inst>> for Obj {
    fn from( widget: Widget<Inst> ) -> Obj {
        widget.path.into()
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    pub(crate) fn tk( &self ) -> Tk<Inst> { Tk::from_inst( self.inst )}

    pub fn path( &self ) -> &'static str { self.path }

    /// Reference to a Tk widget by its widget name.
    pub fn from_name( name: &str, inst: Inst ) -> InterpResult<Option<Self>> {
        let tk = Tk::from_inst( inst );
        let widget_exists = tk.eval(( "winfo", "exists", name ))?;
        if tk.boolean( widget_exists )? {
            Ok( Some( Widget{ path: Tk::<Inst>::make_or_get_path( name ), inst, mark: NOT_SEND_SYNC }))
        } else {
            Ok( None )
        }
    }

    pub(crate) fn from_name_unchecked( name: &str, inst: Inst ) -> Self {
        Widget{ path: Tk::<Inst>::make_or_get_path( name ), inst, mark: NOT_SEND_SYNC }
    }

    pub(crate) fn do_configure<Opts>( &self, opts: PathOptsWidgets<Opts,()> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );
        command.push( self.path.into() );
        command.push( "configure".into() );

        append_opts( &mut command, opts.opts );
        self.tk().run( command )
    }

    pub(crate) fn cget( &self, name: &'static str ) -> InterpResult<Obj> {
        self.tk().eval(( self.path, "cget", name ))
    }

    pub(crate) fn add<Opts>( &self, name: &'static str, path_opts: PathOptsWidgets<Opts,()> ) -> InterpResult<Widget<Inst>>
        where Opts: IntoHomoTuple<OptPair>
    {
        let path = self.tk().next_path( &self.path, path_opts.path );
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );
        command.push( name.into() );
        command.push( path.as_str().into() );

        append_opts( &mut command, path_opts.opts );
        self.tk().eval( command )?;

        Ok( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.inst, mark: NOT_SEND_SYNC })
    }

    /// Adds child widget(s) containing hierachical trees of children widgets, including geometry managers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// fn main() -> TkResult<()> {
    ///     let tk = make_tk!()?;
    ///     let root = tk.root();
    ///
    ///     root.add_widgets(
    ///         -pack(
    ///             -button( -text("hello,tk!") -command("") )
    ///             -pack( -button( -text("oops") ))
    ///             -frame( -pack(
    ///                 -button( -text("exit") -command("destroy .") )
    ///                 -option_menu( "optmenu1", "my_opt", &[ "ALPHA", "BETA", "GAMMA" ])
    ///             ))
    ///         )
    ///         -pack( -expand(1)
    ///             -button( -text("demo") )
    ///             -button( -text("demo") )
    ///             -option_menu( "optmenu2", "your_opt", &[ "alpha", "beta", "gamma" ])
    ///         )
    ///     )?;
    ///
    ///     Ok( main_loop() )
    /// }
    /// ```
    pub fn add_widgets<Widgs,Shape>( &self, path_widgets: PathOptsWidgets<(),Widgs> )
        -> InterpResult<CreatedWidgets<Inst>>
        where Widgs: ConvertTuple
            , <Widgs as ConvertTuple>::Output: DomForest::<(&'static str,&'static str),OptPair,Shape>
    {
        struct GeometryFrame {
            command : Vec<Obj>,
            slaves  : Vec<Obj>,
            depth   : usize,
        }

        let mut geometry_stack = Vec::<GeometryFrame>::new();

        let mut current_path = self.path.to_string();
        let mut command = Vec::<Obj>::new();
        let mut is_geometry_manager = false;

        let widgets = path_widgets.widgets.convert_tuple();

        let mut created_widgets = CreatedWidgets::new( self.path );
        let mut is_branch = false;

        DomForest::<(&'static str,&'static str),OptPair,Shape>::try_preorder( widgets, &mut |visit| -> InterpResult<()> {
            match visit {
                Visit::Branch( (cmd, path) ) => {
                    is_branch = true;
                    is_geometry_manager = match cmd {
                        "pack" | "grid" | "place" => true,
                        _ => false,
                    };
                    command.push( cmd.into() );
                    if !is_geometry_manager {
                        current_path = self.tk().next_path( &current_path, path );
                        geometry_stack.last_mut().map( |last| {
                            last.depth += 1;
                            if last.depth == 1 {
                                last.slaves.push( current_path.as_str().into() );
                            }
                        });
                        command.push( current_path.as_str().into() );
                    }
                },
                Visit::Leaf( (cmd, path) ) => {
                    is_branch = false;
                    current_path = self.tk().next_path( &current_path, path );
                    geometry_stack.last_mut().map( |last| {
                        if last.depth == 0 {
                            last.slaves.push( current_path.as_str().into() );
                        }
                    });
                    command.push( cmd.into() );
                    command.push( current_path.as_str().into() );
                },
                Visit::Frame => {
                    let mut is_geometry_manager = false;
                    geometry_stack.last_mut().map( |last| {
                        if last.depth == 0 {
                            is_geometry_manager = true;
                        } else {
                            last.depth -= 1;
                        }
                    });
                    if is_geometry_manager {
                        let mut top = geometry_stack.pop().unwrap();
                        let mut slaves = Vec::new();
                        mem::swap( &mut slaves, &mut top.slaves );
                        let mut iter = top.command.into_iter();
                        let mut geo_command = vec![ iter.next().unwrap() ];
                        geo_command.extend( slaves );
                        geo_command.extend( iter );
                        self.tk().run( geo_command )?;
                    } else {
                        current_path = Widget::<Inst>::compute_parent_path( &current_path );
                    }
                },
                Visit::AttrsStart( _len ) => (),
                Visit::Attr( opt_pair ) => {
                    if opt_pair.name.len() > 0 {
                        command.push( opt_pair.name.into() );
                    }
                    command.push( opt_pair.value );
                },
                Visit::AttrsEnd => {
                    let mut current_command = Vec::<Obj>::new();
                    mem::swap( &mut command, &mut current_command );
                    if is_geometry_manager {
                        geometry_stack.push( GeometryFrame{ command: current_command, slaves: Vec::new(), depth: 0 });
                        is_geometry_manager = false;
                    } else {
                        let widget_name = current_command[0].to_string();
                        self.tk().run( current_command )?;
                        if let Some( name ) = find_widget_name( &widget_name ) {
                            created_widgets.widgets.push( UpcastableWidget {
                                widget : Widget::from_name_unchecked( &current_path, self.tk().inst ),
                                name   ,
                            });
                        }
                        if !is_branch {
                            // No `Visit::Frame` for `Visit::Leaf`
                            current_path = Widget::<Inst>::compute_parent_path( &current_path );
                        }
                    }
                },
            }
            Ok(())
        })?;
        Ok( created_widgets )
    }

    pub(crate) fn compute_parent_path( path: &str ) -> String {
        match path.rfind('.') {
            Some( last_dot ) => {
                let parent = &path[ 0..last_dot ];
                if parent.len() == 0 {
                    ".".to_owned()
                } else {
                    parent.to_owned()
                }
            },
            None => unreachable!(), // Tk::next_path() provides some dot(s).
        }
    }
}

impl<Inst:TkInstance> TkPackSlave  for Widget<Inst> {}
impl<Inst:TkInstance> TkGridSlave  for Widget<Inst> {}
impl<Inst:TkInstance> TkPlaceSlave for Widget<Inst> {}

pub struct PathOptsWidgets<Opts,Widgs> {
    pub(crate) path    : &'static str,
    pub(crate) opts    : Opts,
    pub(crate) widgets : Widgs,
}

impl From<()> for PathOptsWidgets<(),()> {
    fn from( _: () ) -> Self {
        PathOptsWidgets{ path: "", opts: (), widgets: () }
    }
}

impl From<&'static str> for PathOptsWidgets<(),()> {
    fn from( path: &'static str ) -> Self {
        PathOptsWidgets{ path, opts: (), widgets: () }
    }
}

macro_rules! def_hyphen_notation {
    ($ty:ident) => {
        impl<T> std::ops::Neg for $ty<T> {
            type Output = PathOptsWidgets<(), ($ty<T>,)>;

            fn neg( self ) -> Self::Output {
                PathOptsWidgets {
                    path   : "",
                    opts   : (),
                    widgets: (self,),
                }
            }
        }

        impl<T> std::ops::Sub<$ty<T>> for &'static str {
            type Output = PathOptsWidgets<(), ($ty<T>,)>;

            fn sub( self, rhs: $ty<T> ) -> Self::Output {
                PathOptsWidgets {
                    path   : self,
                    opts   : (),
                    widgets: (rhs,),
                }
            }
        }

        impl<Opts,Widgs,T> std::ops::Sub<$ty<T>> for PathOptsWidgets<Opts,Widgs>
            where Widgs: PushBack<$ty<T>>
        {
            type Output = PathOptsWidgets<Opts, <Widgs as PushBack<$ty<T>>>::Output>;

            fn sub( self, rhs: $ty<T> ) -> Self::Output {
                PathOptsWidgets {
                    path   : self.path,
                    opts   : self.opts,
                    widgets: self.widgets.push_back( rhs ),
                }
            }
        }
    };
}

macro_rules! def_functions {
    ($($function:ident $trait:ident;)*) => {$(
        pub fn $function<Input: $trait>( input: Input ) -> <Input as $trait>::Output {
            <Input as $trait>::output( input )
        }

        pub trait $trait {
            type Output;
            fn output( self ) -> Self::Output;
        }
    )*}
}

def_functions! {
    // geometry managers
    pack                TkPackFn                ;
    grid                TkGridFn                ;
    place               TkPlaceFn               ;

    // tk widgets
    button              TkButtonFn              ;
    canvas              TkCanvasFn              ;
    checkbutton         TkCheckbuttonFn         ;
    entry               TkEntryFn               ;
    frame               TkFrameFn               ;
    label               TkLabelFn               ;
    labelframe          TkLabelframeFn          ;
    listbox             TkListboxFn             ;
    menu                TkMenuFn                ;
    menubutton          TkMenubuttonFn          ;
    message             TkMessageFn             ;
    panedwindow         TkPanedwindowFn         ;
    radiobutton         TkRadiobuttonFn         ;
    scale               TkScaleFn               ;
    scrollbar           TkScrollbarFn           ;
    spinbox             TkSpinboxFn             ;
    text                TkTextFn                ;
    toplevel            TkToplevelFn            ;

    // ttk widgets
    ttk_button          TtkButtonFn             ;
    ttk_checkbutton     TtkCheckbuttonFn        ;
    ttk_combobox        TtkComboboxFn           ;
    ttk_entry           TtkEntryFn              ;
    ttk_frame           TtkFrameFn              ;
    ttk_label           TtkLabelFn              ;
    ttk_labelframe      TtkLabelframeFn         ;
    ttk_menubutton      TtkMenubuttonFn         ;
    ttk_notebook        TtkNotebookFn           ;
    ttk_panedwindow     TtkPanedwindowFn        ;
    ttk_progressbar     TtkProgressbarFn        ;
    ttk_radiobutton     TtkRadiobuttonFn        ;
    ttk_scale           TtkScaleFn              ;
    ttk_scrollbar       TtkScrollbarFn          ;
    ttk_separator       TtkSeparatorFn          ;
    ttk_sizegrip        TtkSizegripFn           ;
    ttk_spinbox         TtkSpinboxFn            ;
    ttk_treeview        TtkTreeviewFn           ;

    // options
    above                       TkAboveFn                       ;
    accelerator                 TkAcceleratorFn                 ;
    activebackground            TkActiveBackgroundFn            ;
    activebitmap                TkActiveBitmapFn                ;
    activeborderwidth           TkActiveBorderWidthFn           ;
    activedash                  TkActiveDashFn                  ;
    activefill                  TkActiveFillFn                  ;
    activeforeground            TkActiveForegroundFn            ;
    activeimage                 TkActiveImageFn                 ;
    activeoutline               TkActiveOutlineFn               ;
    activeoutlinestipple        TkActiveOutlineStippleFn        ;
    activerelief                TkActiveReliefFn                ;
    activestipple               TkActiveStippleFn               ;
    activestyle                 TkActiveStyleFn                 ;
    activewidth                 TkActiveWidthFn                 ;
    after                       TkAfterFn                       ;
    align                       TkAlignFn                       ;
    alpha                       TkAlphaFn                       ;
    anchor                      TkAnchorFn                      ;
    angle                       TkAngleFn                       ;
    arrow                       TkArrowFn                       ;
    arrowshape                  TkArrowShapeFn                  ;
    ascent                      TkAscentFn                      ;
    aspect                      TkAspectFn                      ;
    autoseperators              TkAutoSeperatorsFn              ;
    background                  TkBackgroundFn                  ;
    bd                          TkBdFn                          ;
    before                      TkBeforeFn                      ;
    bg                          TkBgFn                          ;
    bgstipple                   TkBgStippleFn                   ;
    bigincrement                TkBigIncrementFn                ;
    bitmap                      TkBitmapFn                      ;
    blockcursor                 TkBlockCursorFn                 ;
    bordermode                  TkBorderModeFn                  ;
    borderwidth                 TkBorderWidthFn                 ;
  //button                      TkButtonFn                      ;
    buttonbackground            TkButtonBackgroundFn            ;
    buttoncursor                TkButtonCursorFn                ;
    buttondownrelief            TkButtonDownReliefFn            ;
    buttonuprelief              TkButtonUpReliefFn              ;
    capstyle                    TkCapStyleFn                    ;
    channel                     TkChannelFn                     ;
    class                       TkClassFn                       ;
    closeenough                 TkCloseEnoughFn                 ;
    colormap                    TkColorMapFn                    ;
    colormode                   TkColorModeFn                   ;
    column                      TkColumnFn                      ;
    columns                     TkColumnsFn                     ;
    columnspan                  TkColumnSpanFn                  ;
    columnbreak                 TkColumnBreakFn                 ;
    command                     TkCommandFn                     ;
    compositingrule             TkCompositingruleFn             ;
    compound                    TkCompoundFn                    ;
    confine                     TkConfineFn                     ;
    confirmoverwrite            TkConfirmOverwriteFn            ;
    container                   TkContainerFn                   ;
    count                       TkCountFn                       ;
    create                      TkCreateFn                      ;
    cursor                      TkCursorFn                      ;
    dash                        TkDashFn                        ;
    dashoffset                  TkDashOffsetFn                  ;
    data                        TkDataFn                        ;
    default_                    TkDefaultFn                     ;
    defaultextension            TkDefaultExtensionFn            ;
    delta                       TkDeltaFn                       ;
    descent                     TkDescentFn                     ;
    detail                      TkDetailFn                      ;
    digits                      TkDigitsFn                      ;
    direction                   TkDirectionFn                   ;
    disabled                    TkDisabledFn                    ;
    disabledbackground          TkDisabledBackgroundFn          ;
    disabledbitmap              TkDisabledBitmapFn              ;
    disableddash                TkDisabledDashFn                ;
    disabledfill                TkDisabledFillFn                ;
    disabledforeground          TkDisabledForegroundFn          ;
    disabledimage               TkDisabledImageFn               ;
    disabledoutline             TkDisabledOutlineFn             ;
    disabledoutlinestipple      TkDisabledOutlineStippleFn      ;
    disabledstipple             TkDisabledStippleFn             ;
    disabledwidth               TkDisabledWidthFn               ;
    displaycolumns              TkDisplayColumnsFn              ;
    elementborderwidth          TkElementBorderWidthFn          ;
    elide                       TkElideFn                       ;
    endline                     TkEndlineFn                     ;
    expand                      TkExpandFn                      ;
    exportselection             TkExportSelectionFn             ;
    extent                      TkExtentFn                      ;
    family                      TkFamilyFn                      ;
    fgstripple                  TkFgStrippleFn                  ;
    file                        TkFileFn                        ;
    filetypes                   TkFileTypesFn                   ;
    fill                        TkFillFn                        ;
    fixed                       TkFixedFn                       ;
    focus                       TkFocusFn                       ;
    font                        TkFontFn                        ;
    fontmap                     TkFontMapFn                     ;
    foreground                  TkForegroundFn                  ;
    format                      TkFormatFn                      ;
    from                        TkFromFn                        ;
    fullscreen                  TkFullScreenFn                  ;
    gamma                       TkGammaFn                       ;
    grayscale                   TkGrayscaleFn                   ;
    handlepad                   TkHandlePadFn                   ;
    handlesize                  TkHandleSizeFn                  ;
    height                      TkHeightFn                      ;
    hide                        TkHideFn                        ;
    hidemargin                  TkHideMarginFn                  ;
    highlightbackground         TkHighlightBackgroundFn         ;
    highlightcolor              TkHighlightColorFn              ;
    highlightthickness          TkHighlightThicknessFn          ;
    icon                        TkIconFn                        ;
    id                          TkIdFn                          ;
    image                       TkImageFn                       ;
    imargin1                    TkImargin1Fn                    ;
    imargin2                    TkImargin2Fn                    ;
    imargincolor                TkImarginColorFn                ;
    in_                         TkInFn                          ;
    inactiveselectbackground    TkInactiveSelectBackgroundFn    ;
    increment                   TkIncrementFn                   ;
    indicatoron                 TkIndicatorOnFn                 ;
    initialcolor                TkInitialColorFn                ;
    initialdir                  TkInitialDirFn                  ;
    initialfile                 TkInitialFileFn                 ;
    insertbackground            TkInsertBackgroundFn            ;
    insertborderwidth           TkInsertBorderWidthFn           ;
    insertofftime               TkInsertOffTimeFn               ;
    insertontime                TkInsertOnTimeFn                ;
    insertunfocussed            TkInsertUnfocussedFn            ;
    insertwidth                 TkInsertWidthFn                 ;
    invalidcommand              TkInvalidCommandFn              ;
    invcmd                      TkInvCmdFn                      ;
    ipadx                       TkIPadXFn                       ;
    ipady                       TkIPadYFn                       ;
    joinstyle                   TkJoinStyleFn                   ;
    jump                        TkJumpFn                        ;
    justify                     TkJustifyFn                     ;
    keycode                     TkKeyCodeFn                     ;
    keysym                      TkKeySymFn                      ;
  //label                       TkLabelFn                       ;
    labelanchor                 TkLabelAnchorFn                 ;
    labelwidget                 TkLabelWidgetFn                 ;
    length                      TkLengthFn                      ;
    linespace                   TkLinespaceFn                   ;
    listvariable                TkListVariableFn                ;
    maskdata                    TkMaskDataFn                    ;
    maskfile                    TkMaskFileFn                    ;
    maximum                     TkMaximumFn                     ;
    maxundo                     TkMaxUndoFn                     ;
  //menu                        TkMenuFn                        ;
  //message                     TkMessageFn                     ;
    minsize                     TkMinSizeFn                     ;
    minwidth                    TkMinWidthFn                    ;
    mode                        TkModeFn                        ;
    modified                    TkModifiedFn                    ;
    multiple                    TkMultipleFn                    ;
    mustexist                   TkMustExistFn                   ;
    name                        TkNameFn                        ;
    notify                      TkNotifyFn                      ;
    offrelief                   TkOffReliefFn                   ;
    offset                      TkOffsetFn                      ;
    offvalue                    TkOffValueFn                    ;
    onvalue                     TkOnValueFn                     ;
    opaqueresize                TkOpaqueResizeFn                ;
    open                        TkOpenFn                        ;
    orient                      TkOrientFn                      ;
    outline                     TkOutlineFn                     ;
    outlineoffset               TkOutlineOffsetFn               ;
    outlinestipple              TkOutlineStippleFn              ;
    overrelief                  TkOverReliefFn                  ;
    override_                   TkOverrideFn                    ;
    overstrike                  TkOverstrikeFn                  ;
    overstrikefg                TkOverstrikeFgFn                ;
    pad                         TkPadFn                         ;
    padding                     TkPaddingFn                     ;
    padx                        TkPadxFn                        ;
    pady                        TkPadyFn                        ;
    pageanchor                  TkPageAnchorFn                  ;
    pageheight                  TkPageHeightFn                  ;
    pagewidth                   TkPageWidthFn                   ;
    pagex                       TkPageXFn                       ;
    pagey                       TkPageYFn                       ;
    palette                     TkPaletteFn                     ;
    parent                      TkParentFn                      ;
    phase                       TkPhaseFn                       ;
  //place                       TkPlaceFn                       ;
    postcommand                 TkPostCommandFn                 ;
    proxybackground             TkProxyBackgroundFn             ;
    proxyborderwidth            TkProxyBorderWidthFn            ;
    proxyrelief                 TkProxyReliefFn                 ;
    readonlybackground          TkReadOnlyBackgroundFn          ;
    relief                      TkReliefFn                      ;
    relheight                   TkRelHeightFn                   ;
    relwidth                    TkRelWidthFn                    ;
    relx                        TkRelXFn                        ;
    rely                        TkRelYFn                        ;
    repeatdelay                 TkRepeatDelayFn                 ;
    repeatinterval              TkRepeatIntervalFn              ;
    resolution                  TkResolutionFn                  ;
    rmargin                     TkRMarginFn                     ;
    rmargincolor                TkRMarginColorFn                ;
    root                        TkRootFn                        ;
    rootx                       TkRootxFn                       ;
    rooty                       TkRootyFn                       ;
    rotate                      TkRotateFn                      ;
    row                         TkRowFn                         ;
    rowspan                     TkRowSpanFn                     ;
    sashcursor                  TkSashCursorFn                  ;
    sashpad                     TkSashPadFn                     ;
    sashrelief                  TkSashReliefFn                  ;
    sashwidth                   TkSashWidthFn                   ;
    screen                      TkScreenFn                      ;
    scrollregion                TkScrollRegionFn                ;
    selectbackground            TkSelectBackgroundFn            ;
    selectborderwidth           TkSelectBorderWidthFn           ;
    selectcolor                 TkSelectColorFn                 ;
    selectforeground            TkSelectForegroundFn            ;
    selectimage                 TkSelectImageFn                 ;
    selectmode                  TkSelectModeFn                  ;
    sendevent                   TkSendEventFn                   ;
    serial                      TkSerialFn                      ;
    setgrid                     TkSetGridFn                     ;
    settings                    TkSettingsFn                    ;
    show                        TkShowFn                        ;
    showhandle                  TkShowHandleFn                  ;
    showvalue                   TkShowValueFn                   ;
    shrink                      TkShrinkFn                      ;
    side                        TkSideFn                        ;
    size                        TkSizeFn                        ;
    slant                       TkSlantFn                       ;
    sliderelief                 TkSlideReliefFn                 ;
    sliderlength                TkSliderLengthFn                ;
    smooth                      TkSmoothFn                      ;
    spacing1                    TkSpacing1Fn                    ;
    spacing2                    TkSpacing2Fn                    ;
    spacing3                    TkSpacing3Fn                    ;
    splinesteps                 TkSplineStepsFn                 ;
    start                       TkStartFn                       ;
    startline                   TkStartlineFn                   ;
    state                       TkStateFn                       ;
    sticky                      TkStickyFn                      ;
    stipple                     TkStippleFn                     ;
    stretch                     TkStretchFn                     ;
    style                       TkStyleFn                       ;
    subsample                   TkSubsampleFn                   ;
    subwindow                   TkSubWindowFn                   ;
    tabs                        TkTabsFn                        ;
    tabstyle                    TkTabStyleFn                    ;
    tags                        TkTagsFn                        ;
    takefocus                   TkTakeFocusFn                   ;
    tearoff                     TkTearOffFn                     ;
    tearoffcommand              TkTearOffCommandFn              ;
  //text                        TkTextFn                        ;
    textvariable                TkTextVariableFn                ;
    tickinterval                TkTickIntervalFn                ;
    time                        TkTimeFn                        ;
    title                       TkTitleFn                       ;
    titlepath                   TkTitlePathFn                   ;
    to                          TkToFn                          ;
    toolwindow                  TkToolWindowFn                  ;
    topmost                     TkTopmostFn                     ;
    transparent                 TkTransparentFn                 ;
    transparentcolor            TkTransparentColorFn            ;
    tristateimage               TkTristateImageFn               ;
    tristatevalue               TkTristateValueFn               ;
    troughcolor                 TkTroughColorFn                 ;
    type_                       TkTypeFn                        ;
    typevariable                TkTypeVariableFn                ;
    underline                   TkUnderlineFn                   ;
    underlinefg                 TkUnderlineFgFn                 ;
    uniform                     TkUniformFn                     ;
    use_                        TkUseFn                         ;
    variable                    TkVariableFn                    ;
    validate                    TkValidateFn                    ;
    validatecommand             TkValidateCommandFn             ;
    value                       TkValueFn                       ;
    values                      TkValuesFn                      ;
    visual                      TkVisualFn                      ;
    visible                     TkVisibleFn                     ;
    undo                        TkUndoFn                        ;
    warp                        TkWarpFn                        ;
    when                        TkWhenFn                        ;
    wraplength                  TkWrapLengthFn                  ;
    weight                      TkWeightFn                      ;
    width                       TkWidthFn                       ;
    window                      TkWindowFn                      ;
    wrap                        TkWrapFn                        ;
    x_                          TkXFn                           ;
    xscrollcommand              TkXScrollCommandFn              ;
    xscrollincrement            TkXScrollIncrementFn            ;
    y_                          TkYFn                           ;
    yscrollcommand              TkYScrollCommandFn              ;
    yscrollincrement            TkYScrollIncrementFn            ;
    zoom                        TkZoomFn                        ;
    zoomed                      TkZoomedFn                      ;
}

#[derive( Copy, Clone )]
pub struct TkRoot<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> Deref for TkRoot<Inst> {
    type Target = Widget<Inst>;

    fn deref( &self ) -> &Self::Target { &self.0 }
}

impl<Inst:TkInstance> TkRoot<Inst> {
    pub fn configure<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<OptPair>
    {
        self.0.do_configure( opts.into() )
    }

    pub fn as_toplevel( &self ) -> TkToplevel<Inst> {
        TkToplevel( self.0 )
    }

    pub fn grab( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", self.0.path ))
    }

    pub fn grab_global( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", "-global", self.0.path ))
    }

    pub fn grab_set_current( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", "current", self.0.path ))
    }

    pub fn grab_release( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", "release", self.0.path ))
    }

    pub fn grab_set( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", "set", self.0.path ))
    }

    pub fn grab_set_global( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", "set", "-global", self.0.path ))
    }

    pub fn grab_status( &self ) -> InterpResult<()> {
        self.0.tk().run(( "grab", "status", self.0.path ))
    }
}

macro_rules! def_tuple_notation {
    ($str:expr => $ty:ident $trait:ident $valid_opt:path) => {
        pub struct $ty<Tup>( Tup );

        impl<Tup> Convert for $ty<Tup> {
            type Output = Tup;
            fn convert( self ) -> Self::Output { self.0 }
        }

        impl<Opts,Widgs> $trait for PathOptsWidgets<Opts,Widgs>
            where Widgs: ConvertTuple
                , Opts : IntoHomoTuple<$valid_opt>
                       + IntoHomoTuple<OptPair>
                , <Widgs as ConvertTuple>::Output
                       : PushFront<heredom::Node<(&'static str,&'static str),<Opts as IntoHomoTuple<OptPair>>::Output>>
        {
            type Output = $ty<
                <
                    <Widgs as ConvertTuple>::Output as
                    PushFront<
                        heredom::Node<
                            (&'static str,&'static str),
                            <Opts as IntoHomoTuple<OptPair>>::Output
                        >
                    >
                >
                ::Output
            >;

            fn output( self ) -> Self::Output {
                let cmd  = $str;
                let path = self.path;
                let opts = <Opts as IntoHomoTuple<OptPair>>::into_homo_tuple( self.opts );
                $ty( self.widgets.convert_tuple().push_front( heredom::Node( (cmd,path), opts )))
            }
        }

        def_hyphen_notation!( $ty );
    };
}

macro_rules! def_widgets {
    ($($str:expr => $widget_type:ident $ty:ident $trait:ident $add_trait:ident $add:ident $widget_opt:ident;)+) => {$(
        def_tuple_notation!( $str => $ty $trait opt::$widget_opt );

        use crate::$widget_type;

        impl<Inst:TkInstance> Deref for $widget_type<Inst> {
            type Target = Widget<Inst>;

            fn deref( &self ) -> &Self::Target { &self.0 }
        }

        impl<Inst:TkInstance> UpcastFrom<Inst> for $widget_type<Inst> {
            fn upcast_from( upcastable_widget: UpcastableWidget<Inst> ) -> Option<Self> {
                if upcastable_widget.name == $str {
                    Some( $widget_type( upcastable_widget.widget ))
                } else {
                    None
                }
            }
        }

        impl<Inst:TkInstance> From<$widget_type<Inst>> for Obj {
            fn from( widget: $widget_type<Inst> ) -> Obj { Obj::from( widget.0 )}
        }

        impl<Inst:TkInstance> TkPackSlave  for $widget_type<Inst> {}
        impl<Inst:TkInstance> TkGridSlave  for $widget_type<Inst> {}
        impl<Inst:TkInstance> TkPlaceSlave for $widget_type<Inst> {}

        impl<Inst:TkInstance> $widget_type<Inst> {
            pub fn configure<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
                where Opts : IntoHomoTuple<opt::$widget_opt>
                           + IntoHomoTuple<OptPair>
            {
                self.0.do_configure( opts.into() )
            }

            pub fn cget<Opt>( &self, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
                where Opt : TkOption
                          + Into<opt::$widget_opt>
            {
                self.0.cget( <Opt as TkOption>::NAME )
            }

            pub fn grab( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", self.0.path ))
            }

            pub fn grab_global( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", "-global", self.0.path ))
            }

            pub fn grab_set_current( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", "current", self.0.path ))
            }

            pub fn grab_release( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", "release", self.0.path ))
            }

            pub fn grab_set( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", "set", self.0.path ))
            }

            pub fn grab_set_global( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", "set", "-global", self.0.path ))
            }

            pub fn grab_status( &self ) -> InterpResult<()> {
                self.0.tk().run(( "grab", "status", self.0.path ))
            }
        }

        pub trait $add_trait {
            fn $add<Opts,Inst:TkInstance>( &self, path_opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<$widget_type<Inst>>
                where Self : Deref<Target=Widget<Inst>>
                    , Opts : IntoHomoTuple<opt::$widget_opt>
                           + IntoHomoTuple<OptPair>
            {
                self.deref().add( $str, path_opts.into() ).map( |w| $widget_type( w ))
            }
        }

        impl<Widg, Inst:TkInstance> $add_trait for Widg where Widg: Deref<Target=Widget<Inst>> {}
    )+};
}

def_widgets! {
    "button"         => TkButton      TkButtonTup      TkButtonFn      AddTkButton      add_button      TkButtonOpt      ;
    "canvas"         => TkCanvas      TkCanvasTup      TkCanvasFn      AddTkCanvas      add_canvas      TkCanvasOpt      ;
    "checkbutton"    => TkCheckbutton TkCheckbuttonTup TkCheckbuttonFn AddTkCheckbutton add_checkbutton TkCheckbuttonOpt ;
    "entry"          => TkEntry       TkEntryTup       TkEntryFn       AddTkEntry       add_entry       TkEntryOpt       ;
    "frame"          => TkFrame       TkFrameTup       TkFrameFn       AddTkFrame       add_frame       TkFrameOpt       ;
    "label"          => TkLabel       TkLabelTup       TkLabelFn       AddTkLabel       add_label       TkLabelOpt       ;
    "labelframe"     => TkLabelframe  TkLabelframeTup  TkLabelframeFn  AddTkLabelframe  add_labelframe  TkLabelframeOpt  ;
    "listbox"        => TkListbox     TkListboxTup     TkListboxFn     AddTkListbox     add_listbox     TkListboxOpt     ;
    "menu"           => TkMenu        TkMenuTup        TkMenuFn        AddTkMenu        add_menu        TkMenuOpt        ;
    "menubutton"     => TkMenubutton  TkMenubuttonTup  TkMenubuttonFn  AddTkMenubutton  add_menubutton  TkMenubuttonOpt  ;
    "message"        => TkMessage     TkMessageTup     TkMessageFn     AddTkMessage     add_message     TkMessageOpt     ;
    "panedwindow"    => TkPanedwindow TkPanedwindowTup TkPanedwindowFn AddTkPanedwindow add_panedwindow TkPanedwindowOpt ;
    "radiobutton"    => TkRadiobutton TkRadiobuttonTup TkRadiobuttonFn AddTkRadiobutton add_radiobutton TkRadiobuttonOpt ;
    "scale"          => TkScale       TkScaleTup       TkScaleFn       AddTkScale       add_scale       TkScaleOpt       ;
    "scrollbar"      => TkScrollbar   TkScrollbarTup   TkScrollbarFn   AddTkScrollbar   add_scrollbar   TkScrollbarOpt   ;
    "spinbox"        => TkSpinbox     TkSpinboxTup     TkSpinboxFn     AddTkSpinbox     add_spinbox     TkSpinboxOpt     ;
    "text"           => TkText        TkTextTup        TkTextFn        AddTkText        add_text        TkTextOpt        ;
    "toplevel"       => TkToplevel    TkToplevelTup    TkToplevelFn    AddTkToplevel    add_toplevel    TkToplevelOpt    ;
    "ttk::entry"       => TtkEntry       TtkEntryTup       TtkEntryFn       AddTtkEntry       add_ttk_entry       TtkEntryOpt         ;
    "ttk::frame"       => TtkFrame       TtkFrameTup       TtkFrameFn       AddTtkFrame       add_ttk_frame       TtkFrameOpt         ;
    "ttk::label"       => TtkLabel       TtkLabelTup       TtkLabelFn       AddTtkLabel       add_ttk_label       TtkLabelOpt         ;
    "ttk::button"      => TtkButton      TtkButtonTup      TtkButtonFn      AddTtkButton      add_ttk_button      TtkButtonOpt        ;
    "ttk::checkbutton" => TtkCheckbutton TtkCheckbuttonTup TtkCheckbuttonFn AddTtkCheckbutton add_ttk_checkbutton TtkCheckbuttonOpt   ;
    "ttk::combobox"    => TtkCombobox    TtkComboboxTup    TtkComboboxFn    AddTtkCombobox    add_ttk_combobox    TtkComboboxOpt      ;
    "ttk::labelframe"  => TtkLabelframe  TtkLabelframeTup  TtkLabelframeFn  AddTtkLabelframe  add_ttk_labelframe  TtkLabelframeOpt    ;
    "ttk::menubutton"  => TtkMenubutton  TtkMenubuttonTup  TtkMenubuttonFn  AddTtkMenubutton  add_ttk_menubutton  TtkMenubuttonOpt    ;
    "ttk::notebook"    => TtkNotebook    TtkNotebookTup    TtkNotebookFn    AddTtkNotebook    add_ttk_notebook    TtkNotebookOpt      ;
    "ttk::panedwindow" => TtkPanedwindow TtkPanedwindowTup TtkPanedwindowFn AddTtkPanedwindow add_ttk_panedwindow TtkPanedwindowOpt   ;
    "ttk::progressbar" => TtkProgressbar TtkProgressbarTup TtkProgressbarFn AddTtkProgressbar add_ttk_progressbar TtkProgressbarOpt   ;
    "ttk::radiobutton" => TtkRadiobutton TtkRadiobuttonTup TtkRadiobuttonFn AddTtkRadiobutton add_ttk_radiobutton TtkRadiobuttonOpt   ;
    "ttk::scale"       => TtkScale       TtkScaleTup       TtkScaleFn       AddTtkScale       add_ttk_scale       TtkScaleOpt         ;
    "ttk::scrollbar"   => TtkScrollbar   TtkScrollbarTup   TtkScrollbarFn   AddTtkScrollbar   add_ttk_scrollbar   TtkScrollbarOpt     ;
    "ttk::separator"   => TtkSeparator   TtkSeparatorTup   TtkSeparatorFn   AddTtkSeparator   add_ttk_separator   TtkSeparatorOpt     ;
    "ttk::sizegrip"    => TtkSizegrip    TtkSizegripTup    TtkSizegripFn    AddTtkSizegrip    add_ttk_sizegrip    TtkSizegripOpt      ;
    "ttk::spinbox"     => TtkSpinbox     TtkSpinboxTup     TtkSpinboxFn     AddTtkSpinbox     add_ttk_spinbox     TtkSpinboxOpt       ;
    "ttk::treeview"    => TtkTreeview    TtkTreeviewTup    TtkTreeviewFn    AddTtkTreeview    add_ttk_treeview    TtkTreeviewOpt      ;
}

macro_rules! def_geometry_managers {
    ($($str:expr => $fn:ident $fn_cfg:ident $fn_forget:ident $fn_info:ident $ty:ident $trait:ident $slave_trait:ident $geoman_opt:ident;)+) => {$(
        pub struct $ty<Tup>( Tup );

        impl<Tup,Front,Remain> Convert for $ty<Tup>
            where Tup   : PopFront<Front=Front,Remain=Remain>
                , Remain: ConvertTuple
                , <Remain as ConvertTuple>::Output: PushFront<Front>
        {
            type Output = <<Remain as ConvertTuple>::Output as PushFront<Front>>::Output;
            fn convert( self ) -> Self::Output {
                let (front, remain) = (self.0).pop_front();
                remain.convert_tuple().push_front( front )
            }
        }

        impl<Opts,Widgs> $trait for PathOptsWidgets<Opts,Widgs>
            where Widgs: PushFront<heredom::Node<(&'static str,&'static str),<Opts as IntoHomoTuple<OptPair>>::Output>>
                , Opts : IntoHomoTuple<opt::$geoman_opt>
                       + IntoHomoTuple<OptPair>
        {
            type Output = $ty<
                <
                    Widgs as PushFront<
                        heredom::Node<
                            (&'static str,&'static str),
                            <Opts as IntoHomoTuple<OptPair>>::Output
                        >
                    >
                >
                ::Output
            >;

            fn output( self ) -> Self::Output {
                let cmd  = $str;
                let path = self.path;
                let opts = <Opts as IntoHomoTuple<OptPair>>::into_homo_tuple( self.opts );
                $ty( self.widgets.push_front( heredom::Node( (cmd,path), opts )))
            }
        }

        def_hyphen_notation!( $ty );

        pub trait $slave_trait: Sized {
            fn $fn<Opts,Inst:TkInstance>( self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Self>
                where Self : Deref<Target=Widget<Inst>>
                    , Opts : IntoHomoTuple<opt::$geoman_opt>
                           + IntoHomoTuple<OptPair>
            {
                let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );
                command.push( $str.into() );
                command.push( self.path.into() );

                append_opts( &mut command, opts.into().opts );

                self.tk().eval( command )?;
                Ok( self )
            }
        }

        impl<Inst:TkInstance> Widget<Inst> {
            pub fn $fn_cfg<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
                where Opts : IntoHomoTuple<opt::$geoman_opt>
                           + IntoHomoTuple<OptPair>
            {
                let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
                command.push( $str.into() );
                command.push( "configure".into() );
                command.push( self.path.into() );

                append_opts( &mut command, opts.into().opts );

                self.tk().run( command )
            }

            pub fn $fn_forget( &self ) -> InterpResult<()> {
                self.tk().run(( $str, "forget", self.deref().path ))
            }

            pub fn $fn_info( &self ) -> InterpResult<HashMap<String,Obj>> {
                let list = self.tk().eval(( $str, "info", self.deref().path ))?;
                let mut len: c_int = 0;
                let mut m;

                unsafe {
                    let interp = &*self.tk();
                    let tcl_interp = interp.as_ptr();

                    clib::Tcl_ListObjLength( tcl_interp, list.as_ptr(), &mut len as *mut _ )
                        .code_to_result( &*interp )?;
                    m = HashMap::with_capacity( len as usize / 2 );

                    for index in (0..len).step_by(2) {
                        let mut k = null_mut::<clib::Tcl_Obj>();
                        let mut v = null_mut::<clib::Tcl_Obj>();
                        clib::Tcl_ListObjIndex( tcl_interp, list.as_ptr(), index, &mut k as *mut _ )
                            .code_to_result( &*interp )?;
                        clib::Tcl_ListObjIndex( tcl_interp, list.as_ptr(), index+1, &mut v as *mut _ )
                            .code_to_result( &*interp )?;
                        let k = Obj::from_raw( k );
                        let v = Obj::from_raw( v );
                        m.entry( k.to_string() ).or_insert( v );
                    }
                }

                Ok( m )
            }
        }

        impl<Inst:TkInstance> Tk<Inst> {
            pub fn $fn<Opts>( &self, paths_opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
                where Opts : IntoHomoTuple<opt::$geoman_opt>
                           + IntoHomoTuple<OptPair>
            {
                let paths_opts = paths_opts.into();

                let paths = Obj::from( paths_opts.path );
                unsafe {
                    let mut _num_paths: c_int = 0;
                    clib::Tcl_ListObjLength( self.as_ptr(), paths.as_ptr(), &mut _num_paths as *mut _ );
                }

                let cap = <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2;
                let mut all_opts = Vec::<Obj>::with_capacity( cap );
                append_opts( &mut all_opts, paths_opts.opts );

                self.eval((
                    Obj::from( "eval" ),
                    Obj::from( $str ),
                    paths,
                    Obj::from( all_opts ),
                ))
                .map( |_| () )
            }
        }
    )+};
}

def_geometry_managers! {
    "pack"  => pack  pack_configure  pack_forget  pack_info  TkPack  TkPackFn  TkPackSlave  TkPackOpt ;
    "grid"  => grid  grid_configure  grid_forget  grid_info  TkGrid  TkGridFn  TkGridSlave  TkGridOpt ;
    "place" => place place_configure place_forget place_info TkPlace TkPlaceFn TkPlaceSlave TkPlaceOpt;
}

impl<Inst:TkInstance> Widget<Inst> {
    #[cex]
    pub fn grid_slaves<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> Result!( Vec<Widget<Inst>>
        throws InterpError, NotList, WidgetNotFound )
        where Opts : IntoHomoTuple<opt::TkRowColumnOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "grid".into() );
        command.push( "slaves".into() );
        command.push( self.path.into() );

        append_opts( &mut command, opts.into().opts );

        let slaves = self.tk().eval( command )?;
        ret!( self.tk().widgets_from_obj( slaves ))
    }

    #[cex]
    pub fn pack_slaves( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let slaves = self.tk().eval(( "pack", "slaves", self.path ))?;
        ret!( self.tk().widgets_from_obj( slaves ))
    }

    #[cex]
    pub fn place_slaves( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let slaves = self.tk().eval(( "place", "slaves", self.path ))?;
        ret!( self.tk().widgets_from_obj( slaves ))
    }
}

pub(crate) fn append_opts<Opts>( command: &mut Vec<Obj>, opts: Opts )
    where Opts: IntoHomoTuple<OptPair>
{
    for opt_pair in IntoHomoTuple::<OptPair>::into_homo_tuple( opts ).wrap_into_iter() {
        if opt_pair.name.len() > 0 {
            command.push( opt_pair.name.into() );
        }
        command.push( opt_pair.value );
    }
}

pub fn path_seg( path: &'static str ) -> PathOptsWidgets<(),()> {
    PathOptsWidgets{ path, opts:(), widgets:() }
}

pub fn no_arg() -> PathOptsWidgets<(),()> {
    PathOptsWidgets{ path:"", opts:(), widgets:() }
}

impl<Inst:TkInstance> Tk<Inst> {
    #[cex]
    pub(crate) fn widgets_from_obj( &self, obj: Obj ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let mut widgets = Vec::new();
        for path in obj.get_elements()?.map( |obj| obj.to_string() ) {
            match Widget::from_name( &path, self.inst )? {
                Some( widget ) => widgets.push( widget ),
                None => throw!( WidgetNotFound( path )),
            }
        }
        ret!( widgets );
    }
}

impl<Inst:TkInstance> Tk<Inst> {
    pub fn app_name( &self ) -> InterpResult<String> {
        Ok( self.eval(( "tk", "appname" ))?.to_string() )
    }

    pub fn set_app_name( &self, new_name: &str ) -> InterpResult<String> {
        Ok( self.eval(( "tk", "appname", new_name ))?.to_string() )
    }

    pub fn busy( &self, widget: Widget<Inst> ) -> InterpResult<()> {
        self.run(( "tk", "busy", widget.path ))
    }

    pub fn busy_hold<Opts>( &self, widget: Widget<Inst>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<opt::TkBusyHoldOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "tk".into() );
        command.push( "busy".into() );
        command.push( widget.path.into() );

        append_opts( &mut command, opts.into().opts );
        self.run( command )
    }

    pub fn busy_cget<Opt>( &self, widget: Widget<Inst>, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<opt::TkBusyHoldOpt>
    {
        self.eval(( "tk", "busy", "cget", widget.path, <Opt as TkOption>::NAME ))
    }

    pub fn busy_configure<Opts>( &self, widget: Widget<Inst>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<opt::TkBusyHoldOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( "tk".into() );
        command.push( "busy".into() );
        command.push( "configure".into() );
        command.push( widget.path.into() );

        append_opts( &mut command, opts.into().opts );
        self.run( command )
    }

    pub fn busy_forget( &self, widgets: &[ Widget<Inst> ]) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( widgets.len() + 3 );
        command.push( "tk".into() );
        command.push( "busy".into() );
        command.push( "forget".into() );

        command.extend( widgets.iter().map( |widget| widget.path.into() ));
        self.run( command )
    }

    #[cex]
    pub fn busy_current( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let obj = self.eval(( "tk", "busy", "current" ))?;
        ret!( self.widgets_from_obj( obj ));
    }

    #[cex]
    pub fn busy_current_of_pattern( &self, pattern: &str ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let obj = self.eval(( "tk", "busy", "current", pattern ))?;
        ret!( self.widgets_from_obj( obj ));
    }

    pub fn busy_status( &self, widget: Widget<Inst> ) -> InterpResult<bool> {
        let obj = self.eval(( "tk", "busy", "status", widget.path ))?;
        self.boolean( obj )
    }

    #[cex]
    pub fn caret( &self, widget: Widget<Inst> ) -> Result!( TkCaret throws DeError, InterpError ) {
        let obj = self.eval(( "tk", "caret", widget.path ))?;
        ret!( from_obj::<TkCaret>( obj ));
    }

    pub fn set_caret<Opts>( &self, widget: Widget<Inst>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<opt::TkCaretOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "tk".into() );
        command.push( "caret".into() );
        command.push( widget.path.into() );

        append_opts( &mut command, opts.into().opts );
        self.run( command )
    }

    pub fn inactive( &self ) -> InterpResult<c_longlong> {
        let obj = self.eval(( "tk", "inactive" ))?;
        self.longlong( obj )
    }

    pub fn inactive_reset( &self ) -> InterpResult<c_longlong> {
        let obj = self.eval(( "tk", "inactive", "reset" ))?;
        self.longlong( obj )
    }

    pub fn fontchooser_configure<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<opt::TkFontChooserOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "tk".into() );
        command.push( "fontchooser".into() );
        command.push( "configure".into() );

        append_opts( &mut command, opts.into().opts );
        self.run( command )
    }

    pub fn fontchooser_show( &self ) -> InterpResult<()> {
        self.run(( "tk", "fontchooser", "show" ))
    }

    pub fn fontchooser_hide( &self ) -> InterpResult<()> {
        self.run(( "tk", "fontchooser", "hide" ))
    }

    pub fn scaling( &self ) -> InterpResult<c_double> {
        let obj = self.eval(( "tk", "scaling" ))?;
        self.double( obj )
    }

    pub fn set_scaling( &self, number: c_double ) -> InterpResult<()> {
        self.run(( "tk", "scaling", number ))
    }

    pub fn has_used_input_methods( &self ) -> InterpResult<bool> {
        let obj = self.eval(( "tk", "useinputmethods" ))?;
        self.boolean( obj )
    }

    pub fn use_input_methods( &self, should_use: bool ) -> InterpResult<()> {
        self.run(( "tk", "useinputmethods", should_use ))
    }

    #[cex]
    pub fn windowing_system( &self ) -> Result!( TkWindowingSystem throws DeError, InterpError) {
        let obj = self.eval(( "tk", "windowingsystem" ))?;
        ret!( from_obj::<TkWindowingSystem>( obj ));
    }

    pub fn choose_color<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<String>
        where Opts : IntoHomoTuple<opt::TkChooseColorOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 1 );
        command.push( "tk_chooseColor".into() );

        append_opts( &mut command, opts.into().opts );
        self.eval( command ).map( |obj| obj.to_string() )
    }

    pub fn choose_directory<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<PathBuf>
        where Opts : IntoHomoTuple<opt::TkChooseDirectoryOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 1 );
        command.push( "tk_chooseDirectory".into() );

        append_opts( &mut command, opts.into().opts );
        self.eval( command ).map( |obj| obj.to_string().into() )
    }

    pub fn focus_next( &self, widget: Widget<Inst> ) -> InterpResult<Widget<Inst>> {
        let path = self.eval(( "tk_focusNext", widget.path ))?.to_string();
        Ok( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.inst, mark: NOT_SEND_SYNC })
    }

    pub fn focus_prev( &self, widget: Widget<Inst> ) -> InterpResult<Widget<Inst>> {
        let path = self.eval(( "tk_focusPrev", widget.path ))?.to_string();
        Ok( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.inst, mark: NOT_SEND_SYNC })
    }

    pub fn focus_follows_mouse( &self ) -> InterpResult<()> {
        self.run( "tk_focusFollowsMouse" )
    }

    fn get_file<Opts>( &self, cmd: &'static str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Option<PathBuf>>
        where Opts : IntoHomoTuple<opt::TkGetOpenFileOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 1 );
        command.push( cmd.into() );
        append_opts( &mut command, opts.into().opts );

        let s = self.eval( command )?.to_string();

        if s.is_empty() {
            Ok( None )
        } else {
            Ok( Some( s.into() ))
        }
    }

    pub fn get_open_file<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Option<PathBuf>>
        where Opts : IntoHomoTuple<opt::TkGetOpenFileOpt>
                   + IntoHomoTuple<OptPair>
    {
        self.get_file( "tk_getOpenFile", opts.into() )
    }


    pub fn get_save_file<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Option<PathBuf>>
        where Opts : IntoHomoTuple<opt::TkGetOpenFileOpt>
                   + IntoHomoTuple<OptPair>
    {
        self.get_file( "tk_getSaveFile", opts.into() )
    }

    pub fn message_box<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<String>
        where Opts : IntoHomoTuple<opt::TkMessageBoxOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 1 );
        command.push( "tk_messageBox".into() );
        append_opts( &mut command, opts.into().opts );

        self.eval( command ).map( |obj| obj.to_string() )
    }

    pub fn popup( &self, menu: TkMenu<Inst>, x: c_int, y: c_int, entry: Option<c_int> ) -> InterpResult<()> {
        match entry {
            Some( entry ) => self.run(( "tk_popup", menu.path, x, y, entry )),
            None          => self.run(( "tk_popup", menu.path, x, y        )),
        }
    }

    pub fn set_palette_background( &self, background: impl Into<Obj> ) -> InterpResult<()> {
        self.run(( "tk_setPalette", background.into() ))
    }

    pub fn set_palette<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<opt::TkSetPaletteOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 1 );
        command.push( "tk_setPalette".into() );

        for opt_pair in IntoHomoTuple::<OptPair>::into_homo_tuple( opts.into().opts ).wrap_into_iter() {
            if opt_pair.name.len() > 0 {
                command.push( opt_pair.name.trim_start_matches('-').into() );
            }
            command.push( opt_pair.value );
        }

        self.run( command )
    }

    pub fn wait_variable( &self, name: &str ) -> InterpResult<()> {
        self.run(( "tkwait", "variable", name ))
    }

    pub fn wait_visibility( &self, widget: &Widget<Inst> ) -> InterpResult<()> {
        self.run(( "tkwait", "visibility", widget.path ))
    }

    pub fn wait_window( &self, widget: &Widget<Inst> ) -> InterpResult<()> {
        self.run(( "tkwait", "window", widget.path ))
    }

    /// This command deletes the windows given by the window arguments, plus all of
    /// their descendants. If a window "." is deleted then all windows will be destroyed
    /// and the application will (normally) exit. The windows are destroyed in order,
    /// and if an error occurs in destroying a window the command aborts without
    /// destroying the remaining windows. No error is returned if window does not exist.
    ///
    /// # Examples
    ///
    ///```rust
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// let tk = make_tk!()?;
    /// let root = tk.root();
    /// root.add_widgets( -label( "lb" -text("Some label") ))?;
    ///
    /// assert!( tk.winfo_exists( ".lb" )? );
    /// tk.destroy(( ".lb", ))?;
    /// assert!( !tk.winfo_exists( ".lb" )? );
    ///
    /// let lb2 = root.add_label( "lb2" -text("Another label") )?;
    ///
    /// assert!( tk.winfo_exists( ".lb2" )? );
    /// tk.destroy(( lb2, ))?;
    /// assert!( !tk.winfo_exists( ".lb2" )? );
    ///
    /// # TkResult::Ok(())
    ///```
    pub fn destroy<Widgets,Tag>( &self, widgets: Widgets ) -> InterpResult<()>
        where Widgets: IntoHomoTuple<Obj> + NonZeroLen<Tag>
            , <Widgets as IntoHomoTuple<Obj>>::Output : Into<Obj>
    {
        self.run(( "eval", "destroy", widgets.into_homo_tuple() ))
    }

    pub fn grab_current( &self ) -> InterpResult<Widget<Inst>> {
        let path = self.eval(( "grab", "current" ))?.to_string();
        Ok( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.inst, mark: NOT_SEND_SYNC })
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    pub fn has_used_input_methods( &self ) -> InterpResult<bool> {
        let obj = self.tk().eval(( "tk", "useinputmethods", "-displayof", self.path ))?;
        self.tk().boolean( obj )
    }

    pub fn use_input_methods( &self, should_use: bool ) -> InterpResult<()> {
        self.tk().run(( "tk", "useinputmethods", "-displayof", self.path, should_use ))
    }

    pub fn inactive( &self ) -> InterpResult<c_longlong> {
        let obj = self.tk().eval(( "tk", "inactive", "-displayof", self.path ))?;
        self.tk().longlong( obj )
    }

    pub fn inactive_reset( &self ) -> InterpResult<c_longlong> {
        let obj = self.tk().eval(( "tk", "inactive", "-displayof", self.path, "reset" ))?;
        self.tk().longlong( obj )
    }

    pub fn scaling( &self ) -> InterpResult<c_double> {
        let obj = self.tk().eval(( "tk", "scaling", "-displayof", self.path ))?;
        self.tk().double( obj )
    }

    pub fn set_scaling( &self, number: c_double ) -> InterpResult<()> {
        self.tk().run(( "tk", "scaling", "-displayof", self.path, number ))
    }
}

pub struct TkOptionMenuTup<T>( T );

def_hyphen_notation!( TkOptionMenuTup );

impl<Tup> Convert for TkOptionMenuTup<Tup> {
    type Output = Tup;
    fn convert( self ) -> Self::Output { self.0 }
}

/// Creates an option menubutton whose name is `path_seg`, plus an associated menu.
/// Together they allow the user to select one of the items given by the item
/// arguments. The current item will be stored in the global variable whose name is
/// given by `var_name` and it will also be displayed as the label in the option
/// menubutton. The user can click on the menubutton to display a menu containing
/// all of the values and thereby select a new item. Once a new item is selected, it
/// will be stored in the variable and appear in the option menubutton. The current
/// value can also be changed by setting the variable.
///
/// # Example
///
/// ```rust
/// use tcl::*;
/// use tk::*;
/// use tk::cmd::*;
/// fn main() -> TkResult<()> {
///     let tk = make_tk!()?;
///     let root = tk.root();
///     root.add_widgets( -pack( -option_menu( "foo", "myVar", &[ "Foo", "Bar", "Boo", "Spong", "Wibble" ])))?;
///     Ok( main_loop() )
/// }
/// ```
pub fn option_menu( path_seg: &'static str, var_name: &str, items: &[&str] )
    -> TkOptionMenuTup<(heredom::Node<(&'static str,&'static str), (OptPair, OptPair)>,)>
{
    let cmd_and_path = ("tk_rs_option_menu", path_seg);
    let var_name = OptPair::value_only( var_name );
    let items = OptPair::value_only( items );
    TkOptionMenuTup( (heredom::Node( cmd_and_path, (var_name, items) ),) )
}

pub struct TkOptionMenu<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> TkOptionMenu<Inst> {
    pub fn menu( &self ) -> InterpResult<TkMenu<Inst>> {
        let path = self.0.tk().eval( format!( "set rs_tk_widget_extra_data({})", self.0.path ))?.to_string();
        Ok( TkMenu( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.0.inst, mark: NOT_SEND_SYNC }))
    }
}

impl<Inst:TkInstance> TkPackSlave  for TkOptionMenu<Inst> {}
impl<Inst:TkInstance> TkGridSlave  for TkOptionMenu<Inst> {}
impl<Inst:TkInstance> TkPlaceSlave for TkOptionMenu<Inst> {}

impl<Inst:TkInstance> Deref for TkOptionMenu<Inst> {
    type Target = Widget<Inst>;

    fn deref( &self ) -> &Self::Target { &self.0 }
}

impl<Inst:TkInstance> From<TkOptionMenu<Inst>> for Obj {
    fn from( widget: TkOptionMenu<Inst> ) -> Obj { Obj::from( widget.0 )}
}

pub trait AddTkOptionMenu {
    /// Creates an option menubutton whose name is `path_seg`, plus an associated menu.
    /// Together they allow the user to select one of the items given by the item
    /// arguments. The current item will be stored in the global variable whose name is
    /// given by `var_name` and it will also be displayed as the label in the option
    /// menubutton. The user can click on the menubutton to display a menu containing
    /// all of the values and thereby select a new item. Once a new item is selected, it
    /// will be stored in the variable and appear in the option menubutton. The current
    /// value can also be changed by setting the variable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    /// fn main() -> TkResult<()> {
    ///     let tk = make_tk!()?;
    ///     let root = tk.root();
    ///     let _foo = root
    ///         .add_option_menu( "foo", "myVar", &[ "Foo", "Bar", "Boo", "Spong", "Wibble" ])?
    ///         .pack(())?;
    ///     Ok( main_loop() )
    /// }
    /// ```
    fn add_option_menu<Inst:TkInstance>( &self, path_seg: &'static str, var_name: &str, items: &[&str] ) -> InterpResult<TkOptionMenu<Inst>>
        where Self: Deref<Target=Widget<Inst>>
    {
        let path = self.tk().next_path( &self.path, path_seg );
        self.tk().eval(( "tk_rs_option_menu", path.as_str(), var_name, items ))?;

        Ok( TkOptionMenu( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.inst, mark: NOT_SEND_SYNC }))
    }
}

impl<Widg, Inst:TkInstance> AddTkOptionMenu for Widg where Widg: Deref<Target=Widget<Inst>> {}

#[cfg( test )]
mod tests {
    use crate::TkResult;
    use super::*;

    #[test]
    fn hello_world() -> TkResult<()> {
        let tk = make_tk!()?;
        let root = tk.root();

        root.add_label( -text("hello,world!") )?.pack(())?;

        Ok( main_loop() )
    }

    #[test]
    fn all_in_one() -> TkResult<()> {
        let tk = make_tk!()?;
        let root = tk.root();

        root.add_widgets(
            -button( "hello tk" -text("hello,tk!") -command("") )
            -frame( "fr"
                -button( "exit" -text("exit") -command("destroy .") )
            )
            -button( "demo" -text("demo") )
        )?;

        tk.pack( "{.hello tk} .fr .fr.exit" )?;
        tk.pack( ".demo" -expand(1) )?;

        Ok( main_loop() )
    }

    #[test]
    fn test_x11() -> TkResult<()> {
        let tk = make_tk!()?;
        let system = tk.windowing_system()?;
        Ok( println!( "windowing system is {:?}", system ))
    }
}
