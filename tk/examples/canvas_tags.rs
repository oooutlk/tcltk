// cargo run --example canvas_tags

use tk::*;
use tk::canvas::*;
use tk::cmd::*;

fn main() -> TkResult<()> {
    let tk = make_tk!()?;
    let root = tk.root();
    let canvas = root.add_canvas(())?.pack(())?;
    let _tag1 = canvas.create_line( &[ (10.0,10.0), (20.0,20.0) ], -tags("firstline drawing") )?;
    let tag2 = canvas.create_rectangle( 30.0, 30.0, 40.0, 40.0, -tags("firstline drawing") )?;
    canvas.addtag( "rectangle", SearchSpec::WithTag( tag2.clone().into() ))?;
    canvas.addtag( "polygon", SearchSpec::WithTag( item_tag( "drawing" ).into() ))?;

    let tags = canvas.gettags( tag2.clone() )?;
    for name in &[ "drawing", "rectangle", "polygon" ] {
        assert!( tags.iter().find( |&tag| tag.0.as_str() == *name ).is_some() );
    }

    canvas.dtag( tag2.clone(), Some( ItemTag( "polygon".to_owned() )))?;

    let tags = canvas.gettags( tag2.clone() )?;
    for name in &[ "drawing", "rectangle" ] {
        assert!( tags.iter().find( |&tag| tag.0.as_str() == *name ).is_some() );
    }
    assert!( tags.iter().find( |&tag| tag.0.as_str() == "polygon" ).is_none() );

    let items = canvas.find( SearchSpec::WithTag( item_tag( "drawing" ).into() ))?;

    assert_eq!(
        items.get_elements()?.map( |item| item.get_string() ).collect::<Vec<_>>(),
        vec![ "1".to_owned(), "2".to_owned() ]);
 
    Ok( main_loop() )
}
