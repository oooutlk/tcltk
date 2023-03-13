//! High-level bindings to Tk 8.6
//!
//! The crate tk is bindings to Tk commands, aiming at:
//!
//! * Make Tk programers feel at home if possible.
//!
//! * Provide for non-Tk-programers easy-to-use API to start writing Tk GUI programs under constraints of Rust types,
//! without the need of concatenating command strings of too flexible arguments.
//!
//! # A quick glance
//!
//! ```rust
//! use tk::*;
//! use tk::cmd::*;
//!
//! fn main() -> TkResult<()> {
//!     let tk = make_tk!()?;
//!     let root = tk.root();
//!     root.add_label( -text("constructs widgets and layout step by step") )?
//!         .pack(())?;
//!     let f = root
//!         .add_frame(())?
//!         .pack(())?;
//!     let _btn = f
//!         .add_button( "btn" -text("quit") -command("destroy .") )?
//!         .pack(())?;
//!     Ok( main_loop() )
//! }
//! ```
//!
//! # Another glance
//!
//! ```rust
//! use tk::*;
//! use tk::cmd::*;
//!
//! fn main() -> TkResult<()> {
//!     let tk = make_tk!()?;
//!     tk.root().add_widgets(
//!         -pack( -label( -text("constructs widgets and layout in one expression") ))
//!         -pack( -frame( -pack( -button( "btn" -text("quit") -command("destroy .") ))))
//!     )?;
//!     Ok( main_loop() )
//! }
//! ```
//!
//! # The naming conventions in translating Tk commands to Rust bindings
//!
//! 1. Prefix Tk widget constructors with `add_` and put parentheses around option values.
//!
//!     The Tk command to add a widget looks like `Constructor path -options_and_values`, e.g.
//!
//!     ```tcl
//!     label .lb -text "lorem ipsum" -width 50 -height 20
//!     ```
//!
//!     The equivalent Rust statement is as follows.
//!
//!     ```rust_no_run
//!     let lb = root.add_label( /*".lb"*/ -text("lorem ipsum") -width(50) -height(20) )?;
//!     ```
//!
//! 2. Converts Tcl's imperative style to Rust's object style
//!
//!     The Tk command is in the form of "verb noun options", e.g.
//!
//!     ```tcl
//!     pack .lb -fill both
//!     ```
//!
//!     The equivalent Rust statement is in th form of "object method options", as follows.
//!
//!     ```rust_no_run
//!     lb.pack( -fill("both") )?; // use pack(()) without any option.
//!     ```
//!
//! 3. Converts Tk's space-separated commands to Rust's underscore-separated function names.
//!
//!     Tk commands are space-separated, e.g.
//!
//!     ```tcl
//!     tk fontchooser show
//!     ```
//!
//!     The equivalent Rust statement is as follows.
//!
//!     ```rust_no_run
//!     tk.fontchooser_show()?;
//!     ```
//!
//!     Users can look into the Tk command reference and find the "fontchooser" page then search "show".
//!
//! 4. Distinguish between set and get via the `set_` prefix.
//!
//!     In Tk, it is common to distinguish set and get by providing or omitting the value argument, e.g.
//!
//!     `wm title window "Lorem ipsum"` means to set the window's title to "Lorem ipsum",
//!     while `wm title window` means to get the windows' title.
//!
//!     The equivalent Rust statements are as follows.
//!
//!     ```rust_no_run
//!     window.set_wm_title( "Lorem ipsum" )?;
//!     window.wm_title()?;
//!     ```
//!
//! # Why I gave up writing most documents
//!
//! I planned to write doc comments for every public types, traits, macros and functions in this crate.
//! Isn't it very bold? All APIs, 100% well documented in doc comments.
//! My earlier crates' APIs are nearly 100% documented.
//! I thought a well documented crate would make all users excited.
//! It is estimated to take only 20-25 days to fill all APIs with doc comments.
//! But finally I gave up. Why? I read through the [official Tk command references](https://www.tcl.tk/man/tcl/TkCmd),
//! and could not make my mind to translate it in Rust.
//! Do you know what the biggest problem is? Certainly it's an engineering problem.
//! Firstly, in a brief, the biggest problem is development efficiency.
//! No time for documenting, for an experimental crate in its version 0.1.0.
//! Fundamental APIs may evolve in later versions 0.2, 0.3...etc, just like they evolved in unpublished 0.0.x versions.
//! Once changed, it may be necessary to change a lot of doc comments, taking another time budget of 20-25 days.
//! Too slow for rapid developing.
//! Secondly, the probability of wasting the effort to providing document in Rust is more than 50%.
//! The users of this crate may be familiar with Tcl/Tk programming,
//! and they can master immediately 95% usage of this crate after reading a few demonstration code.
//! Even users who are not familiar with Tk, can translate between Tcl and Rust themselves
//! when reading the Tk command reference, with the help of *naming conventions* described above.
//! I have learned that 100% documenting this experimental crate will do harm to development efficiency,
//! and lose a chance to take advantage of the existing high quality documentation of Tcl/Tk library,
//! to which this crate provides bindings. Which is more important, 100% document coverage, or rapid development?
//! It is a pity that `cargo doc` produces little content here, but lucky for me to be sane.
//!
//! # The Tutorial book
//!
//! Luckily, I've got enough time to translate in Rust the [Tk tutorial](https://oooutlk.github.io/tk/).

use enumx::export::*;
use enumx::predefined::*;
use cex::*;

pub(crate) use std::{
    any::TypeId,
    cell::{Cell, RefCell},
    collections::HashSet,
    marker::PhantomData,
    mem,
    ops::Deref,
    os::raw::c_int,
};

#[cfg( feature = "libtk" )]
pub(crate) use std::os::raw::c_char;

pub(crate) use tcl::{
    Interpreter,
    Obj,
    error::{
        InterpError,
        NullInterp,
        TclInitError,
    },
};

#[macro_export]
macro_rules! make_tk {
    () => { Tk::new( ||() ) }
}

pub mod bitmap;
pub use bitmap::Bitmap;

pub mod error;
pub use error::{
    TkError,
    TkResult,
};

pub mod cmd;
pub use cmd::{
    TkRoot,
    Widget,
    path_seg,
};
pub(crate) use cmd::{
    PathOptsWidgets,
};

pub mod key_syms;
pub use key_syms::TkKey;

mod lower;

#[macro_use]
pub mod opt;
use opt::{OptPair, TkOption};

mod option;

pub mod photo;
pub use photo::Photo;

mod raise;

pub mod ttk_style;

pub mod ttk_widget;
pub use ttk_widget::{
    TtkCommonTraits,
    TtkState,
    TtkStateSpec,
};

pub mod bind;

pub mod event;
pub use event::TkEventSeq;

pub mod image;
pub use image::Image;

mod grid;
mod pack;
mod focus;
mod winfo;

pub mod wm;
pub use wm::{
    TkFocusModel,
    WmManage,
};

pub mod range;
pub use range::{
    TkDefaultStart,
    TkDefaultEnd,
};

pub mod traits;
pub use traits::{Delete, TkBBoxTrait, TkEntryTraits, TkXView};

pub mod types;
pub use types::{
    TkBBox,
    TkColor,
    TkCoord,
    TkHandler,
    TkGeometry,
    TkDistance,
    TkDLine,
    TkRGB,
    TkRectangle,
    TkResizable,
    TkRequester,
    TkState,
    TkSize,
    TkScreenName,
    TkVisualClass,
    TkWindowingSystem,
    TtkInsertPos,
    TtkTreeviewRegion,
};

pub mod button;
pub use button::TkButton;

pub mod canvas;
pub use canvas::TkCanvas;

pub mod checkbutton;
pub use checkbutton::TkCheckbutton;

pub mod entry;
pub use entry::{
    TkEntry,
    Index as TkEntryIndex,
};

pub mod frame;
pub use frame::TkFrame;

pub mod label;
pub use label::TkLabel;

pub mod labelframe;
pub use labelframe::TkLabelframe;

pub mod listbox;
pub use listbox::{
    Index as TkListboxIndex,
    TkListbox,
};

pub mod menu;
pub use menu::{
    Index as TkMenuIndex,
    TkMenu,
    TkMenuCloneType,
    TkMenuEntryType,
};

pub mod menubutton;
pub use menubutton::TkMenubutton;

pub mod message;
pub use message::TkMessage;

pub mod panedwindow;
pub use panedwindow::{TkPanedwindow, TkSashOrHandle};

pub mod radiobutton;
pub use radiobutton::TkRadiobutton;

pub mod scale;
pub use scale::{TkScale, TkScaleCoord, TkScalePart};

pub mod scrollbar;
pub use scrollbar::{TkScrollbar, TkScrollbarElement, TkScrollbarDelta};

pub mod spinbox;
pub use spinbox::{TkSpinbox, TkSpinboxElement, TkSpinboxInvokableElement};

pub mod text;
pub use text::{
    TkCmp,
    TkDump,
    TkText,
    TkTextMarkGravity,
    TkTextSearch,
    TkTextSearchAll,
};

pub mod toplevel;
pub use toplevel::TkToplevel;

pub mod ttk_button;
pub use ttk_button::TtkButton;

pub mod ttk_checkbutton;
pub use ttk_checkbutton::TtkCheckbutton;

pub mod ttk_combobox;
pub use ttk_combobox::{
    TtkCombobox,
    Index as TtkComboboxIndex,
};

pub mod ttk_entry;
pub use ttk_entry::{
    Index as TtkEntryIndex,
    TtkEntry,
};

pub mod ttk_frame;
pub use ttk_frame::TtkFrame;

pub mod ttk_label;
pub use ttk_label::TtkLabel;

pub mod ttk_labelframe;
pub use ttk_labelframe::TtkLabelframe;

pub mod ttk_menubutton;
pub use ttk_menubutton::TtkMenubutton;

pub mod ttk_notebook;
pub use ttk_notebook::{TtkNotebook, TtkNotebookTabId};

pub mod ttk_panedwindow;
pub use ttk_panedwindow::TtkPanedwindow;

pub mod ttk_progressbar;
pub use ttk_progressbar::{TtkProgressbar, TtkProgressbarInterval};

pub mod ttk_radiobutton;
pub use ttk_radiobutton::TtkRadiobutton;

pub mod ttk_scale;
pub use ttk_scale::TtkScale;

pub mod ttk_scrollbar;
pub use ttk_scrollbar::TtkScrollbar;

pub mod ttk_separator;
pub use ttk_separator::TtkSeparator;

pub mod ttk_sizegrip;
pub use ttk_sizegrip::TtkSizegrip;

pub mod ttk_spinbox;
pub use ttk_spinbox::TtkSpinbox;

pub mod ttk_treeview;
pub use ttk_treeview::{
    Index as TtkTreeviewIndex,
    TtkTreeview,
    TtkTreeviewColumnId,
};

pub mod font;
pub use font::Font;

const TEST_MAIN_WINDOW: &'static str = "winfo exists .\0";

/// Loop for events until all windows are deleted.
pub fn main_loop() {
    loop {
        unsafe{ clib::Tcl_DoOneEvent( 0 ); }

        let no_main_window = TK_INSTANCES.with( |instances| {
            let script = TEST_MAIN_WINDOW.as_ptr() as *const _;
            for (_, engine) in instances.borrow().iter() {
                let tcl_interp = engine.interpreter.as_ptr();
                if unsafe{ clib::Tcl_Eval( tcl_interp, script )} == clib::TCL_OK as c_int {
                    return false;
                }
            }
            true
        });

        if no_main_window {
            break;
        }
    }
}

/// Error from Tcl interpreter.
pub type InterpResult<T> = Result<T, tcl::error::InterpError>;

/// Main program for Tk-based applications.
#[cfg( feature = "libtk" )]
pub fn main( args: impl Iterator<Item=String>, mut init: clib::Tcl_AppInitProc ) {
    let mut v: Vec<_> = args
        .map( |arg| std::ffi::CString::new( Vec::<u8>::from( arg ))
            .expect( "String should not contain nul character" )
            .into_raw() )
        .collect();
    let argc = v.len() as c_int;
    let argv = v.as_mut_ptr() as *mut *mut c_char;
    mem::forget( v );

    if init == None {
        init = Some( tcl_app_init_proc );
    }

    unsafe {
        clib::Tk_MainEx( argc, argv, init, Interpreter::new().unwrap().as_ptr() );
    }
}

#[doc( hidden )]
#[cfg( feature = "libtk" )]
unsafe extern "C" fn tcl_app_init_proc( interp: *mut clib::Tcl_Interp ) -> c_int {
    const TCL_OK: c_int = clib::TCL_OK as c_int;

    let result = clib::Tk_Init( interp );
    if result != TCL_OK {
        eprintln!( "unable to Initialize Tk!\n" );
        return result;
    }

    return TCL_OK;
}

#[doc( hidden )]
pub struct Engine {
    interpreter : Interpreter,
    serial      : Cell<usize>, // for widgets
}

impl Engine {
    fn incr_serial( &self ) -> usize {
        self.serial.set( self.serial.get() + 1 );
        self.serial.get()
    }
}

impl Deref for Engine {
    type Target = Interpreter;

    fn deref( &self ) -> &Self::Target { &self.interpreter }
}

pub type NotSendSync = PhantomData<*const ()>;

const NOT_SEND_SYNC: NotSendSync = PhantomData;

/// Tk instance.
#[derive( Copy, Clone )]
pub struct Tk<Inst:TkInstance>
{
    inst : Inst,
    mark : NotSendSync,
}

impl<Inst:TkInstance> Deref for Tk<Inst> {
    type Target = Engine;

    fn deref( &self ) -> &Self::Target {
        TK_INSTANCES.with( |instances| {
            let tk_type_id = TypeId::of::<Inst>();
            for (type_id, engine) in instances.borrow().iter() {
                if &tk_type_id == type_id {
                    return unsafe{ &*( engine as *const _ )};
                }
            }
            unreachable!()
        })

    }
}

#[doc( hidden )]
pub trait TkInstance : 'static + Copy + Clone {}

impl<T:'static + Copy + Clone> TkInstance for T {}

thread_local! {
    static TK_INSTANCES: RefCell<Vec<(TypeId, Engine)>> = RefCell::new( Vec::new() );

    static WIDGET_PATH_SET: RefCell<HashSet<&'static str>> = {
        let mut set = HashSet::new();
        set.insert( "." );
        RefCell::new( set )
    };
}

const TK_INIT_SCRIPT: &'static str = r#"
package require Tk
proc tk_rs_option_menu {pathName varName items} {
    global tk_rs_widget_extra_data
    set tk_rs_widget_extra_data($pathName) [eval tk_optionMenu {$pathName} {$varName} $items]
}
"#;

impl<Inst> Tk<Inst>
    where Inst: 'static + Copy + Clone
{
    /// Creates a new instance of Tk.
    /// The recommended invocation is `Tk::new(||{})`.
    #[cex]
    pub fn new( inst: Inst ) -> Result!( Tk<Inst> throws InterpError, NullInterp, TclInitError ) {
        let tk_type_id = TypeId::of::<Inst>();
        let not_unique = TK_INSTANCES.with( |instances| instances
            .borrow()
            .iter()
            .find( |(type_id, _)| &tk_type_id == type_id )
            .is_some()
        );

        if not_unique {
            panic!( "Tk instance exists already" );
        }

        let interpreter = Interpreter::new()?;

        if interpreter.eval( "package present Tk" ).is_err() {
            interpreter.eval( TK_INIT_SCRIPT )?;
        }

        let tk = Tk{ inst, mark: NOT_SEND_SYNC };
        let engine = Engine{ interpreter, serial: Cell::new(0) };

        TK_INSTANCES.with( |instances| {
            instances.borrow_mut().push(( tk_type_id, engine ));
        });

        Ok( tk )
    }

    pub(crate) fn from_inst( inst: Inst ) -> Tk<Inst> { Tk{ inst, mark: NOT_SEND_SYNC }}

    pub(crate) fn make_or_get_path( path: &str ) -> &'static str {
        WIDGET_PATH_SET.with( |path_set| {
            let mut path_set = path_set.borrow_mut();
            match path_set.get( path ) {
                Some( path ) => *path,
                None => {
                    let path: &'static str = Box::leak( path.to_owned().into_boxed_str() );
                    path_set.insert( path );
                    path
                },
            }
        })
    }
}

impl<Inst:TkInstance> Tk<Inst> {
    /// Reference to the root widget of tk. The root's widget path is ".".
    pub fn root( &self ) -> TkRoot<Inst> {
        TkRoot(
            Widget {
                path : ".",
                inst : self.inst,
                mark : NOT_SEND_SYNC,
            }
        )
    }

    pub(crate) fn next_path( &self, parent_path: &str, path: &str ) -> String {
        if path.len() == 0 {
            let serial = self.deref().incr_serial();
            if parent_path == "." {
                format!( ".{}", serial )
            } else {
                format!( "{}.{}", parent_path, serial )
            }
        } else {
            if parent_path == "." {
                format!( ".{}", path )
            } else {
                format!( "{}.{}", parent_path, path )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cex]
    fn it_works() -> Result!( () throws InterpError, NullInterp, TclInitError ) {
        let _tk = Tk::new(|| ())?;
        let _tk = Tk::new(|| ())?;
        Ok( main_loop() )
    }
}
