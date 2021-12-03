pub trait WriteableDisplay {
    fn as_writeable(&self) -> &dyn WriteableDisplay
        where Self : Sized
    {
        self
    }

    fn as_writeable_mut(&mut self) -> &mut dyn WriteableDisplay
        where Self : Sized
    {
        self
    }

    fn listen(&mut self);
}
