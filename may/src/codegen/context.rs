use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Context<'a> {
    Base(&'a [UnshiftedEntry<'a>]),
    Snoc(&'a Context<'a>, &'a [UnshiftedEntry<'a>]),
}

#[derive(Clone, Debug)]
pub struct UnshiftedEntry<'a> {
    pub key: &'a str,
    pub val: znode::Expr,
    pub defines_deb: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Distance(pub usize);

impl Context<'static> {
    pub fn empty() -> Self {
        Context::Base(&[])
    }
}

impl Context<'_> {
    pub fn len(&self) -> usize {
        match self {
            Context::Base(entries) => entries.len(),
            Context::Snoc(context, entries) => context.len() + entries.len(),
        }
    }

    /// If the `key == "_"`, this function returns `None`.
    ///
    /// Otherwise, if the context has an entry with `key`,
    /// this function returns `Some(entry, distance)`.
    ///
    /// We define `distance` as the current length of the context minus the
    /// length of the context at the time the entry was added.
    ///
    /// If there are multiple entries with `key`,
    /// we choose the rightmost.
    pub fn get(&self, key: &str) -> Option<(&UnshiftedEntry, Distance)> {
        if key == "_" {
            return None;
        }

        self.get_unchecked(key)
    }

    /// > Note: This function does **not** check whether `key == "_"`.
    ///
    /// If the context has an entry with `key`,
    /// this function returns `Some(entry, distance)`.
    ///
    /// We define `distance` as the current length of the context minus the
    /// length of the context at the time the entry was added.
    ///
    /// If there are multiple entries with `key`,
    /// we choose the rightmost.
    fn get_unchecked(&self, key: &str) -> Option<(&UnshiftedEntry, Distance)> {
        match self {
            Context::Base(entries) => get_entry_unchecked(key, entries).ok(),
            Context::Snoc(rdc, rac) => match get_entry_unchecked(key, rac) {
                Ok(entry_and_dist) => Some(entry_and_dist),
                Err(num_of_debs_defined) => {
                    let (entry, subdist) = rdc.get_unchecked(key)?;
                    let dist = Distance(subdist.0 + num_of_debs_defined);
                    Some((entry, dist))
                }
            },
        }
    }
}

/// > Note: This function does **not** check whether `key == "_"`.
///
/// - If the slice has an entry with `key`,
///   this function returns `Ok((entry, distance))`.
///   
///   We define `distance` as the number of deb-defining
///   entries between the entry with `key` and the end of the slice.
///   The entry with `key` does **not** count towards `distance`.
///
///   If there are multiple entries with `key`,
///   we choose the rightmost.
///
/// - If the slice does not have an entry with `key`,
///   this function returns `Err(num_of_debs_defined)`.
fn get_entry_unchecked<'a>(
    key: &str,
    entries: &'a [UnshiftedEntry<'a>],
) -> Result<(&'a UnshiftedEntry<'a>, Distance), usize> {
    let mut num_of_debs_defined = 0;

    for entry in entries.iter().rev() {
        if entry.key == key {
            return Ok((entry, Distance(num_of_debs_defined)));
        }
        if entry.defines_deb {
            num_of_debs_defined += 1;
        }
    }

    Err(num_of_debs_defined)
}
