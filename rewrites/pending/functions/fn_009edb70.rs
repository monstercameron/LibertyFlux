// original: 0x009edb70 CPlayerPed::vf32
/// Player-ped virtual slot 32: reset, flag and re-run a sub-object.
///
/// Calls three entries in order on the object at field `0x224` plus
/// `0x84`: a reset, a one-flag call, then the main entry, whose tail jump
/// the rewrite expresses as a forwarding call returning its answer.
export!(thiscall, rw_009edb70(this_ptr: u32) -> u32 {
    unsafe {
        let sub = (*((this_ptr + 0x224) as *const u32)).wrapping_add(0x84);
        let _: u32 = callee_thiscall!(2, u32, sub);
        let _: u32 = callee_thiscall!(3, u32, sub, 1);
        callee_thiscall!(4, u32, sub)
    }
});
