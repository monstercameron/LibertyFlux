// original: 0x00A8A8C0 pool_chain_search (proposed)

/// Search the 24-byte node chain for the first node the helpers take.
///
/// The head pointer (global) selects the start node by its `+0x12` word
/// (`0xFFFF` means the null node); the chain links through each node's
/// `+0x12` the same way and ends at the global end pointer. Each node's
/// index is its byte offset from the base (global) divided by 24, signed.
/// Nodes whose `+0xE` flags meet the `mask` argument are skipped; the rest
/// go to the probe helper (with the node index) and, on a zero low byte,
/// to the accept helper (with the same index). The first accept success
/// returns 1; running out of chain returns 0, as does a null head. Only
/// the low byte is set on the success path. Takes no object: the pushed
/// ECX slot is reused for the index before any read.
///
/// Original: stdcall, one stack word (mask), low byte in AL. Two
/// callees: probe and accept (thiscall, one stack word each, constant
/// relocated scope).
lf_checker_rt::export!(stdcall, rw_00A8A8C0(mask: u32) -> u32 {
    unsafe {
        const HEAD_GLOBAL: u32 = 0x103e8dc;
        const BASE_GLOBAL: u32 = 0x12fb3a8;
        const END_GLOBAL: u32 = 0x103e8d8;
        const SCOPE_FILE_VA: u32 = 0x103e8d0;
        const NEXT_OFF: u32 = 0x12;
        const FLAGS_OFF: u32 = 0xe;
        const NODE_SIZE: u32 = 24;
        const NO_NODE: u16 = 0xffff;
        const PROBE: u32 = 1;
        const ACCEPT: u32 = 2;
        let scope = lf_checker_rt::relocated(SCOPE_FILE_VA);
        let head = (lf_checker_rt::global::<u32>(HEAD_GLOBAL) as *const u32)
            .read_unaligned();
        if head == 0 {
            return 0;
        }
        let base = (lf_checker_rt::global::<u32>(BASE_GLOBAL) as *const u32)
            .read_unaligned();
        let start = ((head + NEXT_OFF) as *const u16).read_unaligned();
        let mut cur = if start == NO_NODE {
            0
        } else {
            base.wrapping_add((start as u32).wrapping_mul(NODE_SIZE))
        };
        let mut end = (lf_checker_rt::global::<u32>(END_GLOBAL) as *const u32)
            .read_unaligned();
        let mut base = base;
        if cur == end {
            return 0;
        }
        loop {
            let idx = (cur as i32)
                .wrapping_sub(base as i32)
                .wrapping_div(NODE_SIZE as i32);
            let next = ((cur + NEXT_OFF) as *const u16).read_unaligned();
            let nxt = if next == NO_NODE {
                0
            } else {
                base.wrapping_add((next as u32).wrapping_mul(NODE_SIZE))
            };
            let flags = ((cur + FLAGS_OFF) as *const u16).read_unaligned() as u32;
            if mask & flags == 0 {
                let arg = (idx as u16) as u32;
                let probe: u32 =
                    lf_checker_rt::callee_thiscall!(PROBE, u32, scope, arg);
                if probe as u8 == 0 {
                    let acc: u32 =
                        lf_checker_rt::callee_thiscall!(ACCEPT, u32, scope, arg);
                    if acc as u8 != 0 {
                        return 1;
                    }
                }
                end = (lf_checker_rt::global::<u32>(END_GLOBAL) as *const u32)
                    .read_unaligned();
                base = (lf_checker_rt::global::<u32>(BASE_GLOBAL) as *const u32)
                    .read_unaligned();
            }
            cur = nxt;
            if cur == end {
                return 0;
            }
        }
    }
});
