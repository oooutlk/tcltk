use crate::*;

use crate::{
    cmd::append_opts,
    error::NotList,
    opt::{
        OptPair,
        TkEventOpt,
    },
};

use tcl::Obj;

use tuplex::*;

/// Single event.
pub struct TkEvent( String );

/// Event sequence.
pub struct TkEventSeq( String );

/// Event modifier not after any event.
pub struct TkModifier( String );

/// Event modifier after some event.
pub struct TkEventModifier( String );

pub fn control  () -> TkModifier { TkModifier( "<Control"  .to_owned() )}
pub fn alt      () -> TkModifier { TkModifier( "<Alt"      .to_owned() )}
pub fn shift    () -> TkModifier { TkModifier( "<Shift"    .to_owned() )}
pub fn lock     () -> TkModifier { TkModifier( "<Lock"     .to_owned() )}
pub fn extended () -> TkModifier { TkModifier( "<Extended" .to_owned() )}
pub fn button_1 () -> TkModifier { TkModifier( "<Button1"  .to_owned() )}
pub fn button_2 () -> TkModifier { TkModifier( "<Button2"  .to_owned() )}
pub fn button_3 () -> TkModifier { TkModifier( "<Button3"  .to_owned() )}
pub fn button_4 () -> TkModifier { TkModifier( "<Button4"  .to_owned() )}
pub fn button_5 () -> TkModifier { TkModifier( "<Button5"  .to_owned() )}
pub fn mod_1    () -> TkModifier { TkModifier( "<Mod1"     .to_owned() )}
pub fn mod_2    () -> TkModifier { TkModifier( "<Mod2"     .to_owned() )}
pub fn mod_3    () -> TkModifier { TkModifier( "<Mod3"     .to_owned() )}
pub fn mod_4    () -> TkModifier { TkModifier( "<Mod4"     .to_owned() )}
pub fn mod_5    () -> TkModifier { TkModifier( "<Mod5"     .to_owned() )}
pub fn command  () -> TkModifier { TkModifier( "<Command"  .to_owned() )}
pub fn option   () -> TkModifier { TkModifier( "<option"   .to_owned() )}
pub fn meta     () -> TkModifier { TkModifier( "<Meta"     .to_owned() )}
pub fn double   () -> TkModifier { TkModifier( "<Double"   .to_owned() )}
pub fn triple   () -> TkModifier { TkModifier( "<Triple"   .to_owned() )}
pub fn quadruple() -> TkModifier { TkModifier( "<Quadruple".to_owned() )}

impl TkModifier {
    pub fn control  ( mut self ) -> Self { self.0.push_str( "-Control"   ); self }
    pub fn alt      ( mut self ) -> Self { self.0.push_str( "-Alt"       ); self }
    pub fn shift    ( mut self ) -> Self { self.0.push_str( "-Shift"     ); self }
    pub fn lock     ( mut self ) -> Self { self.0.push_str( "-Lock"      ); self }
    pub fn extended ( mut self ) -> Self { self.0.push_str( "-Extended"  ); self }
    pub fn button_1 ( mut self ) -> Self { self.0.push_str( "-Button1"   ); self }
    pub fn button_2 ( mut self ) -> Self { self.0.push_str( "-Button2"   ); self }
    pub fn button_3 ( mut self ) -> Self { self.0.push_str( "-Button3"   ); self }
    pub fn button_4 ( mut self ) -> Self { self.0.push_str( "-Button4"   ); self }
    pub fn button_5 ( mut self ) -> Self { self.0.push_str( "-Button5"   ); self }
    pub fn mod_1    ( mut self ) -> Self { self.0.push_str( "-Mod1"      ); self }
    pub fn mod_2    ( mut self ) -> Self { self.0.push_str( "-Mod2"      ); self }
    pub fn mod_3    ( mut self ) -> Self { self.0.push_str( "-Mod3"      ); self }
    pub fn mod_4    ( mut self ) -> Self { self.0.push_str( "-Mod4"      ); self }
    pub fn mod_5    ( mut self ) -> Self { self.0.push_str( "-Mod5"      ); self }
    pub fn command  ( mut self ) -> Self { self.0.push_str( "-Command"   ); self }
    pub fn option   ( mut self ) -> Self { self.0.push_str( "-option"    ); self }
    pub fn meta     ( mut self ) -> Self { self.0.push_str( "-Meta"      ); self }
    pub fn double   ( mut self ) -> Self { self.0.push_str( "-Double"    ); self }
    pub fn triple   ( mut self ) -> Self { self.0.push_str( "-Triple"    ); self }
    pub fn quadruple( mut self ) -> Self { self.0.push_str( "-Quadruple" ); self }
}

pub fn activate         () -> TkEvent { TkEvent( "<activate>"        .to_owned() )}
pub fn button_press     () -> TkEvent { TkEvent( "<ButtonPress"      .to_owned() )}
pub fn button_press_1   () -> TkEvent { TkEvent( "<ButtonPress-1>"   .to_owned() )}
pub fn button_press_2   () -> TkEvent { TkEvent( "<ButtonPress-2>"   .to_owned() )}
pub fn button_press_3   () -> TkEvent { TkEvent( "<ButtonPress-3>"   .to_owned() )}
pub fn button_press_4   () -> TkEvent { TkEvent( "<ButtonPress-4>"   .to_owned() )}
pub fn button_press_5   () -> TkEvent { TkEvent( "<ButtonPress-5>"   .to_owned() )}
pub fn button_release   () -> TkEvent { TkEvent( "<ButtonRelease>"   .to_owned() )}
pub fn button_release_1 () -> TkEvent { TkEvent( "<ButtonRelease-1>" .to_owned() )}
pub fn button_release_2 () -> TkEvent { TkEvent( "<ButtonRelease-2>" .to_owned() )}
pub fn button_release_3 () -> TkEvent { TkEvent( "<ButtonRelease-3>" .to_owned() )}
pub fn button_release_4 () -> TkEvent { TkEvent( "<ButtonRelease-4>" .to_owned() )}
pub fn button_release_5 () -> TkEvent { TkEvent( "<ButtonRelease-5>" .to_owned() )}
pub fn circulate        () -> TkEvent { TkEvent( "<Circulate>"       .to_owned() )}
pub fn circulate_request() -> TkEvent { TkEvent( "<CirculateRequest>".to_owned() )}
pub fn colormap         () -> TkEvent { TkEvent( "<Colormap>"        .to_owned() )}
pub fn configure        () -> TkEvent { TkEvent( "<Configure>"       .to_owned() )}
pub fn configure_request() -> TkEvent { TkEvent( "<ConfigureRequest>".to_owned() )}
pub fn create           () -> TkEvent { TkEvent( "<Create>"          .to_owned() )}
pub fn deactivate       () -> TkEvent { TkEvent( "<Deactivate>"      .to_owned() )}
pub fn destroy          () -> TkEvent { TkEvent( "<Destroy>"         .to_owned() )}
pub fn enter            () -> TkEvent { TkEvent( "<Enter>"           .to_owned() )}
pub fn expose           () -> TkEvent { TkEvent( "<Expose>"          .to_owned() )}
pub fn focus_in         () -> TkEvent { TkEvent( "<FocusIn>"         .to_owned() )}
pub fn focus_out        () -> TkEvent { TkEvent( "<FocusOut>"        .to_owned() )}
pub fn gravity          () -> TkEvent { TkEvent( "<Gravity>"         .to_owned() )}
pub fn map              () -> TkEvent { TkEvent( "<Map>"             .to_owned() )}
pub fn map_request      () -> TkEvent { TkEvent( "<MapRequest>"      .to_owned() )}
pub fn motion           () -> TkEvent { TkEvent( "<Motion>"          .to_owned() )}
pub fn mouse_wheel      () -> TkEvent { TkEvent( "<MouseWheel>"      .to_owned() )}
pub fn leave            () -> TkEvent { TkEvent( "<Leave>"           .to_owned() )}
pub fn property         () -> TkEvent { TkEvent( "<Property>"        .to_owned() )}
pub fn reparent         () -> TkEvent { TkEvent( "<Reparent>"        .to_owned() )}
pub fn resize_request   () -> TkEvent { TkEvent( "<ResizeRequest>"   .to_owned() )}
pub fn unmap            () -> TkEvent { TkEvent( "<Unmap>"           .to_owned() )}
pub fn visibility       () -> TkEvent { TkEvent( "<Visibility>"      .to_owned() )}

pub fn key_press  ( key: impl Into<TkKey> ) -> TkEvent { TkEvent( format!( "<KeyPress-{}>"  , key.into().to_string() ))}
pub fn key_release( key: impl Into<TkKey> ) -> TkEvent { TkEvent( format!( "<KeyRelease-{}>", key.into().to_string() ))}
pub fn any_key_press  () -> TkEvent { TkEvent( "<KeyPress>"  .to_owned() )}
pub fn any_key_release() -> TkEvent { TkEvent( "<KeyRelease>".to_owned() )}

pub fn virtual_event( name: &str ) -> TkEvent { TkEvent( format!( "<<{}>>", name ))}

impl TkModifier {
    pub fn activate         ( mut self ) -> TkEvent { self.0.push_str( "-activate>"         ); TkEvent( self.0 )}
    pub fn button_press     ( mut self ) -> TkEvent { self.0.push_str( "-ButtonPress"       ); TkEvent( self.0 )}
    pub fn button_press_1   ( mut self ) -> TkEvent { self.0.push_str( "-ButtonPress-1>"    ); TkEvent( self.0 )}
    pub fn button_press_2   ( mut self ) -> TkEvent { self.0.push_str( "-ButtonPress-2>"    ); TkEvent( self.0 )}
    pub fn button_press_3   ( mut self ) -> TkEvent { self.0.push_str( "-ButtonPress-3>"    ); TkEvent( self.0 )}
    pub fn button_press_4   ( mut self ) -> TkEvent { self.0.push_str( "-ButtonPress-4>"    ); TkEvent( self.0 )}
    pub fn button_press_5   ( mut self ) -> TkEvent { self.0.push_str( "-ButtonPress-5>"    ); TkEvent( self.0 )}
    pub fn button_release   ( mut self ) -> TkEvent { self.0.push_str( "-ButtonRelease>"    ); TkEvent( self.0 )}
    pub fn button_release_1 ( mut self ) -> TkEvent { self.0.push_str( "-ButtonRelease-1>"  ); TkEvent( self.0 )}
    pub fn button_release_2 ( mut self ) -> TkEvent { self.0.push_str( "-ButtonRelease-2>"  ); TkEvent( self.0 )}
    pub fn button_release_3 ( mut self ) -> TkEvent { self.0.push_str( "-ButtonRelease-3>"  ); TkEvent( self.0 )}
    pub fn button_release_4 ( mut self ) -> TkEvent { self.0.push_str( "-ButtonRelease-4>"  ); TkEvent( self.0 )}
    pub fn button_release_5 ( mut self ) -> TkEvent { self.0.push_str( "-ButtonRelease-5>"  ); TkEvent( self.0 )}
    pub fn circulate        ( mut self ) -> TkEvent { self.0.push_str( "-Circulate>"        ); TkEvent( self.0 )}
    pub fn circulate_request( mut self ) -> TkEvent { self.0.push_str( "-CirculateRequest>" ); TkEvent( self.0 )}
    pub fn colormap         ( mut self ) -> TkEvent { self.0.push_str( "-Colormap>"         ); TkEvent( self.0 )}
    pub fn configure        ( mut self ) -> TkEvent { self.0.push_str( "-Configure>"        ); TkEvent( self.0 )}
    pub fn configure_request( mut self ) -> TkEvent { self.0.push_str( "-ConfigureRequest>" ); TkEvent( self.0 )}
    pub fn create           ( mut self ) -> TkEvent { self.0.push_str( "-Create>"           ); TkEvent( self.0 )}
    pub fn deactivate       ( mut self ) -> TkEvent { self.0.push_str( "-Deactivate>"       ); TkEvent( self.0 )}
    pub fn destroy          ( mut self ) -> TkEvent { self.0.push_str( "-Destroy>"          ); TkEvent( self.0 )}
    pub fn enter            ( mut self ) -> TkEvent { self.0.push_str( "-Enter>"            ); TkEvent( self.0 )}
    pub fn expose           ( mut self ) -> TkEvent { self.0.push_str( "-Expose>"           ); TkEvent( self.0 )}
    pub fn focus_in         ( mut self ) -> TkEvent { self.0.push_str( "-FocusIn>"          ); TkEvent( self.0 )}
    pub fn focus_out        ( mut self ) -> TkEvent { self.0.push_str( "-FocusOut>"         ); TkEvent( self.0 )}
    pub fn gravity          ( mut self ) -> TkEvent { self.0.push_str( "-Gravity>"          ); TkEvent( self.0 )}
    pub fn map              ( mut self ) -> TkEvent { self.0.push_str( "-Map>"              ); TkEvent( self.0 )}
    pub fn map_request      ( mut self ) -> TkEvent { self.0.push_str( "-MapRequest>"       ); TkEvent( self.0 )}
    pub fn motion           ( mut self ) -> TkEvent { self.0.push_str( "-Motion>"           ); TkEvent( self.0 )}
    pub fn mouse_wheel      ( mut self ) -> TkEvent { self.0.push_str( "-MouseWheel>"       ); TkEvent( self.0 )}
    pub fn leave            ( mut self ) -> TkEvent { self.0.push_str( "-Leave>"            ); TkEvent( self.0 )}
    pub fn property         ( mut self ) -> TkEvent { self.0.push_str( "-Property>"         ); TkEvent( self.0 )}
    pub fn reparent         ( mut self ) -> TkEvent { self.0.push_str( "-Reparent>"         ); TkEvent( self.0 )}
    pub fn resize_request   ( mut self ) -> TkEvent { self.0.push_str( "-ResizeRequest>"    ); TkEvent( self.0 )}
    pub fn unmap            ( mut self ) -> TkEvent { self.0.push_str( "-Unmap>"            ); TkEvent( self.0 )}
    pub fn visibility       ( mut self ) -> TkEvent { self.0.push_str( "-Visibility>"       ); TkEvent( self.0 )}

    pub fn key_press  ( mut self, key: impl Into<TkKey> ) -> TkEvent { self.0.push_str( &format!( "-KeyPress-{}>"  , key.into().to_string() )); TkEvent( self.0 )}
    pub fn key_release( mut self, key: impl Into<TkKey> ) -> TkEvent { self.0.push_str( &format!( "-KeyRelease-{}>", key.into().to_string() )); TkEvent( self.0 )}
    pub fn any_key_press  ( mut self ) -> TkEvent { self.0.push_str( "-KeyPress>"   ); TkEvent( self.0 )}
    pub fn any_key_release( mut self ) -> TkEvent { self.0.push_str( "-KeyRelease>" ); TkEvent( self.0 )}
}

impl TkEvent {
    pub fn control  ( mut self ) -> TkEventModifier { self.0.push_str( "<Control"   ); TkEventModifier( self.0 )}
    pub fn alt      ( mut self ) -> TkEventModifier { self.0.push_str( "<Alt"       ); TkEventModifier( self.0 )}
    pub fn shift    ( mut self ) -> TkEventModifier { self.0.push_str( "<Shift"     ); TkEventModifier( self.0 )}
    pub fn lock     ( mut self ) -> TkEventModifier { self.0.push_str( "<Lock"      ); TkEventModifier( self.0 )}
    pub fn extended ( mut self ) -> TkEventModifier { self.0.push_str( "<Extended"  ); TkEventModifier( self.0 )}
    pub fn button_1 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button1"   ); TkEventModifier( self.0 )}
    pub fn button_2 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button2"   ); TkEventModifier( self.0 )}
    pub fn button_3 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button3"   ); TkEventModifier( self.0 )}
    pub fn button_4 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button4"   ); TkEventModifier( self.0 )}
    pub fn button_5 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button5"   ); TkEventModifier( self.0 )}
    pub fn mod_1    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod1"      ); TkEventModifier( self.0 )}
    pub fn mod_2    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod2"      ); TkEventModifier( self.0 )}
    pub fn mod_3    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod3"      ); TkEventModifier( self.0 )}
    pub fn mod_4    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod4"      ); TkEventModifier( self.0 )}
    pub fn mod_5    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod5"      ); TkEventModifier( self.0 )}
    pub fn command  ( mut self ) -> TkEventModifier { self.0.push_str( "<Command"   ); TkEventModifier( self.0 )}
    pub fn option   ( mut self ) -> TkEventModifier { self.0.push_str( "<option"    ); TkEventModifier( self.0 )}
    pub fn meta     ( mut self ) -> TkEventModifier { self.0.push_str( "<Meta"      ); TkEventModifier( self.0 )}
    pub fn double   ( mut self ) -> TkEventModifier { self.0.push_str( "<Double"    ); TkEventModifier( self.0 )}
    pub fn triple   ( mut self ) -> TkEventModifier { self.0.push_str( "<Triple"    ); TkEventModifier( self.0 )}
    pub fn quadruple( mut self ) -> TkEventModifier { self.0.push_str( "<Quadruple" ); TkEventModifier( self.0 )}
}

impl TkEventSeq {
    pub fn control  ( mut self ) -> TkEventModifier { self.0.push_str( "<Control"   ); TkEventModifier( self.0 )}
    pub fn alt      ( mut self ) -> TkEventModifier { self.0.push_str( "<Alt"       ); TkEventModifier( self.0 )}
    pub fn shift    ( mut self ) -> TkEventModifier { self.0.push_str( "<Shift"     ); TkEventModifier( self.0 )}
    pub fn lock     ( mut self ) -> TkEventModifier { self.0.push_str( "<Lock"      ); TkEventModifier( self.0 )}
    pub fn extended ( mut self ) -> TkEventModifier { self.0.push_str( "<Extended"  ); TkEventModifier( self.0 )}
    pub fn button_1 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button1"   ); TkEventModifier( self.0 )}
    pub fn button_2 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button2"   ); TkEventModifier( self.0 )}
    pub fn button_3 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button3"   ); TkEventModifier( self.0 )}
    pub fn button_4 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button4"   ); TkEventModifier( self.0 )}
    pub fn button_5 ( mut self ) -> TkEventModifier { self.0.push_str( "<Button5"   ); TkEventModifier( self.0 )}
    pub fn mod_1    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod1"      ); TkEventModifier( self.0 )}
    pub fn mod_2    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod2"      ); TkEventModifier( self.0 )}
    pub fn mod_3    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod3"      ); TkEventModifier( self.0 )}
    pub fn mod_4    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod4"      ); TkEventModifier( self.0 )}
    pub fn mod_5    ( mut self ) -> TkEventModifier { self.0.push_str( "<Mod5"      ); TkEventModifier( self.0 )}
    pub fn command  ( mut self ) -> TkEventModifier { self.0.push_str( "<Command"   ); TkEventModifier( self.0 )}
    pub fn option   ( mut self ) -> TkEventModifier { self.0.push_str( "<option"    ); TkEventModifier( self.0 )}
    pub fn meta     ( mut self ) -> TkEventModifier { self.0.push_str( "<Meta"      ); TkEventModifier( self.0 )}
    pub fn double   ( mut self ) -> TkEventModifier { self.0.push_str( "<Double"    ); TkEventModifier( self.0 )}
    pub fn triple   ( mut self ) -> TkEventModifier { self.0.push_str( "<Triple"    ); TkEventModifier( self.0 )}
    pub fn quadruple( mut self ) -> TkEventModifier { self.0.push_str( "<Quadruple" ); TkEventModifier( self.0 )}
}

impl TkEventSeq {
    pub fn activate         ( mut self ) -> Self { self.0.push_str( "<activate>"         ); self }
    pub fn button_press     ( mut self ) -> Self { self.0.push_str( "<ButtonPress"       ); self }
    pub fn button_press_1   ( mut self ) -> Self { self.0.push_str( "<ButtonPress-1>"    ); self }
    pub fn button_press_2   ( mut self ) -> Self { self.0.push_str( "<ButtonPress-2>"    ); self }
    pub fn button_press_3   ( mut self ) -> Self { self.0.push_str( "<ButtonPress-3>"    ); self }
    pub fn button_press_4   ( mut self ) -> Self { self.0.push_str( "<ButtonPress-4>"    ); self }
    pub fn button_press_5   ( mut self ) -> Self { self.0.push_str( "<ButtonPress-5>"    ); self }
    pub fn button_release   ( mut self ) -> Self { self.0.push_str( "<ButtonRelease>"    ); self }
    pub fn button_release_1 ( mut self ) -> Self { self.0.push_str( "<ButtonRelease-1>"  ); self }
    pub fn button_release_2 ( mut self ) -> Self { self.0.push_str( "<ButtonRelease-2>"  ); self }
    pub fn button_release_3 ( mut self ) -> Self { self.0.push_str( "<ButtonRelease-3>"  ); self }
    pub fn button_release_4 ( mut self ) -> Self { self.0.push_str( "<ButtonRelease-4>"  ); self }
    pub fn button_release_5 ( mut self ) -> Self { self.0.push_str( "<ButtonRelease-5>"  ); self }
    pub fn circulate        ( mut self ) -> Self { self.0.push_str( "<Circulate>"        ); self }
    pub fn circulate_request( mut self ) -> Self { self.0.push_str( "<CirculateRequest>" ); self }
    pub fn colormap         ( mut self ) -> Self { self.0.push_str( "<Colormap>"         ); self }
    pub fn configure        ( mut self ) -> Self { self.0.push_str( "<Configure>"        ); self }
    pub fn configure_request( mut self ) -> Self { self.0.push_str( "<ConfigureRequest>" ); self }
    pub fn create           ( mut self ) -> Self { self.0.push_str( "<Create>"           ); self }
    pub fn deactivate       ( mut self ) -> Self { self.0.push_str( "<Deactivate>"       ); self }
    pub fn destroy          ( mut self ) -> Self { self.0.push_str( "<Destroy>"          ); self }
    pub fn enter            ( mut self ) -> Self { self.0.push_str( "<Enter>"            ); self }
    pub fn expose           ( mut self ) -> Self { self.0.push_str( "<Expose>"           ); self }
    pub fn focus_in         ( mut self ) -> Self { self.0.push_str( "<FocusIn>"          ); self }
    pub fn focus_out        ( mut self ) -> Self { self.0.push_str( "<FocusOut>"         ); self }
    pub fn gravity          ( mut self ) -> Self { self.0.push_str( "<Gravity>"          ); self }
    pub fn map              ( mut self ) -> Self { self.0.push_str( "<Map>"              ); self }
    pub fn map_request      ( mut self ) -> Self { self.0.push_str( "<MapRequest>"       ); self }
    pub fn motion           ( mut self ) -> Self { self.0.push_str( "<Motion>"           ); self }
    pub fn mouse_wheel      ( mut self ) -> Self { self.0.push_str( "<MouseWheel>"       ); self }
    pub fn leave            ( mut self ) -> Self { self.0.push_str( "<Leave>"            ); self }
    pub fn property         ( mut self ) -> Self { self.0.push_str( "<Property>"         ); self }
    pub fn reparent         ( mut self ) -> Self { self.0.push_str( "<Reparent>"         ); self }
    pub fn resize_request   ( mut self ) -> Self { self.0.push_str( "<ResizeRequest>"    ); self }
    pub fn unmap            ( mut self ) -> Self { self.0.push_str( "<Unmap>"            ); self }
    pub fn visibility       ( mut self ) -> Self { self.0.push_str( "<Visibility>"       ); self }

    pub fn key_press  ( mut self, key: impl Into<TkKey> ) -> Self { self.0.push_str( &format!( "<KeyPress-{}>"  , key.into().to_string() )); self }
    pub fn key_release( mut self, key: impl Into<TkKey> ) -> Self { self.0.push_str( &format!( "<KeyRelease-{}>", key.into().to_string() )); self }
    pub fn any_key_press  ( mut self ) -> Self { self.0.push_str( "<KeyPress>"  ); self }
    pub fn any_key_release( mut self ) -> Self { self.0.push_str( "<KeyRelease>" ); self }
}

impl TkEventSeq {
    pub fn virtual_event( mut self, name: &str ) -> Self { self.0.push_str( &format!( "<<{}>>", name )); self }
}

impl From<TkEvent> for Obj {
    fn from( seq: TkEvent ) -> Obj {
        seq.0.into()
    }
}

impl From<TkEventSeq> for Obj {
    fn from( seq: TkEventSeq ) -> Obj {
        seq.0.into()
    }
}

impl From<TkEvent> for TkEventSeq {
    fn from( event: TkEvent ) -> TkEventSeq { TkEventSeq( event.0 )}
}

pub fn alt_underlined()     -> TkEvent { TkEvent( "<<AltUnderlined>>"   .to_owned() )}
pub fn invoke()             -> TkEvent { TkEvent( "<<Invoke>>"          .to_owned() )}
pub fn listbox_select()     -> TkEvent { TkEvent( "<<ListboxSelect>>"   .to_owned() )}
pub fn menu_select()        -> TkEvent { TkEvent( "<<MenuSelect>>"      .to_owned() )}
pub fn modified()           -> TkEvent { TkEvent( "<<Modified>>"        .to_owned() )}
pub fn selection()          -> TkEvent { TkEvent( "<<Selection>>"       .to_owned() )}
pub fn theme_changed()      -> TkEvent { TkEvent( "<<ThemeChanged>>"    .to_owned() )}
pub fn traverse_in()        -> TkEvent { TkEvent( "<<TraverseIn>>"      .to_owned() )}
pub fn traverse_out()       -> TkEvent { TkEvent( "<<TraverseOut>>"     .to_owned() )}
pub fn undo_stack()         -> TkEvent { TkEvent( "<<UndoStack>>"       .to_owned() )}
pub fn widget_view_sync()   -> TkEvent { TkEvent( "<<WidgetViewSync>>"  .to_owned() )}
pub fn clear()              -> TkEvent { TkEvent( "<<Clear>>"           .to_owned() )}
pub fn copy()               -> TkEvent { TkEvent( "<<Copy>>"            .to_owned() )}
pub fn cut()                -> TkEvent { TkEvent( "<<Cut>>"             .to_owned() )}
pub fn line_end()           -> TkEvent { TkEvent( "<<LineEnd>>"         .to_owned() )}
pub fn line_start()         -> TkEvent { TkEvent( "<<LineStart>>"       .to_owned() )}
pub fn next_char()          -> TkEvent { TkEvent( "<<NextChar>>"        .to_owned() )}
pub fn next_line()          -> TkEvent { TkEvent( "<<NextLine>>"        .to_owned() )}
pub fn next_para()          -> TkEvent { TkEvent( "<<NextPara>>"        .to_owned() )}
pub fn next_word()          -> TkEvent { TkEvent( "<<NextWord>>"        .to_owned() )}
pub fn paste()              -> TkEvent { TkEvent( "<<Paste>>"           .to_owned() )}
pub fn paste_selection()    -> TkEvent { TkEvent( "<<PasteSelection>>"  .to_owned() )}
pub fn prev_char()          -> TkEvent { TkEvent( "<<PrevChar>>"        .to_owned() )}
pub fn prev_line()          -> TkEvent { TkEvent( "<<PrevLine>>"        .to_owned() )}
pub fn prev_para()          -> TkEvent { TkEvent( "<<PrevPara>>"        .to_owned() )}
pub fn prev_window()        -> TkEvent { TkEvent( "<<PrevWindow>>"      .to_owned() )}
pub fn prev_word()          -> TkEvent { TkEvent( "<<PrevWord>>"        .to_owned() )}
pub fn redo()               -> TkEvent { TkEvent( "<<Redo>>"            .to_owned() )}
pub fn select_all()         -> TkEvent { TkEvent( "<<SelectAll>>"       .to_owned() )}
pub fn select_line_end()    -> TkEvent { TkEvent( "<<SelectLineEnd>>"   .to_owned() )}
pub fn select_line_start()  -> TkEvent { TkEvent( "<<SelectLineStart>>" .to_owned() )}
pub fn select_next_char()   -> TkEvent { TkEvent( "<<SelectNextChar>>"  .to_owned() )}
pub fn select_next_line()   -> TkEvent { TkEvent( "<<SelectNextLine>>"  .to_owned() )}
pub fn select_next_para()   -> TkEvent { TkEvent( "<<SelectNextPara>>"  .to_owned() )}
pub fn select_next_word()   -> TkEvent { TkEvent( "<<SelectNextWord>>"  .to_owned() )}
pub fn select_none()        -> TkEvent { TkEvent( "<<SelectNone>>"      .to_owned() )}
pub fn select_prev_char()   -> TkEvent { TkEvent( "<<SelectPrevChar>>"  .to_owned() )}
pub fn select_prev_line()   -> TkEvent { TkEvent( "<<SelectPrevLine>>"  .to_owned() )}
pub fn select_prev_para()   -> TkEvent { TkEvent( "<<SelectPrevPara>>"  .to_owned() )}
pub fn select_prev_word()   -> TkEvent { TkEvent( "<<SelectPrevWord>>"  .to_owned() )}
pub fn toggle_selection()   -> TkEvent { TkEvent( "<<ToggleSelection>>" .to_owned() )}
pub fn undo()               -> TkEvent { TkEvent( "<<Undo>>"            .to_owned() )}

impl<Inst:TkInstance> Tk<Inst> {
    /// Associates the virtual event virtual with the physical event sequence(s) given
    /// by the sequence arguments, so that the virtual event will trigger whenever any
    /// one of the sequences occurs. Virtual may be any string value and sequence may
    /// have any of the values allowed for the sequence argument to the bind command. If
    /// virtual is already defined, the new physical event sequences add to the existing
    /// sequences for the event.
    pub fn event_add( &self, virtual_event: TkEvent, sequences: Vec<TkEventSeq> ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( sequences.len() + 3 );
        command.push( "event".into() );
        command.push( "add".into() );
        command.push( virtual_event.into() );

        command.extend( sequences.into_iter().map( |seq| seq.into() ));
        self.run( command )
    }

    /// Deletes each of the sequences from those associated with the virtual event given
    /// by virtual. Virtual may be any string value and sequence may have any of the
    /// values allowed for the sequence argument to the bind command. Any sequences not
    /// currently associated with virtual are ignored. If no sequence argument is
    /// provided, all physical event sequences are removed for virtual, so that the
    /// virtual event will not trigger anymore.
    pub fn event_delete( &self, virtual_event: TkEvent, sequences: Option<Vec<TkEventSeq>> ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( sequences.as_ref().map( |s| s.len() ).unwrap_or(0) + 3 );
        command.push( "event".into() );
        command.push( "delete".into() );
        command.push( virtual_event.into() );

        if let Some( sequences ) = sequences {
            command.extend( sequences.into_iter().map( |seq| seq.into() ));
        }
        self.run( command )
    }

    /// Returns a list of all the virtual events that are currently defined.
    #[cex]
    pub fn event_info( &self ) -> Result!( Vec<TkEvent> throws InterpError, NotList ) {
        let obj = self.eval(( "event", "info" ))?;
        Ok( obj .get_elements()?
                .map( |elem| TkEvent( elem.to_string() ))
                .collect() )
    }

    /// Returns a list whose elements are the physical event sequences currently defined
    /// for the given virtual event; if the virtual event is not defined then an empty
    /// string is returned.
    ///
    /// Note that virtual events that are not bound to physical event sequences are not
    /// returned.
    #[cex]
    pub fn event_info_sequences_of( &self, virtual_event: TkEvent ) -> Result!( Vec<TkEventSeq>
        throws InterpError, NotList )
    {
        let obj = self.eval(( "event", "info", virtual_event ))?;
        Ok( obj .get_elements()?
                .map( |elem| TkEventSeq( elem.to_string() ))
                .collect() )
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    /// Generates a window event and arranges for it to be processed just as if it had
    /// come from the window system. `event` provides a basic description of the event,
    /// such as `shift_button_2()` or `paste()`. If Window is empty the whole screen is
    /// meant, and coordinates are relative to the screen. Option-value pairs may be
    /// used to specify additional attributes of the event, such as the x and y mouse
    /// position; see EVENT FIELDS for more. If the `-when` option is not specified, the
    /// event is processed immediately: all of the handlers for the event will complete
    /// before the event generate command returns. If the `-when` option is specified
    /// then it determines when the event is processed. Certain events, such as key
    /// events, require that the window has focus to receive the event properly.
    pub fn event_generate<Opts>( &self, event: TkEvent, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts: IntoHomoTuple<TkEventOpt>
                  + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 4 );
        command.push( "event"    .into() );
        command.push( "generate" .into() );
        command.push( self.path.into() );
        command.push( event      .into() );

        append_opts( &mut command, opts.into().opts );

        self.tk().run( command )
    }
}
