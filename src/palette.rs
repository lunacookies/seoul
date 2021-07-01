use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), 0.0, 0.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.89870447, 0.18559207, 97.86518)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.49750343, 0.08471402, 196.96284)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    DarkestBg,
    DarkerBg,
    DarkBg,
    Bg,
    LightBg,
    LighterBg,
    DarkerFg,
    DarkFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.35..1.0)
    }

    fn value(self) -> f32 {
        match self {
            Self::DarkestBg => 0.0,
            Self::DarkerBg => 0.04,
            Self::DarkBg => 0.06,
            Self::Bg => 0.1,
            Self::LightBg => 0.13,
            Self::LighterBg => 0.3,
            Self::DarkerFg => 0.5,
            Self::DarkFg => 0.6,
            Self::Fg => 0.85,
            Self::BrightFg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
