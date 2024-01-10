//! Proc macro for tk.

use bind_syn::Bind;

use proc_macro::TokenStream;

use proc_macro2::Span;

use quote::quote;

use syn::{
    Block,
    Expr,
    ExprBlock,
    ExprClosure,
    Ident,
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
    token,
};

use uuid::Uuid;

struct TkbindInput {
    tk      : Expr,
    binds   : Option<Punctuated<Bind,Token![,]>>,
    closure : ExprClosure,
}

impl Parse for TkbindInput {
    fn parse( input: ParseStream ) -> parse::Result<Self> {
        let tk = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;

        let binds = if input.peek( token::Paren ) {
            let content;
            parenthesized!( content in input );
            Some( Punctuated::parse_terminated( &content )? )
        } else {
            None
        };

        let closure = input.parse::<ExprClosure>()?;

        Ok( TkbindInput{ tk, binds, closure })
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

const BAD_INPUT: &'static str = "tkbind!()'s closure inputs should be `id` or `id:type`.";

const MIX_UP: &'static str = "Not allowed to mix up event-arguments and non-event-arguments.";

/// Helps to register rust closures as Tk commands, usually for event callbacks.
///
/// # Syntax
///
/// 1. `tkbind!( tk, closure )`
///
/// 2. `tkbind!( tk, (colon-separated-binding-list), closure )`
///
/// # Input parameters
///
/// 1. tk, the Tk interpreter instance.
///
/// 2. binding list, for cloning data into the closure, which is similar inside `bind::bind!()`. Optional.
///
/// 3. closure, the closure defined in Rust. Its capture must be `move` and `move` keywords could be omitted.
///   Note: an attribute `#[default(value)]` on a parameter will assign a default `value` for this parameter.
///
/// # Output
///
/// Returns a `String` of the command name.
///
/// To access to the command name inside the closure, use `tk.get("__self__")?`.
///
/// # Example, Event callback
///
/// ```rust,no_run
/// widget.bind( button_press_2(),
///     tkbind!( tk, |evt_x, evt_y| -> TkResult<()> {
///         Ok( tk.popup( menu, evt_x, evt_y, None )? )
///     })
/// )?;
/// ```
///
/// # Example, Poll
///
/// ```rust,no_run
/// tk.run( tkbind!( tk, || {
///     {/* poll and do lots of work, omitted */}
///
///     let this_cmd = tk.get("__self__")?;
///     tk.after( 100, (this_cmd,) )?;
///     Ok(())
/// }))?;
/// ```
#[proc_macro]
pub fn tkbind( input: TokenStream ) -> TokenStream {
    let TkbindInput{ tk, binds, mut closure } = parse_macro_input!( input as TkbindInput );
    let binds = binds.unwrap_or_default();
    let binds = binds.iter();

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
                            colon_token : token::Colon( Span::call_site() ),
                            ty          : Box::new( ty ),
                        });
                    }
                    None => if !args.is_empty() { panic!( "{MIX_UP}" ); }
                }
            }
            _ => panic!( "{BAD_INPUT}" ),
        }
    }

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

    let uuid = make_ident( &format!( "__tcl_closure_wrapper_{}", Uuid::new_v4().to_simple() ));

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

    let random_value = fastrand::usize(..);

    let expanded = quote!{{
        #(#binds)*

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

        let cmd = format!( "__tkbind_closure_{:?}", #random_value.wrapping_add( client_data as usize ));

        unsafe{ #(#proc_definition)* }

        (#tk).set( "__self__", cmd.as_str() );

        format!( "{}{}", cmd, #args )
    }};

    expanded.into()
}

fn make_ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
