#[derive(Clone, Copy, Debug)]
pub enum Context<'a> {
    Base(&'a [Entry]),
    Snoc(&'a Context<'a>, &'a [Entry]),
}

#[derive(Debug)]
pub enum Entry {}

impl Context<'static> {
    pub fn empty() -> Self {
        Context::Base(&[])
    }
}
