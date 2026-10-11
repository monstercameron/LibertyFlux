// original: 0x009DB660 CSwayableAttr::~CSwayableAttr__deleting

//! Destroy selected swayable-attribute pool records.
//!
//! The thiscall receiver points at the first record. A signed record count is
//! stored one word before it, and records are spaced by 0x24 bytes. Flag bit
//! 1 replaces either the receiver's first word or each record's first word
//! with the default attribute table pointer. Flag bit 0 releases the selected
//! base through the intercepted cdecl helper. The result is the receiver, or
//! the count header when the record-array flag is set. Negative and zero
//! counts produce no record writes. Only the low two flag bits are observed.
//! The object and pointer slots use the original 32-bit layout.

const COUNT_HEADER_BACK: u32 = 4;
const RECORD_STRIDE: u32 = 0x24;
const REPLACE_RECORDS: u32 = 0x02;
const RELEASE_BASE: u32 = 0x01;
const DEFAULT_TABLE_FILE_VA: u32 = 0x00E9749C;
const RELEASE_CALLEE: u32 = 1;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    // SAFETY: the caller supplies a mapped pool address and aligned word.
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    // SAFETY: the caller supplies a writable mapped pool address and aligned word.
    unsafe { core::ptr::write_volatile(address as *mut u32, value) }
}

lf_checker_rt::export!(thiscall, rw_009db660(this: u32, flags: u32) -> u32 {
    let default_table = lf_checker_rt::relocated(DEFAULT_TABLE_FILE_VA);
    let returns_header = flags & REPLACE_RECORDS != 0;
    let selected_base = if returns_header {
        let record_count = unsafe { read_word(this.wrapping_sub(COUNT_HEADER_BACK)) } as i32;
        if record_count > 0 {
            for index in (0..record_count as u32).rev() {
                let record = this.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
                unsafe { write_word(record, default_table) };
            }
        }
        this.wrapping_sub(COUNT_HEADER_BACK)
    } else {
        unsafe { write_word(this, default_table) };
        this
    };

    if flags & RELEASE_BASE != 0 {
        let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, selected_base);
    }
    selected_base
});
