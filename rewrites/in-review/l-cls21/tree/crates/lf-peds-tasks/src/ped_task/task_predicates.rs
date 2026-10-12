//! Two read-only predicates on a ped task object: the enabled-with-count
//! test, and the resource-tag test.
//!
//! Both routines take only the task object and read its words; neither
//! writes memory or calls anything. The owning type is [`PredicateTask`],
//! which names the words the two predicates read, at their offsets in the
//! 32-bit layout. The resource a task refers to is an owned value,
//! [`ResourceTag`], present or absent: the original's null-pointer check
//! becomes the `Option`, and the one read through that pointer is the tag
//! word of the resource.
//!
//! The original reads the flag word and the kind byte of the task with byte
//! reads, and the enable word and count with 32-bit reads. The fields keep
//! those widths.

/// Offset of the enable word read by [`PredicateTask::enabled_positive_count`].
pub const ENABLED_OFFSET: u32 = 0x30;
/// Offset of the signed count read by [`PredicateTask::enabled_positive_count`].
pub const COUNT_OFFSET: u32 = 0x38;
/// Offset of the flag byte read by [`PredicateTask::resource_tag`].
pub const FLAGS_OFFSET: u32 = 0x26c;
/// Offset of the kind byte read by [`PredicateTask::resource_tag`].
pub const KIND_OFFSET: u32 = 0x1e2;
/// Offset of the resource pointer read by [`PredicateTask::resource_tag`].
pub const RESOURCE_OFFSET: u32 = 0x1bc;
/// Offset of the tag word inside the resource.
pub const RESOURCE_TAG_OFFSET: u32 = 0x28;

/// Bit of the flag byte that makes [`PredicateTask::resource_tag`] succeed
/// without looking further.
pub const FLAG_SATISFIED: u8 = 0x04;
/// Mask of the kind byte's low nibble, the task type.
pub const KIND_TYPE_MASK: u8 = 0x0f;
/// Smallest task type that can be satisfied through the resource.
pub const KIND_MINIMUM: u8 = 2;
/// Mask of the tag bits that [`PredicateTask::resource_tag`] compares.
pub const TAG_MASK: u32 = 0x3c0;
/// Value the masked tag bits must equal.
pub const TAG_EXPECTED: u32 = 0x80;

/// The resource a task refers to, as far as the predicates read it: its tag
/// word at [`RESOURCE_TAG_OFFSET`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResourceTag {
    /// Word at offset 0x28 of the resource.
    pub tag: u32,
}

/// The words of a ped task that the two predicates read.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PredicateTask {
    /// Word at offset 0x30: the task is enabled when nonzero.
    pub enabled: u32,
    /// Word at offset 0x38: a signed count, positive when the task counts.
    pub count: i32,
    /// Byte at offset 0x26c: flag bits. [`FLAG_SATISFIED`] is one of them.
    pub flags: u8,
    /// Byte at offset 0x1e2: the task kind in its low nibble.
    pub kind: u8,
    /// Word at offset 0x1bc: the resource the task is bound to, or none
    /// when the original's pointer is null.
    pub resource: Option<ResourceTag>,
}

impl PredicateTask {
    /// Whether the task is enabled and its count is positive.
    ///
    /// The enable word is compared with zero as an unsigned word; the count
    /// is compared as a signed word, so a negative count is not positive.
    #[must_use]
    pub fn enabled_positive_count(&self) -> bool {
        self.enabled != 0 && self.count > 0
    }

    /// Whether the task passes the resource-tag test.
    ///
    /// A task whose flag bit is set passes at once. Otherwise its kind must
    /// reach [`KIND_MINIMUM`], it must be bound to a resource, and the
    /// resource's tag bits under [`TAG_MASK`] must equal [`TAG_EXPECTED`].
    #[must_use]
    pub fn resource_tag(&self) -> bool {
        if self.flags & FLAG_SATISFIED != 0 {
            return true;
        }
        if self.kind & KIND_TYPE_MASK < KIND_MINIMUM {
            return false;
        }
        match self.resource {
            None => false,
            Some(resource) => resource.tag & TAG_MASK == TAG_EXPECTED,
        }
    }
}
