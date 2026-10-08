use core::mem::MaybeUninit;
use lf_checker_rt::{callee_thiscall, export};

const SELECTOR_OFFSET: usize = 0x17;
const MODE_OFFSET: usize = 0x1a;
const FLAGS_OFFSET: usize = 0x1b;
const SELECTOR_MASK: u8 = 0x3f;
const LOW_FLAG_MASK: u8 = 0x01;
const HIGH_FLAG_MASK: u8 = 0x02;
const TABLE_BASE_FILE_VA: u32 = 0x016d8de0;
const STREAM_TARGET_FILE_VA: u32 = 0x013baba0;
const ENABLED: u32 = 1;

/// Proof limits: raw vector pointer values are skipped while nine initialized
/// pointee words are compared. Full data-section checking is disabled; return
/// comparison is omitted for this void wrapper.
///
/// Specification: read the selector, mode and two flag bits; call the vector
/// helpers and table lookup; then forward all three four-word vector buffers
/// and the scalar values to the stream target.
///
/// Scope: helper 1/2/4 bodies and the final target body are intercepted. The
/// helpers initialize words 0..=2; word 3 of each native frame buffer is
/// unwritten, although the final target reads it. The proof snapshots every
/// initialized output word and leaves the three unwritten words outside its
/// comparison. The middle buffer is included because the final target reads
/// it. Table lookup returns a scripted heap pointer; its body is outside scope.
unsafe fn emit_two(this: u32) {
    // Each native helper writes three dwords; the final target reads four.
    // Keep a fourth uninitialized word so the buffer has the native read span.
    let mut owner_vector = [MaybeUninit::<u32>::uninit(); 4];
    let owner_vector_ptr = owner_vector.as_mut_ptr().cast::<u32>();
    let _ = callee_thiscall!(1, u32, this, owner_vector_ptr as u32);

    let mut middle_vector = [MaybeUninit::<u32>::uninit(); 4];
    let middle_vector_ptr = middle_vector.as_mut_ptr().cast::<u32>();
    let _ = callee_thiscall!(2, u32, this, middle_vector_ptr as u32);

    let owner = this as *const u8;
    let selector = unsafe { owner.add(SELECTOR_OFFSET).read() & SELECTOR_MASK };
    let table_item = callee_thiscall!(
        3, u32, lf_checker_rt::relocated(TABLE_BASE_FILE_VA), u32::from(selector)
    );

    let mut direction_vector = [MaybeUninit::<u32>::uninit(); 4];
    let direction_vector_ptr = direction_vector.as_mut_ptr().cast::<u32>();
    let _ = callee_thiscall!(4, u32, this, direction_vector_ptr as u32);

    let mode = unsafe { owner.add(MODE_OFFSET).read() };
    let flags = unsafe { owner.add(FLAGS_OFFSET).read() };
    let _ = callee_thiscall!(
        5,
        u32,
        lf_checker_rt::relocated(STREAM_TARGET_FILE_VA),
        table_item,
        u32::from(selector),
        owner_vector_ptr as u32,
        middle_vector_ptr as u32,
        direction_vector_ptr as u32,
        u32::from((flags & HIGH_FLAG_MASK) >> 1),
        u32::from(flags & LOW_FLAG_MASK),
        u32::from(mode),
        ENABLED
    );
}

export!(thiscall, rw_00c03aa0(this: u32) -> () {
    unsafe { emit_two(this) }
});

