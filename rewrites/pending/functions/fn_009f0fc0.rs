// original: 0x009f0fc0 CPlayerPed::vf44
/// Player-ped virtual slot 44: notify while linked, else run the base update.
///
/// When field `0x38` holds a non-null pointer equal to field `0x7b4` the
/// link is live and the trigger entry is called with (0, 1, 0). Otherwise
/// control tail-jumps to the base routine; the rewrite expresses the jump
/// as a forwarding call with the same object and result.
export!(thiscall, rw_009f0fc0(this_ptr: u32) -> u32 {
    unsafe {
        let link = *((this_ptr + 0x38) as *const u32);
        if link != 0 && link == *((this_ptr + 0x7b4) as *const u32) {
            callee_thiscall!(2, u32, this_ptr, 0, 1, 0)
        } else {
            callee_thiscall!(3, u32, this_ptr)
        }
    }
});
