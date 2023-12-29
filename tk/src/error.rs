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

macro_rules! impl_std_error_and_debug {
    ($err:ident, $msg:expr $(, $ty:expr)* ) => {
        impl_std_error!{ $err }
        impl Debug for $err {
            fn fmt( &self, formatter: &mut fmt::Formatter ) -> fmt::Result {
                write!( formatter, $msg, self.0.to_string() $(,$ty)* )
            }
        }
    };
}

pub struct TagRangesNotInPair( pub Obj );
impl_std_error_and_debug!( TagRangesNotInPair,
    "`pathName text tag ranges tagName` should return an even-length list, but returns {}." );

pub struct TkAcceptableSizeParseError( pub Obj );
impl_std_error_and_debug!( TkAcceptableSizeParseError,
    "error occurred when parsing {} as {}", "TkAcceptableSize" );

pub struct TkButtonNoError( pub Obj );
impl_std_error_and_debug!( TkButtonNoError,
    "error occurred when converting {} as {}", "ButtonNo" );

pub enum TkCanvasItemTypeParseError{}
impl_std_error!{ TkCanvasItemTypeParseError }

impl Debug for TkCanvasItemTypeParseError {
    fn fmt( &self, _formatter: &mut fmt::Formatter ) -> fmt::Result {
        unreachable!()
    }
}

pub struct TkDumpParseError( pub Obj );
impl_std_error_and_debug!( TkDumpParseError,
    "error occurred when parsing {} as {}", "TkDump" );

pub struct TkEventTypeError( pub Obj );
impl_std_error_and_debug!( TkEventTypeError,
    "error occurred when converting {} as {}", "TkEventType" );

#[derive( Debug )]
pub struct TkGeometryParseError( pub String );
impl_std_error!{ TkGeometryParseError }

#[derive( Debug, Default )]
pub struct TkIndexParseError( pub String );
impl_std_error!{ TkIndexParseError }

pub struct TkNotifyModeParseError( pub Obj );
impl_std_error_and_debug!( TkNotifyModeParseError,
    "error occurred when converting {} as {}", "TkNotifyMode" );

pub struct TkPlaceOnParseError( pub Obj );
impl_std_error_and_debug!( TkPlaceOnParseError,
    "error occurred when converting {} as {}", "TkPlaceOn" );

#[derive( Debug )]
pub struct TkRequesterParseError( pub String );
impl_std_error!{ TkRequesterParseError }

pub struct TkResizableParseError( pub Obj );
impl_std_error_and_debug!( TkResizableParseError,
    "error occurred when parsing {} as {}", "TkResizable" );

#[derive( Debug )]
pub struct TkScalePartParseError( pub String );
impl_std_error!{ TkScalePartParseError }

#[derive( Debug )]
pub struct TkScreenNameParseError( pub String );
impl_std_error!{ TkScreenNameParseError }

#[derive( Debug )]
pub struct TkTextMarkGravityParseError( pub String );
impl_std_error!{ TkTextMarkGravityParseError }

pub struct TkValidatingActionError( pub Obj );
impl_std_error_and_debug!( TkValidatingActionError,
    "error occurred when converting {} as {}", "TkValidatingAction" );

pub struct TkValidationSetParseError( pub Obj );
impl_std_error_and_debug!( TkValidationSetParseError,
    "error occurred when converting {} as {}", "TkValidationSet" );

pub struct TkValidationOpParseError( pub Obj );
impl_std_error_and_debug!( TkValidationOpParseError,
    "error occurred when converting {} as {}", "TkValidationOp" );

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
        TkButtonNoError                         ,
        TkCanvasItemTypeParseError              ,
        TkDumpParseError                        ,
        TkEventTypeError                        ,
        TkGeometryParseError                    ,
        TkIndexParseError                       ,
        TkNotifyModeParseError                  ,
        TkPlaceOnParseError                     ,
        TkRequesterParseError                   ,
        TkResizableParseError                   ,
        TkScalePartParseError                   ,
        TkScrollbarElementParseError            ,
        TkSpinboxElementParseError              ,
        TkScreenNameParseError                  ,
        TkTextMarkGravityParseError             ,
        TkValidatingActionError                 ,
        TkValidationSetParseError               ,
        TkValidationOpParseError         ,
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
