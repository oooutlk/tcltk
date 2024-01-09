//! Proc macro for tk.

use proc_macro::TokenStream;

use proc_macro2::Span;

use quote::{ToTokens, quote};

use syn::{
    Block,
    Expr,
    ExprAssign,
    ExprBlock,
    ExprClosure,
    ExprPath,
    Ident,
    Pat,
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
    visit::Visit,
};

use uuid::Uuid;

fn extract_the_only_id_in( expr: &Expr ) -> Option<Ident> {
    struct Extractor {
        id  : Option<Ident>,
        cnt : usize,
    }

    impl<'a> Visit<'a> for Extractor {
        fn visit_ident( &mut self, id: &Ident ) {
            if self.cnt == 0 && self.id.is_none() {
                self.id = Some( id.clone() );
            }
            self.cnt += 1;
        }
    }

    let mut extractor = Extractor{ id: None, cnt: 0 };
    extractor.visit_expr( &expr );
    extractor.id
}

enum ExprOrIdent {
    Expr(  Expr  ),
    Ident( Ident ),
}

fn get_expr_or_id( expr: Expr ) -> ExprOrIdent {
    if let Expr::Path( ExprPath{ attrs, qself, path }) = &expr {
        if attrs.is_empty() && qself.is_none() {
            if path.leading_colon.is_none() && path.segments.len() == 1 {
                let seg = path.segments.first().unwrap();
                if seg.arguments.is_none() {
                    return ExprOrIdent::Ident( seg.ident.clone() );
                }
            }
        }
    }
    ExprOrIdent::Expr( expr )
}

#[derive( Clone )]
enum Bind {
       Id(     Ident              ),
    MutId(     Ident              ),
       IdId(   Ident, Ident       ),
    MutIdId(   Ident, Ident       ),
       IdExpr( Ident,        Expr ),
    MutIdExpr( Ident,        Expr ),
         Expr( Ident,        Expr ),
      MutExpr( Ident,        Expr ),
}

impl Parse for Bind {
    fn parse( input: ParseStream ) -> parse::Result<Self> {
        let immutable = if input.peek( Token![mut] ) {
            input.parse::<Token![mut]>()?;
            false
        } else {
            true
        };

        let expr = input.parse::<Expr>()?;

        if let Expr::Assign( expr_assign ) = &expr {
            let ExprAssign{ attrs:_, left, eq_token, right } = expr_assign.clone();
            let _ = eq_token;
            if let ExprOrIdent::Ident( id ) = get_expr_or_id( *left ) {
                match get_expr_or_id( *right ) {
                    ExprOrIdent::Expr( expr ) =>
                        return Ok( if immutable {
                            Bind::IdExpr(    id, expr )
                        } else {
                            Bind::MutIdExpr( id, expr )
                        }),
                    ExprOrIdent::Ident( id0 ) =>
                        return Ok( if immutable {
                            Bind::IdId(      id, id0 )
                        } else {
                            Bind::MutIdId(   id, id0 )
                        }),
                }
            }
        } else {
            match get_expr_or_id( expr ) {
                ExprOrIdent::Expr( expr ) =>
                    match extract_the_only_id_in( &expr ) {
                        Some( id ) =>
                            return Ok( if immutable {
                                Bind::Expr(    id, expr )
                            } else {
                                Bind::MutExpr( id, expr )
                            }),
                        None => (),
                    }
                ExprOrIdent::Ident( id ) =>
                    return Ok( if immutable {
                        Bind::Id(    id )
                    } else {
                        Bind::MutId( id )
                    }),
            }
        }

        panic!( "Invalid input for `tkbind!()`: {input:?}" );
    }
}

impl ToTokens for Bind {
    fn to_tokens( &self, tokens: &mut proc_macro2::TokenStream ) {
        tokens.extend( match self {
            Bind::Id(         id           ) => quote!{ let     #id = #id  .clone(); },
            Bind::MutId(      id           ) => quote!{ let mut #id = #id  .clone(); },
            Bind::IdId(       id, id0      ) => quote!{ let     #id = #id0 .clone(); },
            Bind::MutIdId(    id, id0      ) => quote!{ let mut #id = #id0 .clone(); },
            Bind::IdExpr(     id,     expr ) => quote!{ let     #id = #expr        ; },
            Bind::MutIdExpr(  id,     expr ) => quote!{ let mut #id = #expr        ; },
            Bind::Expr(       id,     expr ) => quote!{ let     #id = #expr        ; },
            Bind::MutExpr(    id,     expr ) => quote!{ let mut #id = #expr        ; },
        });
    }
}

struct TkbindInput {
    tk      : Expr,
    cmd     : Option<Expr>,
    binds   : Option<Punctuated<Bind,Token![,]>>,
    closure : ExprClosure,
}

impl Parse for TkbindInput {
    fn parse( input: ParseStream ) -> parse::Result<Self> {
        let tk = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;

        let cmd;
        if input.peek( token::Paren ) {
            cmd = None;
        } else {
            cmd = Some( input.parse::<Expr>()? );
            input.parse::<Token![,]>()?;
        }

        let binds = if input.peek( token::Paren ) {
            let content;
            parenthesized!( content in input );
            Some( Punctuated::parse_terminated( &content )? )
        } else {
            None
        };

        let closure = input.parse::<ExprClosure>()?;

        Ok( TkbindInput{ tk, cmd, binds, closure })
    }
}

/// Helps to register rust closures as Tk commands as event callbacks.
#[proc_macro]
pub fn tkbind( input: TokenStream ) -> TokenStream {
    let TkbindInput{ tk, cmd, binds, mut closure } = parse_macro_input!( input as TkbindInput );
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

        let address_as_name = format!( "__tclosure_at_{:?}", client_data );
        let cmd = if (#cmd).is_empty() {
            address_as_name
        } else {
            String::from( #cmd )
        };

        unsafe{ #(#proc_definition)* }

        cmd
    }};

    expanded.into()
}

fn make_ident( sym: &str ) -> Ident {
    Ident::new( sym, Span::call_site() )
}
