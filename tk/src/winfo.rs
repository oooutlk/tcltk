use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    NOT_SEND_SYNC,
    Tk,
    TkColor,
    TkCoord,
    TkDistance,
    TkGeometry,
    TkInstance,
    TkRGB,
    TkScreenName,
    TkToplevel,
    TkVisualClass,
    Widget,
    error::{
        TkGeometryParseError,
        TkScreenNameParseError,
        WidgetNotFound,
    },
};

use std::{
    os::raw::{
        c_double,
        c_int,
        c_longlong,
        c_ulong,
    },
    str::FromStr,
};

use tcl::{
    error::{
        DeError,
        NotList,
        NotSeqOf,
        InterpError,
    },
    from_obj,
};

impl<Inst:TkInstance> Widget<Inst> {
    pub fn winfo_atom( &self, name: &str ) -> InterpResult<c_longlong> {
        self.tk()
            .eval(( "winfo", "atom", "-displayof", self.path, name ))
            .and_then( |obj| self.tk().longlong( obj ))
    }

    pub fn winfo_atom_name( &self, id: c_longlong ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "atomname", "-displayof", self.path, id ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_cells( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "cells", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    #[cex]
    pub fn winfo_children( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let children = self.tk().eval(( "winfo", "children", self.path ))?;
        ret!( self.tk().widgets_from_obj( children )? );
    }

    pub fn winfo_class( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "class", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_colormap_full( &self ) -> InterpResult<bool> {
        self.tk()
            .eval(( "winfo", "colormapfull", self.path ))
            .and_then( |obj| self.tk().boolean( obj ))
    }

    pub fn winfo_containing( &self, root_x: c_int, root_y: c_int ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "containing", "-dispayof", self.path, root_x, root_y ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_depth( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "depth", "-dispayof", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_fpixels( &self, number: TkDistance ) -> InterpResult<c_double> {
        self.tk()
            .eval(( "winfo", "fpixels", self.path, number ))
            .and_then( |obj| self.tk().double( obj ))
    }

    #[cex]
    pub fn winfo_geometry( &self ) -> Result!( TkGeometry throws InterpError, TkGeometryParseError ) {
        let s = self.tk().eval(( "winfo", "geometry", self.path ))?.to_string();
        Ok( TkGeometry::from_str( &s )? )
    }

    pub fn winfo_height( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "height", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_id( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "id", self.path ))
            .map( |obj| obj.to_string() )
    }

    /// Returns a list whose members are the names of all Tcl interpreters (e.g. all Tk-based applications) currently
    /// registered for the display of window.
    #[cex]
    pub fn winfo_interps( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.tk().eval(( "winfo", "interps", "-displayof", self.path ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn winfo_ismapped( &self ) -> InterpResult<bool> {
        self.tk()
            .eval(( "winfo", "ismapped", self.path ))
            .and_then( |obj| self.tk().boolean( obj ))
    }

    /// Returns the name of the geometry manager currently responsible for window, or an empty string if window is not
    /// managed by any geometry manager. The name is usually the name of the Tcl command for the geometry manager, such
    /// as pack or place. If the geometry manager is a widget, such as canvases or text, the name is the widget's class
    /// command, such as canvas.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// fn main() -> TkResult<()> {
    ///     let mut tk = make_tk!()?;
    ///     let root = tk.root();
    ///     let label = root.add_label( -text("lorum") )?.pack(())?;
    ///     Ok( assert_eq!( label.winfo_manager().unwrap().as_str(), "pack" ))
    /// }
    pub fn winfo_manager( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "manager", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_name( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "name", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_parent( &self ) -> InterpResult<Option<String>> {
        let parent = self.tk().eval(( "winfo", "parent", self.path ))?.to_string();
        if parent.is_empty() {
            Ok( None )
        } else {
            Ok( Some( parent ))
        }
    }

    pub fn winfo_pathname( &self, id: c_ulong ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "pathname", "-displayof", self.path, id ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_pixels( &self, number: TkDistance ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "pixels", self.path, number ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_pointerx( &self ) -> InterpResult<Option<c_int>> {
        self.tk()
            .eval(( "winfo", "pointerx", self.path ))
            .and_then( |obj| self.tk().int( obj ))
            .map( |x| if x >= 0 { Some( x )} else { None })
    }

    #[cex]
    pub fn winfo_pointerxy( &self ) -> Result!( Option<TkCoord> throws DeError, InterpError ) {
        let obj = self.tk().eval(( "winfo", "pointerxy", self.path ))?;
        ret!( from_obj::<TkCoord>( obj ).map( |coord| coord.wrap() ));
    }

    pub fn winfo_pointery( &self ) -> InterpResult<Option<c_int>> {
        self.tk()
            .eval(( "winfo", "pointery", self.path ))
            .and_then( |obj| self.tk().int( obj ))
            .map( |x| if x >= 0 { Some( x )} else { None })
    }

    pub fn winfo_reqheight( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "reqheight", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_reqwidth( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "reqwidth", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    #[cex]
    pub fn winfo_rgb( &self, color: TkColor ) -> Result!( TkRGB throws DeError, InterpError ) {
        let obj = self.tk().eval(( "winfo", "rgb", self.path, color ))?;
        ret!( from_obj::<TkRGB>( obj ));
    }

    pub fn winfo_rootx( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "rootx", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_rooty( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "rooty", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    #[cex]
    pub fn winfo_screen( &self ) -> Result!( TkScreenName throws InterpError, TkScreenNameParseError ) {
        ret!( TkScreenName::from_str( self.tk().eval(( "winfo", "screen", self.path ))?.to_string().as_str() ));
    }

    pub fn winfo_screencells( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "screencells", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_screendepth( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "screendepth", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_screenheight( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "screenheight", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_screenmmheight( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "screenmmheight", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_screenmmwidth( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "screenmmwidth", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    #[cex]
    pub fn winfo_screenvisual( &self ) -> Result!( TkVisualClass throws DeError, InterpError ) {
        let obj = self.tk().eval(( "winfo", "screenvisual", self.path ))?;
        ret!( from_obj::<TkVisualClass>( obj ));
    }

    pub fn winfo_screenwidth( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "screenwidth", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_server( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "server", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_toplevel( &self ) -> InterpResult<TkToplevel<Inst>> {
        self.tk()
            .eval(( "winfo", "toplevel", self.path ))
            .map( |obj| TkToplevel( Widget{ path: Tk::<Inst>::make_or_get_path( &obj.to_string() ), inst: self.inst, mark: NOT_SEND_SYNC }))
    }

    pub fn winfo_viewable( &self ) -> InterpResult<bool> {
        self.tk()
            .eval(( "winfo", "viewable", self.path ))
            .and_then( |obj| self.tk().boolean( obj ))
    }

    #[cex]
    pub fn winfo_visual( &self ) -> Result!( TkVisualClass throws DeError, InterpError ) {
        let obj = self.tk().eval(( "winfo", "visual", self.path ))?;
        ret!( from_obj::<TkVisualClass>( obj ));
    }

    pub fn winfo_visualid( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "winfo", "visualid", self.path ))
            .map( |obj| obj.to_string() )
    }

    #[cex]
    pub fn winfo_visualsavailable( &self ) -> Result!( Vec<(TkVisualClass, c_int)> throws DeError, InterpError, NotList, NotSeqOf<(TkVisualClass, c_int)> ) {
        let obj = self.tk().eval(( "winfo", "visualsavailable", self.path ))?;

        let list = obj.clone().get_elements()?.collect::<Vec<_>>();
        if list.len() % 2 != 0 {
            throw!( NotSeqOf::<(TkVisualClass, c_int)>::new( obj ));
        }
        let mut result = Vec::new();
        let mut visual_class = TkVisualClass::DirectColor; // provide an initial value to make rustc happy

        for (i, obj) in list.into_iter().enumerate() {
            match i % 2 {
                0 => visual_class = from_obj::<TkVisualClass>( obj )?,
                1 => {
                    let depth = self.tk().int( obj )?;
                    result.push(( visual_class, depth ));
                },
                _ => unreachable!(),
            }
        }
        ret!( result );
    }

    #[cex]
    pub fn winfo_visualsavailable_includeids( &self ) -> Result!( Vec<(TkVisualClass, c_int, String)>
        throws DeError, InterpError, NotList, NotSeqOf<(TkVisualClass, c_int, String)> )
    {
        let obj = self.tk().eval(( "winfo", "visualsavailable", self.path, "includeids" ))?;

        let list = obj.clone().get_elements()?.collect::<Vec<_>>();
        if list.len() % 3 != 0 {
            throw!( NotSeqOf::<(TkVisualClass, c_int, String)>::new( obj ));
        }

        let mut result = Vec::new();

        // provide initial values to make rustc happy
        let mut visual_class = TkVisualClass::DirectColor;
        let mut depth = 0;

        for (i, obj) in list.into_iter().enumerate() {
            match i % 3 {
                0 => visual_class = from_obj::<TkVisualClass>( obj )?,
                1 => depth = self.tk().int( obj )?,
                2 => {
                    result.push(( visual_class, depth, obj.to_string() ));
                },
                _ => unreachable!(),
            }
        }
        ret!( result );
    }

    pub fn winfo_vrootheight( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "vrootheight", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_vrootwidth( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "vrootwidth", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_vrootx( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "vrootx", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_vrooty( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "vrooty", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_width( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "width", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_x( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "x", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }

    pub fn winfo_y( &self ) -> InterpResult<c_int> {
        self.tk()
            .eval(( "winfo", "y", self.path ))
            .and_then( |obj| self.tk().int( obj ))
    }
}

impl<Inst:TkInstance> Tk<Inst> {
    pub fn winfo_atom( &self, name: &str ) -> InterpResult<c_longlong> {
        self.eval(( "winfo", "atom", name ))
            .and_then( |obj| self.longlong( obj ))
    }

    pub fn winfo_atomname( &self, id: c_longlong ) -> InterpResult<String> {
        self.eval(( "winfo", "atomname", id ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_containing( &self, root_x: c_int, root_y: c_int ) -> InterpResult<String> {
        self.eval(( "winfo", "containing", root_x, root_y ))
            .map( |obj| obj.to_string() )
    }

    pub fn winfo_exists( &self, name: &str ) -> InterpResult<bool> {
        self.eval(( "winfo", "exists", name ))
            .and_then( |obj| self.boolean( obj ))
    }

    /// Returns a list whose members are the names of all Tcl interpreters (e.g. all Tk-based applications) currently
    /// registered for the main window.
    #[cex]
    pub fn winfo_interps( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let obj = self.eval(( "winfo", "interps" ))?;
        Ok( obj .get_elements()?
                .map( |obj| obj.to_string() )
                .collect::<Vec<_>>() )
    }

    pub fn winfo_pathname( &self, id: c_ulong ) -> InterpResult<String> {
        self.eval(( "winfo", "pathname", id ))
            .map( |obj| obj.to_string() )
    }
}
