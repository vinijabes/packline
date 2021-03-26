use crate::messages::Message;

pub enum RequestPayload<'a> {
    Packet(Box<dyn Message<'a>>),
    Unknown,
}

pub struct Request<'a> {
    pub route: super::messages::RouteWithVersion,
    pub payload: RequestPayload<'a>,
}

pub trait FromRequest: Sized {
    fn from_request_ref(req: &'static Request) -> Option<&'static Self>;
    //fn from_request_mut(req: &'static mut Request) -> Option<&'static mut Self>;
}

impl<T> FromRequest for T {
    fn from_request_ref(req: &'static Request) -> Option<&'static T> {
        req.payload.as_any().downcast_ref()
    }

    /*    fn from_request_mut(req: &'static mut Request) -> Option<&'static mut T>{
        //req.payload.as_mut_any().downcast_mut()
    }*/
}

// impl Request<'static> {
//     fn to_ref<T>(&self) -> Option<&T>
//         where T: 'static {
//         self.payload.as_any().downcast_ref::<T>()
//     }
//
//     fn to_mut_ref<T>(&mut self) -> Option<&mut T>
//         where T: 'static {
//         self.payload.as_mut_any().downcast_mut::<T>()
//     }
// }
