// original: 0x006f56d0 queue_drain_pair
/// Initializes the paired-queue object, then drains both pending queues.
///
/// Tags the object with its type id, runs the secondary initializer, and pops
/// every pending node from each of the two queues through the shared pop
/// helper until both depths read zero.
export!(thiscall, rw_006f56d0(this: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
        *obj = relocated(0xFE543C);
        callee_thiscall!(1, u32, this);
        while *obj.add(0x58 / 4) != 0 {
            let node = *obj.add(0x50 / 4);
            if node != 0 {
                callee_thiscall!(2, u32, this.wrapping_add(0x50), node);
            }
        }
        while *obj.add(0x4c / 4) != 0 {
            let node = *obj.add(0x44 / 4);
            if node != 0 {
                callee_thiscall!(2, u32, this.wrapping_add(0x44), node);
            }
        }
    }
});
