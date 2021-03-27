use std::any::Any;

pub mod connect;

pub trait Message<'a>: Any + Send {
    fn as_any(&'a self) -> &'a dyn Any;
    fn as_mut_any(&'a mut self) -> &'a mut dyn Any;
}

impl<'a, T: Any + Send> Message<'a> for T {
    fn as_any(&'a self) -> &'a dyn Any {
        return self;
    }
    fn as_mut_any(&'a mut self) -> &'a mut dyn Any {
        return self;
    }
}

pub type Route = u16;
pub type RouteVersion = u16;

pub type RouteWithVersion = (Route, RouteVersion);

pub struct Packet<'a> {
    pub route: RouteWithVersion,
    pub message: &'a dyn Message<'a>,
}

unsafe impl<'a> Send for Packet<'a>{}