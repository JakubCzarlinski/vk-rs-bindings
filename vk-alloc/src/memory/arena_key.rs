use crate::resource::ResourceClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ArenaKey {
    pub(crate) memory_type_index: u32,
    pub(crate) resource_class: ResourceClass,
    pub(crate) pool_id: u32,
    pub(crate) partition_mask: u32,
}
