use crate::{
    InterpResult,
    NOT_SEND_SYNC,
    PathOptsWidgets,
    Tk,
    TkInstance,
    TkOption,
    UpcastableWidget,
    Widget,
    CreatedWidgets,
    opt::{
        TkMenuOpt,
        TkMenuEntryOpt,
        OptPair,
    },
    range::{
        TkDefaultEnd,
        TkDefaultStart,
        TkRange,
    },
    traits::Delete,
};

use heredom::{DomForest, Visit};

use std::{
    ops::{
        Deref,
        RangeFrom,
        RangeInclusive,
        RangeToInclusive,
    },
    os::raw::{
        c_int,
        c_longlong,
    },
};

use tcl::Obj;

use tuplex::*;

#[derive( Copy, Clone )]
pub struct TkMenu<Inst:TkInstance>( pub(crate) Widget<Inst> );

/// Many of the methods of a menu take as one argument an indicator of which
/// entry of the menu to operate on. These indicators are called `Index`es
/// and may be specified in any of the following forms:
///
/// - `Index::Active`
///
/// Indicates the entry that is currently active. If no entry is active then
/// this form is equivalent to none.
///
/// - `Index::End`
///
/// Indicates the bottommost entry in the menu. If there are no entries in the
/// menu then this form is equivalent to none.
///
/// - `Index::None`
///
/// Indicates "no entry at all"; this is used most commonly with the activate
/// option to deactivate all the entries in the menu. In most cases the
/// specification of none causes nothing to happen in the widget command.
///
/// - `Index::At`
///
/// In this form, number is treated as a y-coordinate in the menu's window; the
/// entry closest to that y-coordinate is used. For example, `Index::At(0)`
/// indicates the top-most entry in the window.
///
/// - `Index::Number`
///
/// Specifies the entry numerically, where 0 corresponds to the top-most entry
/// of the menu, 1 to the entry below it, and so on.
///
/// - `Index::Pattern`
///
/// Pattern is pattern-matched against the label of each entry in the menu, in
/// order from the top down, until a matching entry is found. The rules of
/// string match are used.
#[derive( Clone, Debug, PartialEq )]
pub enum Index {
    Active,
    End,
    None,
    At( c_int ),
    Number( c_int ),
    Pattern( String ),
}

impl Index {
    pub fn pattern( pattern: &str ) -> Self { Index::Pattern( pattern.to_owned() )}
}

impl From<c_int> for Index {
    fn from( number: c_int ) -> Self { Index::Number( number )}
}

impl TkDefaultStart for Index {
    fn default_start() -> Self { Index::Number(0) }
}

impl TkDefaultEnd for Index {
    fn default_end() -> Self { Index::End }
}

impl From<RangeFrom<c_int>> for TkRange<Index> { // a..
    fn from( r: RangeFrom<c_int> ) -> Self {
        TkRange {
            start : Index::Number( r.start ),
            end   : Index::default_end()
        }
    }
}

impl From<RangeInclusive<c_int>> for TkRange<Index> { // a..=b
    fn from( r: RangeInclusive<c_int> ) -> Self {
        TkRange {
            start : Index::Number( *r.start() ),
            end   : Index::Number( *r.end() )
        }
    }
}

impl From<RangeToInclusive<c_int>> for TkRange<Index> { // ..=b
    fn from( r: RangeToInclusive<c_int> ) -> Self {
        TkRange {
            start : Index::default_start(),
            end   : Index::Number( r.end ),
        }
    }
}

impl From<Index> for Obj {
    fn from( index: Index ) -> Obj {
        use Index::*;
        match index {
            Active       => "active".into(),
            End          => "end".into(),
            None         => "none".into(),
            At( n )      => format!( "@{}", n ).into(),
            Number( n )  => n.into(),
            Pattern( p ) => p.into(),
        }
    }
}

macro_rules! pub_fn_add {
    ($name:expr, $ident:ident) => {
        pub fn $ident<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
            where Opts: IntoHomoTuple<TkMenuEntryOpt>
                      + IntoHomoTuple<OptPair>
        {
            let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
            command.push( self.path.into() );
            command.push( "add".into() );
            command.push( $name.into() );
            crate::cmd::append_opts( &mut command, opts.into().opts );
            self.tk().run( command )
        }
    };
}

macro_rules! pub_fn_insert {
    ($name:expr, $ident:ident) => {
        pub fn $ident<Opts>( &self, index: impl Into<Index>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
            where Opts: IntoHomoTuple<TkMenuEntryOpt>
                      + IntoHomoTuple<OptPair>
        {
            let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
            command.push( self.path.into() );
            command.push( "insert".into() );
            command.push( index.into().into() );
            command.push( $name.into() );
            crate::cmd::append_opts( &mut command, opts.into().opts );
            self.tk().run( command )
        }
    };
}

/// Used in `TkMenu::clone()` method
pub enum TkMenuCloneType {
    Normal,
    MenuBar,
    TearOff,
}

impl From<TkMenuCloneType> for Obj {
    fn from( menu_clone_type: TkMenuCloneType ) -> Obj {
        match menu_clone_type {
            TkMenuCloneType::Normal  => "normal" .into(),
            TkMenuCloneType::MenuBar => "menubar".into(),
            TkMenuCloneType::TearOff => "tearoff".into(),
        }
    }
}

pub enum TkMenuEntryType {
    Cascade,
    CheckButton,
    Command,
    RadioButton,
    Separator,
    TearOff,
}

impl<Inst:TkInstance> self::TkMenu<Inst> {
    pub fn activate( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "activate", index.into() ))
    }

    pub_fn_add!( "cascade"    , add_cascade     );
    pub_fn_add!( "checkbutton", add_checkbutton );
    pub_fn_add!( "command"    , add_command     );
    pub_fn_add!( "radiobutton", add_radiobutton );
    pub_fn_add!( "separator"  , add_separator   );

    pub fn clone( &self, new_path_name: String, menu_clone_type: TkMenuCloneType ) -> InterpResult<Self> {
        self.0.tk().run(( self.0.path, "clone", new_path_name.as_str(), menu_clone_type ))?;
        Ok( self::TkMenu( Widget{
            path : Tk::<Inst>::make_or_get_path( &new_path_name ),
            inst : self.0.inst,
            mark : NOT_SEND_SYNC,
        }))
    }

    pub fn entrycget<Opt>( &self, index: impl Into<Index>, _name_fn: fn(Obj)->Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkMenuEntryOpt>
    {
        self.0.tk().eval(( self.0.path, "entrycget", index.into(), <Opt as TkOption>::NAME ))
    }

    pub fn entryconfigure<Opts>( &self, index: impl Into<Index>, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkMenuEntryOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( self.path.into() );
        command.push( "entryconfigure".into() );
        command.push( index.into().into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );
        self.tk().run( command )
    }

    pub fn entryconfigure_options( &self, index: impl Into<Index> ) -> InterpResult<Obj> {
        self.tk().eval(( self.path, "entryconfigure", index.into() ))
    }

    pub fn index( &self, index: impl Into<Index> ) -> InterpResult<Option<c_longlong>> {
        let index = index.into();
        if index == Index::None {
            Ok( None )
        } else {
            let obj = self.0.tk().eval(( self.0.path, "index", index ))?;
            self.0.tk().longlong( obj ).map( Some )
        }
    }

    pub_fn_insert!( "cascade"    , insert_cascade     );
    pub_fn_insert!( "checkbutton", insert_checkbutton );
    pub_fn_insert!( "command"    , insert_command     );
    pub_fn_insert!( "radiobutton", insert_radiobutton );
    pub_fn_insert!( "separator"  , insert_separator   );

    pub fn invoke( &self, index: impl Into<Index> ) -> InterpResult<Obj> {
        self.0.tk().eval(( self.0.path, "invoke", index.into() ))
    }

    pub fn post( &self, x: c_int, y: c_int ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "post", x, y ))
    }

    pub fn post_entry( &self, x: c_int, y: c_int, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "post", x, y, index.into() ))
    }

    pub fn postcascade( &self, index: impl Into<Index> ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "postcascade", index.into() ))
    }

    pub fn type_( &self, index: impl Into<Index> ) -> InterpResult<Option<TkMenuEntryType>> {
        Ok( match self.0.tk().eval(( self.0.path, "type", index.into() ))?.to_string().as_str() {
            "cascade"     => Some( TkMenuEntryType::Cascade ),
            "checkbutton" => Some( TkMenuEntryType::CheckButton ),
            "command"     => Some( TkMenuEntryType::Command ),
            "radiobutton" => Some( TkMenuEntryType::RadioButton ),
            "separator"   => Some( TkMenuEntryType::Separator ),
            "tearoff"     => Some( TkMenuEntryType::TearOff ),
            _             => None,
        })
    }

    /// Unmap the window so that it is no longer displayed. If a lower-level
    /// cascaded menu is posted, unpost that menu.
    /// This subcommand does not work on Windows and the Macintosh, as those
    /// platforms have their own way of unposting menus.
    pub fn unpost( &self ) -> InterpResult<()> {
        self.0.tk().run(( self.0.path, "unpost" ))
    }

    /// Returns the x-coordinate within the menu window of the leftmost pixel
    /// in the entry specified by index.
    pub fn xposition( &self, index: impl Into<Index> ) -> InterpResult<c_longlong> {
        let obj = self.0.tk().eval(( self.0.path, "xposition", index.into() ))?;
        self.0.tk().longlong( obj )
    }

    /// Returns the y-coordinate within the menu window of the topmost pixel
    /// in the entry specified by index.
    pub fn yposition( &self, index: impl Into<Index> ) -> InterpResult<c_longlong> {
        let obj = self.0.tk().eval(( self.0.path, "yposition", index.into() ))?;
        self.0.tk().longlong( obj )
    }
}

impl<Inst:TkInstance> Delete<Inst> for TkMenu<Inst> {
    type Index = Index;
}

def_functions! {
    cascade     CascadeFn       ;
    checkbutton CheckbuttonFn   ;
    command     CommandFn       ;
    radiobutton RadiobuttonFn   ;
    separator   SeparatorFn     ;
}

def_tuple_notation!( "menu::cascade"      => CascadeTup       CascadeFn       CascadeOpt      );
def_tuple_notation!( "menu::checkbutton"  => CheckbuttonTup   CheckbuttonFn   CheckbuttonOpt  );
def_tuple_notation!( "menu::command"      => CommandTup       CommandFn       CommandOpt      );
def_tuple_notation!( "menu::radiobutton"  => RadiobuttonTup   RadiobuttonFn   RadiobuttonOpt  );
def_tuple_notation!( "menu::separator"    => SeparatorTup     SeparatorFn     SeparatorOpt    );

def_widget_opts! {
    CascadeOpt: (
        // standard
        crate::opt::TkActiveBackground,
        crate::opt::TkActiveBorderWidth,
        crate::opt::TkActiveForeground,
        crate::opt::TkBackground,
        crate::opt::TkBg,
        crate::opt::TkBorderWidth,
        crate::opt::TkBd,
        crate::opt::TkCursor,
        crate::opt::TkDisabledForeground,
        crate::opt::TkFont,
        crate::opt::TkForeground,
        crate::opt::TkRelief,
        crate::opt::TkTakeFocus,

        // widget-specific
        crate::opt::TkPostCommand,
        crate::opt::TkTearOff,
        crate::opt::TkTearOffCommand,
        crate::opt::TkTitle,
        crate::opt::TkType,

        // TkMenuEntryOpt
        crate::opt::TkAccelerator,
        crate::opt::TkBitmap,
        crate::opt::TkColumnBreak,
        crate::opt::TkCommand,
        crate::opt::TkCompound,
        crate::opt::TkHideMargin,
        crate::opt::TkImage,
        crate::opt::TkLabel,
        crate::opt::TkState,
        crate::opt::TkUnderline,
    ),
    CheckbuttonOpt: (
        crate::opt::TkAccelerator,
        crate::opt::TkActiveBackground,
        crate::opt::TkActiveForeground,
        crate::opt::TkBackground,
        crate::opt::TkBitmap,
        crate::opt::TkColumnBreak,
        crate::opt::TkCommand,
        crate::opt::TkCompound,
        crate::opt::TkFont,
        crate::opt::TkForeground,
        crate::opt::TkHideMargin,
        crate::opt::TkImage,
        crate::opt::TkIndicatorOn,
        crate::opt::TkLabel,
        crate::opt::TkOffValue,
        crate::opt::TkOnValue,
        crate::opt::TkSelectColor,
        crate::opt::TkSelectImage,
        crate::opt::TkState,
        crate::opt::TkUnderline,
        crate::opt::TkVariable,
    ),
    CommandOpt: (
        crate::opt::TkAccelerator,
        crate::opt::TkActiveBackground,
        crate::opt::TkActiveForeground,
        crate::opt::TkBackground,
        crate::opt::TkBitmap,
        crate::opt::TkColumnBreak,
        crate::opt::TkCommand,
        crate::opt::TkCompound,
        crate::opt::TkFont,
        crate::opt::TkForeground,
        crate::opt::TkHideMargin,
        crate::opt::TkImage,
        crate::opt::TkLabel,
        crate::opt::TkState,
        crate::opt::TkUnderline,
        crate::opt::TkVariable,
    ),
    RadiobuttonOpt: (
        crate::opt::TkAccelerator,
        crate::opt::TkActiveBackground,
        crate::opt::TkActiveForeground,
        crate::opt::TkBackground,
        crate::opt::TkBitmap,
        crate::opt::TkColumnBreak,
        crate::opt::TkCommand,
        crate::opt::TkCompound,
        crate::opt::TkFont,
        crate::opt::TkForeground,
        crate::opt::TkHideMargin,
        crate::opt::TkImage,
        crate::opt::TkIndicatorOn,
        crate::opt::TkLabel,
        crate::opt::TkSelectColor,
        crate::opt::TkSelectImage,
        crate::opt::TkState,
        crate::opt::TkUnderline,
        crate::opt::TkValue,
        crate::opt::TkVariable,
    ),
    SeparatorOpt: (
        crate::opt::TkColumnBreak,
        crate::opt::TkCompound,
        crate::opt::TkHideMargin,
    ),
}

impl<Inst:TkInstance> AddMenus<Inst> for TkMenu<Inst> {}
impl<Inst:TkInstance> AddMenus<Inst> for crate::TkMenubutton<Inst> {}
impl<Inst:TkInstance> AddMenus<Inst> for crate::TkToplevel<Inst> {}
impl<Inst:TkInstance> AddMenus<Inst> for crate::TkRoot<Inst> {}

pub trait AddMenus<Inst>
    where Self : Deref<Target=Widget<Inst>>
        , Inst : TkInstance
{
    fn add_menus<Widgs,Opts,Shape>( &self, path_opts_widgets: PathOptsWidgets<Opts,Widgs> )
        -> InterpResult<CreatedWidgets<Inst>>
        where Opts: IntoHomoTuple<TkMenuOpt>
                  + IntoHomoTuple<OptPair>
            , Widgs: ConvertTuple
            , <Widgs as ConvertTuple>::Output: DomForest::<(&'static str,&'static str),OptPair,Shape>
    {
        let path = path_opts_widgets.path;
        let opts = path_opts_widgets.opts;
        let widgets = path_opts_widgets.widgets.convert_tuple();

        let mut created_widgets = CreatedWidgets::new( self.deref().path );

        let top_menu = self.deref().add( "menu", PathOptsWidgets{ path, opts, widgets: () }).map( |w| TkMenu( w ))?;
        self.deref().tk().run(( self.deref().path, "configure", "-menu", top_menu.0.path ))?;

        let mut current_path = top_menu.0.path.to_owned();
        let mut menu_cmd = Vec::<Obj>::new();
        let mut add_cmd = Vec::<Obj>::new();
        let mut is_cascade = false;
        let mut is_branch = false;

        DomForest::<(&'static str,&'static str),OptPair,Shape>::try_preorder( widgets, &mut |visit| -> InterpResult<()> {
            match visit {
                Visit::Branch( (cmd, path) ) => {
                    is_branch = true;

                    if cmd == "menu::cascade" {
                        is_cascade = true;

                        let parent_path = current_path.clone();
                        current_path = self.deref().tk().next_path( &current_path, path );

                        menu_cmd.push( "menu".into() );
                        menu_cmd.push( current_path.as_str().into() );

                        add_cmd.push( parent_path.as_str().into() );
                        add_cmd.push( "add".into() );
                        add_cmd.push( "cascade".into() );
                        add_cmd.push( "-menu".into() );
                        add_cmd.push( current_path.as_str().into() );
                    } else {
                        // should be an error
                    }
                },
                Visit::Leaf( (mut cmd, path) ) => {
                    is_branch = false;

                    match cmd {
                        "menu::cascade"     => is_cascade = true,
                        "menu::command"     => { cmd = "command"    ; is_cascade = false; },
                        "menu::separator"   => { cmd = "separator"  ; is_cascade = false; },
                        "menu::checkbutton" => { cmd = "checkbutton"; is_cascade = false; },
                        "menu::radiobutton" => { cmd = "radiobutton"; is_cascade = false; },
                        _ => (),
                    }
                    if is_cascade {
                        let parent_path = current_path.clone();
                        current_path = self.deref().tk().next_path( &current_path, path );

                        menu_cmd.push( "menu".into() );
                        menu_cmd.push( current_path.as_str().into() );

                        add_cmd.push( parent_path.as_str().into() );
                        add_cmd.push( "add".into() );
                        add_cmd.push( "cascade".into() );
                        add_cmd.push( "-menu".into() );
                        add_cmd.push( current_path.as_str().into() );
                    } else {
                        add_cmd.push( current_path.as_str().into() );
                        add_cmd.push( "add".into() );
                        add_cmd.push( cmd.into() );
                    }
                },
                Visit::Frame => {
                    current_path = Widget::<Inst>::compute_parent_path( &current_path );
                },
                Visit::AttrsStart( _len ) => (),
                Visit::Attr( opt_pair ) => {
                    let menu_opts = [
                        "-activebackground", "-activeborderwidth", "-activeforeground",
                        "-background", "-bg", "-borderwidth", "-bd",
                        "-cursor", "-disabledforeground", "-font", "-foreground", "-relief",
                        "-takefocus", "-postcommand", "-selectcolor", "-tearoff", "-tearoffcommand",
                        "-title", "-type",
                    ];
                    let command = if is_cascade && menu_opts.contains( &opt_pair.name ) {
                        &mut menu_cmd
                    } else {
                        &mut add_cmd
                    };
                    if opt_pair.name.len() > 0 {
                        command.push( opt_pair.name.into() );
                    }
                    command.push( opt_pair.value );
                },
                Visit::AttrsEnd => {
                    if !menu_cmd.is_empty() {
                        self.tk().run( &*menu_cmd )?;
                        menu_cmd.clear();
                        created_widgets.widgets.push( UpcastableWidget {
                            widget : Widget::from_name_unchecked( &current_path, self.tk().inst ),
                            name   : "menu",
                        });
                        if !is_branch {
                            // No `Visit::Frame` for `Visit::Leaf`
                            current_path = Widget::<Inst>::compute_parent_path( &current_path );
                        }
                    }
                    if !add_cmd.is_empty() {
                        self.tk().run( &*add_cmd )?;
                        add_cmd.clear();
                    }
                },
            }
            Ok(())
        })?;

        Ok( created_widgets )
    }
}
