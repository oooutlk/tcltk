use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    NOT_SEND_SYNC,
    Tk,
    TkCoord,
    TkGeometry,
    TkHandler,
    TkInstance,
    TkResizable,
    TkRequester,
    TkSize,
    TkState,
    TkToplevel,
    PathOptsWidgets,
    InterpResult,
    Widget,
    error::{
        TkAcceptableSizeParseError,
        TkGeometryParseError,
        TkRequesterParseError,
        TkResizableParseError,
        WidgetNotFound,
    },
    opt::{
        TkWmAttributesOpt,
        OptPair,
    },
};

use std::{
    os::raw::c_int,
    ops::Deref,
};

use tcl::{
    Obj,
    error::{
        DeError,
        NotList,
        NotSeqOf,
        InterpError,
    },
    from_obj,
};

use tuplex::*;

#[derive( Copy, Clone, Debug, serde::Serialize, serde::Deserialize )]
pub enum TkFocusModel {
    #[serde( rename = "active" )]  Active,
    #[serde( rename = "passive" )] Passive,
}

impl From<TkFocusModel> for Obj {
    fn from( focus_model: TkFocusModel ) -> Obj {
        match focus_model {
            TkFocusModel::Active => "active".into(),
            TkFocusModel::Passive => "passive".into(),
        }
    }
}

#[derive( Copy, Clone, Debug )]
pub struct TkAcceptableSize {
    base_width  : c_int,
    base_height : c_int,
    width_inc   : c_int,
    height_inc  : c_int,
}

impl From<TkAcceptableSize> for Obj {
    fn from( acceptable_size: TkAcceptableSize ) -> Obj {
        (   acceptable_size.base_width,
            acceptable_size.base_height,
            acceptable_size.width_inc,
            acceptable_size.height_inc,
        ).into()
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    pub fn set_wm_aspect( &self,
        min_numer: impl Into<Obj>,
        min_denom: impl Into<Obj>,
        max_numer: impl Into<Obj>,
        max_denom: impl Into<Obj>,
    ) -> InterpResult<()> {
        self.tk().run(( "wm", "aspect", self.path, min_numer, min_denom, max_numer, max_denom ))
    }

    #[cex]
    pub fn wm_aspect( &self ) -> Result!( homo_tuple!(c_int; 4) throws DeError, InterpError ) {
        let obj = self.tk().eval(( "wm", "aspect", self.path ))?;
        ret!( tcl::from_obj( obj ));
    }

    pub fn wm_clear_aspect( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "aspect", self.path, "", "", "", "" ))
    }

    pub fn wm_attributes( &self ) -> InterpResult<Obj> {
        self.tk().eval(( "wm", "attributes", self.path ))
    }

    pub fn set_wm_attributes<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkWmAttributesOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );
        command.push( "wm".into() );
        command.push( "attributes".into() );
        command.push( self.path.into() );
        crate::cmd::append_opts( &mut command, opts.into().opts );

        self.tk().run( command )
    }

    pub fn set_wm_client( &self, name: &str ) -> InterpResult<()> {
        self.tk().run(( "wm", "client", self.path, name ))
    }

    pub fn wm_client( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "wm", "client", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn set_wm_colormapwindows( &self, windows: Vec<Widget<Inst>> ) -> InterpResult<()> {
        self.tk().run(( "wm", "colormapwindows", self.path, windows ))
    }

    #[cex]
    pub fn wm_colormapwindows( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let windows = self.tk().eval(( "wm", "colormapwindows", self.path ))?;
        ret!( self.tk().widgets_from_obj( windows ));
    }

    pub fn set_wm_command( &self, command: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "wm", "command", self.path, command ))
    }

    pub fn wm_command( &self ) -> InterpResult<Obj> {
        self.tk().eval(( "wm", "command", self.path ))
    }

    pub fn wm_deiconify( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "deiconify", self.path ))
    }

    pub fn set_wm_focusmodel( &self, focus_model: TkFocusModel ) -> InterpResult<()> {
        self.tk().run(( "wm", "focusmodel", self.path, focus_model ))
    }

    #[cex]
    pub fn wm_focusmodel( &self ) -> Result!( TkFocusModel throws DeError, InterpError ) {
        let obj = self.tk().eval(( "wm", "focusmodel", self.path ))?;
        ret!( tcl::from_obj( obj ));
    }

    pub fn wm_forget( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "forget", self.path ))
    }

    pub fn wm_frame( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "wm", "frame", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn set_wm_geometry( &self, geometry: TkGeometry ) -> InterpResult<()> {
        self.tk().run(( "wm", "geometry", self.path, geometry ))
    }

    #[cex]
    pub fn wm_geometry( &self ) -> Result!( TkGeometry throws InterpError, TkGeometryParseError ) {
        use std::str::FromStr;
        let s = self.tk().eval(( "wm", "geometry", self.path ))?.to_string();
        Ok( TkGeometry::from_str( &s )? )
    }

    pub fn set_wm_grid( &self, acceptable_size: TkAcceptableSize ) -> InterpResult<()> {
        self.tk().run(( "eval", "wm", "grid", self.path, acceptable_size ))
    }

    #[cex]
    pub fn wm_grid( &self ) -> Result!( TkAcceptableSize throws InterpError, NotList, TkAcceptableSizeParseError ) {
        let obj = self.tk().eval(( "eval", "wm", "grid", self.path ))?;

        let objs = obj.clone().get_elements()?.collect::<Vec<_>>();
        if objs.len() != 4 {
            throw!( TkAcceptableSizeParseError( obj ));
        }

        let mut values = [0; 4];
        for (i,v) in objs.into_iter().enumerate() {
            values[i] = self.tk().int( v )
                .map_err( |_| TkAcceptableSizeParseError( obj.clone() ))?;
        }

        ret!( TkAcceptableSize {
            base_width  : values[0],
            base_height : values[1],
            width_inc   : values[2],
            height_inc  : values[3],
        });
    }

    pub fn set_wm_group( &self, leader: Widget<Inst> ) -> InterpResult<()> {
        self.tk().run(( "wm", "group", self.path, leader.path ))
    }

    pub fn clear_wm_group( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "group", self.path, "" ))
    }

    pub fn wm_group( &self ) -> InterpResult<Option<Widget<Inst>>> {
        Ok( Widget::from_name( &self.tk().eval(( "wm", "group", self.path ))?.to_string(), self.inst )? )
    }

    pub fn set_wm_iconbitmap( &self, bitmap: &str ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconbitmap", self.path, bitmap ))
    }

    pub fn clear_wm_iconbitmap( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconbitmap", self.path, "" ))
    }

    pub fn wm_iconbitmap( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "wm", "iconbitmap", self.path ))
            .map( |obj| obj.to_string() )
    }

    #[cfg( target_os="windows" )]
    pub fn set_wm_iconbitmap_default( &self, bitmap: &str ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconbitmap", self.path, "-default", bitmap ))
    }

    pub fn wm_iconify( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconify", self.path ))
    }

    pub fn set_wm_iconmask( &self, bitmap: &str ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconmask", self.path, bitmap ))
    }

    pub fn clear_wm_iconmask( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconmask", self.path, "" ))
    }

    pub fn wm_iconmask( &self ) -> InterpResult<Option<String>> {
        self.tk()
            .eval(( "wm", "iconmask", self.path ))
            .map( |obj| {
                let bitmap = obj.to_string();
                if bitmap.is_empty() {
                    None
                } else {
                    Some( bitmap )
                }
            })
    }

    pub fn set_wm_iconname( &self, name: &str ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconname", self.path, name ))
    }

    pub fn wm_iconname( &self ) -> InterpResult<String> {
        self.tk()
            .eval(( "wm", "iconname", self.path ))
            .map( |obj| obj.to_string() )
    }

    pub fn set_wm_iconphoto( &self, image: &str, extra_images: Option<&[&str]> ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( extra_images.unwrap_or_default().len() + 4 );

        command.push( "wm".into() );
        command.push( "iconphoto".into() );
        command.push( self.path.into() );
        command.push( image.into() );

        if let Some( extra_images ) = extra_images {
            for extra_image in extra_images {
                command.push( (*extra_image).into() );
            }
        }

        self.tk().run( command )
    }

    pub fn set_wm_iconposition( &self, x: c_int, y: c_int ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconposition", self.path, x, y ))
    }

    pub fn clear_wm_iconposition( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconposition", self.path, "", "" ))
    }

    #[cex]
    pub fn wm_iconposition( &self ) -> Result!( TkCoord throws DeError, InterpError ) {
        let obj = self.tk().eval(( "wm", "iconposition", self.path ))?;
        ret!( from_obj( obj ));
    }

    pub fn set_wm_iconwindow( &self, icon_window: Widget<Inst> ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconwindow", self.path, icon_window.path ))
    }

    pub fn clear_wm_iconwindow( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "iconwindow", self.path, "" ))
    }

    pub fn wm_iconwindow( &self ) -> InterpResult<Option<Widget<Inst>>> {
        Ok( Widget::from_name( &self.tk().eval(( "wm", "iconwindow", self.path ))?.to_string(), self.inst )? )
    }

    pub fn set_wm_maxsize( &self, width: c_int, height: c_int ) -> InterpResult<()> {
        self.tk().run(( "wm", "maxsize", self.path, width, height ))
    }

    #[cex]
    pub fn wm_maxsize( &self ) -> Result!( TkSize throws DeError, InterpError ) {
        let obj = self.tk().eval(( "wm", "maxsize", self.path ))?;
        let v = tcl::from_obj::<[c_int; 2]>( obj )?;
        ret!( TkSize{ width: v[0], height: v[1] });
    }

    pub fn set_wm_minsize( &self, width: c_int, height: c_int ) -> InterpResult<()> {
        self.tk().run(( "wm", "minsize", self.path, width, height ))
    }

    #[cex]
    pub fn wm_minsize( &self ) -> Result!( TkSize throws DeError, InterpError ) {
        let obj = self.tk().eval(( "wm", "minsize", self.path ))?;
        let v = tcl::from_obj::<[c_int; 2]>( obj )?;
        ret!( TkSize{ width: v[0], height: v[1] });
    }

    pub fn set_wm_overrideredirect( &self, overrideredirect: bool ) -> InterpResult<()> {
        self.tk().run(( "wm", "overrideredirect", self.path, overrideredirect ))
    }

    pub fn wm_overrideredirect( &self ) -> InterpResult<bool> {
        self.tk()
            .eval(( "wm", "overrideredirect", self.path ))
            .and_then( |obj| self.tk().boolean( obj ))
    }

    pub fn set_wm_positionfrom( &self, who: TkRequester ) -> InterpResult<()> {
        self.tk().run(( "wm", "positionfrom", self.path, who ))
    }

    pub fn clear_wm_positionfrom( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "positionfrom", self.path, "" ))
    }

    #[cex]
    pub fn wm_positionfrom( &self ) -> Result!( Option<TkRequester> throws InterpError, TkRequesterParseError ) {
        let s = self.tk().eval(( "wm", "positionfrom" ))?.to_string();
        match s.as_str() {
            ""        => ret!( None ),
            "program" => ret!( Some( TkRequester::Program )),
            "user"    => ret!( Some( TkRequester::User )),
            _         => throw!( TkRequesterParseError( s )),
        }
    }

    pub unsafe fn set_wm_protocol( &self, name: &str, command: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "wm", "protocol", self.path, name, command ))
    }

    pub fn clear_wm_protocol_command( &self, name: &str ) -> InterpResult<Obj> {
        self.tk().eval(( "wm", "protocol", self.path, name, "" ))
    }

    pub unsafe fn wm_protocol_command( &self, name: &str ) -> InterpResult<Option<Obj>> {
        let obj = self.tk().eval(( "wm", "protocol", name ))?;
        if obj.is_empty() {
            Ok( None )
        } else {
            Ok( Some( obj ))
        }
    }

    #[cex]
    pub fn wm_protocol( &self ) -> Result!( Vec<TkHandler> throws NotSeqOf<TkHandler>, NotList, InterpError ) {
        let obj = self.tk().eval(( "wm", "protocol" ))?;
        let mut result = Vec::new();
        let mut name = None;

        for name_or_command in obj.clone().get_elements()? {
            if name.is_some() {
                result.push( TkHandler{ name: name.take().unwrap(), command: name_or_command });
            } else {
                name = Some( name_or_command.to_string() );
            }
        }

        if name.is_some() {
            throw!( NotSeqOf::new( obj ));
        }

        ret!( result );
    }

    pub fn set_wm_resizable( &self, width_resizable: bool, height_resizable: bool ) -> InterpResult<()> {
        self.tk().run(( "wm", "resizable", self.path, width_resizable, height_resizable ))
    }

    #[cex]
    pub fn wm_resizable( &self ) -> Result!( TkResizable throws InterpError, TkResizableParseError ) {
        let obj = self.tk().eval(( "wm", "resizable" ))?;
        match tcl::from_obj::<[bool; 2]>( obj.clone() ) {
            Ok( v ) => ret!( TkResizable{ width: v[0], height: v[1] }),
            Err( _ ) => throw!( TkResizableParseError( obj )),
        }
    }

    pub fn set_wm_sizefrom( &self, who: TkRequester ) -> InterpResult<()> {
        self.tk().run(( "wm", "sizefrom", self.path, who ))
    }

    pub fn clear_wm_sizefrom( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "sizefrom", self.path, "" ))
    }

    #[cex]
    pub fn wm_sizefrom( &self ) -> Result!( Option<TkRequester> throws InterpError, TkRequesterParseError ) {
        let s = self.tk().eval(( "wm", "sizefrom" ))?.to_string();
        match s.as_str() {
            ""        => ret!( None ),
            "program" => ret!( Some( TkRequester::Program )),
            "user"    => ret!( Some( TkRequester::User )),
            _         => throw!( TkRequesterParseError( s )),
        }
    }

    #[cex]
    pub fn wm_stackorder( &self ) -> Result!( Vec<Widget<Inst>> throws InterpError, NotList, WidgetNotFound ) {
        let windows = self.tk().eval(( "wm", "stackorder", self.path ))?;
        ret!( self.tk().widgets_from_obj( windows ));
    }

    pub fn wm_stackorder_isabove( &self, widget: &Widget<Inst> ) -> InterpResult<bool> {
        self.tk()
            .eval(( "wm", "stackorder", self.path, "isabove", widget.path ))
            .and_then( |obj| self.tk().boolean( obj ))
    }

    pub fn wm_stackorder_isbelow( &self, widget: &Widget<Inst> ) -> InterpResult<bool> {
        self.tk()
            .eval(( "wm", "stackorder", self.path, "isbelow", widget.path ))
            .and_then( |obj| self.tk().boolean( obj ))
    }

    pub fn set_wm_state( &self, new_state: TkState ) -> InterpResult<()> {
        self.tk().run(( "wm", "state", self.path, new_state ))
    }

    #[cex]
    pub fn wm_state( &self ) -> Result!( TkState throws DeError, InterpError ) {
        let obj = self.tk().eval(( "wm", "state", self.path ))?;
        ret!( tcl::from_obj( obj ));
    }

    pub fn set_wm_title( &self, title: impl Into<Obj> ) -> InterpResult<()> {
        self.tk().run(( "wm", "title", self.path, title.into() ))
    }

    pub fn wm_title( &self ) -> InterpResult<String> {
        Ok( self.tk().eval(( "wm", "title", self.path, "" ))?.to_string() )
    }

    pub fn set_wm_transient( &self, master: &TkToplevel<Inst> ) -> InterpResult<()> {
        self.tk().run(( "wm", "transient", self.path, master.path ))
    }

    pub fn clear_wm_transient( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "transient", self.path, "" ))
    }

    pub fn wm_transient( &self ) -> InterpResult<Option<TkToplevel<Inst>>> {
        let path = self.tk().eval(( "wm", "transient", self.path ))?.to_string();
        if path.is_empty() {
            Ok( None )
        } else {
            Ok( Some( TkToplevel( Widget{ path: Tk::<Inst>::make_or_get_path( &path ), inst: self.inst, mark: NOT_SEND_SYNC })))
        }
    }

    pub fn wm_withdraw( &self ) -> InterpResult<()> {
        self.tk().run(( "wm", "withdraw", self.path ))
    }
}

pub trait WmManage<Inst:TkInstance>
    where Self : Deref<Target=Widget<Inst>>
{
    fn wm_manage( &self ) -> InterpResult<()> {
        self.deref().tk().run(( "wm", "manage", self.deref().path ))
    }
}
