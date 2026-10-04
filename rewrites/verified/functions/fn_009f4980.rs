// original: 0x009f4980 net_stage_03
/// Network stage 3 handler: rate the local object twice, report, stamp, register.
///
/// Fetches an object, rates it through a thiscall check (passing the object's
/// word at +0x228 biased by +0x70, or null when that word is zero), and bumps
/// the rating. Ratings below 6 (signed) trigger a second fetch-and-rate round
/// whose bumped value is used instead; otherwise the rating clamps to 6. The
/// final value goes to the reporter together with a third fetched object, then
/// the shared tick is stamped into stage 3's slot and the stage registers.
/// Returns the registrar's answer.
export!(cdecl, rw_009f4980() -> u32 {{
    unsafe {{
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B61DC;
        /// Stage index passed to the registrar.
        const INDEX: u32 = 3;
        /// Clamp for the bumped rating (the original compares signed).
        const CLAMP: i32 = 6;
        let _: u32 = callee_cdecl!(1, u32,);
        let first: u32 = callee_cdecl!(2, u32, 0);
        let word: u32 = ((first + 0x228) as *const u32).read();
        let rated: u32 = if word == 0 {{
            callee_thiscall!(3, u32, 0)
        }} else {{
            callee_thiscall!(3, u32, word.wrapping_add(0x70))
        }};
        let bumped: u32 = rated.wrapping_add(1);
        let level: u32 = if (bumped as i32) >= CLAMP {{
            CLAMP as u32
        }} else {{
            let second: u32 = callee_cdecl!(2, u32, 0);
            let word2: u32 = ((second + 0x228) as *const u32).read();
            let rated2: u32 = if word2 == 0 {{
                callee_thiscall!(3, u32, 0)
            }} else {{
                callee_thiscall!(3, u32, word2.wrapping_add(0x70))
            }};
            rated2.wrapping_add(1)
        }};
        let target: u32 = callee_cdecl!(2, u32, 0);
        let _: u32 = callee_thiscall!(4, u32, target, level);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(5, u32, INDEX)
    }}
}});
