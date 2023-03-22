fn main() {
    if std::env::var( "CARGO_FEATURE_LIBTK" ).is_ok() {
        inwelling::to( "clib" );
    }
}
