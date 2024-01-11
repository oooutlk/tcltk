//! Proc macro for tcl (and tk).

use bind_syn::Bind;

use proc_macro::TokenStream;

use quote::quote;

use syn::{
    Block,
    DeriveInput,
    Expr,
    ExprBlock,
    ExprClosure,
    FnArg,
    GenericParam,
    Generics,
    Ident,
    Item,
    ItemFn,
    Pat,
    PatIdent,
    PatType,
    ReturnType,
    Stmt,
    Token,
    Type,
    parenthesized,
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    parse_quote,
    punctuated::Punctuated,
    token::{Comma, Colon},
    visit_mut::VisitMut,
};

use proc_macro2::Span;

use uuid::Uuid;

struct TclProc;

impl VisitMut for TclProc {
    fn visit_item_fn_mut( &mut self, item_fn: &mut ItemFn ) {
        let argc = item_fn.sig.inputs.len();
        item_fn.sig.abi = Some( parse_quote!( extern "C" ));

        let variadic = item_fn.sig.variadic.take();
        let is_variadic = variadic.is_some();

        let mut inputs: Punctuated<FnArg,Comma> = parse_quote!(
            __client_data: tcl::reexport_clib::ClientData, __tcl_interp: *mut tcl::reexport_clib::Tcl_Interp, __objc: std::os::raw::c_int, __objv: *const *mut tcl::reexport_clib::Tcl_Obj
        );
        if inputs == item_fn.sig.inputs {
            return;
        }
        std::mem::swap( &mut inputs, &mut item_fn.sig.inputs );

        let mut output: ReturnType = parse_quote!( -> std::os::raw::c_int );
        std::mem::swap( &mut output, &mut item_fn.sig.output );

        let mut existing_stmts = Vec::<Stmt>::new();
        std::mem::swap( &mut existing_stmts, &mut item_fn.block.stmts );

        let mut body: Block = parse_quote! {{
            let mut __interp = unsafe{ tcl::Interp::from_raw( __tcl_interp ).unwrap() };
            let __origin_objs: &[*mut tcl::reexport_clib::Tcl_Obj] = unsafe{ std::slice::from_raw_parts( __objv.offset(1), (__objc-1) as usize )};
            if __origin_objs.len() != #argc {
                if __origin_objs.len() < #argc || !#is_variadic {
                    unsafe {
                        tcl::reexport_clib::Tcl_WrongNumArgs( __tcl_interp, 1, __objv, std::ptr::null() );
                        tcl::CodeToResult::code_to_result( tcl::reexport_clib::TCL_ERROR as std::os::raw::c_int, &__interp )?;
                    }
                }
            }
            let mut __objs = __origin_objs[..#argc].to_vec();
            let mut __variadic_args = __origin_objs[#argc..].to_vec();
            use std::convert::TryFrom;

            let mut __ref_objs = std::collections::HashMap::<&'static str, *mut tcl::reexport_clib::Tcl_Obj>::new();

            macro_rules! tcl_invalidate_str_rep {
                ($ident:ident) => {
                    __ref_objs.get( stringify!( $ident )).map( |tcl_obj| unsafe{
                        tcl::reexport_clib::Tcl_InvalidateStringRep( *tcl_obj );
                    });
                };
            }

            macro_rules! tcl_interp { () => { __interp }}
            macro_rules! tcl_variadic_args { () => { __variadic_args }}
        }};

        body.stmts.reserve( argc * 5 + existing_stmts.len() );

        let mut args = Vec::new();
        for (nth, arg) in inputs.iter().enumerate() {
            match arg {
                FnArg::Receiver(_) => panic!("#[proc] does not support method"),
                FnArg::Typed( pat_type ) => {
                    let pat = &*pat_type.pat;
                    args.push( pat.clone() );

                    let ty = &*pat_type.ty;
                    match &*ty {
                        Type::Reference( type_ref ) => match pat {
                            Pat::Ident( pat_ident ) => {
                                body.stmts.push( parse_quote!( unsafe {
                                    let origin_obj = __objs[ #nth ];
                                    __objs[ #nth ] = __interp.get( Obj::from_raw( origin_obj ))?.as_ptr();
                                }));
                                let ident = &pat_ident.ident;
                                body.stmts.push( parse_quote!(
                                    __ref_objs.entry( stringify!( #ident )).or_insert( __objs[ #nth ]);
                                ));

                                let ty_elem = &*type_ref.elem;
                                body.stmts.push( parse_quote!(
                                    let mut __obj = unsafe{ tcl::Obj::from_raw( __objs[ #nth ])};
                                ));
                                body.stmts.push( parse_quote!(
                                    let #ident = tcl::Tcl::<#ty_elem>::ptr_from_obj( __obj )?;
                                ));
                                if type_ref.mutability.is_none() {
                                    body.stmts.push( parse_quote!(
                                        let #ident: #ty = &*unsafe{ #ident.as_ref() }.deref().borrow();
                                    ));
                                } else {
                                    body.stmts.push( parse_quote!(
                                        let #ident: #ty = &mut *unsafe{ #ident.as_ref() }.deref().borrow_mut();
                                    ));
                                }
                            },
                            _ => panic!("#[tcl_proc] argument should be in the form of `ident: Type`"),
                        },
                        _ => {
                            args.push( pat.clone() );

                            body.stmts.push( parse_quote!(
                                let mut __obj = unsafe{ tcl::Obj::from_raw( __objs[ #nth ])};
                            ));
                            body.stmts.push( parse_quote!(
                                //let #pat = tcl::from_obj::<#ty>( &__obj )?;
                                let #pat = <#ty>::try_from( __obj )?;
                            ));
                        },
                    }
                },
            }
        }

        body.stmts.extend( existing_stmts );

        let tcl_inputs = &item_fn.sig.inputs;
        let mut attrs = Vec::new();
        std::mem::swap( &mut attrs, &mut item_fn.attrs );

        let mut new_body: Block = parse_quote! {{
            use tcl::UnwrapOrAbort;

            std::panic::catch_unwind( || {
                let mut __tcl_completion_code: std::os::raw::c_int = tcl::reexport_clib::TCL_OK as std::os::raw::c_int;

                #(#attrs)* fn __tcl_inner_proc( #tcl_inputs ) #output #body

                match __tcl_inner_proc( __client_data, __tcl_interp, __objc, __objv ) {
                    Ok( value ) => unsafe {
                        tcl::reexport_clib::Tcl_SetObjResult( __tcl_interp, Obj::from( value ).into_raw() );
                    },
                    Err( _ ) => __tcl_completion_code = tcl::reexport_clib::TCL_ERROR as std::os::raw::c_int,
                }

                __tcl_completion_code
            })
            .unwrap_or_abort( "Abort process to prevent undefined behaviour on panic across an FFI boundary." )
        }};

        std::mem::swap( &mut *item_fn.block, &mut new_body );
    }

    fn visit_block_mut( &mut self, block: &mut Block ) {
        if block.stmts.len() == 1 {
            let mut ident = None;
            if let Stmt::Item( Item::Fn( item_fn )) = block.stmts.first().unwrap() {
                ident = Some( item_fn.sig.ident.clone() );
            }
            let ident = ident.unwrap();
            block.stmts.push( parse_quote!( #ident ));

            if let Stmt::Item( Item::Fn( item_fn )) = block.stmts.first_mut().unwrap() {
                self.visit_item_fn_mut( item_fn );
                return;
            }
        }
        panic!("#[proc] attribute supports fn and block only");
    }
}

/// A proc macro attribute for filling the gap between `ObjCmdProc` functions and Rust functions with "normal" input arguments.
///
/// # Purpose
///
/// Generally, a Tcl command's function signature is `extern "C" fn( tcl::reexport_clib::ClientData, *mut tcl::reexport_clib::Tcl_Interp, c_int, *const *mut tcl::reexport_clib::Tcl_Obj ) -> c_int`, aka `ObjCmdProc`.
/// Arguments are stored in an array of `tcl::reexport_clib::Tcl_Obj`, and must be converted to real types before using them, which is boring.
///
/// With the help of the `#[proc]` attribute, The translation of function signatures and conversions of arguments are generated behind the proc macros.
///
/// This attributes will generate code to catch panics and abort the program to avoid undefined behaviour caused by panic across FFI boundary.
///
/// # Constraints
///
/// Two category of parameters are allowed:
///
/// - Owned value
///
///   Implementation detail: using `std::convert::TryFrom` to obtain the argument.
///
/// - Borrowed value
///
///   Implementation detail: borrowing from `Tcl<T>` to obtain the argument which borrows `T`.
///
/// The returning type is preferred but not mandatory a `Result<T,E>`:
///
/// - The `Ok` value will be converted to a Tcl `Obj` which is the result for the interpreter.
///
/// - The `Err` value must be able to be converted from the following errors: `InterpError`, `DeError`, `NullDataPtr`.
///
///   If all the errors are from this crate, you can simply use `-> TclResult<T>`.
///
///   If all the errors implement `std::error::Error`, you can simply use `-> Result<T, Box<dyn std::error::Error>>`.
///
///   Otherwise you can use checked exceptions: `#[cex] fn your_command( /* args omitted */ ) -> Result!( T throws InterpError, DeError, NullDataPtr, YourErrors )`.
#[proc_macro_attribute]
pub fn proc( _args: TokenStream, input: TokenStream ) -> TokenStream {
    if let Ok( mut input ) = syn::parse::<ItemFn>( input.clone() ) {
        // #[proc] item_fn
        TclProc.visit_item_fn_mut( &mut input );
        let expanded = quote!{
            #[allow( unused_macros )]
            #input
        };
        expanded.into()
    } else if let Ok( mut input ) = syn::parse::<Block>( input ) {
        // #[proc] { item_fn }
        TclProc.visit_block_mut( &mut input );
        TokenStream::from( quote!{
            #[allow( unused_macros )]
            #input
        })
    } else {
        panic!("tcl_derive::proc supports functions and blocks only.");
    }
}

fn callback_fn( interp: Expr, cmd: Option<Expr>, args: Option<Expr>, item_fn: ItemFn ) -> TokenStream {
    let ident = &item_fn.sig.ident;
    let inputs = &item_fn.sig.inputs;

    let cmd = cmd.unwrap_or_else( || parse_quote!{ stringify!( #ident )});
    let args = args.unwrap_or_else( || parse_quote!( "" ));

    let mut default_values = Vec::<>::with_capacity( inputs.len() );
    for arg in inputs.iter().rev() {
        if let FnArg::Typed( pat_type ) = arg {
            let default_value =
                (*pat_type).attrs.iter().find_map( |attr| {
                    if let syn::Meta::List( meta_list ) = &attr.meta {
                        let segments = &meta_list.path.segments;
                        if segments.len() == 1 && segments.first().unwrap().ident == "default" {
                            return Some( meta_list.tokens.clone() );
                        }
                    }
                    None
                })
            ;
            match default_value {
                Some( value ) => {
                    default_values.push( value );
                },
                None => break,
            }
        }
    }

    let expanded = if default_values.is_empty() {
        quote! {{
            #[tcl::proc]
            #[allow( unused_macros )]
            #item_fn

            let cmd = #cmd;
            unsafe{ (#interp).def_proc( cmd, #ident ); }
            format!( "{} {}", cmd, #args )
        }}
    } else {
        let argc = inputs.len();
        let optional_argc = default_values.len();
        let required_argc = argc - optional_argc;

        let param_list = format!( "{}{}",
            (0..required_argc   ).fold( String::new(), |acc,n|
                format!( "{acc} arg{n}" )),
            (required_argc..argc).fold( String::new(), |acc,n|
                format!( "{acc} {{ arg{n} {} }}", default_values[ n-required_argc ])),
        );
        let params = (0..argc).fold( String::new(), |acc,n| format!( "{acc} $arg{n}" ));

        let uuid = make_ident( &format!( "__tcl_fn_inner_{}", Uuid::new_v4().simple() ));
        let name: Expr = parse_quote!{ &format!( "{}", stringify!( #uuid ))};

        quote! {{
            #[tcl::proc]
            #[allow( unused_macros )]
            #item_fn

            let cmd = #cmd;
            unsafe {
                (#interp).def_proc( #name, #ident );

                (#interp).run(
                    format!( "proc {} {{ {} }} {{ {} {} }}", cmd, #param_list, #name, #params )
                ).ok();
            }

            format!( "{} {}", cmd, #args )
        }}
    };

    expanded.into()
}

/// Helps to register rust functions as Tcl commands.
///
/// # Syntax
///
/// `tclfn!( interp, cmd, args, func )`
///
/// # Input parameters
///
/// 1. interp, the Tcl interpreter instance.
///
/// 2. cmd, the name of the command being registered in Tcl. Optional.
///
/// 3. args, the arguments provided in Tcl on executing the command. Optional.
///
/// 4. func, the function defined in Rust.
///   Note: an attribute `#[default(value)]` on an parameter will assign a default `value` for this parameter.
///
/// # Output
///
/// Returns a `String` of the command name.
///
/// # Example
///
/// ```rust,no_run
///
/// use tcl::*;
///
/// let interpreter = Interpreter::new()?;
///
/// let cmd = tclfn!( &interpreter, /*cmd: "mul", args: "",*/
///     fn mul( a: i32, b: i32 ) -> TclResult<i32> { Ok( a * b )}
/// );
///
/// let c = interpreter.eval( "mul 3 7" )?;
/// assert_eq!( c.as_i32(), 21 );
/// ```
#[proc_macro]
pub fn tclfn( input: TokenStream ) -> TokenStream {
    struct TclFn {
        interp : Expr,
        cmd    : Option<Expr>,
        args   : Option<Expr>,
        func   : ItemFn,
    }

    impl Parse for TclFn {
        fn parse( input: ParseStream ) -> parse::Result<Self> {
            let interp = input.parse::<Expr>()?;
            input.parse::<Token![,]>()?;

            let (mut cmd, mut args) = (None, None);
            while !input.is_empty() && input.peek( Ident ) {
                match input.parse::<Ident>()?.to_string().as_str() {
                    "cmd"  => {
                        input.parse::<Token![:]>()?;
                        cmd = Some( input.parse::<Expr>()? );
                    },
                    "args" => {
                        input.parse::<Token![:]>()?;
                        args = Some( input.parse::<Expr>()? );
                    },
                    _ => panic!( "unsupported named arguments of tclosure!(), should be `cmd` or `args`."),
                }
                input.parse::<Token![,]>()?;
            }

            let func = input.parse::<ItemFn>()?;
            Ok( TclFn{ interp, cmd, args, func })
        }
    }

    let TclFn{ interp, cmd, args, func } = parse_macro_input!( input as TclFn );
    callback_fn( interp, cmd, args, func )
}

struct TclosureInput {
    tk      : Expr,
    cmd     : Option<Expr>,
    args    : Option<Expr>,
    bind    : Option<Punctuated<Bind,Token![,]>>,
    closure : ExprClosure,
}

impl Parse for TclosureInput {
    fn parse( input: ParseStream ) -> parse::Result<Self> {
        let tk = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;

        let (mut cmd, mut args, mut bind) = (None, None, None);
        while !input.is_empty() && input.peek( Ident ) {
            match input.parse::<Ident>()?.to_string().as_str() {
                "cmd"  => {
                    input.parse::<Token![:]>()?;
                    cmd = Some( input.parse::<Expr>()? );
                }
                "args" => {
                    input.parse::<Token![:]>()?;
                    args = Some( input.parse::<Expr>()? );
                }
                "bind" => {
                    input.parse::<Token![:]>()?;
                    let content;
                    parenthesized!( content in input );
                    bind = Some( Punctuated::parse_terminated( &content )? );
                }
                _ => panic!( "unsupported named arguments of tclosure!(), should be `cmd`, `args` or `bind`."),
            }
            input.parse::<Token![,]>()?;
        }

        let closure = input.parse::<ExprClosure>()?;

        Ok( TclosureInput{ tk, cmd, args, bind, closure })
    }
}

fn tk_event_detail_name_and_type( id: &Ident ) -> Option<(&'static str, Type)> {
    match id.to_string().as_str() {
        "evt_serial"         => Some((" %#", parse_quote!( std::ffi::c_int         ))),
        "evt_above"          => Some((" %a", parse_quote!( std::ffi::c_int         ))),
        "evt_button"         => Some((" %b", parse_quote!( tk::event::ButtonNo     ))),
        "evt_count"          => Some((" %c", parse_quote!( std::ffi::c_int         ))),
        "evt_detail"         => Some((" %d", parse_quote!( tcl::Obj                ))),
        "evt_focus"          => Some((" %f", parse_quote!( bool                    ))),
        "evt_height"         => Some((" %h", parse_quote!( std::ffi::c_int         ))),
        "evt_window"         => Some((" %i", parse_quote!( std::ffi::c_int         ))),
        "evt_keycode"        => Some((" %k", parse_quote!( std::ffi::c_int         ))),
        "evt_mode"           => Some((" %m", parse_quote!( tk::event::TkNotifyMode ))),
        "evt_override"       => Some((" %o", parse_quote!( bool                    ))),
        "evt_place"          => Some((" %p", parse_quote!( tk::event::TkPlaceOn    ))),
        "evt_state"          => Some((" %s", parse_quote!( String                  ))),
        "evt_time"           => Some((" %t", parse_quote!( std::ffi::c_int         ))),
        "evt_width"          => Some((" %w", parse_quote!( std::ffi::c_int         ))),
        "evt_x"              => Some((" %x", parse_quote!( std::ffi::c_int         ))),
        "evt_y"              => Some((" %y", parse_quote!( std::ffi::c_int         ))),
        "evt_unicode"        => Some((" %A", parse_quote!( char                    ))),
        "evt_borderwidth"    => Some((" %B", parse_quote!( std::ffi::c_int         ))),
        "evt_delta"          => Some((" %D", parse_quote!( std::ffi::c_int         ))),
        "evt_sendevent"      => Some((" %E", parse_quote!( bool                    ))),
        "evt_keysym"         => Some((" %K", parse_quote!( char                    ))),
        "evt_matches"        => Some((" %M", parse_quote!( std::ffi::c_int         ))),
        "evt_keysym_decimal" => Some((" %N", parse_quote!( std::ffi::c_int         ))),
        "evt_property"       => Some((" %P", parse_quote!( String                  ))),
        "evt_root"           => Some((" %R", parse_quote!( std::ffi::c_int         ))),
        "evt_subwindow"      => Some((" %S", parse_quote!( std::ffi::c_int         ))),
        "evt_type"           => Some((" %T", parse_quote!( tk::event::TkEventType  ))),
        "evt_window_path"    => Some((" %W", parse_quote!( String                  ))),
        "evt_rootx"          => Some((" %X", parse_quote!( std::ffi::c_int         ))),
        "evt_rooty"          => Some((" %Y", parse_quote!( std::ffi::c_int         ))),
        _ => None,
    }
}

fn id_of_pat( pat: &Pat ) -> Option<Ident> {
    match pat {
        Pat::Ident( pat_ident ) => Some( pat_ident.ident.clone() ),
        _ => None,
    }
}

const BAD_INPUT: &'static str = "tclosure!()'s closure inputs should be `id` or `id:type`.";

const MIX_UP: &'static str = "Not allowed to mix up event-arguments and non-event-arguments.";

/// Helps to register rust closures as Tcl commands or Tk event callbacks.
///
/// # Syntax
///
/// `tclosure!( $interp:expr, cmd:$cmd:expr, args:$args:expr, bind:($($bind: bind_syn::Bind),*), $closure:expr )`
///
/// # Input parameters
///
/// 1. interp, the Tcl interpreter instance.
///
/// 2. cmd, the name of the command being registered in Tcl. Optional. Note: be careful to keep multple closures from sharing
/// the same `cmd` name.
///
/// 3. args, the arguments provided in Tcl on executing the command. Optional.
/// You can provide `args` if you don't want this macro to interpret `evt_*` closure arguments as Tk event callback arguments.
///
/// 4. bind list, for cloning data into the closure, which is similar inside `bind::bind!()`. Optional.
///
/// 5. closure, the closure defined in Rust. Its capture must be `move` and `move` keywords could be omitted.
///   Note: an attribute `#[default(value)]` on a parameter will assign a default `value` for this parameter.
///
/// Without `args` given, the closure's input arguments which starts with `evt_` will be treated as predefined Tk event callback
/// arguments. The annotated types can be omitted or substituted by different types.
///
/// * evt_serial: c_int
///
/// The number of the last client request processed by the server (the serial field from the event). Valid for all event types.
///
/// * evt_above: c_int
///
/// The above field from the event, formatted as a hexadecimal number. Valid only for Configure events. Indicates the sibling
/// window immediately below the receiving window in the stacking order, or 0 if the receiving window is at the bottom.
///
/// * evt_button: tk::event::ButtonNo
///
/// The number of the button that was pressed or released. Valid only for ButtonPress and ButtonRelease events.
///
/// * evt_count: c_int
///
/// The count field from the event. Valid only for Expose events. Indicates that there are count pending Expose events which
/// have not yet been delivered to the window.
///
/// * evt_detail: Obj
///
/// The detail or user_data field from the event.
///
/// * evt_focus: bool
///
/// The focus field from the event (false or true). Valid only for Enter and Leave events, true if the receiving window is the
/// focus window or a descendant of the focus window, false otherwise.
///
/// * evt_height: c_int
///
/// The height field from the event. Valid for the Configure, ConfigureRequest, Create, ResizeRequest, and Expose events.
/// Indicates the new or requested height of the window.
///
/// * evt_window: c_int
///
/// The window field from the event, represented as a hexadecimal integer. Valid for all event types.
///
/// * evt_keycode: c_int
///
/// The keycode field from the event. Valid only for KeyPress and KeyRelease events.
///
/// * evt_mode: tk::event::TkNotifyMode
///
/// The mode field from the event. Valid only for Enter, FocusIn, FocusOut, and Leave events.
///
/// * evt_override: bool
///
/// The override_redirect field from the event. Valid only for Map, Reparent, and Configure events.
///
/// * evt_place: tk::event::TkPlaceOn
///
/// The place field from the event, substituted as one of the strings PlaceOnTop or PlaceOnBottom. Valid only for Circulate and
/// CirculateRequest events.
///
/// * evt_state: String
///
/// The state field from the event. For ButtonPress, ButtonRelease, Enter, KeyPress, KeyRelease, Leave, and Motion events, a
/// decimal string is substituted. For Visibility, one of the strings VisibilityUnobscured, VisibilityPartiallyObscured, and
/// VisibilityFullyObscured is substituted. For Property events, substituted with either the string NewValue (indicating that
/// the property has been created or modified) or Delete (indicating that the property has been removed).
///
/// * evt_time: c_int
///
/// The time field from the event. This is the X server timestamp (typically the time since the last server reset) in
/// milliseconds, when the event occurred. Valid for most events.
///
/// * evt_width: c_int
///
/// The width field from the event. Indicates the new or requested width of the window. Valid only for Configure,
/// ConfigureRequest, Create, ResizeRequest, and Expose events.
///
/// * evt_x: c_int, evt_y: c_int
///
/// The x and y fields from the event. For ButtonPress, ButtonRelease, Motion, KeyPress, KeyRelease, and MouseWheel events,
/// evt_x and evt_y indicate the position of the mouse pointer relative to the receiving window. For key events on the Macintosh
/// these are the coordinates of the mouse at the moment when an X11 KeyEvent is sent to Tk, which could be slightly later than
/// the time of the physical press or release. For Enter and Leave events, the position where the mouse pointer crossed the
/// window, relative to the receiving window. For Configure and Create requests, the x and y coordinates of the window relative
/// to its parent window.
///
/// * evt_unicode: char
///
/// Substitutes the UNICODE character corresponding to the event, or the empty string if the event does not correspond to a
/// UNICODE character (e.g. the shift key was pressed). On X11, XmbLookupString (or XLookupString when input method support is
/// turned off) does all the work of translating from the event to a UNICODE character. On X11, valid only for KeyPress event.
/// On Windows and macOS/aqua, valid only for KeyPress and KeyRelease events.
///
/// * evt_borderwidth: c_int
///
/// The border_width field from the event. Valid only for Configure, ConfigureRequest, and Create events.
///
/// * evt_delta: c_int
///
/// This reports the delta value of a MouseWheel event. The delta value represents the rotation units the mouse wheel has been
/// moved. The sign of the value represents the direction the mouse wheel was scrolled.
///
/// * evt_sendevent: bool
///
/// The send_event field from the event. Valid for all event types, false indicates that this is a “normal” event, true
/// indicates that it is a “synthetic” event generated by SendEvent.
///
/// * evt_keysym: char
///
/// The keysym corresponding to the event, as a textual `char`. Valid only for KeyPress and KeyRelease events.
///
/// * evt_matches: c_int
///
/// The number of script-based binding patterns matched so far for the event. Valid for all event types.
///
/// * evt_keysym_decimal: c_int
///
/// The keysym corresponding to the event, substituted as a number. Valid only for KeyPress and KeyRelease events.
///
/// * evt_property: String
///
/// The name of the property being updated or deleted (which may be converted to an XAtom using winfo atom.) Valid only for
/// Property events.
///
/// * evt_root: c_int
///
/// The root window identifier from the event. Valid only for events containing a root field.
///
/// * evt_subwindow: c_int
///
/// The subwindow window identifier from the event, formatted as a hexadecimal number. Valid only for events containing a
/// subwindow field.
///
/// * evt_type: tk::event::TkEventType
///
/// The type field from the event. Valid for all event types.
///
/// * evt_window_path: String
///
/// The path name of the window to which the event was reported (the window field from the event). Valid for all event types.
///
/// * evt_rootx: c_int, evt_rooty: c_int
///
/// The x_root and y_root fields from the event. If a virtual-root window manager is being used then the substituted values are
/// the corresponding x-coordinate and y-coordinate in the virtual root. Valid only for ButtonPress, ButtonRelease, Enter,
/// KeyPress, KeyRelease, Leave and Motion events. Same meaning as evt_x and evt_y, except relative to the (virtual) root
/// window.
///
/// # Output
///
/// Returns a `String` of the command name.
///
/// # Example, Tk Event callback
///
/// ```rust,no_run
/// widget.bind( button_press_2(),
///     tclosure!( tk, |evt_x, evt_y| -> TkResult<()> {
///         Ok( tk.popup( menu, evt_x, evt_y, None )? )
///     })
/// )?;
/// ```
///
/// # Example, Poll
///
/// ```rust,no_run
/// tk.run( tclosure!( tk, cmd:"poll" || {
///     {/* poll and do lots of work, omitted */}
///     tk.after( 100, ("poll",) )?;
///     Ok(())
/// }))?;
/// ```
#[proc_macro]
pub fn tclosure( input: TokenStream ) -> TokenStream {
    let TclosureInput{ tk, cmd, args, bind, mut closure } = parse_macro_input!( input as TclosureInput );
    let bind = bind.unwrap_or_default();
    let bind = bind.iter();

    let output = match &closure.output {
        ReturnType::Default => parse_quote!( Result<(), tk::error::InterpError> ),
        ReturnType::Type( _, ty ) => ty.clone(),
    };

    let attrs = closure.attrs;
    let argc = closure.inputs.len();

    let is_variadic = closure.inputs
        .last_mut()
        .map( |pat| if let Pat::Rest(_) = pat {true} else {false} )
        .unwrap_or( false );
    if is_variadic {
        closure.inputs.pop();
    }
    let non_variadic_argc = if is_variadic { argc-1 } else { argc };

    let args = args.unwrap_or_else( || {
        let mut args = String::new();
        for pat in closure.inputs.iter_mut() {
            match pat {
                Pat::Type( pat_ty ) => match id_of_pat( &*pat_ty.pat ) {
                    Some( id ) => match tk_event_detail_name_and_type( &id ) {
                        Some((name, _)) => args.push_str( name ),
                        None => if !args.is_empty() { panic!( "{MIX_UP}" ); }
                    }
                    None => panic!( "{BAD_INPUT}" ),
                }
                Pat::Ident( pat_ident ) => {
                    let ident = pat_ident.ident.clone();
                    match tk_event_detail_name_and_type( &ident ) {
                        Some((name, ty)) => {
                            args.push_str( name );
                            *pat = Pat::Type( PatType {
                                attrs       : vec![],
                                pat         : Box::new( Pat::Ident( PatIdent{
                                    attrs       : vec![],
                                    by_ref      : None,
                                    mutability  : None,
                                    ident       ,
                                    subpat      : None, })),
                                colon_token : Colon( Span::call_site() ),
                                ty          : Box::new( ty ),
                            });
                        }
                        None => if !args.is_empty() { panic!( "{MIX_UP}" ); }
                    }
                }
                _ => panic!( "{BAD_INPUT}" ),
            }
        }
        parse_quote!( #args )
    });

    let body = &*closure.body;
    let expr_block: ExprBlock = if let Expr::Block( block ) = body {
        block.clone()
    } else {
        parse_quote!{{ #body }}
    };

    let existing_stmts = expr_block.block.stmts;

    let mut body: Block = parse_quote! {{
        let mut __interp = unsafe{ tcl::Interp::from_raw( __tcl_interp ).unwrap() };
        let __origin_objs: &[*mut tcl::reexport_clib::Tcl_Obj] = unsafe{ std::slice::from_raw_parts( __objv.offset(1), (__objc-1) as usize )};

        if __origin_objs.len() != #argc {
            if __origin_objs.len() < #argc || !#is_variadic {
                unsafe {
                    tcl::reexport_clib::Tcl_WrongNumArgs( __tcl_interp, 1, __objv, std::ptr::null() );
                    use tcl::CodeToResult;
                    tcl::CodeToResult::code_to_result( tcl::reexport_clib::TCL_ERROR as std::os::raw::c_int, &__interp )?;
                }
            }
        }

        let mut __objs = __origin_objs[..#non_variadic_argc].to_vec();
        let mut __variadic_args = __origin_objs[#non_variadic_argc..]
            .iter().map( |obj_ptr| unsafe{ Obj::from_raw( *obj_ptr )}).collect::<Vec<Obj>>();
        use std::convert::TryFrom;

        let mut __ref_objs = std::collections::HashMap::<&'static str, *mut tcl::reexport_clib::Tcl_Obj>::new();

        macro_rules! tcl_invalidate_str_rep {
            ($ident:ident) => {
                __ref_objs.get( stringify!( $ident )).map( |tcl_obj| unsafe{
                    tcl::reexport_clib::Tcl_InvalidateStringRep( *tcl_obj );
                });
            };
        }

        macro_rules! tcl_interp { () => { __interp }}
        macro_rules! tcl_va_args { () => { __variadic_args }}
    }};

    body.stmts.reserve( argc * 5 + existing_stmts.len() );
    let mut default_values = Vec::<>::with_capacity( closure.inputs.len() );
    for (nth, arg) in closure.inputs.iter().enumerate() {
        match arg {
            Pat::Type( pat_type ) => {
                let pat = &*pat_type.pat;
                let ty = &*pat_type.ty;
                let default_value =
                    (*pat_type).attrs.iter().find_map( |attr| {
                        if let syn::Meta::List( meta_list ) = &attr.meta {
                            let segments = &meta_list.path.segments;
                            if segments.len() == 1 && segments.first().unwrap().ident == "default" {
                                return Some( meta_list.tokens.clone() );
                            }
                        }
                        None
                    })
                ;
                match default_value {
                    Some( value ) => default_values.push( value ),
                    None => default_values.clear(),
                }
                match &*ty {
                    Type::Reference( type_ref ) => match pat {
                        Pat::Ident( pat_ident ) => {
                            body.stmts.push( parse_quote!( unsafe {
                                let origin_obj = __objs[ #nth ];
                                __objs[ #nth ] = __interp.get( Obj::from_raw( origin_obj ))?.as_ptr();
                            }));
                            let ident = &pat_ident.ident;
                            body.stmts.push( parse_quote!(
                                __ref_objs.entry( stringify!( #ident )).or_insert( __objs[ #nth ]);
                            ));

                            let ty_elem = &*type_ref.elem;
                            body.stmts.push( parse_quote!(
                                let mut __obj = unsafe{ tcl::Obj::from_raw( __objs[ #nth ])};
                            ));
                            body.stmts.push( parse_quote!(
                                let #ident = tcl::Tcl::<#ty_elem>::ptr_from( __obj )?;
                            ));
                            if type_ref.mutability.is_none() {
                                body.stmts.push( parse_quote!(
                                    let #ident: #ty = &*unsafe{ #ident.as_ref() }.deref().borrow();
                                ));
                            } else {
                                body.stmts.push( parse_quote!(
                                    let #ident: #ty = &mut *unsafe{ #ident.as_ref() }.deref().borrow_mut();
                                ));
                            }
                        },
                        _ => panic!("#[tcl_proc] argument should be in the form of `ident: Type`"),
                    },
                    _ => {
                        body.stmts.push( parse_quote!(
                            let mut __obj = unsafe{ tcl::Obj::from_raw( __objs[ #nth ])};
                        ));
                        body.stmts.push( parse_quote!(
                            let #pat = <#ty>::try_from( __obj )?;
                        ));
                    },
                }
            },
            _ => panic!("#[tcl_proc] argument should be in the form of `ident: Type`"),
        }
    }

    body.stmts.extend( existing_stmts );

    let uuid = make_ident( &format!( "__tcl_closure_wrapper_{}", Uuid::new_v4().simple() ));
    let cmd = cmd.unwrap_or_else( || parse_quote!("") );

    let proc_definition: Vec<Stmt> = if default_values.is_empty() {
        parse_quote!{
             (#tk).def_proc_with_client_data( cmd.as_str(), #uuid, client_data, Some( __deleter ));
        }
    } else {
        let name: Expr = parse_quote!{ &format!( "__tcl_fn_{}", stringify!( #uuid ))};

        let optional_argc = default_values.len();
        let required_argc = argc - optional_argc;

        let param_list = format!( "{}{}",
            (0..required_argc   ).fold( String::new(), |acc,n|
                format!( "{acc} arg{n}" )),
            (required_argc..argc).fold( String::new(), |acc,n|
                format!( "{acc} {{ arg{n} {} }}", default_values[ n-required_argc ])),
        );
        let params = (0..argc).fold( String::new(), |acc,n| format!( "{acc} $arg{n}" ));

        parse_quote!{
            (#tk).def_proc_with_client_data( #name, #uuid, client_data, Some( __deleter ));
            (#tk).run(
                format!( "proc {} {{ {} }} {{ {} {} }}", cmd, #param_list, #name, #params )
            ).ok();
        }
    };

    let (_, random_value) = Uuid::new_v4().as_u64_pair();

    let expanded = quote!{{
        #(#bind)*

        extern "C" fn #uuid( __client_data: tcl::reexport_clib::ClientData, __tcl_interp: *mut tcl::reexport_clib::Tcl_Interp, __objc: std::os::raw::c_int, __objv: *const *mut tcl::reexport_clib::Tcl_Obj ) -> std::os::raw::c_int {
            let closure: &mut Box<dyn Fn( tcl::reexport_clib::ClientData, *mut tcl::reexport_clib::Tcl_Interp, std::os::raw::c_int, *const *mut tcl::reexport_clib::Tcl_Obj )->#output> = unsafe{ &mut *( __client_data as *mut _ )};
            match closure( std::ptr::null_mut(), __tcl_interp, __objc, __objv ) {
                Ok( value ) => {
                    unsafe{ tcl::reexport_clib::Tcl_SetObjResult( __tcl_interp, Obj::from( value ).into_raw() )};
                    tcl::reexport_clib::TCL_OK as std::os::raw::c_int
                },
                Err( _ ) => tcl::reexport_clib::TCL_ERROR as std::os::raw::c_int,
            }
        }

        extern "C" fn __deleter( __client_data: tcl::reexport_clib::ClientData ) {
            let _: Box<Box<dyn Fn( tcl::reexport_clib::ClientData, *mut tcl::reexport_clib::Tcl_Interp, std::os::raw::c_int, *const *mut tcl::reexport_clib::Tcl_Obj )->#output>> = unsafe{ Box::from_raw( __client_data as *mut _ )};
        }

        fn __box_new_static_closure<F>( f: F ) -> Box<F>
            where F: 'static + Fn( tcl::reexport_clib::ClientData, *mut tcl::reexport_clib::Tcl_Interp, std::os::raw::c_int, *const *mut tcl::reexport_clib::Tcl_Obj ) -> #output
        {
            Box::new( f )
        }

        let closure: Box<Box<dyn Fn( tcl::reexport_clib::ClientData, *mut tcl::reexport_clib::Tcl_Interp, std::os::raw::c_int, *const *mut tcl::reexport_clib::Tcl_Obj )->#output>> = Box::new( __box_new_static_closure(
            #(#attrs)*
            #[allow( unused_macros )]
            move |__client_data: tcl::reexport_clib::ClientData, __tcl_interp: *mut tcl::reexport_clib::Tcl_Interp, __objc: std::os::raw::c_int, __objv: *const *mut tcl::reexport_clib::Tcl_Obj| -> #output #body
        ));

        let client_data = Box::into_raw( closure ) as tcl::reexport_clib::ClientData;

        let address_as_name = format!( "__tkbind_closure_{:?}", #random_value.wrapping_add( client_data as u64 ));
        let cmd = if (#cmd).is_empty() {
            address_as_name
        } else {
            String::from( #cmd )
        };

        unsafe{ #(#proc_definition)* }

        format!( "{} {}", cmd, #args )
    }};

    expanded.into()
}

/// Derives `std::from::TryFrom<tcl::Obj>`, based on `serde::Deserialize`.
#[proc_macro_derive( TryFromDe )]
pub fn derive_try_from_de( input: TokenStream ) -> TokenStream {
    let derive_input = parse_macro_input!( input as DeriveInput );
    let name = derive_input.ident;
    let generics = add_trait_bounds( derive_input.generics );
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics std::convert::TryFrom<Obj> for #name #ty_generics #where_clause {
            type Error = tcl::error::DeError;

            fn try_from( obj: Obj ) -> Result<Self, Self::Error> {
                from_obj( obj )
            }
        }
    };
    expanded.into()
}

fn add_trait_bounds( mut generics: Generics ) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type( ref mut type_param ) = *param {
            let bound = syn::parse_str( "serde::Deserialize" ).unwrap();
            type_param.bounds.push( bound );
        }
    }
    generics
}

fn make_ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
