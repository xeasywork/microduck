//! Small pet expressions that do not need a dedicated ONNX network.
//!
//! They are smooth command trajectories fed through the standing policy.  This keeps
//! them inside the same observation, joint-limit and safety path as manual head/body
//! commands; no expression ever writes a joint target directly.

use duck_ipc_proto::{Skill, SoundTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    HeadTilt,
    CuriousScan,
    Greet,
    InvitePlay,
}

impl Kind {
    fn of(skill: Skill) -> Option<Self> {
        match skill {
            Skill::HeadTilt => Some(Self::HeadTilt),
            Skill::CuriousScan => Some(Self::CuriousScan),
            Skill::Greet => Some(Self::Greet),
            Skill::InvitePlay => Some(Self::InvitePlay),
            _ => None,
        }
    }

    fn duration(self) -> f64 {
        match self {
            Self::HeadTilt => 3.0,
            Self::CuriousScan => 6.0,
            Self::Greet => 3.2,
            Self::InvitePlay => 4.5,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::HeadTilt => "head_tilt",
            Self::CuriousScan => "curious_scan",
            Self::Greet => "greet",
            Self::InvitePlay => "invite_play",
        }
    }

    fn sound(self) -> Option<SoundTag> {
        match self {
            Self::Greet => Some(SoundTag::Greet),
            Self::InvitePlay => Some(SoundTag::Inquire),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    /// neck_pitch, head_pitch, head_yaw, head_roll.
    pub head: [f64; 4],
    /// z, roll, pitch offsets for the standing policy.
    pub body: [f64; 3],
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct PetExpression {
    kind: Kind,
    elapsed: f64,
}

impl PetExpression {
    pub fn start(skill: Skill) -> Option<(Self, Option<SoundTag>)> {
        let kind = Kind::of(skill)?;
        Some((Self { kind, elapsed: 0.0 }, kind.sound()))
    }

    /// Advance once and return this tick's smooth command. `None` means complete.
    pub fn tick(&mut self, dt: f64) -> Option<Frame> {
        self.elapsed += dt.max(0.0);
        let duration = self.kind.duration();
        if self.elapsed >= duration {
            return None;
        }

        let phase = (self.elapsed / duration).clamp(0.0, 1.0);
        // Zero at both ends. Multiplying every pose by this envelope means an
        // interrupt or natural completion never leaves a discontinuous final target.
        let envelope = (std::f64::consts::PI * phase).sin();
        let wave = (std::f64::consts::TAU * phase).sin();
        let (head, body) = match self.kind {
            Kind::HeadTilt => ([0.0, -0.08 * envelope, 0.0, 0.22 * envelope], [0.0; 3]),
            Kind::CuriousScan => ([0.0, -0.04 * envelope, 0.48 * wave, 0.04 * wave], [0.0; 3]),
            Kind::Greet => (
                [0.0, -0.10 * envelope, 0.16 * wave, 0.10 * wave],
                [-0.006 * envelope, 0.0, 0.04 * envelope],
            ),
            Kind::InvitePlay => (
                [0.0, 0.12 * envelope, 0.20 * wave, -0.08 * wave],
                [-0.012 * envelope, 0.04 * wave, -0.05 * envelope],
            ),
        };
        Some(Frame {
            head,
            body,
            label: self.kind.label(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_starts_and_returns_to_neutral() {
        let (mut expression, sound) = PetExpression::start(Skill::Greet).unwrap();
        assert_eq!(sound, Some(SoundTag::Greet));
        let first = expression.tick(0.001).unwrap();
        assert!(first.head.iter().all(|value| value.abs() < 0.001));
        assert_eq!(first.label, "greet");
        assert!(expression.tick(3.2).is_none());
    }

    #[test]
    fn trained_skills_are_not_mistaken_for_expressions() {
        assert!(PetExpression::start(Skill::KickLeft).is_none());
    }
}
