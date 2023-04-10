macro_rules! def_tuple_notation {
    ($str:expr => $ty:ident $trait:ident $valid_opt:ident) => {
        pub struct $ty<Tup>( Tup );

        impl<Tup> Convert for $ty<Tup> {
            type Output = Tup;
            fn convert( self ) -> Self::Output { self.0 }
        }

        impl<Opts,Widgs> $trait for PathOptsWidgets<Opts,Widgs>
            where Widgs: ConvertTuple
                , Opts : IntoHomoTuple<opt::$valid_opt>
                       + IntoHomoTuple<OptPair>
                , <Widgs as ConvertTuple>::Output
                       : PushFront<heredom::Node<(&'static str,&'static str),<Opts as IntoHomoTuple<OptPair>>::Output>>
        {
            type Output = $ty<
                <
                    <Widgs as ConvertTuple>::Output as
                    PushFront<
                        heredom::Node<
                            (&'static str,&'static str),
                            <Opts as IntoHomoTuple<OptPair>>::Output
                        >
                    >
                >
                ::Output
            >;

            fn output( self ) -> Self::Output {
                let cmd  = $str;
                let path = self.path;
                let opts = <Opts as IntoHomoTuple<OptPair>>::into_homo_tuple( self.opts );
                $ty( self.widgets.convert_tuple().push_front( heredom::Node( (cmd,path), opts )))
            }
        }

        def_hyphen_notation!( $ty );
    };
}

macro_rules! def_functions {
    ($($function:ident $trait:ident;)*) => {$(
        pub fn $function<Input: $trait>( input: Input ) -> <Input as $trait>::Output {
            <Input as $trait>::output( input )
        }

        pub trait $trait {
            type Output;
            fn output( self ) -> Self::Output;
        }
    )*}
}
