const BANK_WORDS: usize = 64;
const BANK_STRIDE: u32 = 0x100;
const SECURITY_COOKIE_VA: u32 = 0x01057fb4;

#[inline(always)]
unsafe fn copy_bank(owner: u32) -> [u32; BANK_WORDS] {
    let mut snapshot = [0u32; BANK_WORDS];
    for (index, word) in snapshot.iter_mut().enumerate() {
        *word = unsafe { ((owner as *const u8).add(index * 4) as *const u32).read_unaligned() };
    }
    snapshot
}

#[inline(always)]
fn resync_items(owner: u32, snapshot: &[u32; BANK_WORDS], item_count: u32) {
    for item_index in 0..item_count {
        let item_id = lf_checker_rt::callee_thiscall!(
            5,
            u32,
            snapshot.as_ptr() as u32,
            item_index
        );
        let use_primary_bank = lf_checker_rt::callee_thiscall!(6, u8, owner, item_id) != 0;
        let destination = if use_primary_bank {
            owner
        } else {
            owner.wrapping_add(BANK_STRIDE)
        };
        let _ = lf_checker_rt::callee_thiscall!(7, (), destination, item_id);
    }
}

/// Copies both 64-entry banks before clearing their live arrays, then replays
/// each copied bank through the count, item lookup, route predicate and insert
/// helpers. The contract checks every copied word, all direct call arguments,
/// both route outcomes, zero/one/two-item loop counts and the helper writes that
/// clear and refill the live banks. Helper internals remain separately scoped.
lf_checker_rt::export!(thiscall, rw_00c69970(this: u32) -> () {
    unsafe {
        let first_snapshot = copy_bank(this);
        let second_snapshot = copy_bank(this.wrapping_add(BANK_STRIDE));

        let _ = lf_checker_rt::callee_thiscall!(1, (), this);
        let _ = lf_checker_rt::callee_thiscall!(2, (), this.wrapping_add(BANK_STRIDE));

        let first_count = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            first_snapshot.as_ptr() as u32
        );
        resync_items(this, &first_snapshot, first_count);

        let second_count = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            second_snapshot.as_ptr() as u32
        );
        resync_items(this, &second_snapshot, second_count);

        let security_cookie = lf_checker_rt::global::<u32>(SECURITY_COOKIE_VA).read_unaligned();
        let _ = lf_checker_rt::callee_thiscall!(8, (), security_cookie);
    }
});
