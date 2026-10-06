//! Handle-indexed pool pages: datum and flag reads.
//!
//! Lifted from three verified pure readers (no class) that share one
//! lookup: the signed 16-bit handle at a slot's +0x2E indexes a global
//! table of entries, each entry's word at +0x70 is its datum page, and
//! the page carries the datum word at +8 and the flag word at +0x6C. The
//! lift owns the pages; the table base stays at the boundary.

/// One datum page: the two words the readers load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    /// Datum word (page +8).
    pub datum: u32,
    /// Flag word (page +0x6C).
    pub flags: u32,
}

/// The pool pages behind the global handle table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlePool {
    /// One page per table entry, in table order.
    pub pages: Vec<Page>,
}

impl HandlePool {
    /// Resolves a handle to its page.
    ///
    /// # Panics
    ///
    /// When the handle is negative or past the table (the original reads
    /// the wrapped table address regardless).
    fn page(&self, handle: i16) -> Page {
        assert!(
            handle >= 0,
            "handle {handle} is negative: the original reads before the table"
        );
        *self
            .pages
            .get(handle as usize)
            .unwrap_or_else(|| panic!("handle {handle} past {} pages", self.pages.len()))
    }

    /// Reads the datum word behind a slot handle.
    #[must_use]
    pub fn datum_field(&self, handle: i16) -> u32 {
        self.page(handle).datum
    }

    /// Tests one bit of the flag word behind a slot handle. The two
    /// verified instances test bits 15 and 10; the bit travels as an
    /// argument and each instance is proven against its own bit.
    ///
    /// # Panics
    ///
    /// When `bit` is 32 or more.
    #[must_use]
    pub fn flag_bit(&self, handle: i16, bit: u32) -> bool {
        assert!(bit < 32, "flag bit {bit} is outside the flag word");
        self.page(handle).flags & (1 << bit) != 0
    }
}
