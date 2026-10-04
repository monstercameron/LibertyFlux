//! 5-byte jump hooks with trampolines.
//!
//! A hook overwrites the first bytes of a function with `E9 <rel32>` to the
//! detour. The overwritten bytes (whole instructions, at least 5) are copied
//! to a trampoline that ends with a jump back, so the detour can call the
//! original. Relative branches inside the moved range are rewritten to reach
//! the same absolute targets from the trampoline.

// A hook handle crosses threads only while held under the patch lock;
// the `unsafe impl Send` below carries that argument.
#![allow(unsafe_code)]

use crate::decode::{BranchKind, decode};
use crate::mem;
use std::sync::Mutex;

static PATCH_LOCK: Mutex<()> = Mutex::new(());

/// Maximum bytes ever moved into a trampoline.
pub const MAX_MOVED: usize = 16;
/// Size of the `E9 rel32` patch.
pub const PATCH_LEN: usize = 5;

/// Hook creation/installation failure. Inspecting bytes never patches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HookError {
    /// Target memory is not readable.
    UnreadableTarget,
    /// Could not decode instructions at the target.
    DecodeFailed,
    /// Function returns before 5 patchable bytes.
    TooShort,
    /// Needs more than [`MAX_MOVED`] moved bytes.
    TooLong,
    /// Unrelocatable instruction in the patch range.
    Unmovable,
    /// Detour is out of rel32 range from the patch site.
    DetourOutOfRange,
    /// Trampoline allocation failed (Win32 error).
    AllocFailed(u32),
    /// Memory patch failed (Win32 error).
    PatchFailed(u32),
}

impl std::fmt::Display for HookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HookError::UnreadableTarget => write!(f, "target memory is not readable"),
            HookError::DecodeFailed => write!(f, "could not decode instructions at target"),
            HookError::TooShort => write!(f, "function returns before 5 patchable bytes"),
            HookError::TooLong => write!(f, "needs more than 16 moved bytes"),
            HookError::Unmovable => write!(f, "unrelocatable instruction in patch range"),
            HookError::DetourOutOfRange => write!(f, "detour is out of rel32 range"),
            HookError::AllocFailed(e) => write!(f, "trampoline alloc failed ({e})"),
            HookError::PatchFailed(e) => write!(f, "memory patch failed ({e})"),
        }
    }
}

/// Whether `create` follows leading thunk jumps to the real body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FollowJumps {
    /// Follow up to 4 leading `E9`/`FF 25` jumps before patching.
    On,
    /// Patch the given address as-is.
    Off,
}

struct PlannedInsn {
    /// Offset within the moved range.
    off: usize,
    len: usize,
    branch: BranchKind,
    /// Signed displacement of a relative branch (0 for non-branches).
    rel: i64,
}

fn plan_overwrite(code: &[u8]) -> Result<Vec<PlannedInsn>, HookError> {
    let mut plans = Vec::new();
    let mut off = 0usize;
    while off < PATCH_LEN {
        let d = decode(&code[off..]).ok_or(HookError::DecodeFailed)?;
        if d.is_ret {
            return Err(HookError::TooShort);
        }
        match d.branch {
            BranchKind::Loop => return Err(HookError::Unmovable),
            BranchKind::RelFull | BranchKind::Rel8 if d.rel_size == 2 => {
                // 16-bit relative branch: refuse rather than widen.
                return Err(HookError::Unmovable);
            }
            _ => {}
        }
        let len = d.len as usize;
        let rel = match d.branch {
            BranchKind::Rel8 => i64::from(code[off + 1] as i8),
            BranchKind::RelFull => i64::from(i32::from_le_bytes([
                code[off + len - 4],
                code[off + len - 3],
                code[off + len - 2],
                code[off + len - 1],
            ])),
            _ => 0,
        };
        plans.push(PlannedInsn {
            off,
            len,
            branch: d.branch,
            rel,
        });
        off += len;
        if off > MAX_MOVED {
            return Err(HookError::TooLong);
        }
    }
    Ok(plans)
}

/// Resolve `E9 rel32` / `FF 25 [disp32]` chains to the final target.
fn follow_jumps(mut target: usize) -> Result<usize, HookError> {
    for _ in 0..4 {
        let code = mem::read_bytes(target, 6).ok_or(HookError::UnreadableTarget)?;
        if code[0] == 0xE9 {
            let rel = i32::from_le_bytes([code[1], code[2], code[3], code[4]]) as isize;
            target = target.wrapping_add(5).wrapping_add(rel as usize);
            continue;
        }
        if code[0] == 0xFF && code[1] == 0x25 {
            let slot = u32::from_le_bytes([code[2], code[3], code[4], code[5]]) as usize;
            let ptr = mem::read_bytes(slot, 4).ok_or(HookError::UnreadableTarget)?;
            target = u32::from_le_bytes([ptr[0], ptr[1], ptr[2], ptr[3]]) as usize;
            continue;
        }
        break;
    }
    Ok(target)
}

fn rel32_bytes(from_end: usize, to: usize) -> Option<[u8; 4]> {
    let delta = (to as i64).wrapping_sub(from_end as i64);
    if (-0x8000_0000..0x8000_0000).contains(&delta) {
        Some((delta as i32).to_le_bytes())
    } else {
        None
    }
}

/// Build trampoline bytes: relocated copies of the moved instructions plus a
/// `push back; ret` tail. `plans` offsets are relative to `target`.
fn build_trampoline(target: usize, code: &[u8], plans: &[PlannedInsn], tramp_at: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for p in plans {
        let src = &code[p.off..p.off + p.len];
        // Absolute target of this branch at its original address.
        let abs = target
            .wrapping_add(p.off)
            .wrapping_add(p.len)
            .wrapping_add(p.rel as usize);
        match p.branch {
            BranchKind::None | BranchKind::Loop => {
                out.extend_from_slice(src);
            }
            BranchKind::Rel8 => {
                // Widen to a 32-bit branch. rel8 Jcc (70..7F) becomes
                // 0F 8x rel32; EB becomes E9 rel32.
                if src[0] == 0xEB {
                    let rel = rel32_bytes(tramp_at + out.len() + 5, abs).expect("rel32 range");
                    out.push(0xE9);
                    out.extend_from_slice(&rel);
                } else {
                    let cc = src[0] & 0x0F;
                    let rel = rel32_bytes(tramp_at + out.len() + 6, abs).expect("rel32 range");
                    out.push(0x0F);
                    out.push(0x80 | cc);
                    out.extend_from_slice(&rel);
                }
            }
            BranchKind::RelFull => {
                if src[0] == 0x0F {
                    out.push(0x0F);
                    out.push(src[1]);
                    let rel = rel32_bytes(tramp_at + out.len() + 4, abs).expect("rel32 range");
                    out.extend_from_slice(&rel);
                } else {
                    out.push(src[0]); // E8 or E9
                    let rel = rel32_bytes(tramp_at + out.len() + 4, abs).expect("rel32 range");
                    out.extend_from_slice(&rel);
                }
            }
        }
    }
    // Absolute tail: push back_addr; ret. Works at any distance.
    let back = (target + plans.iter().map(|p| p.len).sum::<usize>()) as u32;
    out.push(0x68);
    out.extend_from_slice(&back.to_le_bytes());
    out.push(0xC3);
    out
}

/// An installed-or-not 5-byte hook. Created disabled; `enable` patches.
pub struct Detour {
    target: usize,
    detour: usize,
    trampoline: *mut u8,
    moved: usize,
    saved: [u8; MAX_MOVED],
    patch: [u8; PATCH_LEN],
    enabled: bool,
}

// The raw pointer only crosses threads under PATCH_LOCK / registry locks.
unsafe impl Send for Detour {}

impl Detour {
    /// Prepare a hook: resolve jump thunks, decode, build the trampoline.
    /// No target bytes are touched until `enable`.
    pub fn create(target: usize, detour: usize, follow: FollowJumps) -> Result<Self, HookError> {
        let t = match follow {
            FollowJumps::On => follow_jumps(target)?,
            FollowJumps::Off => target,
        };
        let code = mem::read_bytes(t, MAX_MOVED).ok_or(HookError::UnreadableTarget)?;
        let plans = plan_overwrite(&code)?;
        let moved: usize = plans.iter().map(|p| p.len).sum();
        // Detour must be reachable by rel32 from the patch site.
        let rel = rel32_bytes(t + PATCH_LEN, detour).ok_or(HookError::DetourOutOfRange)?;
        let mut patch = [0u8; PATCH_LEN];
        patch[0] = 0xE9;
        patch[1..].copy_from_slice(&rel);

        // Trampoline size: moved bytes + worst-case widening (each Rel8 grows
        // by 4) + 6-byte tail.
        let extra = plans
            .iter()
            .filter(|p| p.branch == BranchKind::Rel8)
            .count()
            * 4;
        let tramp_len = moved + extra + 6;
        let tramp = mem::alloc_exec(tramp_len).map_err(HookError::AllocFailed)?;
        let bytes = build_trampoline(t, &code, &plans, tramp as usize);
        debug_assert!(bytes.len() == tramp_len);
        // SAFETY: `tramp` is a fresh `tramp_len`-byte allocation.
        unsafe { mem::write_exec(tramp, &bytes) };

        let mut saved = [0u8; MAX_MOVED];
        saved[..moved].copy_from_slice(&code[..moved]);
        Ok(Detour {
            target: t,
            detour,
            trampoline: tramp,
            moved,
            saved,
            patch,
            enabled: false,
        })
    }

    /// Patched address (after thunk following, if enabled).
    #[must_use]
    pub fn target(&self) -> usize {
        self.target
    }

    /// Address the patch jumps to.
    #[must_use]
    pub fn detour(&self) -> usize {
        self.detour
    }

    /// Callable original. Cast to the function's real type at the call site.
    #[must_use]
    pub fn trampoline(&self) -> usize {
        self.trampoline as usize
    }

    /// Bytes moved into the trampoline (5..=16).
    #[must_use]
    pub fn moved_len(&self) -> usize {
        self.moved
    }

    /// Whether the patch is currently written to the target.
    #[must_use]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Patch the target. Suspends other threads across the write.
    pub fn enable(&mut self) -> Result<usize, HookError> {
        let _lock = PATCH_LOCK.lock().unwrap();
        if self.enabled {
            return Ok(0);
        }
        let freeze = mem::FreezeGuard::suspend_others();
        let frozen = freeze.frozen();
        let r = mem::patch_memory(self.target, &self.patch);
        drop(freeze);
        r.map_err(HookError::PatchFailed)?;
        self.enabled = true;
        Ok(frozen)
    }

    /// Restore the original bytes.
    pub fn disable(&mut self) -> Result<usize, HookError> {
        let _lock = PATCH_LOCK.lock().unwrap();
        if !self.enabled {
            return Ok(0);
        }
        let freeze = mem::FreezeGuard::suspend_others();
        let frozen = freeze.frozen();
        let r = mem::patch_memory(self.target, &self.saved[..self.moved]);
        drop(freeze);
        r.map_err(HookError::PatchFailed)?;
        self.enabled = false;
        Ok(frozen)
    }

    /// True when the target currently holds the expected bytes.
    #[must_use]
    pub fn verify(&self) -> bool {
        let want: &[u8] = if self.enabled {
            &self.patch
        } else {
            &self.saved[..self.moved]
        };
        match mem::read_bytes(self.target, want.len()) {
            Some(cur) => cur == want,
            None => false,
        }
    }

    /// Re-apply the expected bytes (used by the integrity self-check).
    pub fn restore(&self) -> Result<(), HookError> {
        let _lock = PATCH_LOCK.lock().unwrap();
        let want: &[u8] = if self.enabled {
            &self.patch
        } else {
            &self.saved[..self.moved]
        };
        mem::patch_memory(self.target, want).map_err(HookError::PatchFailed)
    }
}

impl Drop for Detour {
    fn drop(&mut self) {
        // Best-effort unpatch; a failing restore is still better than a
        // dangling jump into freed trampoline memory... but only when the
        // target is still mapped (process teardown order is not guaranteed).
        if self.enabled && mem::is_readable(self.target, self.moved) {
            let _ = mem::patch_memory(self.target, &self.saved[..self.moved]);
            self.enabled = false;
        }
        if !self.trampoline.is_null() {
            mem::free_exec(self.trampoline);
            self.trampoline = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_counts_whole_instructions() {
        // nop;nop;nop;mov eax,imm32;ret  -> 1+1+1+5 = 8 moved bytes
        let code = [0x90, 0x90, 0x90, 0xB8, 1, 2, 3, 4, 0xC3];
        let plans = plan_overwrite(&code).unwrap();
        let total: usize = plans.iter().map(|p| p.len).sum();
        assert_eq!(total, 8);
    }

    #[test]
    fn plan_refuses_short_function() {
        // xor eax,eax;ret -> ret before 5 bytes
        let code = [0x33, 0xC0, 0xC3, 0x90, 0x90, 0x90];
        assert!(matches!(plan_overwrite(&code), Err(HookError::TooShort)));
    }

    #[test]
    fn rel32_range_check() {
        assert!(rel32_bytes(0x1000, 0x2000).is_some());
        assert!(rel32_bytes(0x1000, 0xFFFF_FFFF).is_none());
    }
}
