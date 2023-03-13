use std::{
    os::raw::c_int,
    ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive},
};

pub trait TkDefaultStart {
    fn default_start() -> Self;
}

pub trait TkDefaultEnd {
    fn default_end() -> Self;
}

impl TkDefaultStart for c_int { fn default_start() -> Self { 0 }}

pub struct TkRange<T> {
    pub start : T,
    pub end   : T,
}

impl<T:TkDefaultEnd> From<RangeFrom<T>> for TkRange<T> { // a..
    fn from( r: RangeFrom<T> ) -> Self { TkRange{ start: r.start, end: T::default_end() }}
}

impl<T:TkDefaultStart+TkDefaultEnd> From<RangeFull> for TkRange<T> { // ..
    fn from( _r: RangeFull ) -> Self { TkRange{ start: T::default_start(), end: T::default_end() }}
}

impl<T:Clone> From<RangeInclusive<T>> for TkRange<T> { // a..=b
    fn from( r: RangeInclusive<T> ) -> Self { TkRange{ start: r.start().clone(), end: r.end().clone() }}
}

impl<T:TkDefaultStart> From<RangeToInclusive<T>> for TkRange<T> { // ..=b
    fn from( r: RangeToInclusive<T> ) -> Self { TkRange{ start: T::default_start(), end: r.end }}
}
