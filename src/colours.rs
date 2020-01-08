const TERM: &str = "\x1b[0m";

trait Painter {
    fn unescaped(&self) -> String;
    fn escaped(&self) -> String {
        colour_escape(&self.unescaped())
    }
    fn paint(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", self.escaped(), s)
    }
    fn start(&self) {
        print!("{}", self.escaped());
    }
    fn reset() {
        print!("{}", TERM);
    }
}

fn colour_escape(s: &str) -> String {
    format!("\x1b[{}m", s)
}

pub struct RGB(u8, u8, u8);

impl RGB {
    fn unescaped(&self, plane: Plane) -> String {
        format!("{};2;{};{};{}", plane as u8, self.0, self.1, self.2)
    }
}

struct FGColour(RGB);

impl Painter for FGColour {
    fn unescaped(&self) -> String {
        self.0.unescaped(Plane::FG)
    }

    fn reset() {
        print!("\x1b[39m");
    }
}

struct BGColour(RGB);

impl Painter for BGColour {
    fn unescaped(&self) -> String {
        self.0.unescaped(Plane::BG)
    }

    fn reset() {
        print!("\x1b[49m");
    }
}

pub struct Colour{
    fg: Option<FGColour>, 
    bg: Option<BGColour>,
}

impl Colour {
    pub fn fg(rgb: RGB) -> Self {
        Colour{
            fg: Some(FGColour(rgb)),
            bg: None
        }
    }

    pub fn bg(rgb: RGB) -> Self {
        Colour{
            fg: None,
            bg: Some(BGColour(rgb)),
        }
    }

    pub fn fg_bg(fg: RGB, bg: RGB) -> Self {
        Colour{
            fg: Some(FGColour(fg)),
            bg: Some(BGColour(bg)),
        }
    }
}

impl Painter for Colour {
    fn unescaped(&self) -> String {
        match (&self.fg, &self.bg) {
            (None, None) => "".to_string(),
            (None, Some(bg)) => bg.unescaped(),
            (Some(fg), None) => fg.unescaped(),
            (Some(fg), Some(bg)) => format!("{};{}", fg.unescaped(), bg.unescaped()),
        }
    }
}

enum Plane {
    FG = 38,
    BG = 48,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("\x1b[4m{}", Colour::bg(RGB(255,0,0)).paint("crossed out"));
    }

    #[test]
    fn basic() {
        let blue = Colour::fg(RGB(0,0,255));
        let painted = blue.paint("hello!");
        assert_eq!(painted, "\x1b[38;2;0;0;255mhello!\x1b[0m");
        println!("{}", painted);
    }

    #[test]
    fn test_rgb() {
        let burlywood = FGColour(rgb::BurlyWood);
        assert_eq!(burlywood.unescaped(), "38;2;222;184;135");
        println!("{}", burlywood.paint("hello, burly wood!"));
        burlywood.start();
        println!("how's it going?");
        Colour::reset();
        println!("how's it going?");
    }

    #[test]
    fn test_fg_bg() {
        let cyan_on_red = Colour::fg_bg(rgb::Cyan, rgb::IndianRed);
        cyan_on_red.start();
        print!("this is in cyan on red.");
        BGColour::reset();
        print!(" and this white on red\n");
        Colour::reset();
    }
}

pub mod rgb {
    #![allow(non_upper_case_globals)]
    use super::RGB;
    pub const AliceBlue: RGB = RGB(240,248,255);
    pub const AntiqueWhite: RGB = RGB(250,235,215);
    pub const Aqua: RGB = RGB(0,255,255);
    pub const Aquamarine: RGB = RGB(127,255,212);
    pub const Azure: RGB = RGB(240,255,255);
    pub const Beige: RGB = RGB(245,245,220);
    pub const Bisque: RGB = RGB(255,228,196);
    pub const Black: RGB = RGB(0,0,0);
    pub const BlanchedAlmond: RGB = RGB(255,235,205);
    pub const Blue: RGB = RGB(0,0,255);
    pub const BlueViolet: RGB = RGB(138,43,226);
    pub const Brown: RGB = RGB(165,42,42);
    pub const BurlyWood: RGB = RGB(222,184,135);
    pub const CadetBlue: RGB = RGB(95,158,160);
    pub const Chartreuse: RGB = RGB(127,255,0);
    pub const Chocolate: RGB = RGB(210,105,30);
    pub const Coral: RGB = RGB(255,127,80);
    pub const CornflowerBlue: RGB = RGB(100,149,237);
    pub const Cornsilk: RGB = RGB(255,248,220);
    pub const Crimson: RGB = RGB(220,20,60);
    pub const Cyan: RGB = RGB(0,255,255);
    pub const DarkBlue: RGB = RGB(0,0,139);
    pub const DarkCyan: RGB = RGB(0,139,139);
    pub const DarkGoldenRod: RGB = RGB(184,134,11);
    pub const DarkGray: RGB = RGB(169,169,169);
    pub const DarkGrey: RGB = RGB(169,169,169);
    pub const DarkGreen: RGB = RGB(0,100,0);
    pub const DarkKhaki: RGB = RGB(189,183,107);
    pub const DarkMagenta: RGB = RGB(139,0,139);
    pub const DarkOliveGreen: RGB = RGB(85,107,47);
    pub const DarkOrange: RGB = RGB(255,140,0);
    pub const DarkOrchid: RGB = RGB(153,50,204);
    pub const DarkRed: RGB = RGB(139,0,0);
    pub const DarkSalmon: RGB = RGB(233,150,122);
    pub const DarkSeaGreen: RGB = RGB(143,188,143);
    pub const DarkSlateBlue: RGB = RGB(72,61,139);
    pub const DarkSlateGray: RGB = RGB(47,79,79);
    pub const DarkSlateGrey: RGB = RGB(47,79,79);
    pub const DarkTurquoise: RGB = RGB(0,206,209);
    pub const DarkViolet: RGB = RGB(148,0,211);
    pub const DeepPink: RGB = RGB(255,20,147);
    pub const DeepSkyBlue: RGB = RGB(0,191,255);
    pub const DimGray: RGB = RGB(105,105,105);
    pub const DimGrey: RGB = RGB(105,105,105);
    pub const DodgerBlue: RGB = RGB(30,144,255);
    pub const FireBrick: RGB = RGB(178,34,34);
    pub const FloralWhite: RGB = RGB(255,250,240);
    pub const ForestGreen: RGB = RGB(34,139,34);
    pub const Fuchsia: RGB = RGB(255,0,255);
    pub const Gainsboro: RGB = RGB(220,220,220);
    pub const GhostWhite: RGB = RGB(248,248,255);
    pub const Gold: RGB = RGB(255,215,0);
    pub const GoldenRod: RGB = RGB(218,165,32);
    pub const Gray: RGB = RGB(128,128,128);
    pub const Grey: RGB = RGB(128,128,128);
    pub const Green: RGB = RGB(0,128,0);
    pub const GreenYellow: RGB = RGB(173,255,47);
    pub const HoneyDew: RGB = RGB(240,255,240);
    pub const HotPink: RGB = RGB(255,105,180);
    pub const IndianRed: RGB = RGB(205,92,92);
    pub const Indigo: RGB = RGB(75,0,130);
    pub const Ivory: RGB = RGB(255,255,240);
    pub const Khaki: RGB = RGB(240,230,140);
    pub const Lavender: RGB = RGB(230,230,250);
    pub const LavenderBlush: RGB = RGB(255,240,245);
    pub const LawnGreen: RGB = RGB(124,252,0);
    pub const LemonChiffon: RGB = RGB(255,250,205);
    pub const LightBlue: RGB = RGB(173,216,230);
    pub const LightCoral: RGB = RGB(240,128,128);
    pub const LightCyan: RGB = RGB(224,255,255);
    pub const LightGoldenRodYellow: RGB = RGB(250,250,210);
    pub const LightGray: RGB = RGB(211,211,211);
    pub const LightGrey: RGB = RGB(211,211,211);
    pub const LightGreen: RGB = RGB(144,238,144);
    pub const LightPink: RGB = RGB(255,182,193);
    pub const LightSalmon: RGB = RGB(255,160,122);
    pub const LightSeaGreen: RGB = RGB(32,178,170);
    pub const LightSkyBlue: RGB = RGB(135,206,250);
    pub const LightSlateGray: RGB = RGB(119,136,153);
    pub const LightSlateGrey: RGB = RGB(119,136,153);
    pub const LightSteelBlue: RGB = RGB(176,196,222);
    pub const LightYellow: RGB = RGB(255,255,224);
    pub const Lime: RGB = RGB(0,255,0);
    pub const LimeGreen: RGB = RGB(50,205,50);
    pub const Linen: RGB = RGB(250,240,230);
    pub const Magenta: RGB = RGB(255,0,255);
    pub const Maroon: RGB = RGB(128,0,0);
    pub const MediumAquaMarine: RGB = RGB(102,205,170);
    pub const MediumBlue: RGB = RGB(0,0,205);
    pub const MediumOrchid: RGB = RGB(186,85,211);
    pub const MediumPurple: RGB = RGB(147,112,219);
    pub const MediumSeaGreen: RGB = RGB(60,179,113);
    pub const MediumSlateBlue: RGB = RGB(123,104,238);
    pub const MediumSpringGreen: RGB = RGB(0,250,154);
    pub const MediumTurquoise: RGB = RGB(72,209,204);
    pub const MediumVioletRed: RGB = RGB(199,21,133);
    pub const MidnightBlue: RGB = RGB(25,25,112);
    pub const MintCream: RGB = RGB(245,255,250);
    pub const MistyRose: RGB = RGB(255,228,225);
    pub const Moccasin: RGB = RGB(255,228,181);
    pub const NavajoWhite: RGB = RGB(255,222,173);
    pub const Navy: RGB = RGB(0,0,128);
    pub const OldLace: RGB = RGB(253,245,230);
    pub const Olive: RGB = RGB(128,128,0);
    pub const OliveDrab: RGB = RGB(107,142,35);
    pub const Orange: RGB = RGB(255,165,0);
    pub const OrangeRed: RGB = RGB(255,69,0);
    pub const Orchid: RGB = RGB(218,112,214);
    pub const PaleGoldenRod: RGB = RGB(238,232,170);
    pub const PaleGreen: RGB = RGB(152,251,152);
    pub const PaleTurquoise: RGB = RGB(175,238,238);
    pub const PaleVioletRed: RGB = RGB(219,112,147);
    pub const PapayaWhip: RGB = RGB(255,239,213);
    pub const PeachPuff: RGB = RGB(255,218,185);
    pub const Peru: RGB = RGB(205,133,63);
    pub const Pink: RGB = RGB(255,192,203);
    pub const Plum: RGB = RGB(221,160,221);
    pub const PowderBlue: RGB = RGB(176,224,230);
    pub const Purple: RGB = RGB(128,0,128);
    pub const RebeccaPurple: RGB = RGB(102,51,153);
    pub const Red: RGB = RGB(255,0,0);
    pub const RosyBrown: RGB = RGB(188,143,143);
    pub const RoyalBlue: RGB = RGB(65,105,225);
    pub const SaddleBrown: RGB = RGB(139,69,19);
    pub const Salmon: RGB = RGB(250,128,114);
    pub const SandyBrown: RGB = RGB(244,164,96);
    pub const SeaGreen: RGB = RGB(46,139,87);
    pub const SeaShell: RGB = RGB(255,245,238);
    pub const Sienna: RGB = RGB(160,82,45);
    pub const Silver: RGB = RGB(192,192,192);
    pub const SkyBlue: RGB = RGB(135,206,235);
    pub const SlateBlue: RGB = RGB(106,90,205);
    pub const SlateGray: RGB = RGB(112,128,144);
    pub const SlateGrey: RGB = RGB(112,128,144);
    pub const Snow: RGB = RGB(255,250,250);
    pub const SpringGreen: RGB = RGB(0,255,127);
    pub const SteelBlue: RGB = RGB(70,130,180);
    pub const Tan: RGB = RGB(210,180,140);
    pub const Teal: RGB = RGB(0,128,128);
    pub const Thistle: RGB = RGB(216,191,216);
    pub const Tomato: RGB = RGB(255,99,71);
    pub const Turquoise: RGB = RGB(64,224,208);
    pub const Violet: RGB = RGB(238,130,238);
    pub const Wheat: RGB = RGB(245,222,179);
    pub const White: RGB = RGB(255,255,255);
    pub const WhiteSmoke: RGB = RGB(245,245,245);
    pub const Yellow: RGB = RGB(255,255,0);
    pub const YellowGreen: RGB = RGB(154,205,50);
}
