// original: 0x00AC5CE0 stream_keyset_remove (proposed)

/// Remove the key at or below `time` from the streaming key set.
///
/// The original walks the tree rooted at `this + 4` with the time float at
/// `key + 4` (go right while the key is strictly above the node time,
/// otherwise step left), then removes the last node at or below the key
/// through the unlink callee, frees a detached node through the free callee
/// when one comes back, decrements the count at `this + 0x10` and returns 1
/// (thiscall, one stack pointer). An empty tree, a tree whose keys all lie
/// above the key, or no candidate returns 0.
lf_checker_rt::export!(thiscall, rw_00AC5CE0(this: u32, key: u32) -> u32 {
    unsafe {
        const UNLINK: u32 = 1;
        const FREE: u32 = 2;
        const ROOT: u32 = 4;
        const COUNT: u32 = 0x10;
        const LEFT: u32 = 8;
        const RIGHT: u32 = 0x0c;
        const TIME: u32 = 0x14;
        let root = (this.wrapping_add(ROOT) as *const u32).read_unaligned();
        if root == 0 {
            return 0;
        }
        let want = (key.wrapping_add(4) as *const f32).read_unaligned();
        let mut cand = this;
        let mut cur = root;
        loop {
            let t = (cur.wrapping_add(TIME) as *const f32).read_unaligned();
            if want > t {
                cur = (cur.wrapping_add(RIGHT) as *const u32).read_unaligned();
            } else {
                cand = cur;
                cur = (cur.wrapping_add(LEFT) as *const u32).read_unaligned();
            }
            if cur == 0 {
                break;
            }
        }
        if cand == this {
            return 0;
        }
        let ct = (cand.wrapping_add(TIME) as *const f32).read_unaligned();
        if ct > want {
            return 0;
        }
        let detached = lf_checker_rt::callee_cdecl!(
            UNLINK, u32, cand, this.wrapping_add(4), this.wrapping_add(8), this.wrapping_add(0x0c)
        );
        if detached != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, detached);
        }
        let c = (this.wrapping_add(COUNT) as *mut u32);
        *c = c.read_unaligned().wrapping_sub(1);
        1
    }
});
