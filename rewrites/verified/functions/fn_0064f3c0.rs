// original: 0x0064F3C0 rage::ptxSprite::vf13
/// Particle-sprite list sweep: walks the live sprite list, runs the per-flag
/// update helper on each node, then either keeps the node (refreshing its two
/// children and appending it to its bucket list) or releases it (tearing down
/// its children, unlinking it, and pushing it onto the owner's free list),
/// depending on whether the node's key is below the global threshold.
///
/// `this` is the sprite manager; `ctx` is an opaque value forwarded to the
/// update helper; `list` is the live-list head; `_r1`/`_r2` are unread.
/// Returns `this`.
lf_checker_rt::export!(thiscall, rb87_fn1(this: u32, ctx: u32, list: u32, _r1: u32, _r2: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_UPDATE: u32 = 1; // per-node flag update helper (thiscall/3)
    const CAL_COMBINE: u32 = 2; // combine two vectors into a frame block (thiscall/2)
    const CAL_APPLY: u32 = 3; // apply the frame block to the child (thiscall/1)
    const CAL_RELEASE: u32 = 4; // release a child (thiscall/1)

    // Globals read (file VAs; resolved through the worker's image base).
    const LIMIT_VA: u32 = 0x00FE88E8; // keep/release threshold (1.0)
    const SIGNMASK_VA: u32 = 0x00FE8FA0; // sign bit for vector negation
    const GVEC_VA: u32 = 0x018D2170; // three-word vector combined per child
    const GCTX_VA: u32 = 0x01BB6678; // context pointer; byte at +1 is the bucket index

    const THIS_POOL: u32 = 0x184; // free-list pool
    const HEAD_NEXT: u32 = 0x00;
    const HEAD_COUNT: u32 = 0x1C;
    const N_NEXT: u32 = 0x00;
    const N_PREV: u32 = 0x04;
    const N_CHILD_A: u32 = 0x78;
    const N_CHILD_B: u32 = 0x7C;
    const N_FLAG_A: u32 = 0x83;
    const N_FLAG_B: u32 = 0x84;
    const N_ALT: u32 = 0xB0; // vector used on the release path (3 words)
    const N_POS: u32 = 0xD0; // position vector (3 words)
    const N_KEY: u32 = 0xF0; // compared against the threshold
    const C_BACK: u32 = 0x25C; // back-pointer to the owning node
    const C_FLAGS: u32 = 0x240; // bit 5: combine; bit 6: negate first
    const C_POS: u32 = 0x190; // position copy (3 words)
    const P_FREE: u32 = 0x40;
    const P_COUNT: u32 = 0x5C;
    const P_TAIL: u32 = 0x68;

    #[inline(always)]
    fn h32(base: u32, off: u32) -> u32 {
        unsafe { *(base.wrapping_add(off) as *const u32) }
    }

    #[inline(always)]
    fn wh32(base: u32, off: u32, v: u32) {
        unsafe {
            *(base.wrapping_add(off) as *mut u32) = v;
        }
    }

    #[inline(always)]
    fn h8(base: u32, off: u32) -> u8 {
        unsafe { *(base.wrapping_add(off) as *const u8) }
    }

    #[inline(always)]
    fn hf(base: u32, off: u32) -> f32 {
        unsafe { *(base.wrapping_add(off) as *const f32) }
    }

    #[inline(always)]
    fn g32(file_va: u32) -> u32 {
        unsafe { *(lf_checker_rt::global::<u32>(file_va) as *const u32) }
    }

    #[inline(always)]
    fn gf(file_va: u32) -> f32 {
        unsafe { *(lf_checker_rt::global::<f32>(file_va) as *const f32) }
    }

    /// Release path for one slot: call the release helper with the node's
    /// alternate vector when the child still points back, then clear the slot.
    fn release_slot(node: u32, slot: u32) {
        let child = h32(node, slot);
        if child != 0 {
            if h32(child, C_BACK) == node {
                let v = [h32(node, N_ALT), h32(node, N_ALT + 4), h32(node, N_ALT + 8)];
                lf_checker_rt::callee_thiscall!(CAL_RELEASE, u32, child, v.as_ptr() as u32);
                wh32(child, C_BACK, 0);
            }
            wh32(node, slot, 0);
        }
    }

    /// Keep path for one slot: refresh the child's position copy and, when its
    /// combine flag is set, build the combined vector pair and apply it.
    fn update_slot(node: u32, slot: u32) {
        let child = h32(node, slot);
        if child == 0 {
            return;
        }
        if h32(child, C_BACK) != node {
            wh32(node, slot, 0);
            return;
        }
        wh32(child, C_POS, h32(node, N_POS));
        wh32(child, C_POS + 4, h32(node, N_POS + 4));
        wh32(child, C_POS + 8, h32(node, N_POS + 8));
        let flags = h32(child, C_FLAGS);
        if flags & 0x20 == 0 {
            return;
        }
        let mut a = [h32(node, N_POS), h32(node, N_POS + 4), h32(node, N_POS + 8)];
        let mut b = [g32(GVEC_VA), g32(GVEC_VA + 4), g32(GVEC_VA + 8)];
        if flags & 0x40 != 0 {
            let m = g32(SIGNMASK_VA);
            let mut i = 0;
            while i < 3 {
                a[i] ^= m;
                b[i] ^= m;
                i += 1;
            }
        }
        // The original's block is uninitialized stack; the contract defines the
        // fill as zero, so an explicit zero block matches on both sides.
        let out = [0u32; 7];
        lf_checker_rt::callee_thiscall!(
            CAL_COMBINE,
            u32,
            out.as_ptr() as u32,
            a.as_ptr() as u32,
            b.as_ptr() as u32
        );
        lf_checker_rt::callee_thiscall!(CAL_APPLY, u32, child, out.as_ptr() as u32);
    }

    /// Unlink `node` from the live list and push it onto the owner's free list.
    fn unlink_and_free(this: u32, list: u32, node: u32, next: u32) {
        let pool = h32(this, THIS_POOL);
        wh32(list, HEAD_COUNT, h32(list, HEAD_COUNT).wrapping_sub(1));
        let prev = h32(node, N_PREV);
        if h32(list, HEAD_NEXT) == node {
            wh32(list, HEAD_NEXT, next);
        }
        if next != 0 {
            wh32(next, N_PREV, prev);
        }
        if prev != 0 {
            wh32(prev, N_NEXT, h32(node, N_NEXT));
        }
        wh32(node, N_PREV, 0);
        if h32(pool, P_FREE) == 0 {
            wh32(pool, P_TAIL, node);
        }
        let head = h32(pool, P_FREE);
        wh32(node, N_NEXT, head);
        wh32(node, N_PREV, 0);
        wh32(pool, P_COUNT, h32(pool, P_COUNT).wrapping_add(1));
        wh32(pool, P_FREE, node);
    }

    /// Append `node` to the tail of its bucket list.
    fn bucket_append(list: u32, node: u32) {
        let g = g32(GCTX_VA);
        let b = (h8(g, 1) as u32).wrapping_mul(4);
        wh32(
            list,
            b.wrapping_add(0x14),
            h32(list, b.wrapping_add(0x14)).wrapping_add(1),
        );
        if h32(list, b.wrapping_add(4)) == 0 {
            wh32(list, b.wrapping_add(0xC), 0);
            wh32(list, b.wrapping_add(4), node);
        }
        let tail = h32(list, b.wrapping_add(0xC));
        if tail != 0 {
            wh32(tail, b.wrapping_add(8), node);
        }
        wh32(node, b.wrapping_add(8), 0);
        wh32(list, b.wrapping_add(0xC), node);
    }

    let mut node = h32(list, HEAD_NEXT);
    if node == 0 {
        return this;
    }
    loop {
        let next = h32(node, N_NEXT);
        if h8(node, N_FLAG_A) != 0 {
            lf_checker_rt::callee_thiscall!(CAL_UPDATE, u32, this, node, 0, ctx);
        }
        if h8(node, N_FLAG_B) != 0 {
            lf_checker_rt::callee_thiscall!(CAL_UPDATE, u32, this, node, 1, ctx);
        }
        let limit = gf(LIMIT_VA);
        let key = hf(node, N_KEY);
        // The original compares with comiss and branches on below-or-unordered;
        // `!(limit >= key)` is true in exactly those cases, NaN included.
        if !(limit >= key) {
            release_slot(node, N_CHILD_A);
            release_slot(node, N_CHILD_B);
            unlink_and_free(this, list, node, next);
        } else {
            update_slot(node, N_CHILD_A);
            update_slot(node, N_CHILD_B);
            bucket_append(list, node);
        }
        node = next;
        if node == 0 {
            break;
        }
    }
    this
});
