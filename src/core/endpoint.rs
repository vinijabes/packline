pub trait Endpoint {
    type Request;
    type Response;
    type Error;

    fn call(&mut self, req: Self::Request) -> Self::Response;
}

pub type Body = [u8];

impl<'a, T> Endpoint for &'a mut T
where
    T: Endpoint + 'a,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = T::Error;

    fn call(&mut self, req: Self::Request) -> Self::Response {
        (**self).call(req)
    }
}

impl<'a, T> Endpoint for Box<T>
where
    T: Endpoint + ?Sized,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = T::Error;

    fn call(&mut self, req: Self::Request) -> Self::Response {
        (**self).call(req)
    }
}
