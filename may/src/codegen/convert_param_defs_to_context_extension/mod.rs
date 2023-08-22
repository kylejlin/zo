use super::*;

mod typed_params;
pub use typed_params::*;

mod untyped_params;

impl MayConverter {
    pub(crate) fn get_deb_defining_entry<'a>(&mut self, key: &'a str) -> UnshiftedEntry<'a> {
        let val = self.cache_deb(znode::DebNode {
            deb: Deb(0),
            aux_data: (),
        });
        UnshiftedEntry {
            key,
            val,
            defines_deb: true,
        }
    }
}
