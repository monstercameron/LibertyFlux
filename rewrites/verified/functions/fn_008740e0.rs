// original: 0x008740e0 crmt_adopt_child
// Adopt a child: when the candidate carries a linked record, hand that
// record to the shared attach hook; otherwise attach the previous child
// and keep the returned one, taking a reference when this slot was empty.
// (thiscall/2)
export!(thiscall, rw_008740e0(this_ptr: u32, arg0: u32, candidate: u32) -> () {
    unsafe {
        let base = this_ptr as *mut u32;
        let mut linked = 0u32;
        if candidate != 0 {
            linked = (candidate as *const u32).add(2).read();
        }
        if candidate != 0 && linked != 0 {
            callee_thiscall!(1, u32, arg0, this_ptr, linked);
        } else {
            let prev = base.add(2).read();
            let kept = callee_thiscall!(1, u32, arg0, this_ptr, prev);
            base.add(2).write(kept);
            if prev == 0 && kept != 0 {
                let count = kept.wrapping_add(4) as *mut u16;
                count.write(count.read().wrapping_add(1));
            }
        }
    }
});
