use std::error::Error;

trait IntoBoxedError: Sized {
    type E: ?Sized;

    fn into_boxed(self) -> Box<Self::E>;
}

impl<Err: Error + 'static> IntoBoxedError for Err {
    type E = dyn Error;
    fn into_boxed(self) -> Box<Self::E> {
        Box::new(self) as Box<_>
    }
}

trait IntoBoxedResult<A>: Sized {
    type E: ?Sized;

    fn boxed_error(self) -> Result<A, Box<Self::E>>;
}

impl<A, Err: Error + 'static> IntoBoxedResult<A> for Result<A, Err> {
    type E = dyn Error;

    fn boxed_error(self) -> Result<A, Box<Self::E>> {
        self.map_err(Box::new).map_err(|e| e as _)
    }
}

fn main() {
    println!("Hello, world!");
}
