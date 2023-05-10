/// This module contains some widgets which does not exist in Tcl/Tk library.
///
/// 1. HBox: A frame containing two frames(so called west/east), placed horizontally and are resizable.
///
/// 2. VBox: A frame containing two frames(so called north/south), placed vertically and are resizable.

use crate::{
    InterpResult,
    TkInstance,
    OptPair,
    PathOptsWidgets,
    TkResult,
    TtkFrame,
    Widget,
    cmd::*,
    event,
    opt,
    path_seg,
};

use std::{
    ffi::c_int,
    ops::Deref,
};

use tcl::*;
use tuplex::*;

/// A frame containing two frames(so called west/east), placed horizontally and are resizable.
pub struct HBox<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> Deref for HBox<Inst> {
    type Target = Widget<Inst>;

    fn deref( &self ) -> &Self::Target { &self.0 }
}

impl<Inst:TkInstance> TkPackSlave  for HBox<Inst> {}
impl<Inst:TkInstance> TkGridSlave  for HBox<Inst> {}
impl<Inst:TkInstance> TkPlaceSlave for HBox<Inst> {}

impl<Inst:TkInstance> HBox<Inst> {
    /// Returns the left frame in the HBox.
    pub fn west( &self ) -> TtkFrame<Inst> {
        let path = format!( "{}.west", self.0.path );
        TtkFrame( Widget::from_name_unchecked( &path, self.0.inst ))
    }

    /// Returns the right frame in the HBox.
    pub fn east( &self ) -> TtkFrame<Inst> {
        let path = format!( "{}.east", self.0.path );
        TtkFrame( Widget::from_name_unchecked( &path, self.0.inst ))
    }
}

/// Defines the behaviour on HBox's resizing:
///
/// `HBoxResize::Both`, resize both west and east frames
///
/// `HBoxResize::West`, resize west frame while leave east frame unchanged.
///
/// `HBoxResize::East`, resize east frame while leave west frame unchanged.
pub enum HBoxResize {
    Both,
    West,
    East,
}

pub trait AddHBox {
    fn add_hbox<Opts,Inst:TkInstance>(
        &self,
        initial_ratio : f64,
        hbox_resize   : HBoxResize,
        path_opts     : impl Into<PathOptsWidgets<Opts,()>>
    ) -> InterpResult<HBox<Inst>>
        where Self : Sized
                   + Deref<Target=Widget<Inst>>
            , Opts : IntoHomoTuple<opt::TtkFrameOpt>
                   + IntoHomoTuple<OptPair>
    {
        let tk = self.deref().tk();
        let ratio = initial_ratio;

        let hbox = self.add_ttk_frame( path_opts )?.pack( -fill("both") -expand(true) )?;
        tk.update()?;
        let w: c_int = tk.int( hbox.winfo_width()? )?;
        let ( west_w, east_w ) = if ratio <= 0.0 {
            (      0,     w-1 )
        } else if ratio >= 1.0 {
            (    w-1,       0 )
        } else {
            let west_w = (ratio * w as f64) as c_int;
            let east_w = w -west_w -1;
            ( west_w, east_w )
        };

        let west = hbox
            .add_ttk_frame( path_seg("west") )?
            .place( -x_(0) -y_(0) -relheight(1) -width(west_w) )?;

        let sep = hbox
            .add_ttk_separator( "sep" -orient("vertical") )?
            .place( -x_(west_w) -y_(0) -relheight(1) )?;
        let east = hbox
            .add_ttk_frame( path_seg("east") )?
            .place( -x_(west_w+1) -y_(0) -relheight(1) -width(east_w) )?;

        sep.bind( event::motion(), tclosure!( tk, move || -> InterpResult<()> {
            sep.configure( -cursor("crosshair") )
        }))?;

        sep.bind( event::button_1().motion(), tclosure!( tk, args: "%x", move |dx: c_int| -> TkResult<()> {
            let place_info_west = west.place_info()?;
            let west_width = if let Some( w ) = place_info_west.get("-width") { tk.int( w.clone() )? } else { 0 };
            let place_info_east = east.place_info()?;
            let east_width = if let Some( w ) = place_info_east.get("-width") { tk.int( w.clone() )? } else { 0 };
            if dx < 9 {
                let west_width  = west_width  + dx;
                let east_width = east_width - dx;
                if west_width > 0 && east_width > 0 {
                    west.place_configure( -width(west_width) )?;
                    sep .place_configure( -x_(west_width) )?;
                    east.place_configure( -width(east_width) -x_(west_width+1) )?;
                }
            }
            Ok(())
        }))?;

        hbox.bind( event::configure(), tclosure!( tk, move || -> TkResult<()> {
            let hbox_width = hbox.winfo_width()?;
            let place_west_width = if let Some( w ) = west.place_info()?.get("-width") { tk.int( w.clone() )? } else { 0 };
            let place_east_width = if let Some( w ) = east.place_info()?.get("-width") { tk.int( w.clone() )? } else { 0 };

            let (west_width, east_width) = match hbox_resize {
                HBoxResize::Both => {
                    let ratio = place_west_width as f64 / ( place_west_width + place_east_width + 1 ) as f64;
                    let west_width = ( ratio * hbox_width as f64 ) as c_int;
                    let east_width = hbox_width - west_width - 1;
                    (west_width, east_width)
                },
                HBoxResize::West => {
                    let east_width = place_east_width;
                    let west_width = hbox_width - east_width - 1;
                    (west_width, east_width)
                },
                HBoxResize::East => {
                    let west_width = place_west_width;
                    let east_width = hbox_width - west_width - 1;
                    (west_width, east_width)
                },
            };

            west.place_configure( -width(west_width) )?;
            sep .place_configure( -x_(west_width+0) )?;
            east.place_configure( -width(east_width) -x_(west_width+1) )?;

            Ok(())
        }))?;

        Ok( HBox( hbox.0 ))
    }
}

impl<Widg,Inst> AddHBox for Widg
    where Widg : Deref<Target=Widget<Inst>>
        , Inst : TkInstance
{
}

/// A frame containing two frames(so called north/south), placed vertically and are resizable.
pub struct VBox<Inst:TkInstance>( pub(crate) Widget<Inst> );

impl<Inst:TkInstance> Deref for VBox<Inst> {
    type Target = Widget<Inst>;

    fn deref( &self ) -> &Self::Target { &self.0 }
}

impl<Inst:TkInstance> TkPackSlave  for VBox<Inst> {}
impl<Inst:TkInstance> TkGridSlave  for VBox<Inst> {}
impl<Inst:TkInstance> TkPlaceSlave for VBox<Inst> {}

impl<Inst:TkInstance> VBox<Inst> {
    /// Returns the top frame in the HBox.
    pub fn north( &self ) -> TtkFrame<Inst> {
        let path = format!( "{}.north", self.0.path );
        TtkFrame( Widget::from_name_unchecked( &path, self.0.inst ))
    }

    /// Returns the bottom frame in the HBox.
    pub fn south( &self ) -> TtkFrame<Inst> {
        let path = format!( "{}.south", self.0.path );
        TtkFrame( Widget::from_name_unchecked( &path, self.0.inst ))
    }
}

/// Defines the behaviour on VBox's resizing:
///
/// `HBoxResize::Both`, resize both west and east frames
///
/// `HBoxResize::North`, resize north frame while leave south frame unchanged.
///
/// `HBoxResize::South`, resize south frame while leave north frame unchanged.
pub enum VBoxResize {
    Both,
    North,
    South,
}

pub trait AddVBox {
    fn add_vbox<Opts,Inst:TkInstance>(
        &self,
        initial_ratio : f64,
        vbox_resize   : VBoxResize,
        path_opts     : impl Into<PathOptsWidgets<Opts,()>>
    ) -> InterpResult<VBox<Inst>>
        where Self : Sized
                   + Deref<Target=Widget<Inst>>
            , Opts : IntoHomoTuple<opt::TtkFrameOpt>
                   + IntoHomoTuple<OptPair>
    {
        let tk = self.deref().tk();
        let ratio = initial_ratio;

        let vbox = self.add_ttk_frame( path_opts )?.pack( -fill("both") -expand(true) )?;
        tk.update()?;
        let h: c_int = tk.int( vbox.winfo_height()? )?;
        let ( north_h, south_h ) = if ratio <= 0.0 {
            (      0,     h-1 )
        } else if ratio >= 1.0 {
            (    h-1,       0 )
        } else {
            let north_h = (ratio * h as f64) as c_int;
            let south_h = h -north_h -1;
            ( north_h, south_h )
        };

        let north = vbox
            .add_ttk_frame( path_seg("north") )?
            .place( -x_(0) -y_(0) -relwidth(1) -height(north_h) )?;

        let sep = vbox
            .add_ttk_separator( "sep" -orient("horizontal") )?
            .place( -x_(0) -y_(north_h) -relwidth(1) )?;
        let south = vbox
            .add_ttk_frame( path_seg("south") )?
            .place( -x_(0) -y_(north_h+1) -relwidth(1) -height(south_h) )?;

        sep.bind( event::motion(), tclosure!( tk, move || -> InterpResult<()> {
            sep.configure( -cursor("crosshair") )
        }))?;

        sep.bind( event::button_1().motion(), tclosure!( tk, args: "%y", move |dy: c_int| -> TkResult<()> {
            let place_info_north = north.place_info()?;
            let north_height = if let Some( h ) = place_info_north.get("-height") { tk.int( h.clone() )? } else { 0 };
            let place_info_south = south.place_info()?;
            let south_height = if let Some( h ) = place_info_south.get("-height") { tk.int( h.clone() )? } else { 0 };
            if dy < 9 {
                let north_height  = north_height  + dy;
                let south_height = south_height - dy;
                if north_height > 0 && south_height > 0 {
                    north.place_configure( -height(north_height) )?;
                    sep  .place_configure( -y_(north_height) )?;
                    south.place_configure( -height(south_height) -y_(north_height+1) )?;
                }
            }
            Ok(())
        }))?;

        vbox.bind( event::configure(), tclosure!( tk, move || -> TkResult<()> {
            let vbox_height = vbox.winfo_height()?;
            let place_north_height = if let Some( h ) = north.place_info()?.get("-height") { tk.int( h.clone() )? } else { 0 };
            let place_south_height = if let Some( h ) = south.place_info()?.get("-height") { tk.int( h.clone() )? } else { 0 };

            let (north_height, south_height) = match vbox_resize {
                VBoxResize::Both => {
                    let ratio = place_north_height as f64 / ( place_north_height + place_south_height + 1 ) as f64;
                    let north_height = ( ratio * vbox_height as f64 ) as c_int;
                    let south_height = vbox_height - north_height - 1;
                    (north_height, south_height)
                },
                VBoxResize::North => {
                    let south_height = place_south_height;
                    let north_height = vbox_height - south_height - 1;
                    (north_height, south_height)
                },
                VBoxResize::South => {
                    let north_height = place_north_height;
                    let south_height = vbox_height - north_height - 1;
                    (north_height, south_height)
                },
            };

            north.place_configure( -height(north_height) )?;
            sep  .place_configure( -y_(north_height+0) )?;
            south.place_configure( -height(south_height) -y_(north_height+1) )?;

            Ok(())
        }))?;

        Ok( VBox( vbox.0 ))
    }
}

impl<Widg,Inst> AddVBox for Widg
    where Widg : Deref<Target=Widget<Inst>>
        , Inst : TkInstance
{
}
