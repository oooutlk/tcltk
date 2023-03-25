extern crate proc_macro;

use self::proc_macro::TokenStream;

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
    ReturnType,
    Stmt,
    Token,
    Type,
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    parse_quote,
    punctuated::Punctuated,
    token::Comma,
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

                __tcl_inner_proc( __client_data, __tcl_interp, __objc, __objv )
                .map( |value| unsafe {
                    tcl::reexport_clib::Tcl_SetObjResult( __tcl_interp, Obj::from( value ).into_raw() );
                }).ok();

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
    let cmd = cmd.unwrap_or_else( || parse_quote!{ stringify!( #ident )});
    let args = args.unwrap_or_else( || parse_quote!( "" ));

    let expanded = quote! {{
        #[tcl::proc]
        #[allow( unused_macros )]
        #item_fn

        let cmd = #cmd;
        unsafe{ (#interp).def_proc( cmd, #ident ); }
        format!( "{} {}", cmd, #args )
    }};

    expanded.into()
}

/// Helps to register rust functions as Tcl commands
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

/// Helps to register rust closures as Tcl commands
///
/// # Syntax
///
/// `tclosure!( interp, cmd, args, closure )`
///
/// # Input parameters
///
/// 1. interp, the Tcl interpreter instance.
///
/// 2. cmd, the name of the command being registered in Tcl. Optional.
///
/// 3. args, the arguments provided in Tcl on executing the command. Optional.
///
/// 4. closure, the closure defined in Rust.
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
/// let offset = 0;
/// let interpreter = Interpreter::new()?;
///
/// let cmd = tclosure!( &interpreter, /*cmd: "mul", args: "",*/
///     move |a: i32, b: i32| -> TclResult<i32> { Ok( a * b + offset )}
/// );
///
/// let a = 3;
/// let b = 7;
/// let c = interpreter.eval(( "eval", cmd, a, b ))?;
/// assert_eq!( c.as_i32(), 21 );
/// ```
#[proc_macro]
pub fn tclosure( input: TokenStream ) -> TokenStream {
    struct TclosureInput {
        interp  : Expr,
        cmd     : Option<Expr>,
        args    : Option<Expr>,
        closure : ExprClosure,
    }

    impl Parse for TclosureInput {
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

            let closure = input.parse::<ExprClosure>()?;
            Ok( TclosureInput{ interp, cmd, args, closure })
        }
    }

    let TclosureInput{ interp, cmd, args, closure } = parse_macro_input!( input as TclosureInput );
    callback_closure( interp, cmd, args, closure )
}

fn callback_closure( interp: Expr, cmd: Option<Expr>, args: Option<Expr>, mut closure: ExprClosure ) -> TokenStream {
    let ident = make_ident( &format!( "__tcl_closure_wrapper_{}", Uuid::new_v4().to_simple() ));
    let cmd = cmd.unwrap_or_else( || parse_quote! {
        &format!( "__tcl_fn_{}", stringify!( #ident ))
    });
    let args = args.unwrap_or_else( || parse_quote!( "" ));

    let output = match &closure.output {
        ReturnType::Default => parse_quote!( Result<(), tcl::error::InterpError> ),
        ReturnType::Type( _, ty ) => ty.clone(),
    };

    let attrs = closure.attrs;
    let capture = &closure.capture;
    let argc = closure.inputs.len();

    let is_variadic = closure.inputs
        .last_mut()
        .map( |pat| if let Pat::Rest(_) = pat {true} else {false} )
        .unwrap_or( false );
    if is_variadic {
        closure.inputs.pop();
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
    for (nth, arg) in closure.inputs.iter().enumerate() {
        match arg {
            Pat::Type( pat_type ) => {
                let pat = &*pat_type.pat;
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

    let expanded = quote! {{
        extern "C" fn #ident( __client_data: tcl::reexport_clib::ClientData, __tcl_interp: *mut tcl::reexport_clib::Tcl_Interp, __objc: std::os::raw::c_int, __objv: *const *mut tcl::reexport_clib::Tcl_Obj ) -> std::os::raw::c_int {
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
            #capture |__client_data: tcl::reexport_clib::ClientData, __tcl_interp: *mut tcl::reexport_clib::Tcl_Interp, __objc: std::os::raw::c_int, __objv: *const *mut tcl::reexport_clib::Tcl_Obj| -> #output #body
        ));
        let client_data = Box::into_raw( closure ) as tcl::reexport_clib::ClientData;

        let cmd = #cmd;
        unsafe{ (#interp).def_proc_with_client_data( cmd, #ident, client_data, Some( __deleter )); }
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
