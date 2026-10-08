use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const STREAM_TABLE_FILE_VA: u32 = 0x013b6798;
const STREAM_TARGET_FILE_VA: u32 = 0x013baba0;
const SELECTOR_MASK: u8 = 0x3f;
const OWNER_LOOKUP_OFFSET: usize = 0x1c;

#[repr(C, align(16))]
struct VectorBuffer([u32; 4]);

#[repr(C, align(16))]
struct RequestRecord([u32; 21]);

#[inline(always)]
unsafe fn byte_at(base: u32, offset: usize) -> u8 {
    unsafe { (base as *const u8).add(offset).read() }
}

#[inline(always)]
unsafe fn word_at(base: u32, offset: usize) -> u32 {
    unsafe { ((base as *const u8).add(offset) as *const u32).read_unaligned() }
}

/// Produces three vectors, validates the owner selector, and forwards a
/// request record when both the stream table item and owner lookup allow it.
/// Each vector helper writes three words. The aligned fourth word is left out
/// of snapshots because its native value is unwritten; the request record's
/// defined fields and the helper outputs that feed them are observed.
unsafe fn emit(this: u32, wrong: bool) {
    let mut first_vector = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(1, u32, this, first_vector.0.as_mut_ptr() as u32);

    let mut second_vector = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(2, u32, this, second_vector.0.as_mut_ptr() as u32);

    let selector_byte = unsafe { byte_at(this, 0x16) };
    let secondary_byte = unsafe { byte_at(this, 0x17) };
    let stream_item = callee_thiscall!(
        3,
        u32,
        lf_checker_rt::relocated(STREAM_TABLE_FILE_VA),
        u32::from(secondary_byte & SELECTOR_MASK),
        u32::from(selector_byte & SELECTOR_MASK),
        u32::from((selector_byte >> 6) & 1)
    );

    let mut third_vector = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(4, u32, this, third_vector.0.as_mut_ptr() as u32);

    if wrong {
        second_vector.0[1] = second_vector.0[1].wrapping_add(1);
    }

    let lookup_key = unsafe { word_at(this, OWNER_LOOKUP_OFFSET) };
    let lookup_result = if lookup_key == u32::MAX || lookup_key == u32::MAX - 1 {
        0
    } else {
        callee_cdecl!(5, u32, 1, lookup_key)
    };
    let accepted_key = lookup_key == u32::MAX || lookup_key == u32::MAX - 1 || lookup_result != 0;
    if !accepted_key {
        return;
    }

    if stream_item != 0 {
        let mut record = RequestRecord([0; 21]);
        record.0[0] = lookup_result;
        record.0[4..7].copy_from_slice(&first_vector.0[..3]);
        record.0[8..11].copy_from_slice(&second_vector.0[..3]);
        record.0[12..15].copy_from_slice(&third_vector.0[..3]);
        record.0[16] = unsafe { word_at(this, 0x20) };
        record.0[17] = unsafe { ((this as *const u8).add(0x1a) as *const u16).read_unaligned() as u32 };
        record.0[18] = u32::from(unsafe { byte_at(this, 0x16) } & SELECTOR_MASK);
        record.0[19] = unsafe { word_at(this, 0x24) };
        let _ = callee_thiscall!(
            6,
            u32,
            lf_checker_rt::relocated(STREAM_TARGET_FILE_VA),
            stream_item,
            record.0.as_mut_ptr() as u32,
            1
        );
    }
}

export!(thiscall, rw_00c03dd0(this: u32) -> () {
    unsafe { emit(this, false) }
});

