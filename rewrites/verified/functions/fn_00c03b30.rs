use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const SELECTOR_TABLE_FILE_VA: u32 = 0x01723be0;
const STREAM_TARGET_FILE_VA: u32 = 0x013baba0;
const SELECTOR_MASK: u8 = 0x3f;
const OWNER_ID_OFFSET: usize = 0x18;
const OWNER_SCALAR_OFFSET: usize = 0x1c;

#[repr(C, align(16))]
struct VectorBuffer([u32; 4]);

#[inline(always)]
unsafe fn byte_at(base: u32, offset: usize) -> u8 {
    unsafe { (base as *const u8).add(offset).read() }
}

#[inline(always)]
unsafe fn word_at(base: u32, offset: usize) -> u32 {
    unsafe { ((base as *const u8).add(offset) as *const u32).read_unaligned() }
}

/// Builds two owner-derived vectors, looks up the stream item, and conditionally
/// forwards the three initialized vector words plus selector and scalar inputs.
/// The vector helpers are stubbed and each writes three words; their fourth
/// buffer word is allocated for aligned downstream access but remains outside
/// this proof because the native helpers leave it unwritten.
/// The global-derived vector also occupies four aligned words, but the native
/// wrapper writes only its first three words. Its unwritten fourth word is
/// excluded from the downstream snapshot and remains outside this proof.
unsafe fn emit(this: u32, wrong: bool) {
    let mut owner_vector = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(1, u32, this, owner_vector.0.as_mut_ptr() as u32);

    let mut direction_vector = VectorBuffer([0; 4]);
    let _ = callee_thiscall!(2, u32, this, direction_vector.0.as_mut_ptr() as u32);

    if wrong {
        owner_vector.0[0] = owner_vector.0[0].wrapping_add(1);
    }

    let selector_byte = unsafe { byte_at(this, 0x16) };
    let secondary_byte = unsafe { byte_at(this, 0x17) };
    let stream_item = callee_thiscall!(
        3,
        u32,
        lf_checker_rt::relocated(SELECTOR_TABLE_FILE_VA),
        u32::from(secondary_byte & SELECTOR_MASK),
        u32::from(selector_byte & SELECTOR_MASK)
    );

    let owner_id = unsafe { word_at(this, OWNER_ID_OFFSET) };
    let hash_result = if owner_id == u32::MAX || owner_id == u32::MAX - 1 {
        0
    } else {
        callee_cdecl!(4, u32, 1, owner_id)
    };

    let accepted_id = owner_id == u32::MAX || owner_id == u32::MAX - 1 || hash_result != 0;
    if stream_item != 0 && accepted_id {
        let global_vector = VectorBuffer([
            unsafe { lf_checker_rt::global::<u32>(0x0110db10).read_unaligned() },
            unsafe { lf_checker_rt::global::<u32>(0x0110db14).read_unaligned() },
            unsafe { lf_checker_rt::global::<u32>(0x0110db18).read_unaligned() },
            0,
        ]);
        let normalized_flag = u32::from((selector_byte >> 7) & 1);
        let scalar_bits = unsafe { word_at(this, OWNER_SCALAR_OFFSET) };
        let _ = callee_thiscall!(
            5,
            u32,
            lf_checker_rt::relocated(STREAM_TARGET_FILE_VA),
            stream_item,
            hash_result,
            owner_vector.0.as_ptr() as u32,
            direction_vector.0.as_ptr() as u32,
            global_vector.0.as_ptr() as u32,
            normalized_flag,
            scalar_bits,
            1
        );
    }
}

export!(thiscall, rw_00c03b30(this: u32) -> () {
    unsafe { emit(this, false) }
});

