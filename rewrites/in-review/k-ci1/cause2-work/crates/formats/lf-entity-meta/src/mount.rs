//! A classified attachment point: a bone name plus its positions.
//!
//! [`Mount`] is shared by the vehicle, ped and weapon readers. It borrows
//! nothing: names are owned strings so layouts can outlive the parsed model.

/// One attachment point on a model.
#[derive(Debug, Clone)]
pub struct Mount {
    /// Bone name as stored in the file.
    pub name: String,
    /// Index of the bone in its skeleton.
    pub bone: usize,
    /// Local position (x, y, z) from the bone record.
    pub local: [f32; 3],
    /// World position (x, y, z) from the skeleton's global transforms,
    /// or `None` when the file stores no global pose.
    pub world: Option<[f32; 3]>,
}

impl Mount {
    /// Build a mount from a bone index, local position and optional world.
    #[must_use]
    pub fn new(name: &str, bone: usize, local: [f32; 3], world: Option<[f32; 3]>) -> Mount {
        Mount {
            name: name.to_string(),
            bone,
            local,
            world,
        }
    }

    /// Best available position: world when known, else local.
    #[must_use]
    pub fn position(&self) -> [f32; 3] {
        self.world.unwrap_or(self.local)
    }
}
