// original: 0x00cb73d0 CTaskComplexMoveFollowPointRoute::vf7
/// Copies the current route point (12 bytes) from the +0x20 table to `dst`; returns `dst`.
#[allow(clippy::all)]
#[allow(non_snake_case)]
#[allow(unused_unsafe)]
export!(thiscall, rw_cb73d0(this_ptr: u32, dst: u32) -> u32 {
    unsafe {
        let index = ((this_ptr.wrapping_add(0x24)) as *const u32).read();
        let table = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
        let row = table.wrapping_add(index.wrapping_add(1).wrapping_mul(16));
        ((dst) as *mut u32).write((row as *const u32).read());
        ((dst.wrapping_add(4)) as *mut u32).write(((row.wrapping_add(4)) as *const u32).read());
        ((dst.wrapping_add(8)) as *mut u32).write(((row.wrapping_add(8)) as *const u32).read());
        dst
    }
});
