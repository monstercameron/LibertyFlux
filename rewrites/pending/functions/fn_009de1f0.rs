// original: 0x009de1f0 list_drain_notify
// fn_009de1f0: list drain with notify (thiscall/0).
//
// Unlinks every node of the `{head, next}` list at `this+0` one at a time
// (head splice, or predecessor search for a non-head node) and reports each
// removed node to the notify step together with the global owner.
export!(thiscall, rw_009de1f0(this: *mut u32) -> u32 {
    unsafe {
        let mut cur = *this;
        if cur == 0 {
            return 0;
        }
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        loop {
            let head = *this;
            let next = *((cur.wrapping_add(4)) as *const u32);
            if head == cur {
                *this = next;
            } else if head != 0 {
                let mut pred = head;
                loop {
                    let cand = *((pred.wrapping_add(4)) as *const u32);
                    if cand == cur {
                        *((pred.wrapping_add(4)) as *mut u32) = next;
                        break;
                    }
                    pred = cand;
                    if pred == 0 {
                        break;
                    }
                }
            }
            notify(*global::<u32>(0x12B4164), cur);
            cur = next;
            if cur == 0 {
                break;
            }
        }
        0
    }
});
