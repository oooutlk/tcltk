use crate::{
    TkInstance,
    Widget,
};

pub trait UpcastFrom<Inst:TkInstance>
    where Self: Sized
{
    fn upcast_from( upcastable_widget: UpcastableWidget<Inst> ) -> Option<Self>;
}

pub struct CreatedWidgets<Inst:TkInstance> {
    pub(crate) widgets   : Vec<UpcastableWidget<Inst>>,
    pub(crate) base_path : String,
}

impl<Inst:TkInstance> CreatedWidgets<Inst> {
    pub(crate) fn new( path: &str ) -> Self {
        CreatedWidgets{ widgets: vec![], base_path: path.to_owned() }
    }

    pub fn preorder_iter<'a,'s>( &'s self ) -> impl Iterator<Item=UpcastableWidget<Inst>> + 'a
        where 's: 'a
    {
        self.widgets.iter().copied()
    }

    pub fn query_upcastable( &self, relative_path: &str ) -> Option<UpcastableWidget<Inst>> {
        let path = if self.base_path == "." {
            format!( ".{}", relative_path )
        } else {
            format!( "{}.{}", self.base_path, relative_path )
        };

        self.widgets
            .iter()
            .find( |&w| w.widget.path == path )
            .copied()
    }

    pub fn query<Widg:UpcastFrom<Inst>>( &self, relative_path: &str ) -> Option<Widg> {
        self.query_upcastable( relative_path )
            .and_then( |upcastable_widget| <Widg as UpcastFrom<Inst>>::upcast_from( upcastable_widget ))
    }
}

#[derive( Copy, Clone )]
pub struct UpcastableWidget<Inst:TkInstance> {
    pub(crate) widget : Widget<Inst>,
    pub(crate) name   : &'static str,
}

impl<Inst:TkInstance> UpcastableWidget<Inst> {
    pub fn upcast<Widg>( self ) -> Option<Widg>
        where Widg : UpcastFrom<Inst>
    {
        <Widg as UpcastFrom<Inst>>::upcast_from( self )
    }
}
