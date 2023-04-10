use crate::*;
use crate::cmd::*;
use std::ops::{Neg, Sub};
use tuplex::*;

#[derive( Debug )]
pub struct OptPair {
    pub(crate) name : &'static str,
    pub(crate) value: Obj,
}

impl OptPair {
    pub(crate) fn value_only( value: impl Into<Obj> ) -> Self {
        OptPair{ name: "", value: value.into() }
    }
}

pub trait TkOption {
    const NAME: &'static str;
}

macro_rules! def_opts {
    ($($ty:ident $trait:ident $str:expr;)+) => {$(
        #[derive( Debug )]
        pub struct $ty( Obj );

        impl crate::TkOption for $ty {
            const NAME: &'static str = $str;
        }

        impl<T:Into<Obj>> $trait for T {
            type Output = $ty;

            fn output( self ) -> Self::Output { $ty( self.into() )}
        }

        impl From<$ty> for crate::OptPair {
            fn from( opt: $ty ) -> Self {
                crate::OptPair{ name: <$ty as crate::TkOption>::NAME, value: opt.0 }
            }
        }

        impl Neg for $ty {
            type Output = crate::cmd::PathOptsWidgets<($ty,), ()>;

            fn neg( self ) -> Self::Output {
                crate::cmd::PathOptsWidgets {
                    path   : "",
                    opts   : (self,),
                    widgets: (),
                }
            }
        }

        impl Sub<$ty> for &'static str {
            type Output = crate::cmd::PathOptsWidgets<($ty,), ()>;

            fn sub( self, rhs: $ty ) -> Self::Output {
                crate::cmd::PathOptsWidgets {
                    path   : self,
                    opts   : (rhs,),
                    widgets: (),
                }
            }
        }

        impl<O,L> Sub<$ty> for crate::cmd::PathOptsWidgets<O,L>
            where O: PushBack<$ty>
        {
            type Output = crate::cmd::PathOptsWidgets<<O as PushBack<$ty>>::Output,L>;

            fn sub( self, rhs: $ty ) -> Self::Output {
                crate::cmd::PathOptsWidgets {
                    path   : self.path,
                    opts   : self.opts.push_back( rhs ),
                    widgets: self.widgets,
                }
            }
        }
    )+};
}

def_opts! {
    TkAbove                     TkAboveFn                       "-above"                        ;
    TkAccelerator               TkAcceleratorFn                 "-accelerator"                  ;
    TkActiveBackground          TkActiveBackgroundFn            "-activebackground"             ;
    TkActiveBitmap              TkActiveBitmapFn                "-activebitmap"                 ;
    TkActiveBorderWidth         TkActiveBorderWidthFn           "-activeborderwidth"            ;
    TkActiveDash                TkActiveDashFn                  "-activedash"                   ;
    TkActiveFill                TkActiveFillFn                  "-activefill"                   ;
    TkActiveForeground          TkActiveForegroundFn            "-activeforeground"             ;
    TkActiveImage               TkActiveImageFn                 "-activeimage"                  ;
    TkActiveOutline             TkActiveOutlineFn               "-activeoutline"                ;
    TkActiveOutlineStipple      TkActiveOutlineStippleFn        "-activeoutlinestipple"         ;
    TkActiveRelief              TkActiveReliefFn                "-activerelief"                 ;
    TkActiveStipple             TkActiveStippleFn               "-activestipple"                ;
    TkActiveStyle               TkActiveStyleFn                 "-activestyle"                  ;
    TkActiveWidth               TkActiveWidthFn                 "-activewidth"                  ;
    TkAfter                     TkAfterFn                       "-after"                        ;
    TkAlign                     TkAlignFn                       "-align"                        ;
    TkAlpha                     TkAlphaFn                       "-alpha"                        ;
    TkAnchor                    TkAnchorFn                      "-anchor"                       ;
    TkAngle                     TkAngleFn                       "-angle"                        ;
    TkArrow                     TkArrowFn                       "-arrow"                        ;
    TkArrowShape                TkArrowShapeFn                  "-arrowshape"                   ;
    TkAscent                    TkAscentFn                      "-ascent"                       ;
    TkAspect                    TkAspectFn                      "-aspect"                       ;
    TkAutoSeperators            TkAutoSeperatorsFn              "-autoseperators"               ;
    TkBackground                TkBackgroundFn                  "-background"                   ;
    TkBd                        TkBdFn                          "-bd"                           ;
    TkBefore                    TkBeforeFn                      "-before"                       ;
    TkBg                        TkBgFn                          "-bg"                           ;
    TkBgStipple                 TkBgStippleFn                   "-bgstipple"                    ;
    TkBigIncrement              TkBigIncrementFn                "-bigincrement"                 ;
    TkBitmap                    TkBitmapFn                      "-bitmap"                       ;
    TkBlockCursor               TkBlockCursorFn                 "-blockcursor"                  ;
    TkBorderMode                TkBorderModeFn                  "-bordermode"                   ;
    TkBorderWidth               TkBorderWidthFn                 "-borderwidth"                  ;
    TkButton                    TkButtonFn                      "-button"                       ;
    TkButtonBackground          TkButtonBackgroundFn            "-buttonbackground"             ;
    TkButtonCursor              TkButtonCursorFn                "-buttoncursor"                 ;
    TkButtonDownRelief          TkButtonDownReliefFn            "-buttondownrelief"             ;
    TkButtonUpRelief            TkButtonUpReliefFn              "-buttonupRelief"               ;
    TkCapStyle                  TkCapStyleFn                    "-capstyle"                     ;
    TkChannel                   TkChannelFn                     "-channel"                      ;
    TkClass                     TkClassFn                       "-class"                        ;
    TkCloseEnough               TkCloseEnoughFn                 "-closeenough"                  ;
    TkColorMap                  TkColorMapFn                    "-colormap"                     ;
    TkColorMode                 TkColorModeFn                   "-colormode"                    ;
    TkColumn                    TkColumnFn                      "-column"                       ;
    TkColumnBreak               TkColumnBreakFn                 "-columnbreak"                  ;
    TkColumnSpan                TkColumnSpanFn                  "-columnspan"                   ;
    TkColumns                   TkColumnsFn                     "-columns"                      ;
    TkCommand                   TkCommandFn                     "-command"                      ;
    TkCompositingRule           TkCompositingruleFn             "-compositingrule"              ;
    TkCompound                  TkCompoundFn                    "-compound"                     ;
    TkConfine                   TkConfineFn                     "-confine"                      ;
    TkConfirmOverwrite          TkConfirmOverwriteFn            "-confirmoverwrite"             ;
    TkContainer                 TkContainerFn                   "-container"                    ;
    TkCount                     TkCountFn                       "-count"                        ;
    TkCreate                    TkCreateFn                      "-create"                       ;
    TkCursor                    TkCursorFn                      "-cursor"                       ;
    TkDash                      TkDashFn                        "-dash"                         ;
    TkDashOffset                TkDashOffsetFn                  "-dashoffset"                   ;
    TkData                      TkDataFn                        "-data"                         ;
    TkDescent                   TkDescentFn                     "-descent"                      ;
    TkDefault                   TkDefaultFn                     "-default"                      ;
    TkDefaultExtension          TkDefaultExtensionFn            "-defaultextension"             ;
    TkDelta                     TkDeltaFn                       "-delta"                        ;
    TkDetail                    TkDetailFn                      "-detail"                       ;
    TkDigits                    TkDigitsFn                      "-digits"                       ;
    TkDirection                 TkDirectionFn                   "-direction"                    ;
    TkDisabled                  TkDisabledFn                    "-disabled"                     ;
    TkDisabledBackground        TkDisabledBackgroundFn          "-disabledbackground"           ;
    TkDisabledBitmap            TkDisabledBitmapFn              "-disabledbitmap"               ;
    TkDisabledDash              TkDisabledDashFn                "-disableddash"                 ;
    TkDisabledFill              TkDisabledFillFn                "-disabledfill"                 ;
    TkDisabledForeground        TkDisabledForegroundFn          "-disabledforeground"           ;
    TkDisabledImage             TkDisabledImageFn               "-disabledimage"                ;
    TkDisabledOutline           TkDisabledOutlineFn             "-disabledoutline"              ;
    TkDisabledOutlineStipple    TkDisabledOutlineStippleFn      "-disabledoutlinestipple"       ;
    TkDisabledStipple           TkDisabledStippleFn             "-disabledstipple"              ;
    TkDisabledWidth             TkDisabledWidthFn               "-disabledwidth"                ;
    TkDisplayColumns            TkDisplayColumnsFn              "displaycolumns"                ;
    TkElementBorderWidth        TkElementBorderWidthFn          "-elementborderwidth"           ;
    TkElide                     TkElideFn                       "-elide"                        ;
    TkEndline                   TkEndlineFn                     "-endline"                      ;
    TkExpand                    TkExpandFn                      "-expand"                       ;
    TkExportSelection           TkExportSelectionFn             "-exportselection"              ;
    TkExtent                    TkExtentFn                      "-extent"                       ;
    TkFamily                    TkFamilyFn                      "-family"                       ;
    TkFgStripple                TkFgStrippleFn                  "-fgstripple"                   ;
    TkFile                      TkFileFn                        "-file"                         ;
    TkFileTypes                 TkFileTypesFn                   "-filetypes"                    ;
    TkFill                      TkFillFn                        "-fill"                         ;
    TkFixed                     TkFixedFn                       "-fixed"                        ;
    TkFocus                     TkFocusFn                       "-focus"                        ;
    TkFont                      TkFontFn                        "-font"                         ;
    TkFontMap                   TkFontMapFn                     "-fontmap"                      ;
    TkForeground                TkForegroundFn                  "-foreground"                   ;
    TkFormat                    TkFormatFn                      "-format"                       ;
    TkFrom                      TkFromFn                        "-from"                         ;
    TkFullScreen                TkFullScreenFn                  "-fullscreen"                   ;
    TkGamma                     TkGammaFn                       "-gamma"                        ;
    TkGrayscale                 TkGrayscaleFn                   "-grayscale"                    ;
    TkHandlePad                 TkHandlePadFn                   "-handlepad"                    ;
    TkHandleSize                TkHandleSizeFn                  "-handlesize"                   ;
    TkHeight                    TkHeightFn                      "-height"                       ;
    TkHide                      TkHideFn                        "-hide"                         ;
    TkHideMargin                TkHideMarginFn                  "-hidemargin"                   ;
    TkHighlightBackground       TkHighlightBackgroundFn         "-highlightbackground"          ;
    TkHighlightColor            TkHighlightColorFn              "-highlightcolor"               ;
    TkHighlightThickness        TkHighlightThicknessFn          "-highlightthickness"           ;
    TkIcon                      TkIconFn                        "-icon"                         ;
    TkId                        TkIdFn                          "-id"                           ;
    TkImage                     TkImageFn                       "-image"                        ;
    TkImargin1                  TkImargin1Fn                    "-imargin1"                     ;
    TkImargin2                  TkImargin2Fn                    "-imargin2"                     ;
    TkImarginColor              TkImarginColorFn                "-imargincolor"                 ;
    TkIn                        TkInFn                          "-in"                           ;
    TkInactiveSelectBackground  TkInactiveSelectBackgroundFn    "-inactiveselectbackground"     ;
    TkIncrement                 TkIncrementFn                   "-increment"                    ;
    TkIndicatorOn               TkIndicatorOnFn                 "-indicatoron"                  ;
    TkInitialColor              TkInitialColorFn                "-initialcolor"                 ;
    TkInitialDir                TkInitialDirFn                  "-initialdir"                   ;
    TkInitialFile               TkInitialFileFn                 "-initialfile"                  ;
    TkInsertBackground          TkInsertBackgroundFn            "-insertbackground"             ;
    TkInsertBorderWidth         TkInsertBorderWidthFn           "-insertborderwidth"            ;
    TkInsertOffTime             TkInsertOffTimeFn               "-insertofftime"                ;
    TkInsertOnTime              TkInsertOnTimeFn                "-insertontime"                 ;
    TkInsertUnfocussed          TkInsertUnfocussedFn            "-insertunfocussed"             ;
    TkInsertWidth               TkInsertWidthFn                 "-insertwidth"                  ;
    TkInvCmd                    TkInvCmdFn                      "-invcmd"                       ;
    TkInvalidCommand            TkInvalidCommandFn              "-invalidcommand"               ;
    TkIPadX                     TkIPadXFn                       "-ipadx"                        ;
    TkIPadY                     TkIPadYFn                       "-ipady"                        ;
    TkJoinStyle                 TkJoinStyleFn                   "-joinstyle"                    ;
    TkJump                      TkJumpFn                        "-jump"                         ;
    TkJustify                   TkJustifyFn                     "-justify"                      ;
    TkKeyCode                   TkKeyCodeFn                     "-keycode"                      ;
    TkKeySym                    TkKeySymFn                      "-keysym"                       ;
    TkLabel                     TkLabelFn                       "-label"                        ;
    TkLabelAnchor               TkLabelAnchorFn                 "-labelanchor"                  ;
    TkLabelWidget               TkLabelWidgetFn                 "-labelwidget"                  ;
    TkLength                    TkLengthFn                      "-length"                       ;
    TkLinespace                 TkLinespaceFn                   "-linespace"                    ;
    TkListVariable              TkListVariableFn                "-listvariable"                 ;
    TkMaskData                  TkMaskDataFn                    "-maskdata"                     ;
    TkMaskFile                  TkMaskFileFn                    "-maskfile"                     ;
    TkMaxUndo                   TkMaxUndoFn                     "-maxundo"                      ;
    TkMaximum                   TkMaximumFn                     "-maximum"                      ;
    TkMenu                      TkMenuFn                        "-menu"                         ;
    TkMessage                   TkMessageFn                     "-message"                      ;
    TkMinSize                   TkMinSizeFn                     "-minsize"                      ;
    TkMinWidth                  TkMinWidthFn                    "-minwidth"                     ;
    TkMode                      TkModeFn                        "-mode"                         ;
    TkModified                  TkModifiedFn                    "-modified"                     ;
    TkMultiple                  TkMultipleFn                    "-multiple"                     ;
    TkMustExist                 TkMustExistFn                   "-mustexist"                    ;
    TkName                      TkNameFn                        "-tkname"                       ;
    TkNotify                    TkNotifyFn                      "-notify"                       ;
    TkOffRelief                 TkOffReliefFn                   "-offrelief"                    ;
    TkOffValue                  TkOffValueFn                    "-offvalue"                     ;
    TkOffset                    TkOffsetFn                      "-offset"                       ;
    TkOnValue                   TkOnValueFn                     "-onvalue"                      ;
    TkOpaqueResize              TkOpaqueResizeFn                "-opaqueresize"                 ;
    TkOpen                      TkOpenFn                        "-open"                         ;
    TkOrient                    TkOrientFn                      "-orient"                       ;
    TkOutline                   TkOutlineFn                     "-outline"                      ;
    TkOutlineOffset             TkOutlineOffsetFn               "-outlineoffset"                ;
    TkOutlineStipple            TkOutlineStippleFn              "-outlinestipple"               ;
    TkOverRelief                TkOverReliefFn                  "-overrelief"                   ;
    TkOverride                  TkOverrideFn                    "-override"                     ;
    TkOverstrike                TkOverstrikeFn                  "-overstrike"                   ;
    TkOverstrikeFg              TkOverstrikeFgFn                "-overstrikefg"                 ;
    TkPadding                   TkPaddingFn                     "-padding"                      ;
    TkPad                       TkPadFn                         "-pad"                          ;
    TkPadX                      TkPadxFn                        "-padx"                         ;
    TkPadY                      TkPadyFn                        "-pady"                         ;
    TkPageAnchor                TkPageAnchorFn                  "-pageanchor"                   ;
    TkPageHeight                TkPageHeightFn                  "-pageheight"                   ;
    TkPageWidth                 TkPageWidthFn                   "-pagewidth"                    ;
    TkPageX                     TkPageXFn                       "-pagex"                        ;
    TkPageY                     TkPageYFn                       "-pagey"                        ;
    TkPalette                   TkPaletteFn                     "-palette"                      ;
    TkParent                    TkParentFn                      "-parent"                       ;
    TkPhase                     TkPhaseFn                       "-phase"                        ;
    TkPlace                     TkPlaceFn                       "-place"                        ;
    TkPostCommand               TkPostCommandFn                 "-postcommand"                  ;
    TkProxyBackground           TkProxyBackgroundFn             "-proxybackground"              ;
    TkProxyBorderWidth          TkProxyBorderWidthFn            "-proxyborderwidth"             ;
    TkProxyRelief               TkProxyReliefFn                 "-proxyrelief"                  ;
    TkRMargin                   TkRMarginFn                     "-rmargin"                      ;
    TkRMarginColor              TkRMarginColorFn                "-rmargincolor"                 ;
    TkReadOnlyBackground        TkReadOnlyBackgroundFn          "-readonlybackground"           ;
    TkRelief                    TkReliefFn                      "-relief"                       ;
    TkRelHeight                 TkRelHeightFn                   "-relheight"                    ;
    TkRelWidth                  TkRelWidthFn                    "-relwidth"                     ;
    TkRelX                      TkRelXFn                        "-relx"                         ;
    TkRelY                      TkRelYFn                        "-rely"                         ;
    TkRepeatDelay               TkRepeatDelayFn                 "-repeatdelay"                  ;
    TkRepeatInterval            TkRepeatIntervalFn              "-repeatinterval"               ;
    TkResolution                TkResolutionFn                  "-resolution"                   ;
    TkRoot                      TkRootFn                        "-root"                         ;
    TkRootX                     TkRootxFn                       "-rootx"                        ;
    TkRootY                     TkRootyFn                       "-rooty"                        ;
    TkRotate                    TkRotateFn                      "-rotate"                       ;
    TkRow                       TkRowFn                         "-row"                          ;
    TkRowSpan                   TkRowSpanFn                     "-rowspan"                      ;
    TkSashCursor                TkSashCursorFn                  "-sashcursor"                   ;
    TkSashPad                   TkSashPadFn                     "-sashpad"                      ;
    TkSashRelief                TkSashReliefFn                  "-sashrelief"                   ;
    TkSashWidth                 TkSashWidthFn                   "-sashwidth"                    ;
    TkScreen                    TkScreenFn                      "-screen"                       ;
    TkScrollRegion              TkScrollRegionFn                "-scrollregion"                 ;
    TkSelectBackground          TkSelectBackgroundFn            "-selectbackground"             ;
    TkSelectBorderWidth         TkSelectBorderWidthFn           "-selectborderwidth"            ;
    TkSelectColor               TkSelectColorFn                 "-selectcolor"                  ;
    TkSelectForeground          TkSelectForegroundFn            "-selectforeground"             ;
    TkSelectImage               TkSelectImageFn                 "-selectimage"                  ;
    TkSelectMode                TkSelectModeFn                  "-selectmode"                   ;
    TkSendEvent                 TkSendEventFn                   "-sendevent"                    ;
    TkSerial                    TkSerialFn                      "-serial"                       ;
    TkSetGrid                   TkSetGridFn                     "-setgrid"                      ;
    TkSettings                  TkSettingsFn                    "-settings"                     ;
    TkShow                      TkShowFn                        "-show"                         ;
    TkShowHandle                TkShowHandleFn                  "-showhandle"                   ;
    TkShowValue                 TkShowValueFn                   "-showvalue"                    ;
    TkShrink                    TkShrinkFn                      "-shrink "                      ;
    TkSide                      TkSideFn                        "-side"                         ;
    TkSize                      TkSizeFn                        "-size"                         ;
    TkSlant                     TkSlantFn                       "-slant"                        ;
    TkSliderLength              TkSliderLengthFn                "-sliderlength"                 ;
    TkSlideRelief               TkSlideReliefFn                 "-sliderelief"                  ;
    TkSmooth                    TkSmoothFn                      "-smooth"                       ;
    TkSpacing1                  TkSpacing1Fn                    "-spacing1"                     ;
    TkSpacing2                  TkSpacing2Fn                    "-spacing2"                     ;
    TkSpacing3                  TkSpacing3Fn                    "-spacing3"                     ;
    TkSplineSteps               TkSplineStepsFn                 "-splinesteps"                  ;
    TkStart                     TkStartFn                       "-start"                        ;
    TkStartline                 TkStartlineFn                   "-startline"                    ;
    TkState                     TkStateFn                       "-state"                        ;
    TkSticky                    TkStickyFn                      "-sticky"                       ;
    TkStipple                   TkStippleFn                     "-stipple"                      ;
    TkStretch                   TkStretchFn                     "-stretch"                      ;
    TkStyle                     TkStyleFn                       "-style"                        ;
    TkSubSample                 TkSubsampleFn                   "-subsample"                    ;
    TkSubWindow                 TkSubWindowFn                   "-subwindow"                    ;
    TkTabStyle                  TkTabStyleFn                    "-tabstyle"                     ;
    TkTabs                      TkTabsFn                        "-tabs"                         ;
    TkTags                      TkTagsFn                        "-tags"                         ;
    TkTakeFocus                 TkTakeFocusFn                   "-takefocus"                    ;
    TkTearOff                   TkTearOffFn                     "-tearoff"                      ;
    TkTearOffCommand            TkTearOffCommandFn              "-tearoffcommand"               ;
    TkText                      TkTextFn                        "-text"                         ;
    TkTextVariable              TkTextVariableFn                "-textvariable"                 ;
    TkTickInterval              TkTickIntervalFn                "-tickinterval"                 ;
    TkTitle                     TkTitleFn                       "-title"                        ;
    TkTitlePath                 TkTitlePathFn                   "-titlepath"                    ;
    TkTime                      TkTimeFn                        "-time"                         ;
    TkTo                        TkToFn                          "-to"                           ;
    TkToolWindow                TkToolWindowFn                  "-toolwindow"                   ;
    TkTopmost                   TkTopmostFn                     "-topmost"                      ;
    TkTransparent               TkTransparentFn                 "-transparent"                  ;
    TkTransparentColor          TkTransparentColorFn            "-transparentcolor"             ;
    TkTristateImage             TkTristateImageFn               "-tristateimage"                ;
    TkTristateValue             TkTristateValueFn               "-tristatevalue"                ;
    TkTroughColor               TkTroughColorFn                 "-troughcolor"                  ;
    TkType                      TkTypeFn                        "-type"                         ;
    TkTypeVariable              TkTypeVariableFn                "-typevariable"                 ;
    TkUnderline                 TkUnderlineFn                   "-underline"                    ;
    TkUnderlineFg               TkUnderlineFgFn                 "-underlinefg"                  ;
    TkUndo                      TkUndoFn                        "-undo"                         ;
    TkUniform                   TkUniformFn                     "-uniform"                      ;
    TkUse                       TkUseFn                         "-use"                          ;
    TkVariable                  TkVariableFn                    "-variable"                     ;
    TkValidate                  TkValidateFn                    "-validate"                     ;
    TkValidateCommand           TkValidateCommandFn             "-validatecommand"              ;
    TkValue                     TkValueFn                       "-value"                        ;
    TkValues                    TkValuesFn                      "-values"                       ;
    TkVisible                   TkVisibleFn                     "-visible"                      ;
    TkVisual                    TkVisualFn                      "-visual"                       ;
    TkWeight                    TkWeightFn                      "-weight"                       ;
    TkWidth                     TkWidthFn                       "-width"                        ;
    TkWindow                    TkWindowFn                      "-window"                       ;
    TkWarp                      TkWarpFn                        "-warp"                         ;
    TkWhen                      TkWhenFn                        "-when"                         ;
    TkWrap                      TkWrapFn                        "-wrap"                         ;
    TkWrapLength                TkWrapLengthFn                  "-wraplength"                   ;
    TkX                         TkXFn                           "-x"                            ;
    TkXScrollCommand            TkXScrollCommandFn              "-xscrollcommand"               ;
    TkXScrollIncrement          TkXScrollIncrementFn            "-xscrollincrement"             ;
    TkY                         TkYFn                           "-y"                            ;
    TkYScrollCommand            TkYScrollCommandFn              "-yscrollcommand"               ;
    TkYScrollIncrement          TkYScrollIncrementFn            "-yscrollincrement"             ;
    TkZoom                      TkZoomFn                        "-zoom"                         ;
    TkZoomed                    TkZoomedFn                      "-zoomed"                       ;
}

macro_rules! def_widget_opts {
    ($($widget_opt:ident: ($($opt:path,)*),)+) => {$(
        pub struct $widget_opt( crate::OptPair );

        $(
            impl From<$opt> for $widget_opt {
                fn from( opt: $opt ) -> Self {
                    $widget_opt( crate::OptPair::from( opt ))
                }
            }
        )*
    )+}
}

def_widget_opts! {
    TkBusyHoldOpt: (
        TkCursor,
    ),
    TkCaretOpt: (
        TkX,
        TkY,
        TkHeight,
    ),
    TkFontChooserOpt: (
        TkCommand,
        TkFont,
        TkParent,
        TkTitle,
        TkVisible,
    ),
    TkChooseColorOpt: (
        TkInitialColor,
        TkParent,
        TkTitle,
    ),
    TkChooseDirectoryOpt: (
        TkCommand,
        TkInitialDir,
        TkMessage,
        TkMustExist,
        TkParent,
        TkTitle,
    ),
    TkGetOpenFileOpt: (
        TkCommand,
        TkConfirmOverwrite,
        TkDefaultExtension,
        TkFileTypes,
        TkInitialDir,
        TkInitialFile,
        TkMessage,
        TkMultiple,
        TkParent,
        TkTitle,
        TkTypeVariable,
    ),
    TkMessageBoxOpt: (
        TkCommand,
        TkDefault,
        TkDetail,
        TkIcon,
        TkMessage,
        TkParent,
        TkTitle,
        TkType,
    ),
    TkSetPaletteOpt: (
        TkActiveBackground,
        TkActiveForeground,
        TkBackground,
        TkDisabledForeground,
        TkForeground,
        TkHighlightBackground,
        TkHighlightColor,
        TkInsertBackground,
        TkSelectColor,
        TkSelectBackground,
        TkSelectForeground,
        TkTroughColor,
    ),
    TkThemeCreateOpt: (
        TkParent,
        TkSettings,
    ),
}

include!( "opt/button.rs"           );
include!( "opt/canvas.rs"           );
include!( "opt/checkbutton.rs"      );
include!( "opt/entry.rs"            );
include!( "opt/event.rs"            );
include!( "opt/frame.rs"            );
include!( "opt/font.rs"             );
include!( "opt/image.rs"            );
include!( "opt/label.rs"            );
include!( "opt/labelframe.rs"       );
include!( "opt/listbox.rs"          );
include!( "opt/menu.rs"             );
include!( "opt/menubutton.rs"       );
include!( "opt/message.rs"          );
include!( "opt/panedwindow.rs"      );
include!( "opt/radiobutton.rs"      );
include!( "opt/scale.rs"            );
include!( "opt/scrollbar.rs"        );
include!( "opt/spinbox.rs"          );
include!( "opt/text.rs"             );
include!( "opt/toplevel.rs"         );
include!( "opt/ttk_button.rs"       );
include!( "opt/ttk_checkbutton.rs"  );
include!( "opt/ttk_combobox.rs"     );
include!( "opt/ttk_entry.rs"        );
include!( "opt/ttk_frame.rs"        );
include!( "opt/ttk_label.rs"        );
include!( "opt/ttk_labelframe.rs"   );
include!( "opt/ttk_menubutton.rs"   );
include!( "opt/ttk_notebook.rs"     );
include!( "opt/ttk_panedwindow.rs"  );
include!( "opt/ttk_progressbar.rs"  );
include!( "opt/ttk_radiobutton.rs"  );
include!( "opt/ttk_scale.rs"        );
include!( "opt/ttk_scrollbar.rs"    );
include!( "opt/ttk_separator.rs"    );
include!( "opt/ttk_sizegrip.rs"     );
include!( "opt/ttk_spinbox.rs"      );
include!( "opt/ttk_treeview.rs"     );
include!( "opt/wm.rs"               );

def_widget_opts! {
    TkGridOpt                : (TkColumn, TkColumnSpan, TkIn, TkIPadX, TkIPadY, TkPadX, TkPadY, TkRow, TkRowSpan, TkSticky,),
    TkPackOpt                : (TkAfter, TkAnchor, TkBefore, TkExpand, TkFill, TkIn, TkIPadX, TkIPadY, TkPadX, TkPadY, TkSide,),
    TkPlaceOpt               : (TkAnchor, TkBorderMode, TkHeight, TkIn, TkRelHeight, TkRelWidth, TkRelX, TkRelY, TkWidth, TkX, TkY,),
    TkGridColumnConfigureOpt : (TkMinSize, TkWeight, TkUniform, TkPad,),
    TkGridRowConfigureOpt    : (TkMinSize, TkWeight, TkUniform, TkPad,),
    TkRowColumnOpt           : (TkRow, TkColumn,),
}
