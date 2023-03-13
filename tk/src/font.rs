use enumx::export::*;
use enumx::predefined::*;
use cex::*;

use crate::{
    InterpResult,
    OptPair,
    PathOptsWidgets,
    Tk,
    TkInstance,
    TkOption,
    Widget,
    cmd::append_opts,
    opt::TkFontOpt,
};

use std::os::raw::{c_int, c_longlong};

use tcl::{
    Obj,
    error::{
        NotList,
        NotSeqOf,
        InterpError,
    },
};

use tuplex::*;

#[derive( Copy, Clone )]
pub enum Style {
    Normal,
    Bold,
    Roman,
    Italic,
    Underline,
    Overstrike,
}

impl From<Style> for Obj {
    fn from( style: Style ) -> Obj {
        match style {
            Style::Normal       => "normal"     .into(),
            Style::Bold         => "bold"       .into(),
            Style::Roman        => "roman"      .into(),
            Style::Italic       => "italic"     .into(),
            Style::Underline    => "underline"  .into(),
            Style::Overstrike   => "overstrike" .into(),
        }
    }
}

pub enum Font<'a, Opts> {
    Name( &'a str ),
    System( &'a str ),
    Family{ family: &'a str, size: Option<c_int>, styles: &'a [Style] },
    XFont( &'a str ),
    Opts( Opts ),
}

impl<'a, Opts> From<Font<'a, Opts>> for Obj
    where Opts : IntoHomoTuple<TkFontOpt>
               + IntoHomoTuple<OptPair>
{
    fn from( font: Font<'a, Opts> ) -> Obj {
        match font {
            Font::Name(   name ) |
            Font::System( name ) |
            Font::XFont(  name ) => name.into(),
            Font::Family{ family, size, styles } => {
                (family, size, styles.to_vec() ).into()
            },
            Font::Opts( opts ) => {
                let mut v = Vec::new();
                append_opts( &mut v, opts );
                v.into()
            },
        }
    }
}

impl<Inst:TkInstance> Tk<Inst> {
    /// Returns the value of the actual attribute that are obtained when font is used on
    /// the main window's display; the actual attribute obtained may differ from the
    /// attribute requested due to platform-dependent limitations, such as the
    /// availability of font families and point sizes. If the char argument is supplied,
    /// the font attribute returned will be those of the specific font used to render
    /// that character, which will be different from the base font if the base font does
    /// not contain the given character.
    pub fn font_actual<'a, Opt, Opts>( &self, font: Font<'a, Opts>, _option: Opt, ch: Option<char> ) -> InterpResult<Obj>
        where Opt  : TkOption
                   + Into<TkFontOpt>
            , Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        let option = Obj::from( <Opt as TkOption>::NAME );

        match ch {
            Some( '-' ) => self.eval(( "font", "actual", font, option, "--", "-" )),
            Some( ch ) => self.eval(( "font", "actual", font, option, Obj::from(ch) )),
            None => self.eval(( "font", "actual", font, option )),
        }
    }

    /// Returns a list of all the atual attributes and their values that are obtained
    /// when font is used on the main window's display; the actual attributes obtained
    /// may differ from the attributes requested due to platform-dependent limitations,
    /// such as the availability of font families and point sizes. If the char argument
    /// is supplied, the font attributes returned will be those of the specific font
    /// used to render that character, which will be different from the base font if the
    /// base font does not contain the given character.
    pub fn font_actual_get_all<'a, Opts>( &self, font: Font<'a, Opts> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        self.eval(( "font", "actual", font ))
    }

    pub fn font_configure<Opts>( &self, fontname: &str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<()>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );

        command.push( "font".into() );
        command.push( "configure".into() );
        command.push( fontname.into() );

        append_opts( &mut command, opts.into().opts );
        self.run( command )
    }

    pub fn font_configure_get<Opt>( &self, fontname: &str, _opt: Opt ) -> InterpResult<Obj>
        where Opt : TkOption
                  + Into<TkFontOpt>
    {
        self.eval(( "font", "configure", fontname, <Opt as TkOption>::NAME ))
    }

    #[cex]
    pub fn font_configure_get_all( &self, fontname: &str ) -> Result!( Vec<OptPair>
        throws InterpError, NotList, NotSeqOf<OptPair> )
    {
        let mut opt_pairs = Vec::new();
        let obj = self.eval(( "font", "configure", fontname ))?;
        let mut pairs = obj.clone().get_elements()?;
        while let Some( name ) = pairs.next() {
            let value = if let Some( value ) = pairs.next() {
                value.clone()
            } else {
                throw!( NotSeqOf::new( obj.clone() ));
            };
            let name = match name.get_string().as_str() {
                "-family"     => "family"    ,
                "-size"       => "size"      ,
                "-weight"     => "weight"    ,
                "-slant"      => "slant"     ,
                "-underline"  => "underline" ,
                "-overstrike" => "overstrike",
                _             => throw!( NotSeqOf::new( obj )),
            };
            opt_pairs.push( OptPair{ name, value });
        }
        Ok( opt_pairs )
    }

    pub fn font_create<Opts>( &self, fontname: &str, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 3 );

        command.push( "font".into() );
        command.push( "create".into() );
        command.push( fontname.into() );

        append_opts( &mut command, opts.into().opts );
        self.eval( command )
    }

    pub fn font_create_with_auto_name<Opts>( &self, opts: impl Into<PathOptsWidgets<Opts,()>> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let mut command = Vec::<Obj>::with_capacity( <<Opts as IntoHomoTuple<OptPair>>::Output as tuplex::Len>::LEN * 2 + 2 );

        command.push( "font".into() );
        command.push( "create".into() );

        append_opts( &mut command, opts.into().opts );
        self.eval( command )
    }

    pub fn font_delete( &self, names: &[Obj] ) -> InterpResult<()> {
        let mut command = Vec::<Obj>::with_capacity( names.len() + 2 );
        command.push( "font".into() );
        command.push( "delete".into() );
        command.extend( names.iter().map( |name| name.clone() ));
        self.run( command )
    }

    #[cex]
    pub fn font_families( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let list = self.eval(( "font", "families" ))?;
        Ok( list.get_elements()?.map( |obj| obj.get_string() ).collect() )
    }

    pub fn font_measure<'a, Opts>( &self, font: Font<'a, Opts>, text: impl Into<String> ) -> InterpResult<c_longlong>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        self.longlong( self.eval(( "font", "measure", font, text.into() ))? )
    }

    pub fn font_metrics<'a, Opt, Opts>( &self, font: Font<'a, Opts>, _opt: Opt ) -> InterpResult<Obj>
        where Opt  : TkOption
                   + Into<TkFontOpt>
            , Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        let option = Obj::from( <Opt as TkOption>::NAME );
        self.eval(( "font", "metrics", font, option ))
    }

    #[cex]
    pub fn font_metrics_get_all<'a, Opts>( &self, font: Font<'a, Opts> ) -> Result!( Vec<OptPair>
        throws InterpError, NotList, NotSeqOf<OptPair> )
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        let mut opt_pairs = Vec::new();
        let obj = self.eval(( "font", "metrics", font ))?;
        let mut pairs = obj.clone().get_elements()?;
        while let Some( name ) = pairs.next() {
            let value = if let Some( value ) = pairs.next() {
                value.clone()
            } else {
                throw!( NotSeqOf::new( obj.clone() ));
            };
            let name = match name.get_string().as_str() {
                "-ascent"    => "ascent"   ,
                "-descent"   => "descent"  ,
                "-linespace" => "linespace",
                "-fixed"     => "fixed"    ,
                _            => throw!( NotSeqOf::new( obj )),
            };
            opt_pairs.push( OptPair{ name, value });
        }
        Ok( opt_pairs )
    }

    /// The return value is a list of all the named fonts that are currently defined.
    ///
    /// # Examples
    ///
    /// ```
    /// use tcl::*;
    /// use tk::*;
    /// use tk::cmd::*;
    ///
    /// let tk = make_tk!()?;
    /// let font_names = tk.font_names().unwrap();
    /// assert!( font_names.len() > 0 );
    ///
    /// # TkResult::Ok(())
    /// ```
    #[cex]
    pub fn font_names( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let list = self.eval(( "font", "names" ))?;
        Ok( list.get_elements()?.map( |obj| obj.get_string() ).collect() )
    }
}

impl<Inst:TkInstance> Widget<Inst> {
    /// Returns the value of the actual attributes that are obtained when font is used
    /// on window's display; the actual attribute obtained may differ from the attribute
    /// requested due to platform-dependent limitations, such as the availability of
    /// font families and point sizes. If the char argument is supplied, the font
    /// attributes returned will be those of the specific font used to render that
    /// character, which will be different from the base font if the base font does not
    /// contain the given character.
    pub fn font_actual<'a, Opt, Opts>( &self, font: Font<'a, Opts>, _option: Opt, ch: Option<char> ) -> InterpResult<Obj>
        where Opt  : TkOption
                   + Into<TkFontOpt>
            , Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        let option = Obj::from( <Opt as TkOption>::NAME );

        match ch {
            Some( '-' ) => self.tk().eval(( "font", "actual", font, "-displayof", self.path, option, "--", "-" )),
            Some( ch ) => self.tk().eval(( "font", "actual", font, "-displayof", self.path, option, Obj::from(ch) )),
            None => self.tk().eval(( "font", "actual", font, "-displayof", self.path, option )),
        }
    }

    /// Returns a list of all the atual attributes and their values that are obtained
    /// when font is used on window's display; the actual attributes obtained may differ
    /// from the attributes requested due to platform-dependent limitations, such as the
    /// availability of font families and point sizes. If the char argument is supplied,
    /// the font attributes returned will be those of the specific font used to render
    /// that character, which will be different from the base font if the base font does
    /// not contain the given character.
    pub fn font_actual_get_all<'a, Opts>( &self, font: Font<'a, Opts> ) -> InterpResult<Obj>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        self.tk().eval(( "font", "actual", font, "-displayof", self.path ))
    }

    #[cex]
    pub fn font_families( &self ) -> Result!( Vec<String> throws InterpError, NotList ) {
        let list = self.tk().eval(( "font", "families", "-displayof", self.path ))?;
        Ok( list.get_elements()?.map( |obj| obj.get_string() ).collect() )
    }

    pub fn font_measure<'a, Opts>( &self, font: Font<'a, Opts>, text: impl Into<String> ) -> InterpResult<c_longlong>
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        self.tk().longlong( self.tk().eval(( "font", "measure", font, "-displayof", self.path, text.into() ))? )
    }

    pub fn font_metrics<'a, Opt, Opts>( &self, font: Font<'a, Opts>, _opt: Opt ) -> InterpResult<Obj>
        where Opt  : TkOption
                   + Into<TkFontOpt>
            , Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        let option = Obj::from( <Opt as TkOption>::NAME );
        self.tk().eval(( "font", "metrics", font, "-displayof", self.path, option ))
    }

    #[cex]
    pub fn font_metrics_get_all<'a, Opts>( &self, font: Font<'a, Opts> ) -> Result!( Vec<OptPair>
        throws InterpError, NotList, NotSeqOf<OptPair> )
        where Opts : IntoHomoTuple<TkFontOpt>
                   + IntoHomoTuple<OptPair>
    {
        let font = Obj::from( font );
        let mut opt_pairs = Vec::new();
        let obj = self.tk().eval(( "font", "metrics", font, "-displayof", self.path ))?;
        let mut pairs = obj.clone().get_elements()?;
        while let Some( name ) = pairs.next() {
            let value = if let Some( value ) = pairs.next() {
                value.clone()
            } else {
                throw!( NotSeqOf::new( obj.clone() ));
            };
            let name = match name.get_string().as_str() {
                "-ascent"    => "ascent"   ,
                "-descent"   => "descent"  ,
                "-linespace" => "linespace",
                "-fixed"     => "fixed"    ,
                _           => throw!( NotSeqOf::new( obj )),
            };
            opt_pairs.push( OptPair{ name, value });
        }
        Ok( opt_pairs )
    }
}
