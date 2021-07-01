use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), 0.0, 0.0)
    }

    pub(crate) fn pink(&self) -> Oklch {
        oklch(0.86225134, 0.074080676, 20.706781)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.8098629, 0.10114792, 84.903305)
    }

    pub(crate) fn light_orange(&self) -> Oklch {
        oklch(0.912709, 0.09400704, 85.074005)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.89870447, 0.18559207, 97.86518)
    }

    pub(crate) fn cream(&self) -> Oklch {
        oklch(0.89280736, 0.043737248, 105.87018)
    }

    pub(crate) fn dark_yellow(&self) -> Oklch {
        oklch(0.7771238, 0.094686694, 107.07088)
    }

    pub(crate) fn light_yellow(&self) -> Oklch {
        oklch(0.88245827, 0.08747428, 107.37559)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.7591353, 0.062844805, 145.6965)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(0.49750343, 0.08471402, 196.96284)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.7692352, 0.038741853, 198.56277)
    }

    pub(crate) fn strong_cyan(&self) -> Oklch {
        oklch(0.74603397, 0.07639755, 196.76222)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.7849458, 0.061410464, 244.10135)
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
