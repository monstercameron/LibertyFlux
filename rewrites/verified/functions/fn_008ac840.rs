//! Accepted complete ring-buffer/update body within the declared object fixture.
//! The direct helper and callback are scripted; native collaborator effects are outside scope.
//! The helper loop limit is zero, and the update flag uses declared zero/one values.
//! x87 state is unchecked; enabled stack/global comparisons observed no writes.
//! The private wrong-destination switch is unchanged and selected false by the sole export.
//! This source projection is unbuilt; the exact tested source/build/DLL receipts bind acceptance.
//! Rotates a three-row output buffer and expands packed points when updates are enabled.
//!
//! Proof scope: the Q176 stock contract completed 1,000 original trials, passed all nine
//! comparison categories, and exercised both declared recorder callees. Its meaningful
//! same-contract mutation completed three originals and failed all three with return and
//! heap mismatches.
//!
//! Fixture limits: callee 1 is intercepted, and owner+0x70 is pinned to zero, bypassing
//! its positive-count table-copy loop. The callback and callees are fabricated recorder
//! stubs; callback coverage is limited to the declared fixture. owner+0x150 is configured
//! for zero and one without per-value frequency counts. Stack and global writes were zero
//! in the recorded originals, and x87 state was unchecked. This evidence describes the
//! tested fixture only.
//!
//! The test helper wrong_destination switch is preserved; this export passes false.

mod function_impl {
use lf_checker_rt::{callee_thiscall, global};

const ROW_TABLE_OFFSET: usize = 0x78;
const ROW_SIZE: usize = 72;
const RING_ROWS: u32 = 3;
const INPUT_POINTER_OFFSET: usize = 0x04;
const CALLBACK_POINTER_OFFSET: usize = 0x08;
const ROW_INDEX_OFFSET: usize = 0x30;
const HELPER_LIMIT_OFFSET: usize = 0x70;
const SKIP_UPDATE_OFFSET: usize = 0x150;
const SCALE_FILE_VA: u32 = 0x00FE_870C;
const HALFWORD_COUNT: usize = 6;

/// Implements the buffer rotation and optional packed-point expansion.
///
/// Contract bounds: the preceding helper call is patched to callee 1 and its
/// byte loop limit at owner+0x70 is pinned to zero. The helper's nonzero
/// table-copy behavior is therefore outside this candidate's contract.
pub(crate) unsafe fn run(owner: *mut u8, wrong_destination: bool) -> u32 {
    // This is the direct E8 helper call at file VA 0x008AC849. Its return is
    // overwritten before it can affect the outer method's result.
    let _helper_result = callee_thiscall!(1, u32, owner as u32);

    let old_row = owner.add(ROW_INDEX_OFFSET).cast::<u32>().read_unaligned();
    let advanced = old_row.wrapping_add(1);
    let next_row = advanced % RING_ROWS;
    let destination_row = if wrong_destination {
        advanced.wrapping_add(1) % RING_ROWS
    } else {
        next_row
    };

    let table = owner.add(ROW_TABLE_OFFSET);
    let source = table.add(old_row.wrapping_mul(ROW_SIZE as u32) as usize);
    let destination = table.add(destination_row as usize * ROW_SIZE);
    core::ptr::copy_nonoverlapping(source, destination, ROW_SIZE);

    let skip_update = owner.add(SKIP_UPDATE_OFFSET).read() != 0;
    let mut null_result = advanced / RING_ROWS;
    if !skip_update {
        let input = owner
            .add(INPUT_POINTER_OFFSET)
            .cast::<*const u8>()
            .read_unaligned();

        // The native code reads the packed words at unaligned offsets 15,
        // 17 and 19. The first is unsigned; the latter two are signed.
        let packed_index =
            u16::from_le_bytes([input.add(15).read(), input.add(16).read()]);
        let x_units = input.add(17).cast::<i16>().read_unaligned() as i32;
        let y_units = input.add(19).cast::<i16>().read_unaligned() as i32;
        let scale = global::<f32>(SCALE_FILE_VA).read();
        let x = (x_units as f32) * scale;
        let y = (y_units as f32) * scale;

        for item in 0..HALFWORD_COUNT {
            let offset = item * 4;
            destination
                .add(offset)
                .cast::<u32>()
                .write_unaligned(u32::from(packed_index));
            destination
                .add(24 + offset)
                .cast::<f32>()
                .write_unaligned(x);
            destination
                .add(48 + offset)
                .cast::<f32>()
                .write_unaligned(y);
        }

        // After the six stores, the native EAX points to the next row.
        null_result = destination as u32 + ROW_SIZE as u32;
    }

    owner
        .add(ROW_INDEX_OFFSET)
        .cast::<u32>()
        .write_unaligned(next_row);

    let callback = owner
        .add(CALLBACK_POINTER_OFFSET)
        .cast::<u32>()
        .read_unaligned();
    if callback == 0 {
        return null_result;
    }

    // The native tail-jump loads the callback object's vtable and jumps
    // through slot 5. The contract plants callee 2 in this fake slot.
    let vtable = (callback as *const u32).read_unaligned();
    let method = ((vtable as *const u8).add(0x14) as *const u32).read_unaligned();
    let call: extern "thiscall" fn(u32) -> u32 =
        core::mem::transmute(method as usize);
    call(callback)
}
}
lf_checker_rt::export!(thiscall, rw_a_q176_008ac840(this: *mut u8) -> u32 {
    unsafe { function_impl::run(this, false) }
});