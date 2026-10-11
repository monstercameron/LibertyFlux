//! Weapon mounts: grip, muzzle, ejector and magazine bones.
//!
//! Firearm drawables name one bone per mount point; the game attaches the
//! ped's hands, spawns muzzle flash and shells, and animates the magazine
//! from these positions. Melee weapons and ambient props usually carry a
//! single root bone and no mounts.

use crate::mount::Mount;

/// Mount points of one weapon or prop drawable.
#[derive(Debug, Clone, Default)]
pub struct WeaponMounts {
    /// Root bone: the first bone that is not a known mount, if any.
    pub root: Option<Mount>,
    /// `gun_grip`: where the hand holds the weapon.
    pub grip: Option<Mount>,
    /// `gun_grip2`: the second hand hold.
    pub grip2: Option<Mount>,
    /// `gun_muzzle`: where projectiles and flash spawn.
    pub muzzle: Option<Mount>,
    /// `gun_ejac` (one file: `gun_ejac01`): shell ejection.
    pub ejector: Option<Mount>,
    /// `Mag` (one file: `mag`): the magazine.
    pub magazine: Option<Mount>,
    /// `sight_adjust`, when present.
    pub sight: Option<Mount>,
    /// Total bones in the drawable skeleton (0 when absent).
    pub bone_count: usize,
}

impl WeaponMounts {
    /// Build mounts from a parsed weapon drawable.
    #[must_use]
    pub fn from_drawable(draw: &lf_model::Drawable) -> WeaponMounts {
        Self::from_skeleton(draw.skeleton.as_ref())
    }

    /// Build mounts from a parsed fragment (the two breakable prop bags
    /// ship as fragments, not drawables).
    #[must_use]
    pub fn from_fragment(frag: &lf_model::Fragment) -> WeaponMounts {
        Self::from_skeleton(frag.drawable.skeleton.as_ref())
    }

    fn from_skeleton(skel: Option<&lf_model::Skeleton>) -> WeaponMounts {
        let mut out = WeaponMounts::default();
        let Some(skel) = skel else {
            return out;
        };
        out.bone_count = skel.bones.len();
        for (i, bone) in skel.bones.iter().enumerate() {
            let local = [bone.position[0], bone.position[1], bone.position[2]];
            let world = skel.global_pose.get(i).map(|m| [m[3][0], m[3][1], m[3][2]]);
            let mount = Mount::new(&bone.name, i, local, world);
            match bone.name.as_str() {
                "gun_grip" => out.grip = Some(mount),
                "gun_grip2" => out.grip2 = Some(mount),
                "gun_muzzle" => out.muzzle = Some(mount),
                "gun_ejac" | "gun_ejac01" => out.ejector = Some(mount),
                "Mag" | "mag" => out.magazine = Some(mount),
                "sight_adjust" => out.sight = Some(mount),
                _ => {
                    if out.root.is_none() {
                        out.root = Some(mount);
                    }
                }
            }
        }
        out
    }

    /// True when the drawable has a muzzle or a second grip: the mark of
    /// a firearm as opposed to a melee weapon or prop.
    #[must_use]
    pub fn is_firearm(&self) -> bool {
        self.muzzle.is_some() || self.grip2.is_some()
    }

    /// Iterate over the mounts present (grip, grip2, muzzle, ejector,
    /// magazine, sight), skipping the root.
    pub fn mounts(&self) -> impl Iterator<Item = &Mount> {
        self.grip
            .iter()
            .chain(self.grip2.iter())
            .chain(self.muzzle.iter())
            .chain(self.ejector.iter())
            .chain(self.magazine.iter())
            .chain(self.sight.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drawable_with(names: &[&str]) -> lf_model::Drawable {
        let bones = names
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let idx = i16::try_from(i).expect("fixture fits");
                lf_model::skeleton::Bone {
                    offset: 0,
                    name: n.to_string(),
                    index: idx,
                    bone_id: idx,
                    dofs: 0,
                    position: [f32::from(idx), 0.0, 0.0, 1.0],
                    rotation_euler: [0.0; 4],
                    rotation_quat: [0.0; 4],
                    parent: 0,
                    first_child: 0,
                    next_sibling: 0,
                }
            })
            .collect();
        lf_model::Drawable {
            vtable: 0,
            shaders: None,
            skeleton: Some(lf_model::Skeleton {
                bones,
                parents: vec![],
                default_pose: vec![],
                inverse_pose: vec![],
                global_pose: vec![],
                id_mappings: vec![],
            }),
            center: [0.0; 4],
            bounds_min: [0.0; 4],
            bounds_max: [0.0; 4],
            abs_max: [0.0; 4],
            lods: vec![],
        }
    }

    #[test]
    // Fixture test: the hand-built positions must pass through exactly.
    #[allow(clippy::float_cmp)]
    fn firearm_mounts() {
        let draw = drawable_with(&[
            "W_RIFLE",
            "gun_grip",
            "gun_grip2",
            "gun_muzzle",
            "gun_ejac",
            "Mag",
        ]);
        let w = WeaponMounts::from_drawable(&draw);
        assert!(w.is_firearm());
        assert_eq!(w.root.as_ref().unwrap().name, "W_RIFLE");
        assert_eq!(w.mounts().count(), 5);
        assert_eq!(w.muzzle.as_ref().unwrap().local, [3.0, 0.0, 0.0]);
    }

    #[test]
    fn alternate_spellings() {
        let draw = drawable_with(&["W_PSG1", "gun_ejac01", "mag", "sight_adjust"]);
        let w = WeaponMounts::from_drawable(&draw);
        assert!(w.ejector.is_some());
        assert!(w.magazine.is_some());
        assert!(w.sight.is_some());
        assert!(!w.is_firearm());
    }

    #[test]
    fn prop_has_root_only() {
        let draw = drawable_with(&["AMB_broom"]);
        let w = WeaponMounts::from_drawable(&draw);
        assert!(!w.is_firearm());
        assert_eq!(w.mounts().count(), 0);
        assert_eq!(w.root.as_ref().unwrap().name, "AMB_broom");
    }
}
