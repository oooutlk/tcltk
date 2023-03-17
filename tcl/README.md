This project provides safe and easy to use API bindings to Tcl programming language.

# Features

1. Convert between Rust values and Tcl objects.

2. Serde data format of Tcl objects.

3. Run Tcl scripts.

4. Register Rust functions/closures as tcl commands.

# Quickstart

## `std::Convert` between Rust values and Tcl objects.

```rust
use std::convert::TryFrom;
use tcl::*;

let obj = Obj::from( 0 );
assert_eq!( obj.to_string(), "0" );
assert_eq!( i32::try_from( obj )?, 0 );

let obj = Obj::from( 1 );
assert_eq!( obj.as_i32(), 1 );
assert_eq!( obj.as_f64(), 1.0 );
assert_eq!( obj.as_bool(), true );

let obj = Obj::from(( false, 42, "answer".to_owned() ));
assert_eq!( obj.to_string(), "0 42 answer" );
assert_eq!( <(bool,i32,String)>::try_from( obj )?,
    (false, 42, "answer".to_owned() )
);

let v = vec![ "alpha".to_owned(), "beta".to_owned(), "gamma".to_owned() ];
let obj: Obj = v.clone().into();
assert_eq!( obj.to_string(), "alpha beta gamma" );
assert_eq!( Vec::<String>::try_from( obj )?, v );

use std::collections::HashMap;

let mut map = HashMap::new();
map.insert( "alpha".to_owned(), 1 );
map.insert( "beta" .to_owned(), 2 );
map.insert( "gamma".to_owned(), 3 );

let obj: Obj = map.clone().into();
assert_eq!( HashMap::<String, i32>::try_from( obj )?, map );

# Ok::<(),TclError>(())
```

## User-defined types `deserialize`d / `try_from` Tcl objects.

```rust
use tcl::*;

#[derive( Clone, PartialEq, Debug, serde::Deserialize )]
#[derive( TryFromDe )]
struct Struct{ a: i32, b: bool, c: f64 }

let obj = Obj::from( "a 1 b false c 3.14" );
let v: Struct = from_obj( obj.clone() )?;
assert_eq!( v, Struct{ a: 1, b: false, c: 3.14 });

let v: Struct = obj.clone().try_into()?;
assert_eq!( v, Struct{ a: 1, b: false, c: 3.14 });

# Ok::<(),TclError>(())
```

## Use `Tcl<T>` to store Rust values in Tcl `Obj`s, an vice-vesa.

```rust
use std::convert::TryFrom;
use tcl::*;

let obj = Tcl::new_obj( vec![ 1, 1, 2, 3, 5, 8 ]);
let tcl_obj = Tcl::<Vec<i32>>::try_from( obj )?;
assert_eq!( tcl_obj.into_inner(), vec![ 1, 1, 2, 3, 5, 8 ]);

# Ok::<(),TclError>(())
```

## Run Tcl scripts

```rust
use tcl::*;

let interpreter = Interpreter::new()?;
let a = 3;
let b = 7;
let c = interpreter.eval(( "expr", a, "*", b ))?;
assert_eq!( a*b, c.as_i32() );

# Ok::<(),TclError>(())
```

## Register Rust functions as tcl commands, the unsafe way

```rust
use tcl::*;

#[proc] fn mul( a: i32, b: i32 ) -> TclResult<i32> { Ok( a * b )}

let interpreter = Interpreter::new()?;
unsafe { // it's safe for `#[proc] fn`.
    interpreter.def_proc( "mul", mul );
}
let c = interpreter.eval( "mul 3 7" )?;
assert_eq!( c.as_i32(), 21 );

# Ok::<(),TclError>(())
```

## Register Rust functions as tcl commands, the safe way

```rust
use tcl::*;

let interpreter = Interpreter::new()?;

let cmd = tclfn!( &interpreter, /*cmd: "mul", args: "",*/
    fn mul( a: i32, b: i32 ) -> TclResult<i32> { Ok( a * b )}
);

let c = interpreter.eval( "mul 3 7" )?;
assert_eq!( c.as_i32(), 21 );

# Ok::<(),TclError>(())
```

## Register Rust closures as tcl commands

```rust
use tcl::*;

let offset = 0;
let interpreter = Interpreter::new()?;

let cmd = tclosure!( &interpreter, /*cmd: "mul", args: "",*/
    move |a: i32, b: i32| -> TclResult<i32> { Ok( a * b + offset )}
);

let a = 3;
let b = 7;
let c = interpreter.eval(( "eval", cmd, a, b ))?;
assert_eq!( c.as_i32(), 21 );

# Ok::<(),TclError>(())
```

# License

Under Apache License 2.0 or MIT License, at your will.
