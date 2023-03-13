use enumx::def_impls;
use enumx::predefined::*;
use cex::{
    crate_error,
    impl_std_error,
};

use std::{
    fmt::{self, Debug, Display},
};

pub use tcl::{
    Obj,
    error::{
        DeError,
        DeKind,
        TclInitError,
        NullInterp,
        MismatchedObjType,
        MoveBorrowedValue,
        MoveSharedObj,
        NullDataPtr,
        NotSeq,
        NotSeqOf,
        NotList,
        NotDict,
        MutateSharedDict,
        InterpError,
    },
};

pub struct TagRangesNotInPair( pub Obj );
impl_std_error!{ TagRangesNotInPair }

impl Debug for TagRangesNotInPair {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter,
            "`pathName text tag ranges tagName` should return an even-length list, but returns {}.",
            self.0.to_string() )
    }
}

pub struct TkAcceptableSizeParseError( pub Obj );
impl_std_error!{ TkAcceptableSizeParseError }

impl Debug for TkAcceptableSizeParseError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "error occurred when parsing {} as TkAcceptableSize", self.0.to_string() )
    }
}

pub enum TkCanvasItemTypeParseError{}
impl_std_error!{ TkCanvasItemTypeParseError }

impl Debug for TkCanvasItemTypeParseError {
    fn fmt( &self, _formatter: &mut fmt::Formatter ) -> fmt::Result {
        unreachable!()
    }
}

pub struct TkDumpParseError( pub Obj );
impl_std_error!{ TkDumpParseError }

impl Debug for TkDumpParseError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "error occurred when parsing {} as TkDump", self.0.to_string() )
    }
}

#[derive( Debug )]
pub struct TkGeometryParseError( pub String );
impl_std_error!{ TkGeometryParseError }

#[derive( Debug, Default )]
pub struct TkIndexParseError( pub String );
impl_std_error!{ TkIndexParseError }

#[derive( Debug )]
pub struct TkRequesterParseError( pub String );
impl_std_error!{ TkRequesterParseError }

pub struct TkResizableParseError( pub Obj );
impl_std_error!{ TkResizableParseError }

impl Debug for TkResizableParseError {
    fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
        write!( formatter, "error occurred when parsing {} as TkResizable", self.0.to_string() )
    }
}

#[derive( Debug )]
pub struct TkScalePartParseError( pub String );
impl_std_error!{ TkScalePartParseError }

#[derive( Debug )]
pub struct TkScreenNameParseError( pub String );
impl_std_error!{ TkScreenNameParseError }

#[derive( Debug )]
pub struct TkTextMarkGravityParseError( pub String );
impl_std_error!{ TkTextMarkGravityParseError }

#[derive( Debug )]
pub struct TkScrollbarElementParseError( pub String );
impl_std_error!{ TkScrollbarElementParseError }

#[derive( Debug )]
pub struct TkSpinboxElementParseError( pub String );
impl_std_error!{ TkSpinboxElementParseError }

#[derive( Debug )]
pub struct TtkStateParseError( pub String );
impl_std_error!{ TtkStateParseError }

#[derive( Debug )]
pub struct UnexpectedPanedwindowIdentifyResult( pub String );
impl_std_error!{ UnexpectedPanedwindowIdentifyResult }

#[derive( Debug )]
pub struct UnexpectedScrollbarElementActivatedError( pub String );
impl_std_error!{ UnexpectedScrollbarElementActivatedError }

#[derive( Debug )]
pub struct WidgetNotFound( pub String );
impl_std_error!{ WidgetNotFound }

crate_error!{
    #[derive( Debug )]
    pub enum TkError {
        TagRangesNotInPair                      ,
        TkAcceptableSizeParseError              ,
        TkCanvasItemTypeParseError              ,
        TkDumpParseError                        ,
        TkGeometryParseError                    ,
        TkIndexParseError                       ,
        TkRequesterParseError                   ,
        TkResizableParseError                   ,
        TkScalePartParseError                   ,
        TkScrollbarElementParseError            ,
        TkSpinboxElementParseError              ,
        TkScreenNameParseError                  ,
        TkTextMarkGravityParseError             ,
        TtkStateParseError                      ,
        UnexpectedPanedwindowIdentifyResult     ,
        UnexpectedScrollbarElementActivatedError,
        WidgetNotFound                          ,

        // errors from tcl crate
        DeError                                 ,
        TclInitError                            ,
        NullInterp                              ,
        MismatchedObjType                       ,
        MoveBorrowedValue                       ,
        MoveSharedObj                           ,
        NullDataPtr                             ,
        NotSeq                                  ,
        NotList                                 ,
        NotDict                                 ,
        MutateSharedDict                        ,
        InterpError                             ,
    }
}

impl_std_error!{ TkError }

#[cfg( not( any( feature="cex_log", feature="cex_env_log" )))]
impl<T> From<NotSeqOf<T>> for TkError {
    fn from( e: NotSeqOf<T> ) -> Self { TkError::NotSeq( NotSeq::from( e ))}
}

#[cfg( any( feature="cex_log", feature="cex_env_log" ))]
impl<T> From<cex::Log<NotSeqOf<T>>> for TkError {
    fn from( e: cex::Log<NotSeqOf<T>> ) -> Self {
        TkError::NotSeq( cex::Log{ error: NotSeq::from( e.error ), agent: e.agent })
    }
}

#[doc( hidden )]
pub trait IntoTkError {
    fn into_tk_error( self ) -> TkError;
}

impl<E: IntoTkError> From<E> for TkError {
    fn from( e: E ) -> Self { e.into_tk_error() }
}

impl IntoTkError for std::convert::Infallible {
    fn into_tk_error( self ) -> TkError {
        unreachable!()
    }
}

def_impls! {
    impl IntoTkError for Enum![1..=4]
        where _Variants!(): Into<TkError>
    {
        fn into_tk_error( self ) -> TkError {
            _match!(
                _variant!().into()
            )
        }
    }
}

/// The crate result type for users who do not want to use checked exceptions( `#[cex]` ).
pub type TkResult<T> = Result<T, TkError>;
