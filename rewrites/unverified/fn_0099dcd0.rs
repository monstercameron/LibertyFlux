// original: 0x0099dcd0 aud_gta_audio_entity_teardown_3
/// Partial teardown of an audio entity, then a tail call.
///
/// Stamps the initial table address, and unless the flag word at offset 4 is
/// 0xffff, runs the release sequence through callees 1 to 5. Then it frees
/// the pointer at offset 0x1f8 through callee 6 when set (nulling it), frees
/// the pointer at 0x204 through callee 7 when set (nulling it), runs callee 8
/// on the two embedded members, stamps the final table address and tail-calls
/// callee 9. Callee 2 takes two stack words (a zero on top of the dword at
/// offset 0x9c); the stub pops both, which is the only cleanup that keeps
/// the original's own pops and tail return aligned. The export body keeps no
/// frame so the tail stub returns straight to the caller.
fn fn_0099dcd0_body(this: u32) {
    unsafe {
        const VTABLE_INIT: u32 = 0xe91260;
        const VTABLE_FIN: u32 = 0xe83134;
        const MANAGER: u32 = 0x1288780;
        *(this as *mut u32) = relocated(VTABLE_INIT);
        if *((this.wrapping_add(4)) as *const u16) != 0xffff {
            callee_thiscall!(1, u32, this);
            let slot = *((this.wrapping_add(0x9c)) as *const u32);
            let r2 = callee_thiscall!(2, u32, this, 0, slot);
            callee_thiscall!(3, u32, relocated(MANAGER), r2);
            callee_thiscall!(4, u32, this, 0);
            callee_thiscall!(5, u32, relocated(MANAGER), this);
        }
        let p = *((this.wrapping_add(0x1f8)) as *const u32);
        if p != 0 {
            callee_thiscall!(6, u32, p, this.wrapping_add(0x1f8));
            *((this.wrapping_add(0x1f8)) as *mut u32) = 0;
        }
        let q = *((this.wrapping_add(0x204)) as *const u32);
        if q != 0 {
            callee_cdecl!(7, u32, q);
            *((this.wrapping_add(0x204)) as *mut u32) = 0;
        }
        callee_thiscall!(8, u32, this.wrapping_add(0x120));
        callee_thiscall!(8, u32, this.wrapping_add(0xd0));
        *(this as *mut u32) = relocated(VTABLE_FIN);
    }
}
export!(thiscall, rw_0099dcd0(this: u32) -> () {
    fn_0099dcd0_body(this);
    callee_thiscall!(9, u32, this);
});
