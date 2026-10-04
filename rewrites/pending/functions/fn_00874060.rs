// original: 0x00874060 crmt_full_teardown
// Full teardown: release the reference-counted child (letting its owner
// reclaim it when the count hits zero), detach the two plain children,
// then clear the trailing fields. (thiscall/0)
export!(thiscall, rw_00874060(this_ptr: u32) -> () {
    unsafe {
        let base = this_ptr as *mut u32;
        let child = base.add(2).read();
        if child != 0 {
            let child_vt = (child as *const u32).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(((child_vt + 4) as *const u32).read() as usize);
            release(child);
            let live = base.add(2).read();
            let count = live.wrapping_add(4) as *mut u16;
            count.write(count.read().wrapping_add(0xFFFF));
            if count.read() == 0 {
                let owner = (live as *const u32).add(6).read();
                if owner != 0 {
                    callee_thiscall!(3, u32, owner, live);
                } else {
                    let live_vt = (live as *const u32).read();
                    let delete: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute((live_vt as *const u32).read() as usize);
                    delete(live, 1);
                }
            }
            base.add(2).write(0);
        }
        for slot in [3usize, 4usize] {
            let plain = base.add(slot).read();
            if plain != 0 {
                let plain_vt = (plain as *const u32).read();
                let delete: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute((plain_vt as *const u32).read() as usize);
                delete(plain, 1);
                base.add(slot).write(0);
            }
        }
        base.add(7).write(0);
        base.add(6).write(0);
    }
});
