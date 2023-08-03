#[derive(Clone, Copy, Debug)]
pub enum Context<'a> {
    Base(&'a [UnshiftedEntry]),
    Snoc(&'a Context<'a>, &'a [UnshiftedEntry]),
}

#[derive(Debug)]
pub enum UnshiftedEntry {}

impl Context<'static> {
    pub fn empty() -> Self {
        Context::Base(&[])
    }
}
