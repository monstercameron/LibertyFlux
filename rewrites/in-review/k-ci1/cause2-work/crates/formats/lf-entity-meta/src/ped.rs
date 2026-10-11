//! Ped models: the shared animation rig plus per-component drawables.
//!
//! A ped ships as three files with the same base name: a fragment (`.wft`)
//! carrying the 80-bone animation rig, a drawable dictionary (`.wdd`) with
//! one drawable per body component, and a texture dictionary (`.wtd`).
//! Props (hats, glasses) ship as `.wdd` + `.wtd` pairs with no fragment.

use crate::classify::BoneKind;
use crate::classify::classify_bone;

/// How the toe bones are spelled in this rig.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToeStyle {
    /// `Char_L_Toe0` / `Char_R_Toe0` (most peds).
    Numbered,
    /// `Char_L_Toe` / `Char_R_Toe`.
    Plain,
    /// Neither spelling found.
    Other,
}

/// The animation rig inside a ped fragment.
#[derive(Debug, Clone)]
pub struct PedRig {
    /// Bones in the fragment skeleton.
    pub bone_count: usize,
    /// True when every bone classifies as rig and the count is 80.
    pub is_standard: bool,
    /// Toe spelling used.
    pub toes: ToeStyle,
    /// Fragment child records (ragdoll parts) in the file.
    pub child_count: usize,
}

impl PedRig {
    /// Describe the rig inside a parsed ped fragment.
    #[must_use]
    pub fn from_fragment(frag: &lf_model::Fragment) -> PedRig {
        let bones: &[lf_model::skeleton::Bone] = frag
            .drawable
            .skeleton
            .as_ref()
            .map_or(&[], |s| s.bones.as_slice());
        let all_rig = bones
            .iter()
            .all(|b| classify_bone(&b.name) == BoneKind::PedRig);
        let has = |n: &str| bones.iter().any(|b| b.name == n);
        let toes = if has("Char_L_Toe0") || has("Char_R_Toe0") {
            ToeStyle::Numbered
        } else if has("Char_L_Toe") || has("Char_R_Toe") {
            ToeStyle::Plain
        } else {
            ToeStyle::Other
        };
        PedRig {
            bone_count: bones.len(),
            is_standard: bones.len() == 80 && all_rig,
            toes,
            child_count: frag.children.len(),
        }
    }
}

/// One body component: one entry of the ped's drawable dictionary.
#[derive(Debug, Clone)]
pub struct Component {
    /// Name hash from the dictionary (algorithm unknown).
    pub hash: u32,
    /// LOD groups present in the drawable.
    pub lods: usize,
    /// Models across all LODs.
    pub models: usize,
    /// Vertices across all LODs.
    pub vertices: u32,
    /// Indices across all LODs.
    pub indices: u32,
}

impl Component {
    /// Describe one dictionary entry.
    #[must_use]
    pub fn from_drawable(hash: u32, draw: &lf_model::Drawable) -> Component {
        Component {
            hash,
            lods: draw.lods.len(),
            models: draw.models().count(),
            vertices: draw.vertex_count(),
            indices: draw.index_count(),
        }
    }
}

/// One ped model: rig plus components.
#[derive(Debug, Clone)]
pub struct PedModel {
    /// The animation rig from the `.wft`.
    pub rig: PedRig,
    /// Body components from the `.wdd`, in entry order. Empty for props
    /// parsed without a dictionary and when no dictionary was given.
    pub components: Vec<Component>,
}

impl PedModel {
    /// Build a ped from its parsed fragment and optional dictionary.
    pub fn from_parts(
        frag: &lf_model::Fragment,
        dict: Option<&lf_model::DrawableDictionary>,
    ) -> PedModel {
        let components = dict.map(components_of).unwrap_or_default();
        PedModel {
            rig: PedRig::from_fragment(frag),
            components,
        }
    }

    /// Number of body components.
    #[must_use]
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Iterate over the components.
    pub fn iter(&self) -> impl Iterator<Item = &Component> {
        self.components.iter()
    }
}

/// Describe every entry of a drawable dictionary (peds and props alike).
#[must_use]
pub fn components_of(dict: &lf_model::DrawableDictionary) -> Vec<Component> {
    dict.entries
        .iter()
        .enumerate()
        .map(|(i, d)| Component::from_drawable(dict.hashes.get(i).copied().unwrap_or(0), d))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fragment_with(names: &[&str]) -> lf_model::Fragment {
        let bones = names
            .iter()
            .enumerate()
            .map(|(i, n)| lf_model::skeleton::Bone {
                offset: 0,
                name: n.to_string(),
                index: i16::try_from(i).expect("fixture fits"),
                bone_id: i16::try_from(i).expect("fixture fits"),
                dofs: 0,
                position: [0.0; 4],
                rotation_euler: [0.0; 4],
                rotation_quat: [0.0; 4],
                parent: 0,
                first_child: 0,
                next_sibling: 0,
            })
            .collect();
        lf_model::Fragment {
            drawable: lf_model::Drawable {
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
            },
            children: vec![],
        }
    }

    #[test]
    fn toe_styles() {
        let numbered = fragment_with(&["Char", "Char_L_Toe0", "Char_R_Toe0"]);
        assert_eq!(PedRig::from_fragment(&numbered).toes, ToeStyle::Numbered);
        let plain = fragment_with(&["Char", "Char_L_Toe", "Char_R_Toe"]);
        assert_eq!(PedRig::from_fragment(&plain).toes, ToeStyle::Plain);
        let other = fragment_with(&["Char", "Char_Pelvis"]);
        assert_eq!(PedRig::from_fragment(&other).toes, ToeStyle::Other);
    }

    #[test]
    fn standard_needs_80_rig_bones() {
        let small = fragment_with(&["Char", "Char_Pelvis"]);
        let rig = PedRig::from_fragment(&small);
        assert!(!rig.is_standard);
        assert_eq!(rig.bone_count, 2);
        let mixed = fragment_with(&["Char", "seat_dside_f"]);
        assert!(!PedRig::from_fragment(&mixed).is_standard);
    }

    #[test]
    fn components_pair_hashes_with_drawables() {
        let dict = lf_model::DrawableDictionary {
            vtable: 0,
            usage_count: 1,
            hashes: vec![0x1111_1111, 0x2222_2222],
            entries: vec![
                lf_model::Drawable {
                    vtable: 0,
                    shaders: None,
                    skeleton: None,
                    center: [0.0; 4],
                    bounds_min: [0.0; 4],
                    bounds_max: [0.0; 4],
                    abs_max: [0.0; 4],
                    lods: vec![],
                },
                lf_model::Drawable {
                    vtable: 0,
                    shaders: None,
                    skeleton: None,
                    center: [0.0; 4],
                    bounds_min: [0.0; 4],
                    bounds_max: [0.0; 4],
                    abs_max: [0.0; 4],
                    lods: vec![],
                },
            ],
        };
        let comps = components_of(&dict);
        assert_eq!(comps.len(), 2);
        assert_eq!(comps[0].hash, 0x1111_1111);
        assert_eq!(comps[1].hash, 0x2222_2222);
    }
}
