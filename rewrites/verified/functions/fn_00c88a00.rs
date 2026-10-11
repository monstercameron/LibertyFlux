// original: 0x00c88a00 audio_scene_update
//! The routine returns false without updating when the request's activity bit
//! is clear, its view pointer is null, the pointed view has the invalid count
//! sentinel, a valid view has no receiver list, or a receiver list is present
//! with a zero count. The proof checks output clearing and the zero-count
//! finalization. Its helper calls are scripted; populated-list processing and
//! the double-valued helper remain outside this proof.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const REQUEST_FLAGS_OFFSET: usize = 0x24;
const REQUEST_VIEW_OFFSET: usize = 0x38;
const VIEW_COUNT_OFFSET: usize = 8;
const RECEIVER_ENTRIES_OFFSET: usize = 4;
const RECEIVER_COUNT_OFFSET: usize = 8;
const RECEIVER_STATE_OFFSET: usize = 0x20;
const OBJECT_MODE_OFFSET: usize = 0x28;
const REQUEST_ACTIVE_BIT: u8 = 1;
const INVALID_VIEW_COUNT: u16 = u16::MAX;

/// Recognize three confirmed exits before the update loop. On inputs in the
/// proof contract, each gate returns false and performs no heap writes.
#[inline(always)]
unsafe fn confirmed_no_update(request: *const u8) -> bool {
    let flags = core::ptr::read(request.add(REQUEST_FLAGS_OFFSET));
    if flags & REQUEST_ACTIVE_BIT == 0 {
        return true;
    }

    let view = core::ptr::read_unaligned(request.add(REQUEST_VIEW_OFFSET) as *const u32);
    if view == 0 {
        return true;
    }

    let count = core::ptr::read_unaligned(
        view.wrapping_add(VIEW_COUNT_OFFSET as u32) as *const u16,
    );
    count == INVALID_VIEW_COUNT
}

#[inline(always)]
pub(crate) unsafe fn partial_update(
    receiver: u32,
    request: *mut u8,
    reset_time: *mut u32,
    output: *mut u32,
    inverted: bool,
) -> u32 {
    if unsafe { confirmed_no_update(request) } {
        u32::from(inverted)
    } else {
        if !reset_time.is_null() {
            core::ptr::write_unaligned(reset_time, 0);
        }
        if !output.is_null() {
            core::ptr::write_unaligned(output, 0);
            core::ptr::write_unaligned(output.add(1), 0);
            core::ptr::write_unaligned(output.add(2), 0);
        }

        let helper_object = callee_thiscall!(1, u32, request as u32);
        let vtable = core::ptr::read_unaligned(helper_object as *const u32);
        let sample_slot = core::ptr::read_unaligned(
            vtable.wrapping_add(0x24) as *const u32,
        );
        let sample: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(sample_slot as usize) };
        let _sampled_value = sample(helper_object);

        let mode = core::ptr::read_unaligned(
            request.add(OBJECT_MODE_OFFSET) as *const u32,
        ) & 0x3c0;
        let receiver_entries = core::ptr::read_unaligned(
            receiver.wrapping_add(RECEIVER_ENTRIES_OFFSET as u32) as *const u32,
        );

        // Mode zero with no receiver list returns after the initializers.
        if mode == 0 && receiver_entries == 0 {
            return u32::from(inverted);
        }

        if mode == 0 && receiver_entries != 0 {
            let receiver_count = core::ptr::read_unaligned(
                receiver.wrapping_add(RECEIVER_COUNT_OFFSET as u32) as *const u16,
            );
            let mut position_result = core::mem::MaybeUninit::<[u32; 3]>::uninit();
            let has_position = callee_cdecl!(
                3,
                u8,
                0u32,
                0u32,
                0u32,
                position_result.as_mut_ptr() as u32,
                0x40c00000u32,
                0x41a00000u32,
                0u32,
            );

            if has_position == 0 {
                core::ptr::write_unaligned(
                    receiver.wrapping_add(RECEIVER_STATE_OFFSET as u32) as *mut u32,
                    0,
                );
                return u32::from(inverted);
            }

            if receiver_count == 0 && has_position != 0 {
                core::ptr::write_unaligned(
                    receiver.wrapping_add(RECEIVER_STATE_OFFSET as u32) as *mut u32,
                    2,
                );
                let mut entry_cache = [0u32; 45];
                let _ = callee_thiscall!(
                    4,
                    u32,
                    receiver,
                    request as u32,
                    entry_cache.as_mut_ptr() as u32,
                    0u32,
                );
                return u32::from(inverted);
            }
        }

        0
    }
}

export!(thiscall, rw_00c88a00(
    receiver: u32,
    request: *const u8,
    _arg1: u32,
    _arg2: u32,
    _arg3: u32,
    reset_time: *mut u32,
    output: *mut u32,
) -> u32 {
    unsafe { partial_update(receiver, request as *mut u8, reset_time, output, false) }
});
