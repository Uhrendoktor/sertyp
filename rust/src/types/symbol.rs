use std::borrow::Cow;

use derive_more::{Deref, DerefMut, Display, From};

#[cfg(feature = "content")]
use crate::Content;
use crate::{Item, types::string::String};

/// For more information visit the typst documentation: [symbol](https://typst.app/docs/reference/foundations/symbol/)
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Deref,
    DerefMut,
    Default,
    Hash,
    Display,
    From,
)]
#[display("{}", **self)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct Symbol<'a>(#[serde(borrow)] pub Cow<'a, char>);

impl<'a> From<String<'a>> for Symbol<'a> {
    fn from(value: String<'a>) -> Self {
        Symbol(Cow::Owned(value.chars().next().unwrap_or_default()))
    }
}

impl<'a> From<&'a char> for Symbol<'a> {
    fn from(value: &'a char) -> Self {
        Symbol(Cow::Borrowed(value))
    }
}

impl<'a> From<char> for Symbol<'a> {
    fn from(value: char) -> Self {
        Symbol(Cow::Owned(value))
    }
}

#[cfg(feature = "content")]
impl<'a> From<&'a char> for Content<'a> {
    fn from(value: &'a char) -> Self {
        Symbol::from(value).into()
    }
}

#[cfg(feature = "content")]
impl<'a> From<char> for Content<'a> {
    fn from(value: char) -> Self {
        Symbol::from(value).into()
    }
}

impl<'a> From<std::string::String> for Symbol<'a> {
    fn from(value: std::string::String) -> Self {
        Symbol(Cow::Owned(value.chars().next().unwrap_or_default()))
    }
}

impl<'a> From<Symbol<'a>> for String<'a> {
    fn from(value: Symbol<'a>) -> Self {
        String::from(std::string::String::from(value))
    }
}

impl<'a> From<Symbol<'a>> for std::string::String {
    fn from(value: Symbol<'a>) -> Self {
        value.0.to_string()
    }
}

crate::impl_all!(Item<'a>::Symbol, Symbol<'a>{'a}, "symbol");

#[allow(unused, non_upper_case_globals)]
/// 𝔸: Mathematical Double Struck Capital A
pub const SYMBOL_AA: char = '\u{1d538}';
#[allow(unused, non_upper_case_globals)]
/// Α: Greek Capital Letter Alpha
pub const SYMBOL_Alpha: char = '\u{391}';
#[allow(unused, non_upper_case_globals)]
/// 𝔹: Mathematical Double Struck Capital B
pub const SYMBOL_BB: char = '\u{1d539}';
#[allow(unused, non_upper_case_globals)]
/// Β: Greek Capital Letter Beta
pub const SYMBOL_Beta: char = '\u{392}';
#[allow(unused, non_upper_case_globals)]
/// ℂ: Double Struck Capital C
pub const SYMBOL_CC: char = '\u{2102}';
#[allow(unused, non_upper_case_globals)]
/// Χ: Greek Capital Letter Chi
pub const SYMBOL_Chi: char = '\u{3a7}';
#[allow(unused, non_upper_case_globals)]
/// 𝔻: Mathematical Double Struck Capital D
pub const SYMBOL_DD: char = '\u{1d53b}';
#[allow(unused, non_upper_case_globals)]
/// Δ: Greek Capital Letter Delta
pub const SYMBOL_Delta: char = '\u{394}';
#[allow(unused, non_upper_case_globals)]
/// Ϝ: Greek Letter Digamma
pub const SYMBOL_Digamma: char = '\u{3dc}';
#[allow(unused, non_upper_case_globals)]
/// 𝔼: Mathematical Double Struck Capital E
pub const SYMBOL_EE: char = '\u{1d53c}';
#[allow(unused, non_upper_case_globals)]
/// Ε: Greek Capital Letter Epsilon
pub const SYMBOL_Epsilon: char = '\u{395}';
#[allow(unused, non_upper_case_globals)]
/// Η: Greek Capital Letter Eta
pub const SYMBOL_Eta: char = '\u{397}';
#[allow(unused, non_upper_case_globals)]
/// 𝔽: Mathematical Double Struck Capital F
pub const SYMBOL_FF: char = '\u{1d53d}';
#[allow(unused, non_upper_case_globals)]
/// 𝔾: Mathematical Double Struck Capital G
pub const SYMBOL_GG: char = '\u{1d53e}';
#[allow(unused, non_upper_case_globals)]
/// Γ: Greek Capital Letter Gamma
pub const SYMBOL_Gamma: char = '\u{393}';
#[allow(unused, non_upper_case_globals)]
/// ℍ: Double Struck Capital H
pub const SYMBOL_HH: char = '\u{210d}';
#[allow(unused, non_upper_case_globals)]
/// 𝕀: Mathematical Double Struck Capital I
pub const SYMBOL_II: char = '\u{1d540}';
#[allow(unused, non_upper_case_globals)]
/// ℑ: Black Letter Capital I
pub const SYMBOL_Im: char = '\u{2111}';
#[allow(unused, non_upper_case_globals)]
/// Ι: Greek Capital Letter Iota
pub const SYMBOL_Iota: char = '\u{399}';
#[allow(unused, non_upper_case_globals)]
/// 𝕁: Mathematical Double Struck Capital J
pub const SYMBOL_JJ: char = '\u{1d541}';
#[allow(unused, non_upper_case_globals)]
/// 𝕂: Mathematical Double Struck Capital K
pub const SYMBOL_KK: char = '\u{1d542}';
#[allow(unused, non_upper_case_globals)]
/// Κ: Greek Capital Letter Kappa
pub const SYMBOL_Kappa: char = '\u{39a}';
#[allow(unused, non_upper_case_globals)]
/// 𝕃: Mathematical Double Struck Capital L
pub const SYMBOL_LL: char = '\u{1d543}';
#[allow(unused, non_upper_case_globals)]
/// Λ: Greek Capital Letter Lamda
pub const SYMBOL_Lambda: char = '\u{39b}';
#[allow(unused, non_upper_case_globals)]
/// 𝕄: Mathematical Double Struck Capital M
pub const SYMBOL_MM: char = '\u{1d544}';
#[allow(unused, non_upper_case_globals)]
/// Μ: Greek Capital Letter Mu
pub const SYMBOL_Mu: char = '\u{39c}';
#[allow(unused, non_upper_case_globals)]
/// ℕ: Double Struck Capital N
pub const SYMBOL_NN: char = '\u{2115}';
#[allow(unused, non_upper_case_globals)]
/// Ν: Greek Capital Letter Nu
pub const SYMBOL_Nu: char = '\u{39d}';
#[allow(unused, non_upper_case_globals)]
/// 𝕆: Mathematical Double Struck Capital O
pub const SYMBOL_OO: char = '\u{1d546}';
#[allow(unused, non_upper_case_globals)]
/// Ω: Greek Capital Letter Omega
pub const SYMBOL_Omega: char = '\u{3a9}';
#[allow(unused, non_upper_case_globals)]
/// ℧: Inverted Ohm Sign
pub const SYMBOL_Omega_inv: char = '\u{2127}';
#[allow(unused, non_upper_case_globals)]
/// Ο: Greek Capital Letter Omicron
pub const SYMBOL_Omicron: char = '\u{39f}';
#[allow(unused, non_upper_case_globals)]
/// ℙ: Double Struck Capital P
pub const SYMBOL_PP: char = '\u{2119}';
#[allow(unused, non_upper_case_globals)]
/// Φ: Greek Capital Letter Phi
pub const SYMBOL_Phi: char = '\u{3a6}';
#[allow(unused, non_upper_case_globals)]
/// Π: Greek Capital Letter Pi
pub const SYMBOL_Pi: char = '\u{3a0}';
#[allow(unused, non_upper_case_globals)]
/// Ψ: Greek Capital Letter Psi
pub const SYMBOL_Psi: char = '\u{3a8}';
#[allow(unused, non_upper_case_globals)]
/// ℚ: Double Struck Capital Q
pub const SYMBOL_QQ: char = '\u{211a}';
#[allow(unused, non_upper_case_globals)]
/// ℝ: Double Struck Capital R
pub const SYMBOL_RR: char = '\u{211d}';
#[allow(unused, non_upper_case_globals)]
/// ℜ: Black Letter Capital R
pub const SYMBOL_Re: char = '\u{211c}';
#[allow(unused, non_upper_case_globals)]
/// Ρ: Greek Capital Letter Rho
pub const SYMBOL_Rho: char = '\u{3a1}';
#[allow(unused, non_upper_case_globals)]
/// 𝕊: Mathematical Double Struck Capital S
pub const SYMBOL_SS: char = '\u{1d54a}';
#[allow(unused, non_upper_case_globals)]
/// Ш: Cyrillic Capital Letter Sha
pub const SYMBOL_Sha: char = '\u{428}';
#[allow(unused, non_upper_case_globals)]
/// Σ: Greek Capital Letter Sigma
pub const SYMBOL_Sigma: char = '\u{3a3}';
#[allow(unused, non_upper_case_globals)]
/// 𝕋: Mathematical Double Struck Capital T
pub const SYMBOL_TT: char = '\u{1d54b}';
#[allow(unused, non_upper_case_globals)]
/// Τ: Greek Capital Letter Tau
pub const SYMBOL_Tau: char = '\u{3a4}';
#[allow(unused, non_upper_case_globals)]
/// Θ: Greek Capital Letter Theta
pub const SYMBOL_Theta: char = '\u{398}';
#[allow(unused, non_upper_case_globals)]
/// ϴ: Greek Capital Theta Symbol
pub const SYMBOL_Theta_alt: char = '\u{3f4}';
#[allow(unused, non_upper_case_globals)]
/// 𝕌: Mathematical Double Struck Capital U
pub const SYMBOL_UU: char = '\u{1d54c}';
#[allow(unused, non_upper_case_globals)]
/// Υ: Greek Capital Letter Upsilon
pub const SYMBOL_Upsilon: char = '\u{3a5}';
#[allow(unused, non_upper_case_globals)]
/// 𝕍: Mathematical Double Struck Capital V
pub const SYMBOL_VV: char = '\u{1d54d}';
#[allow(unused, non_upper_case_globals)]
/// 𝕎: Mathematical Double Struck Capital W
pub const SYMBOL_WW: char = '\u{1d54e}';
#[allow(unused, non_upper_case_globals)]
/// 𝕏: Mathematical Double Struck Capital X
pub const SYMBOL_XX: char = '\u{1d54f}';
#[allow(unused, non_upper_case_globals)]
/// Ξ: Greek Capital Letter Xi
pub const SYMBOL_Xi: char = '\u{39e}';
#[allow(unused, non_upper_case_globals)]
/// 𝕐: Mathematical Double Struck Capital Y
pub const SYMBOL_YY: char = '\u{1d550}';
#[allow(unused, non_upper_case_globals)]
/// ℤ: Double Struck Capital Z
pub const SYMBOL_ZZ: char = '\u{2124}';
#[allow(unused, non_upper_case_globals)]
/// Ζ: Greek Capital Letter Zeta
pub const SYMBOL_Zeta: char = '\u{396}';
#[allow(unused, non_upper_case_globals)]
/// ´: Acute Accent
pub const SYMBOL_acute: char = '\u{b4}';
#[allow(unused, non_upper_case_globals)]
/// ˝: Double Acute Accent
pub const SYMBOL_acute_double: char = '\u{2dd}';
#[allow(unused, non_upper_case_globals)]
/// ؋: Afghani Sign
pub const SYMBOL_afghani: char = '\u{60b}';
#[allow(unused, non_upper_case_globals)]
/// א: Hebrew Letter Alef
pub const SYMBOL_aleph: char = '\u{5d0}';
#[allow(unused, non_upper_case_globals)]
/// α: Greek Small Letter Alpha
pub const SYMBOL_alpha: char = '\u{3b1}';
#[allow(unused, non_upper_case_globals)]
/// &: Ampersand
pub const SYMBOL_amp: char = '\u{26}';
#[allow(unused, non_upper_case_globals)]
/// ⅋: Turned Ampersand
pub const SYMBOL_amp_inv: char = '\u{214b}';
#[allow(unused, non_upper_case_globals)]
/// ∧: Logical And
pub const SYMBOL_and: char = '\u{2227}';
#[allow(unused, non_upper_case_globals)]
/// ⋀: N Ary Logical And
pub const SYMBOL_and_big: char = '\u{22c0}';
#[allow(unused, non_upper_case_globals)]
/// ⋏: Curly Logical And
pub const SYMBOL_and_curly: char = '\u{22cf}';
#[allow(unused, non_upper_case_globals)]
/// ⟑: And With Dot
pub const SYMBOL_and_dot: char = '\u{27d1}';
#[allow(unused, non_upper_case_globals)]
/// ⩓: Double Logical And
pub const SYMBOL_and_double: char = '\u{2a53}';
#[allow(unused, non_upper_case_globals)]
/// ∠: Angle
pub const SYMBOL_angle: char = '\u{2220}';
#[allow(unused, non_upper_case_globals)]
/// ⦟: Acute Angle
pub const SYMBOL_angle_acute: char = '\u{299f}';
#[allow(unused, non_upper_case_globals)]
/// ∡: Measured Angle
pub const SYMBOL_angle_arc: char = '\u{2221}';
#[allow(unused, non_upper_case_globals)]
/// ⦛: Measured Angle Opening Left
pub const SYMBOL_angle_arc_rev: char = '\u{299b}';
#[allow(unused, non_upper_case_globals)]
/// ⍼: Right Angle With Downwards Zigzag Arrow
pub const SYMBOL_angle_azimuth: char = '\u{237c}';
#[allow(unused, non_upper_case_globals)]
/// ⦦: Oblique Angle Opening Up
pub const SYMBOL_angle_obtuse: char = '\u{29a6}';
#[allow(unused, non_upper_case_globals)]
/// ⦣: Reversed Angle
pub const SYMBOL_angle_rev: char = '\u{29a3}';
#[allow(unused, non_upper_case_globals)]
/// ∟: Right Angle
pub const SYMBOL_angle_right: char = '\u{221f}';
#[allow(unused, non_upper_case_globals)]
/// ⯾: Reversed Right Angle
pub const SYMBOL_angle_right_rev: char = '\u{2bfe}';
#[allow(unused, non_upper_case_globals)]
/// ⊾: Right Angle With Arc
pub const SYMBOL_angle_right_arc: char = '\u{22be}';
#[allow(unused, non_upper_case_globals)]
/// ⦝: Measured Right Angle With Dot
pub const SYMBOL_angle_right_dot: char = '\u{299d}';
#[allow(unused, non_upper_case_globals)]
/// ⦜: Right Angle Variant With Square
pub const SYMBOL_angle_right_square: char = '\u{299c}';
#[allow(unused, non_upper_case_globals)]
/// ⦞: Angle With S Inside
pub const SYMBOL_angle_s: char = '\u{299e}';
#[allow(unused, non_upper_case_globals)]
/// ⟀: Three Dimensional Angle
pub const SYMBOL_angle_spatial: char = '\u{27c0}';
#[allow(unused, non_upper_case_globals)]
/// ∢: Spherical Angle
pub const SYMBOL_angle_spheric: char = '\u{2222}';
#[allow(unused, non_upper_case_globals)]
/// ⦠: Spherical Angle Opening Left
pub const SYMBOL_angle_spheric_rev: char = '\u{29a0}';
#[allow(unused, non_upper_case_globals)]
/// ⦡: Spherical Angle Opening Up
pub const SYMBOL_angle_spheric_t: char = '\u{29a1}';
#[allow(unused, non_upper_case_globals)]
/// Å: Latin Capital Letter A With Ring Above
pub const SYMBOL_angstrom: char = '\u{c5}';
#[allow(unused, non_upper_case_globals)]
/// ⍼: Right Angle With Downwards Zigzag Arrow
pub const SYMBOL_angzarr: char = '\u{237c}';
#[allow(unused, non_upper_case_globals)]
/// ≈: Almost Equal To
pub const SYMBOL_approx: char = '\u{2248}';
#[allow(unused, non_upper_case_globals)]
/// ≊: Almost Equal Or Equal To
pub const SYMBOL_approx_eq: char = '\u{224a}';
#[allow(unused, non_upper_case_globals)]
/// ≉: Not Almost Equal To
pub const SYMBOL_approx_not: char = '\u{2249}';
#[allow(unused, non_upper_case_globals)]
/// ⩯: Almost Equal To With Circumflex Accent
pub const SYMBOL_approx_hat: char = '\u{2a6f}';
#[allow(unused, non_upper_case_globals)]
/// →: Rightwards Arrow
pub const SYMBOL_arrow_r: char = '\u{2192}';
#[allow(unused, non_upper_case_globals)]
/// ⟼: Long Rightwards Arrow From Bar
pub const SYMBOL_arrow_r_long_bar: char = '\u{27fc}';
#[allow(unused, non_upper_case_globals)]
/// ↦: Rightwards Arrow From Bar
pub const SYMBOL_arrow_r_bar: char = '\u{21a6}';
#[allow(unused, non_upper_case_globals)]
/// ⤷: Arrow Pointing Downwards Then Curving Rightwards
pub const SYMBOL_arrow_r_curve: char = '\u{2937}';
#[allow(unused, non_upper_case_globals)]
/// ⮎: Anticlockwise Triangle Headed Left U Shaped Arrow
pub const SYMBOL_arrow_r_turn: char = '\u{2b8e}';
#[allow(unused, non_upper_case_globals)]
/// ⇢: Rightwards Dashed Arrow
pub const SYMBOL_arrow_r_dashed: char = '\u{21e2}';
#[allow(unused, non_upper_case_globals)]
/// ⤑: Rightwards Arrow With Dotted Stem
pub const SYMBOL_arrow_r_dotted: char = '\u{2911}';
#[allow(unused, non_upper_case_globals)]
/// ⇒: Rightwards Double Arrow
pub const SYMBOL_arrow_r_double: char = '\u{21d2}';
#[allow(unused, non_upper_case_globals)]
/// ⤇: Rightwards Double Arrow From Bar
pub const SYMBOL_arrow_r_double_bar: char = '\u{2907}';
#[allow(unused, non_upper_case_globals)]
/// ⟹: Long Rightwards Double Arrow
pub const SYMBOL_arrow_r_double_long: char = '\u{27f9}';
#[allow(unused, non_upper_case_globals)]
/// ⟾: Long Rightwards Double Arrow From Bar
pub const SYMBOL_arrow_r_double_long_bar: char = '\u{27fe}';
#[allow(unused, non_upper_case_globals)]
/// ⇏: Rightwards Double Arrow With Stroke
pub const SYMBOL_arrow_r_double_not: char = '\u{21cf}';
#[allow(unused, non_upper_case_globals)]
/// ⤃: Rightwards Double Arrow With Vertical Stroke
pub const SYMBOL_arrow_r_double_struck: char = '\u{2903}';
#[allow(unused, non_upper_case_globals)]
/// ➡︎: Black Rightwards Arrow
pub const SYMBOL_arrow_r_filled: char = '\u{27a1}';
#[allow(unused, non_upper_case_globals)]
/// ↪︎: Rightwards Arrow With Hook
pub const SYMBOL_arrow_r_hook: char = '\u{21aa}';
#[allow(unused, non_upper_case_globals)]
/// ⟶: Long Rightwards Arrow
pub const SYMBOL_arrow_r_long: char = '\u{27f6}';
#[allow(unused, non_upper_case_globals)]
/// ⟿: Long Rightwards Squiggle Arrow
pub const SYMBOL_arrow_r_long_squiggly: char = '\u{27ff}';
#[allow(unused, non_upper_case_globals)]
/// ↬: Rightwards Arrow With Loop
pub const SYMBOL_arrow_r_loop: char = '\u{21ac}';
#[allow(unused, non_upper_case_globals)]
/// ↛: Rightwards Arrow With Stroke
pub const SYMBOL_arrow_r_not: char = '\u{219b}';
#[allow(unused, non_upper_case_globals)]
/// ⭆: Rightwards Quadruple Arrow
pub const SYMBOL_arrow_r_quad: char = '\u{2b46}';
#[allow(unused, non_upper_case_globals)]
/// ⇝: Rightwards Squiggle Arrow
pub const SYMBOL_arrow_r_squiggly: char = '\u{21dd}';
#[allow(unused, non_upper_case_globals)]
/// ⇥: Rightwards Arrow To Bar
pub const SYMBOL_arrow_r_stop: char = '\u{21e5}';
#[allow(unused, non_upper_case_globals)]
/// ⇨: Rightwards White Arrow
pub const SYMBOL_arrow_r_stroked: char = '\u{21e8}';
#[allow(unused, non_upper_case_globals)]
/// ⇸: Rightwards Arrow With Vertical Stroke
pub const SYMBOL_arrow_r_struck: char = '\u{21f8}';
#[allow(unused, non_upper_case_globals)]
/// ⇻: Rightwards Arrow With Double Vertical Stroke
pub const SYMBOL_arrow_r_dstruck: char = '\u{21fb}';
#[allow(unused, non_upper_case_globals)]
/// ↣: Rightwards Arrow With Tail
pub const SYMBOL_arrow_r_tail: char = '\u{21a3}';
#[allow(unused, non_upper_case_globals)]
/// ⤔: Rightwards Arrow With Tail With Vertical Stroke
pub const SYMBOL_arrow_r_tail_struck: char = '\u{2914}';
#[allow(unused, non_upper_case_globals)]
/// ⤕: Rightwards Arrow With Tail With Double Vertical Stroke
pub const SYMBOL_arrow_r_tail_dstruck: char = '\u{2915}';
#[allow(unused, non_upper_case_globals)]
/// ⥲: Tilde Operator Above Rightwards Arrow
pub const SYMBOL_arrow_r_tilde: char = '\u{2972}';
#[allow(unused, non_upper_case_globals)]
/// ⇛: Rightwards Triple Arrow
pub const SYMBOL_arrow_r_triple: char = '\u{21db}';
#[allow(unused, non_upper_case_globals)]
/// ↠: Rightwards Two Headed Arrow
pub const SYMBOL_arrow_r_twohead: char = '\u{21a0}';
#[allow(unused, non_upper_case_globals)]
/// ⤅: Rightwards Two Headed Arrow From Bar
pub const SYMBOL_arrow_r_twohead_bar: char = '\u{2905}';
#[allow(unused, non_upper_case_globals)]
/// ⤀: Rightwards Two Headed Arrow With Vertical Stroke
pub const SYMBOL_arrow_r_twohead_struck: char = '\u{2900}';
#[allow(unused, non_upper_case_globals)]
/// ⤁: Rightwards Two Headed Arrow With Double Vertical Stroke
pub const SYMBOL_arrow_r_twohead_dstruck: char = '\u{2901}';
#[allow(unused, non_upper_case_globals)]
/// ⤖: Rightwards Two Headed Arrow With Tail
pub const SYMBOL_arrow_r_twohead_tail: char = '\u{2916}';
#[allow(unused, non_upper_case_globals)]
/// ⤗: Rightwards Two Headed Arrow With Tail With Vertical Stroke
pub const SYMBOL_arrow_r_twohead_tail_struck: char = '\u{2917}';
#[allow(unused, non_upper_case_globals)]
/// ⤘: Rightwards Two Headed Arrow With Tail With Double Vertical Stroke
pub const SYMBOL_arrow_r_twohead_tail_dstruck: char = '\u{2918}';
#[allow(unused, non_upper_case_globals)]
/// ⇾: Rightwards Open Headed Arrow
pub const SYMBOL_arrow_r_open: char = '\u{21fe}';
#[allow(unused, non_upper_case_globals)]
/// ↝: Rightwards Wave Arrow
pub const SYMBOL_arrow_r_wave: char = '\u{219d}';
#[allow(unused, non_upper_case_globals)]
/// ←: Leftwards Arrow
pub const SYMBOL_arrow_l: char = '\u{2190}';
#[allow(unused, non_upper_case_globals)]
/// ↤: Leftwards Arrow From Bar
pub const SYMBOL_arrow_l_bar: char = '\u{21a4}';
#[allow(unused, non_upper_case_globals)]
/// ⤶: Arrow Pointing Downwards Then Curving Leftwards
pub const SYMBOL_arrow_l_curve: char = '\u{2936}';
#[allow(unused, non_upper_case_globals)]
/// ⮌: Anticlockwise Triangle Headed Right U Shaped Arrow
pub const SYMBOL_arrow_l_turn: char = '\u{2b8c}';
#[allow(unused, non_upper_case_globals)]
/// ⇠: Leftwards Dashed Arrow
pub const SYMBOL_arrow_l_dashed: char = '\u{21e0}';
#[allow(unused, non_upper_case_globals)]
/// ⬸: Leftwards Arrow With Dotted Stem
pub const SYMBOL_arrow_l_dotted: char = '\u{2b38}';
#[allow(unused, non_upper_case_globals)]
/// ⇐: Leftwards Double Arrow
pub const SYMBOL_arrow_l_double: char = '\u{21d0}';
#[allow(unused, non_upper_case_globals)]
/// ⤆: Leftwards Double Arrow From Bar
pub const SYMBOL_arrow_l_double_bar: char = '\u{2906}';
#[allow(unused, non_upper_case_globals)]
/// ⟸: Long Leftwards Double Arrow
pub const SYMBOL_arrow_l_double_long: char = '\u{27f8}';
#[allow(unused, non_upper_case_globals)]
/// ⟽: Long Leftwards Double Arrow From Bar
pub const SYMBOL_arrow_l_double_long_bar: char = '\u{27fd}';
#[allow(unused, non_upper_case_globals)]
/// ⇍: Leftwards Double Arrow With Stroke
pub const SYMBOL_arrow_l_double_not: char = '\u{21cd}';
#[allow(unused, non_upper_case_globals)]
/// ⤂: Leftwards Double Arrow With Vertical Stroke
pub const SYMBOL_arrow_l_double_struck: char = '\u{2902}';
#[allow(unused, non_upper_case_globals)]
/// ⬅︎: Leftwards Black Arrow
pub const SYMBOL_arrow_l_filled: char = '\u{2b05}';
#[allow(unused, non_upper_case_globals)]
/// ↩︎: Leftwards Arrow With Hook
pub const SYMBOL_arrow_l_hook: char = '\u{21a9}';
#[allow(unused, non_upper_case_globals)]
/// ⟵: Long Leftwards Arrow
pub const SYMBOL_arrow_l_long: char = '\u{27f5}';
#[allow(unused, non_upper_case_globals)]
/// ⟻: Long Leftwards Arrow From Bar
pub const SYMBOL_arrow_l_long_bar: char = '\u{27fb}';
#[allow(unused, non_upper_case_globals)]
/// ⬳: Long Leftwards Squiggle Arrow
pub const SYMBOL_arrow_l_long_squiggly: char = '\u{2b33}';
#[allow(unused, non_upper_case_globals)]
/// ↫: Leftwards Arrow With Loop
pub const SYMBOL_arrow_l_loop: char = '\u{21ab}';
#[allow(unused, non_upper_case_globals)]
/// ↚: Leftwards Arrow With Stroke
pub const SYMBOL_arrow_l_not: char = '\u{219a}';
#[allow(unused, non_upper_case_globals)]
/// ⭅: Leftwards Quadruple Arrow
pub const SYMBOL_arrow_l_quad: char = '\u{2b45}';
#[allow(unused, non_upper_case_globals)]
/// ⇜: Leftwards Squiggle Arrow
pub const SYMBOL_arrow_l_squiggly: char = '\u{21dc}';
#[allow(unused, non_upper_case_globals)]
/// ⇤: Leftwards Arrow To Bar
pub const SYMBOL_arrow_l_stop: char = '\u{21e4}';
#[allow(unused, non_upper_case_globals)]
/// ⇦: Leftwards White Arrow
pub const SYMBOL_arrow_l_stroked: char = '\u{21e6}';
#[allow(unused, non_upper_case_globals)]
/// ⇷: Leftwards Arrow With Vertical Stroke
pub const SYMBOL_arrow_l_struck: char = '\u{21f7}';
#[allow(unused, non_upper_case_globals)]
/// ⇺: Leftwards Arrow With Double Vertical Stroke
pub const SYMBOL_arrow_l_dstruck: char = '\u{21fa}';
#[allow(unused, non_upper_case_globals)]
/// ↢: Leftwards Arrow With Tail
pub const SYMBOL_arrow_l_tail: char = '\u{21a2}';
#[allow(unused, non_upper_case_globals)]
/// ⬹: Leftwards Arrow With Tail With Vertical Stroke
pub const SYMBOL_arrow_l_tail_struck: char = '\u{2b39}';
#[allow(unused, non_upper_case_globals)]
/// ⬺: Leftwards Arrow With Tail With Double Vertical Stroke
pub const SYMBOL_arrow_l_tail_dstruck: char = '\u{2b3a}';
#[allow(unused, non_upper_case_globals)]
/// ⭉: Tilde Operator Above Leftwards Arrow
pub const SYMBOL_arrow_l_tilde: char = '\u{2b49}';
#[allow(unused, non_upper_case_globals)]
/// ⇚: Leftwards Triple Arrow
pub const SYMBOL_arrow_l_triple: char = '\u{21da}';
#[allow(unused, non_upper_case_globals)]
/// ↞: Leftwards Two Headed Arrow
pub const SYMBOL_arrow_l_twohead: char = '\u{219e}';
#[allow(unused, non_upper_case_globals)]
/// ⬶: Leftwards Two Headed Arrow From Bar
pub const SYMBOL_arrow_l_twohead_bar: char = '\u{2b36}';
#[allow(unused, non_upper_case_globals)]
/// ⬴: Leftwards Two Headed Arrow With Vertical Stroke
pub const SYMBOL_arrow_l_twohead_struck: char = '\u{2b34}';
#[allow(unused, non_upper_case_globals)]
/// ⬵: Leftwards Two Headed Arrow With Double Vertical Stroke
pub const SYMBOL_arrow_l_twohead_dstruck: char = '\u{2b35}';
#[allow(unused, non_upper_case_globals)]
/// ⬻: Leftwards Two Headed Arrow With Tail
pub const SYMBOL_arrow_l_twohead_tail: char = '\u{2b3b}';
#[allow(unused, non_upper_case_globals)]
/// ⬼: Leftwards Two Headed Arrow With Tail With Vertical Stroke
pub const SYMBOL_arrow_l_twohead_tail_struck: char = '\u{2b3c}';
#[allow(unused, non_upper_case_globals)]
/// ⬽: Leftwards Two Headed Arrow With Tail With Double Vertical Stroke
pub const SYMBOL_arrow_l_twohead_tail_dstruck: char = '\u{2b3d}';
#[allow(unused, non_upper_case_globals)]
/// ⇽: Leftwards Open Headed Arrow
pub const SYMBOL_arrow_l_open: char = '\u{21fd}';
#[allow(unused, non_upper_case_globals)]
/// ↜: Leftwards Wave Arrow
pub const SYMBOL_arrow_l_wave: char = '\u{219c}';
#[allow(unused, non_upper_case_globals)]
/// ↑: Upwards Arrow
pub const SYMBOL_arrow_t: char = '\u{2191}';
#[allow(unused, non_upper_case_globals)]
/// ↥: Upwards Arrow From Bar
pub const SYMBOL_arrow_t_bar: char = '\u{21a5}';
#[allow(unused, non_upper_case_globals)]
/// ⤴︎: Arrow Pointing Rightwards Then Curving Upwards
pub const SYMBOL_arrow_t_curve: char = '\u{2934}';
#[allow(unused, non_upper_case_globals)]
/// ⮍: Anticlockwise Triangle Headed Bottom U Shaped Arrow
pub const SYMBOL_arrow_t_turn: char = '\u{2b8d}';
#[allow(unused, non_upper_case_globals)]
/// ⇡: Upwards Dashed Arrow
pub const SYMBOL_arrow_t_dashed: char = '\u{21e1}';
#[allow(unused, non_upper_case_globals)]
/// ⇑: Upwards Double Arrow
pub const SYMBOL_arrow_t_double: char = '\u{21d1}';
#[allow(unused, non_upper_case_globals)]
/// ⬆︎: Upwards Black Arrow
pub const SYMBOL_arrow_t_filled: char = '\u{2b06}';
#[allow(unused, non_upper_case_globals)]
/// ⟰: Upwards Quadruple Arrow
pub const SYMBOL_arrow_t_quad: char = '\u{27f0}';
#[allow(unused, non_upper_case_globals)]
/// ⤒: Upwards Arrow To Bar
pub const SYMBOL_arrow_t_stop: char = '\u{2912}';
#[allow(unused, non_upper_case_globals)]
/// ⇧: Upwards White Arrow
pub const SYMBOL_arrow_t_stroked: char = '\u{21e7}';
#[allow(unused, non_upper_case_globals)]
/// ⤉: Upwards Arrow With Horizontal Stroke
pub const SYMBOL_arrow_t_struck: char = '\u{2909}';
#[allow(unused, non_upper_case_globals)]
/// ⇞: Upwards Arrow With Double Stroke
pub const SYMBOL_arrow_t_dstruck: char = '\u{21de}';
#[allow(unused, non_upper_case_globals)]
/// ⤊: Upwards Triple Arrow
pub const SYMBOL_arrow_t_triple: char = '\u{290a}';
#[allow(unused, non_upper_case_globals)]
/// ↟: Upwards Two Headed Arrow
pub const SYMBOL_arrow_t_twohead: char = '\u{219f}';
#[allow(unused, non_upper_case_globals)]
/// ↓: Downwards Arrow
pub const SYMBOL_arrow_b: char = '\u{2193}';
#[allow(unused, non_upper_case_globals)]
/// ↧: Downwards Arrow From Bar
pub const SYMBOL_arrow_b_bar: char = '\u{21a7}';
#[allow(unused, non_upper_case_globals)]
/// ⤵︎: Arrow Pointing Rightwards Then Curving Downwards
pub const SYMBOL_arrow_b_curve: char = '\u{2935}';
#[allow(unused, non_upper_case_globals)]
/// ⮏: Anticlockwise Triangle Headed Top U Shaped Arrow
pub const SYMBOL_arrow_b_turn: char = '\u{2b8f}';
#[allow(unused, non_upper_case_globals)]
/// ⇣: Downwards Dashed Arrow
pub const SYMBOL_arrow_b_dashed: char = '\u{21e3}';
#[allow(unused, non_upper_case_globals)]
/// ⇓: Downwards Double Arrow
pub const SYMBOL_arrow_b_double: char = '\u{21d3}';
#[allow(unused, non_upper_case_globals)]
/// ⬇︎: Downwards Black Arrow
pub const SYMBOL_arrow_b_filled: char = '\u{2b07}';
#[allow(unused, non_upper_case_globals)]
/// ⟱: Downwards Quadruple Arrow
pub const SYMBOL_arrow_b_quad: char = '\u{27f1}';
#[allow(unused, non_upper_case_globals)]
/// ⤓: Downwards Arrow To Bar
pub const SYMBOL_arrow_b_stop: char = '\u{2913}';
#[allow(unused, non_upper_case_globals)]
/// ⇩: Downwards White Arrow
pub const SYMBOL_arrow_b_stroked: char = '\u{21e9}';
#[allow(unused, non_upper_case_globals)]
/// ⤈: Downwards Arrow With Horizontal Stroke
pub const SYMBOL_arrow_b_struck: char = '\u{2908}';
#[allow(unused, non_upper_case_globals)]
/// ⇟: Downwards Arrow With Double Stroke
pub const SYMBOL_arrow_b_dstruck: char = '\u{21df}';
#[allow(unused, non_upper_case_globals)]
/// ⤋: Downwards Triple Arrow
pub const SYMBOL_arrow_b_triple: char = '\u{290b}';
#[allow(unused, non_upper_case_globals)]
/// ↡: Downwards Two Headed Arrow
pub const SYMBOL_arrow_b_twohead: char = '\u{21a1}';
#[allow(unused, non_upper_case_globals)]
/// ↔︎: Left Right Arrow
pub const SYMBOL_arrow_l_r: char = '\u{2194}';
#[allow(unused, non_upper_case_globals)]
/// ⇔: Left Right Double Arrow
pub const SYMBOL_arrow_l_r_double: char = '\u{21d4}';
#[allow(unused, non_upper_case_globals)]
/// ⟺: Long Left Right Double Arrow
pub const SYMBOL_arrow_l_r_double_long: char = '\u{27fa}';
#[allow(unused, non_upper_case_globals)]
/// ⇎: Left Right Double Arrow With Stroke
pub const SYMBOL_arrow_l_r_double_not: char = '\u{21ce}';
#[allow(unused, non_upper_case_globals)]
/// ⤄: Left Right Double Arrow With Vertical Stroke
pub const SYMBOL_arrow_l_r_double_struck: char = '\u{2904}';
#[allow(unused, non_upper_case_globals)]
/// ⬌: Left Right Black Arrow
pub const SYMBOL_arrow_l_r_filled: char = '\u{2b0c}';
#[allow(unused, non_upper_case_globals)]
/// ⟷: Long Left Right Arrow
pub const SYMBOL_arrow_l_r_long: char = '\u{27f7}';
#[allow(unused, non_upper_case_globals)]
/// ↮: Left Right Arrow With Stroke
pub const SYMBOL_arrow_l_r_not: char = '\u{21ae}';
#[allow(unused, non_upper_case_globals)]
/// ⬄: Left Right White Arrow
pub const SYMBOL_arrow_l_r_stroked: char = '\u{2b04}';
#[allow(unused, non_upper_case_globals)]
/// ⇹: Left Right Arrow With Vertical Stroke
pub const SYMBOL_arrow_l_r_struck: char = '\u{21f9}';
#[allow(unused, non_upper_case_globals)]
/// ⇼: Left Right Arrow With Double Vertical Stroke
pub const SYMBOL_arrow_l_r_dstruck: char = '\u{21fc}';
#[allow(unused, non_upper_case_globals)]
/// ⇿: Left Right Open Headed Arrow
pub const SYMBOL_arrow_l_r_open: char = '\u{21ff}';
#[allow(unused, non_upper_case_globals)]
/// ↭: Left Right Wave Arrow
pub const SYMBOL_arrow_l_r_wave: char = '\u{21ad}';
#[allow(unused, non_upper_case_globals)]
/// ↕︎: Up Down Arrow
pub const SYMBOL_arrow_t_b: char = '\u{2195}';
#[allow(unused, non_upper_case_globals)]
/// ⇕: Up Down Double Arrow
pub const SYMBOL_arrow_t_b_double: char = '\u{21d5}';
#[allow(unused, non_upper_case_globals)]
/// ⬍: Up Down Black Arrow
pub const SYMBOL_arrow_t_b_filled: char = '\u{2b0d}';
#[allow(unused, non_upper_case_globals)]
/// ⇳: Up Down White Arrow
pub const SYMBOL_arrow_t_b_stroked: char = '\u{21f3}';
#[allow(unused, non_upper_case_globals)]
/// ↗︎: North East Arrow
pub const SYMBOL_arrow_tr: char = '\u{2197}';
#[allow(unused, non_upper_case_globals)]
/// 🢹: North East Arrow From Bar
pub const SYMBOL_arrow_tr_bar: char = '\u{1f8b9}';
#[allow(unused, non_upper_case_globals)]
/// ⇗: North East Double Arrow
pub const SYMBOL_arrow_tr_double: char = '\u{21d7}';
#[allow(unused, non_upper_case_globals)]
/// ⬈: North East Black Arrow
pub const SYMBOL_arrow_tr_filled: char = '\u{2b08}';
#[allow(unused, non_upper_case_globals)]
/// ⤤: North East Arrow With Hook
pub const SYMBOL_arrow_tr_hook: char = '\u{2924}';
#[allow(unused, non_upper_case_globals)]
/// ⬀: North East White Arrow
pub const SYMBOL_arrow_tr_stroked: char = '\u{2b00}';
#[allow(unused, non_upper_case_globals)]
/// ↘︎: South East Arrow
pub const SYMBOL_arrow_br: char = '\u{2198}';
#[allow(unused, non_upper_case_globals)]
/// 🢺: South East Arrow From Bar
pub const SYMBOL_arrow_br_bar: char = '\u{1f8ba}';
#[allow(unused, non_upper_case_globals)]
/// ⇘: South East Double Arrow
pub const SYMBOL_arrow_br_double: char = '\u{21d8}';
#[allow(unused, non_upper_case_globals)]
/// ⬊: South East Black Arrow
pub const SYMBOL_arrow_br_filled: char = '\u{2b0a}';
#[allow(unused, non_upper_case_globals)]
/// ⤥: South East Arrow With Hook
pub const SYMBOL_arrow_br_hook: char = '\u{2925}';
#[allow(unused, non_upper_case_globals)]
/// ⬂: South East White Arrow
pub const SYMBOL_arrow_br_stroked: char = '\u{2b02}';
#[allow(unused, non_upper_case_globals)]
/// ↖︎: North West Arrow
pub const SYMBOL_arrow_tl: char = '\u{2196}';
#[allow(unused, non_upper_case_globals)]
/// 🢸: North West Arrow From Bar
pub const SYMBOL_arrow_tl_bar: char = '\u{1f8b8}';
#[allow(unused, non_upper_case_globals)]
/// ⇖: North West Double Arrow
pub const SYMBOL_arrow_tl_double: char = '\u{21d6}';
#[allow(unused, non_upper_case_globals)]
/// ⬉: North West Black Arrow
pub const SYMBOL_arrow_tl_filled: char = '\u{2b09}';
#[allow(unused, non_upper_case_globals)]
/// ⤣: North West Arrow With Hook
pub const SYMBOL_arrow_tl_hook: char = '\u{2923}';
#[allow(unused, non_upper_case_globals)]
/// ⬁: North West White Arrow
pub const SYMBOL_arrow_tl_stroked: char = '\u{2b01}';
#[allow(unused, non_upper_case_globals)]
/// ↙︎: South West Arrow
pub const SYMBOL_arrow_bl: char = '\u{2199}';
#[allow(unused, non_upper_case_globals)]
/// 🢻: South West Arrow From Bar
pub const SYMBOL_arrow_bl_bar: char = '\u{1f8bb}';
#[allow(unused, non_upper_case_globals)]
/// ⇙: South West Double Arrow
pub const SYMBOL_arrow_bl_double: char = '\u{21d9}';
#[allow(unused, non_upper_case_globals)]
/// ⬋: South West Black Arrow
pub const SYMBOL_arrow_bl_filled: char = '\u{2b0b}';
#[allow(unused, non_upper_case_globals)]
/// ⤦: South West Arrow With Hook
pub const SYMBOL_arrow_bl_hook: char = '\u{2926}';
#[allow(unused, non_upper_case_globals)]
/// ⬃: South West White Arrow
pub const SYMBOL_arrow_bl_stroked: char = '\u{2b03}';
#[allow(unused, non_upper_case_globals)]
/// ⤡: North West And South East Arrow
pub const SYMBOL_arrow_tl_br: char = '\u{2921}';
#[allow(unused, non_upper_case_globals)]
/// ⤢: North East And South West Arrow
pub const SYMBOL_arrow_tr_bl: char = '\u{2922}';
#[allow(unused, non_upper_case_globals)]
/// ↺: Anticlockwise Open Circle Arrow
pub const SYMBOL_arrow_ccw: char = '\u{21ba}';
#[allow(unused, non_upper_case_globals)]
/// ↶: Anticlockwise Top Semicircle Arrow
pub const SYMBOL_arrow_ccw_half: char = '\u{21b6}';
#[allow(unused, non_upper_case_globals)]
/// ↻: Clockwise Open Circle Arrow
pub const SYMBOL_arrow_cw: char = '\u{21bb}';
#[allow(unused, non_upper_case_globals)]
/// ↷: Clockwise Top Semicircle Arrow
pub const SYMBOL_arrow_cw_half: char = '\u{21b7}';
#[allow(unused, non_upper_case_globals)]
/// ↯: Downwards Zigzag Arrow
pub const SYMBOL_arrow_zigzag: char = '\u{21af}';
#[allow(unused, non_upper_case_globals)]
/// ⌃: Up Arrowhead
pub const SYMBOL_arrowhead_t: char = '\u{2303}';
#[allow(unused, non_upper_case_globals)]
/// ⌄: Down Arrowhead
pub const SYMBOL_arrowhead_b: char = '\u{2304}';
#[allow(unused, non_upper_case_globals)]
/// ⇉: Rightwards Paired Arrows
pub const SYMBOL_arrows_rr: char = '\u{21c9}';
#[allow(unused, non_upper_case_globals)]
/// ⇇: Leftwards Paired Arrows
pub const SYMBOL_arrows_ll: char = '\u{21c7}';
#[allow(unused, non_upper_case_globals)]
/// ⇈: Upwards Paired Arrows
pub const SYMBOL_arrows_tt: char = '\u{21c8}';
#[allow(unused, non_upper_case_globals)]
/// ⇊: Downwards Paired Arrows
pub const SYMBOL_arrows_bb: char = '\u{21ca}';
#[allow(unused, non_upper_case_globals)]
/// ⇆: Leftwards Arrow Over Rightwards Arrow
pub const SYMBOL_arrows_lr: char = '\u{21c6}';
#[allow(unused, non_upper_case_globals)]
/// ↹: Leftwards Arrow To Bar Over Rightwards Arrow To Bar
pub const SYMBOL_arrows_lr_stop: char = '\u{21b9}';
#[allow(unused, non_upper_case_globals)]
/// ⇄: Rightwards Arrow Over Leftwards Arrow
pub const SYMBOL_arrows_rl: char = '\u{21c4}';
#[allow(unused, non_upper_case_globals)]
/// 🣐: null
pub const SYMBOL_arrows_rl_long: char = '\u{1f8d0}';
#[allow(unused, non_upper_case_globals)]
/// ⇅: Upwards Arrow Leftwards Of Downwards Arrow
pub const SYMBOL_arrows_tb: char = '\u{21c5}';
#[allow(unused, non_upper_case_globals)]
/// ⇵: Downwards Arrow Leftwards Of Upwards Arrow
pub const SYMBOL_arrows_bt: char = '\u{21f5}';
#[allow(unused, non_upper_case_globals)]
/// ⇶: Three Rightwards Arrows
pub const SYMBOL_arrows_rrr: char = '\u{21f6}';
#[allow(unused, non_upper_case_globals)]
/// ⬱: Three Leftwards Arrows
pub const SYMBOL_arrows_lll: char = '\u{2b31}';
#[allow(unused, non_upper_case_globals)]
/// ∗: Asterisk Operator
pub const SYMBOL_ast_op: char = '\u{2217}';
#[allow(unused, non_upper_case_globals)]
/// ⊛: Circled Asterisk Operator
pub const SYMBOL_ast_op_o: char = '\u{229b}';
#[allow(unused, non_upper_case_globals)]
/// *︎: Asterisk
pub const SYMBOL_ast_basic: char = '\u{2a}';
#[allow(unused, non_upper_case_globals)]
/// ⁎: Low Asterisk
pub const SYMBOL_ast_low: char = '\u{204e}';
#[allow(unused, non_upper_case_globals)]
/// ⁑: Two Asterisks Aligned Vertically
pub const SYMBOL_ast_double: char = '\u{2051}';
#[allow(unused, non_upper_case_globals)]
/// ⁂: Asterism
pub const SYMBOL_ast_triple: char = '\u{2042}';
#[allow(unused, non_upper_case_globals)]
/// ⧆: Squared Asterisk
pub const SYMBOL_ast_square: char = '\u{29c6}';
#[allow(unused, non_upper_case_globals)]
/// ≍: Equivalent To
pub const SYMBOL_asymp: char = '\u{224d}';
#[allow(unused, non_upper_case_globals)]
/// ≭: Not Equivalent To
pub const SYMBOL_asymp_not: char = '\u{226d}';
#[allow(unused, non_upper_case_globals)]
/// @: Commercial At
pub const SYMBOL_at: char = '\u{40}';
#[allow(unused, non_upper_case_globals)]
/// \: Reverse Solidus
pub const SYMBOL_backslash: char = '\u{5c}';
#[allow(unused, non_upper_case_globals)]
/// ⦸: Circled Reverse Solidus
pub const SYMBOL_backslash_o: char = '\u{29b8}';
#[allow(unused, non_upper_case_globals)]
/// ⧷: Reverse Solidus With Horizontal Stroke
pub const SYMBOL_backslash_not: char = '\u{29f7}';
#[allow(unused, non_upper_case_globals)]
/// ⟅: Left S Shaped Bag Delimiter
pub const SYMBOL_bag_l: char = '\u{27c5}';
#[allow(unused, non_upper_case_globals)]
/// ⟆: Right S Shaped Bag Delimiter
pub const SYMBOL_bag_r: char = '\u{27c6}';
#[allow(unused, non_upper_case_globals)]
/// ฿: Thai Currency Symbol Baht
pub const SYMBOL_baht: char = '\u{e3f}';
#[allow(unused, non_upper_case_globals)]
/// ☐: Ballot Box
pub const SYMBOL_ballot: char = '\u{2610}';
#[allow(unused, non_upper_case_globals)]
/// ☒: Ballot Box With X
pub const SYMBOL_ballot_cross: char = '\u{2612}';
#[allow(unused, non_upper_case_globals)]
/// ☑︎: Ballot Box With Check
pub const SYMBOL_ballot_check: char = '\u{2611}';
#[allow(unused, non_upper_case_globals)]
/// 🗹: Ballot Box With Bold Check
pub const SYMBOL_ballot_check_heavy: char = '\u{1f5f9}';
#[allow(unused, non_upper_case_globals)]
/// |: Vertical Line
pub const SYMBOL_bar_v: char = '\u{7c}';
#[allow(unused, non_upper_case_globals)]
/// ‖: Double Vertical Line
pub const SYMBOL_bar_v_double: char = '\u{2016}';
#[allow(unused, non_upper_case_globals)]
/// ⦀: Triple Vertical Bar Delimiter
pub const SYMBOL_bar_v_triple: char = '\u{2980}';
#[allow(unused, non_upper_case_globals)]
/// ¦: Broken Bar
pub const SYMBOL_bar_v_broken: char = '\u{a6}';
#[allow(unused, non_upper_case_globals)]
/// ⦶: Circled Vertical Bar
pub const SYMBOL_bar_v_o: char = '\u{29b6}';
#[allow(unused, non_upper_case_globals)]
/// ―: Horizontal Bar
pub const SYMBOL_bar_h: char = '\u{2015}';
#[allow(unused, non_upper_case_globals)]
/// ∵: Because
pub const SYMBOL_because: char = '\u{2235}';
#[allow(unused, non_upper_case_globals)]
/// β: Greek Small Letter Beta
pub const SYMBOL_beta: char = '\u{3b2}';
#[allow(unused, non_upper_case_globals)]
/// ϐ: Greek Beta Symbol
pub const SYMBOL_beta_alt: char = '\u{3d0}';
#[allow(unused, non_upper_case_globals)]
/// ב: Hebrew Letter Bet
pub const SYMBOL_beth: char = '\u{5d1}';
#[allow(unused, non_upper_case_globals)]
/// ₿: Bitcoin Sign
pub const SYMBOL_bitcoin: char = '\u{20bf}';
#[allow(unused, non_upper_case_globals)]
/// ⊥: Up Tack
pub const SYMBOL_bot: char = '\u{22a5}';
#[allow(unused, non_upper_case_globals)]
/// ⋈: Bowtie
pub const SYMBOL_bowtie_stroked: char = '\u{22c8}';
#[allow(unused, non_upper_case_globals)]
/// ⨝: Join
pub const SYMBOL_bowtie_stroked_big: char = '\u{2a1d}';
#[allow(unused, non_upper_case_globals)]
/// ⟕: Left Outer Join
pub const SYMBOL_bowtie_stroked_big_l: char = '\u{27d5}';
#[allow(unused, non_upper_case_globals)]
/// ⟖: Right Outer Join
pub const SYMBOL_bowtie_stroked_big_r: char = '\u{27d6}';
#[allow(unused, non_upper_case_globals)]
/// ⟗: Full Outer Join
pub const SYMBOL_bowtie_stroked_big_l_r: char = '\u{27d7}';
#[allow(unused, non_upper_case_globals)]
/// ⧓: Black Bowtie
pub const SYMBOL_bowtie_filled: char = '\u{29d3}';
#[allow(unused, non_upper_case_globals)]
/// ⧑: Bowtie With Left Half Black
pub const SYMBOL_bowtie_filled_l: char = '\u{29d1}';
#[allow(unused, non_upper_case_globals)]
/// ⧒: Bowtie With Right Half Black
pub const SYMBOL_bowtie_filled_r: char = '\u{29d2}';
#[allow(unused, non_upper_case_globals)]
/// {: Left Curly Bracket
pub const SYMBOL_brace_l: char = '\u{7b}';
#[allow(unused, non_upper_case_globals)]
/// ⦃: Left White Curly Bracket
pub const SYMBOL_brace_l_stroked: char = '\u{2983}';
#[allow(unused, non_upper_case_globals)]
/// }: Right Curly Bracket
pub const SYMBOL_brace_r: char = '\u{7d}';
#[allow(unused, non_upper_case_globals)]
/// ⦄: Right White Curly Bracket
pub const SYMBOL_brace_r_stroked: char = '\u{2984}';
#[allow(unused, non_upper_case_globals)]
/// ⏞: Top Curly Bracket
pub const SYMBOL_brace_t: char = '\u{23de}';
#[allow(unused, non_upper_case_globals)]
/// ⏟: Bottom Curly Bracket
pub const SYMBOL_brace_b: char = '\u{23df}';
#[allow(unused, non_upper_case_globals)]
/// [: Left Square Bracket
pub const SYMBOL_bracket_l: char = '\u{5b}';
#[allow(unused, non_upper_case_globals)]
/// ⦍: Left Square Bracket With Tick In Top Corner
pub const SYMBOL_bracket_l_tick_t: char = '\u{298d}';
#[allow(unused, non_upper_case_globals)]
/// ⦏: Left Square Bracket With Tick In Bottom Corner
pub const SYMBOL_bracket_l_tick_b: char = '\u{298f}';
#[allow(unused, non_upper_case_globals)]
/// ⟦: Mathematical Left White Square Bracket
pub const SYMBOL_bracket_l_stroked: char = '\u{27e6}';
#[allow(unused, non_upper_case_globals)]
/// ]: Right Square Bracket
pub const SYMBOL_bracket_r: char = '\u{5d}';
#[allow(unused, non_upper_case_globals)]
/// ⦐: Right Square Bracket With Tick In Top Corner
pub const SYMBOL_bracket_r_tick_t: char = '\u{2990}';
#[allow(unused, non_upper_case_globals)]
/// ⦎: Right Square Bracket With Tick In Bottom Corner
pub const SYMBOL_bracket_r_tick_b: char = '\u{298e}';
#[allow(unused, non_upper_case_globals)]
/// ⟧: Mathematical Right White Square Bracket
pub const SYMBOL_bracket_r_stroked: char = '\u{27e7}';
#[allow(unused, non_upper_case_globals)]
/// ⎴: Top Square Bracket
pub const SYMBOL_bracket_t: char = '\u{23b4}';
#[allow(unused, non_upper_case_globals)]
/// ⎵: Bottom Square Bracket
pub const SYMBOL_bracket_b: char = '\u{23b5}';
#[allow(unused, non_upper_case_globals)]
/// ˘: Breve
pub const SYMBOL_breve: char = '\u{2d8}';
#[allow(unused, non_upper_case_globals)]
/// •: Bullet
pub const SYMBOL_bullet: char = '\u{2022}';
#[allow(unused, non_upper_case_globals)]
/// ∙: Bullet Operator
pub const SYMBOL_bullet_op: char = '\u{2219}';
#[allow(unused, non_upper_case_globals)]
/// ⦿: Circled Bullet
pub const SYMBOL_bullet_o: char = '\u{29bf}';
#[allow(unused, non_upper_case_globals)]
/// ◦: White Bullet
pub const SYMBOL_bullet_stroked: char = '\u{25e6}';
#[allow(unused, non_upper_case_globals)]
/// ⦾: Circled White Bullet
pub const SYMBOL_bullet_stroked_o: char = '\u{29be}';
#[allow(unused, non_upper_case_globals)]
/// ◘: Inverse Bullet
pub const SYMBOL_bullet_hole: char = '\u{25d8}';
#[allow(unused, non_upper_case_globals)]
/// ⁃: Hyphen Bullet
pub const SYMBOL_bullet_hyph: char = '\u{2043}';
#[allow(unused, non_upper_case_globals)]
/// ‣: Triangular Bullet
pub const SYMBOL_bullet_tri: char = '\u{2023}';
#[allow(unused, non_upper_case_globals)]
/// ⁌: Black Leftwards Bullet
pub const SYMBOL_bullet_l: char = '\u{204c}';
#[allow(unused, non_upper_case_globals)]
/// ⁍: Black Rightwards Bullet
pub const SYMBOL_bullet_r: char = '\u{204d}';
#[allow(unused, non_upper_case_globals)]
/// ‸: Caret
pub const SYMBOL_caret: char = '\u{2038}';
#[allow(unused, non_upper_case_globals)]
/// ˇ: Caron
pub const SYMBOL_caron: char = '\u{2c7}';
#[allow(unused, non_upper_case_globals)]
/// 🅭: Circled Cc
pub const SYMBOL_cc: char = '\u{1f16d}';
#[allow(unused, non_upper_case_globals)]
/// 🅯: Circled Human Figure
pub const SYMBOL_cc_by: char = '\u{1f16f}';
#[allow(unused, non_upper_case_globals)]
/// 🄏: Circled Dollar Sign With Overlaid Backslash
pub const SYMBOL_cc_nc: char = '\u{1f10f}';
#[allow(unused, non_upper_case_globals)]
/// ⊜: Circled Equals
pub const SYMBOL_cc_nd: char = '\u{229c}';
#[allow(unused, non_upper_case_globals)]
/// 🅮: Circled C With Overlaid Backslash
pub const SYMBOL_cc_public: char = '\u{1f16e}';
#[allow(unused, non_upper_case_globals)]
/// 🄎: Circled Anticlockwise Arrow
pub const SYMBOL_cc_sa: char = '\u{1f10e}';
#[allow(unused, non_upper_case_globals)]
/// 🄍: Circled Zero With Slash
pub const SYMBOL_cc_zero: char = '\u{1f10d}';
#[allow(unused, non_upper_case_globals)]
/// ₵: Cedi Sign
pub const SYMBOL_cedi: char = '\u{20b5}';
#[allow(unused, non_upper_case_globals)]
/// ⌈: Left Ceiling
pub const SYMBOL_ceil_l: char = '\u{2308}';
#[allow(unused, non_upper_case_globals)]
/// ⌉: Right Ceiling
pub const SYMBOL_ceil_r: char = '\u{2309}';
#[allow(unused, non_upper_case_globals)]
/// ¢: Cent Sign
pub const SYMBOL_cent: char = '\u{a2}';
#[allow(unused, non_upper_case_globals)]
/// ✓: Check Mark
pub const SYMBOL_checkmark: char = '\u{2713}';
#[allow(unused, non_upper_case_globals)]
/// 🗸: Light Check Mark
pub const SYMBOL_checkmark_light: char = '\u{1f5f8}';
#[allow(unused, non_upper_case_globals)]
/// ✔︎: Heavy Check Mark
pub const SYMBOL_checkmark_heavy: char = '\u{2714}';
#[allow(unused, non_upper_case_globals)]
/// ⟨: Mathematical Left Angle Bracket
pub const SYMBOL_chevron_l: char = '\u{27e8}';
#[allow(unused, non_upper_case_globals)]
/// ⧼: Left Pointing Curved Angle Bracket
pub const SYMBOL_chevron_l_curly: char = '\u{29fc}';
#[allow(unused, non_upper_case_globals)]
/// ⦑: Left Angle Bracket With Dot
pub const SYMBOL_chevron_l_dot: char = '\u{2991}';
#[allow(unused, non_upper_case_globals)]
/// ⦉: Z Notation Left Binding Bracket
pub const SYMBOL_chevron_l_closed: char = '\u{2989}';
#[allow(unused, non_upper_case_globals)]
/// ⟪: Mathematical Left Double Angle Bracket
pub const SYMBOL_chevron_l_double: char = '\u{27ea}';
#[allow(unused, non_upper_case_globals)]
/// ⟩: Mathematical Right Angle Bracket
pub const SYMBOL_chevron_r: char = '\u{27e9}';
#[allow(unused, non_upper_case_globals)]
/// ⧽: Right Pointing Curved Angle Bracket
pub const SYMBOL_chevron_r_curly: char = '\u{29fd}';
#[allow(unused, non_upper_case_globals)]
/// ⦒: Right Angle Bracket With Dot
pub const SYMBOL_chevron_r_dot: char = '\u{2992}';
#[allow(unused, non_upper_case_globals)]
/// ⦊: Z Notation Right Binding Bracket
pub const SYMBOL_chevron_r_closed: char = '\u{298a}';
#[allow(unused, non_upper_case_globals)]
/// ⟫: Mathematical Right Double Angle Bracket
pub const SYMBOL_chevron_r_double: char = '\u{27eb}';
#[allow(unused, non_upper_case_globals)]
/// χ: Greek Small Letter Chi
pub const SYMBOL_chi: char = '\u{3c7}';
#[allow(unused, non_upper_case_globals)]
/// ○: White Circle
pub const SYMBOL_circle_stroked: char = '\u{25cb}';
#[allow(unused, non_upper_case_globals)]
/// ∘: Ring Operator
pub const SYMBOL_circle_stroked_tiny: char = '\u{2218}';
#[allow(unused, non_upper_case_globals)]
/// ⚬: Medium Small White Circle
pub const SYMBOL_circle_stroked_small: char = '\u{26ac}';
#[allow(unused, non_upper_case_globals)]
/// ◯: Large Circle
pub const SYMBOL_circle_stroked_big: char = '\u{25ef}';
#[allow(unused, non_upper_case_globals)]
/// ●: Black Circle
pub const SYMBOL_circle_filled: char = '\u{25cf}';
#[allow(unused, non_upper_case_globals)]
/// ⦁: Z Notation Spot
pub const SYMBOL_circle_filled_tiny: char = '\u{2981}';
#[allow(unused, non_upper_case_globals)]
/// ∙: Bullet Operator
pub const SYMBOL_circle_filled_small: char = '\u{2219}';
#[allow(unused, non_upper_case_globals)]
/// ⬤: Black Large Circle
pub const SYMBOL_circle_filled_big: char = '\u{2b24}';
#[allow(unused, non_upper_case_globals)]
/// ◌: Dotted Circle
pub const SYMBOL_circle_dotted: char = '\u{25cc}';
#[allow(unused, non_upper_case_globals)]
/// ℅: Care Of
pub const SYMBOL_co: char = '\u{2105}';
#[allow(unused, non_upper_case_globals)]
/// :: Colon
pub const SYMBOL_colon: char = '\u{3a}';
#[allow(unused, non_upper_case_globals)]
/// ₡: Colon Sign
pub const SYMBOL_colon_currency: char = '\u{20a1}';
#[allow(unused, non_upper_case_globals)]
/// ∷: Proportion
pub const SYMBOL_colon_double: char = '\u{2237}';
#[allow(unused, non_upper_case_globals)]
/// ⁝: Tricolon
pub const SYMBOL_colon_tri: char = '\u{205d}';
#[allow(unused, non_upper_case_globals)]
/// ⫶: Triple Colon Operator
pub const SYMBOL_colon_tri_op: char = '\u{2af6}';
#[allow(unused, non_upper_case_globals)]
/// ≔: Colon Equals
pub const SYMBOL_colon_eq: char = '\u{2254}';
#[allow(unused, non_upper_case_globals)]
/// ⩴: Double Colon Equal
pub const SYMBOL_colon_double_eq: char = '\u{2a74}';
#[allow(unused, non_upper_case_globals)]
/// ,: Comma
pub const SYMBOL_comma: char = '\u{2c}';
#[allow(unused, non_upper_case_globals)]
/// ⸲: Turned Comma
pub const SYMBOL_comma_inv: char = '\u{2e32}';
#[allow(unused, non_upper_case_globals)]
/// ⹁: Reversed Comma
pub const SYMBOL_comma_rev: char = '\u{2e41}';
#[allow(unused, non_upper_case_globals)]
/// ∁: Complement
pub const SYMBOL_complement: char = '\u{2201}';
#[allow(unused, non_upper_case_globals)]
/// ∘: Ring Operator
pub const SYMBOL_compose: char = '\u{2218}';
#[allow(unused, non_upper_case_globals)]
/// ⊚: Circled Ring Operator
pub const SYMBOL_compose_o: char = '\u{229a}';
#[allow(unused, non_upper_case_globals)]
/// ␆: Symbol For Acknowledge
pub const SYMBOL_control_ack: char = '\u{2406}';
#[allow(unused, non_upper_case_globals)]
/// ␇: Symbol For Bell
pub const SYMBOL_control_bel: char = '\u{2407}';
#[allow(unused, non_upper_case_globals)]
/// ␈: Symbol For Backspace
pub const SYMBOL_control_bs: char = '\u{2408}';
#[allow(unused, non_upper_case_globals)]
/// ␘: Symbol For Cancel
pub const SYMBOL_control_can: char = '\u{2418}';
#[allow(unused, non_upper_case_globals)]
/// ␍: Symbol For Carriage Return
pub const SYMBOL_control_cr: char = '\u{240d}';
#[allow(unused, non_upper_case_globals)]
/// ␑: Symbol For Device Control One
pub const SYMBOL_control_dc_one: char = '\u{2411}';
#[allow(unused, non_upper_case_globals)]
/// ␒: Symbol For Device Control Two
pub const SYMBOL_control_dc_two: char = '\u{2412}';
#[allow(unused, non_upper_case_globals)]
/// ␓: Symbol For Device Control Three
pub const SYMBOL_control_dc_three: char = '\u{2413}';
#[allow(unused, non_upper_case_globals)]
/// ␔: Symbol For Device Control Four
pub const SYMBOL_control_dc_four: char = '\u{2414}';
#[allow(unused, non_upper_case_globals)]
/// ␡: Symbol For Delete
pub const SYMBOL_control_del: char = '\u{2421}';
#[allow(unused, non_upper_case_globals)]
/// ␐: Symbol For Data Link Escape
pub const SYMBOL_control_dle: char = '\u{2410}';
#[allow(unused, non_upper_case_globals)]
/// ␙: Symbol For End Of Medium
pub const SYMBOL_control_em: char = '\u{2419}';
#[allow(unused, non_upper_case_globals)]
/// ␅: Symbol For Enquiry
pub const SYMBOL_control_enq: char = '\u{2405}';
#[allow(unused, non_upper_case_globals)]
/// ␄: Symbol For End Of Transmission
pub const SYMBOL_control_eot: char = '\u{2404}';
#[allow(unused, non_upper_case_globals)]
/// ␛: Symbol For Escape
pub const SYMBOL_control_esc: char = '\u{241b}';
#[allow(unused, non_upper_case_globals)]
/// ␗: Symbol For End Of Transmission Block
pub const SYMBOL_control_etb: char = '\u{2417}';
#[allow(unused, non_upper_case_globals)]
/// ␃: Symbol For End Of Text
pub const SYMBOL_control_etx: char = '\u{2403}';
#[allow(unused, non_upper_case_globals)]
/// ␌: Symbol For Form Feed
pub const SYMBOL_control_ff: char = '\u{240c}';
#[allow(unused, non_upper_case_globals)]
/// ␜: Symbol For File Separator
pub const SYMBOL_control_fs: char = '\u{241c}';
#[allow(unused, non_upper_case_globals)]
/// ␝: Symbol For Group Separator
pub const SYMBOL_control_gs: char = '\u{241d}';
#[allow(unused, non_upper_case_globals)]
/// ␉: Symbol For Horizontal Tabulation
pub const SYMBOL_control_ht: char = '\u{2409}';
#[allow(unused, non_upper_case_globals)]
/// ␊: Symbol For Line Feed
pub const SYMBOL_control_lf: char = '\u{240a}';
#[allow(unused, non_upper_case_globals)]
/// ␕: Symbol For Negative Acknowledge
pub const SYMBOL_control_nak: char = '\u{2415}';
#[allow(unused, non_upper_case_globals)]
/// ␤: Symbol For Newline
pub const SYMBOL_control_nl: char = '\u{2424}';
#[allow(unused, non_upper_case_globals)]
/// ␀: Symbol For Null
pub const SYMBOL_control_nul: char = '\u{2400}';
#[allow(unused, non_upper_case_globals)]
/// ␞: Symbol For Record Separator
pub const SYMBOL_control_rs: char = '\u{241e}';
#[allow(unused, non_upper_case_globals)]
/// ␏: Symbol For Shift In
pub const SYMBOL_control_si: char = '\u{240f}';
#[allow(unused, non_upper_case_globals)]
/// ␎: Symbol For Shift Out
pub const SYMBOL_control_so: char = '\u{240e}';
#[allow(unused, non_upper_case_globals)]
/// ␁: Symbol For Start Of Heading
pub const SYMBOL_control_soh: char = '\u{2401}';
#[allow(unused, non_upper_case_globals)]
/// ␠: Symbol For Space
pub const SYMBOL_control_sp: char = '\u{2420}';
#[allow(unused, non_upper_case_globals)]
/// ␂: Symbol For Start Of Text
pub const SYMBOL_control_stx: char = '\u{2402}';
#[allow(unused, non_upper_case_globals)]
/// ␚: Symbol For Substitute
pub const SYMBOL_control_sub: char = '\u{241a}';
#[allow(unused, non_upper_case_globals)]
/// ␖: Symbol For Synchronous Idle
pub const SYMBOL_control_syn: char = '\u{2416}';
#[allow(unused, non_upper_case_globals)]
/// ␟: Symbol For Unit Separator
pub const SYMBOL_control_us: char = '\u{241f}';
#[allow(unused, non_upper_case_globals)]
/// ␋: Symbol For Vertical Tabulation
pub const SYMBOL_control_vt: char = '\u{240b}';
#[allow(unused, non_upper_case_globals)]
/// ∗: Asterisk Operator
pub const SYMBOL_convolve: char = '\u{2217}';
#[allow(unused, non_upper_case_globals)]
/// ⊛: Circled Asterisk Operator
pub const SYMBOL_convolve_o: char = '\u{229b}';
#[allow(unused, non_upper_case_globals)]
/// 🄯: Copyleft Symbol
pub const SYMBOL_copyleft: char = '\u{1f12f}';
#[allow(unused, non_upper_case_globals)]
/// ©︎: Copyright Sign
pub const SYMBOL_copyright: char = '\u{a9}';
#[allow(unused, non_upper_case_globals)]
/// ℗: Sound Recording Copyright
pub const SYMBOL_copyright_sound: char = '\u{2117}';
#[allow(unused, non_upper_case_globals)]
/// ⌜: Top Left Corner
pub const SYMBOL_corner_l_t: char = '\u{231c}';
#[allow(unused, non_upper_case_globals)]
/// ⌞: Bottom Left Corner
pub const SYMBOL_corner_l_b: char = '\u{231e}';
#[allow(unused, non_upper_case_globals)]
/// ⌝: Top Right Corner
pub const SYMBOL_corner_r_t: char = '\u{231d}';
#[allow(unused, non_upper_case_globals)]
/// ⌟: Bottom Right Corner
pub const SYMBOL_corner_r_b: char = '\u{231f}';
#[allow(unused, non_upper_case_globals)]
/// ✗: Ballot X
pub const SYMBOL_crossmark: char = '\u{2717}';
#[allow(unused, non_upper_case_globals)]
/// ✘: Heavy Ballot X
pub const SYMBOL_crossmark_heavy: char = '\u{2718}';
#[allow(unused, non_upper_case_globals)]
/// ¤: Currency Sign
pub const SYMBOL_currency: char = '\u{a4}';
#[allow(unused, non_upper_case_globals)]
/// †: Dagger
pub const SYMBOL_dagger: char = '\u{2020}';
#[allow(unused, non_upper_case_globals)]
/// ‡: Double Dagger
pub const SYMBOL_dagger_double: char = '\u{2021}';
#[allow(unused, non_upper_case_globals)]
/// ⹋: Triple Dagger
pub const SYMBOL_dagger_triple: char = '\u{2e4b}';
#[allow(unused, non_upper_case_globals)]
/// ⸶: Dagger With Left Guard
pub const SYMBOL_dagger_l: char = '\u{2e36}';
#[allow(unused, non_upper_case_globals)]
/// ⸷: Dagger With Right Guard
pub const SYMBOL_dagger_r: char = '\u{2e37}';
#[allow(unused, non_upper_case_globals)]
/// ⸸: Turned Dagger
pub const SYMBOL_dagger_inv: char = '\u{2e38}';
#[allow(unused, non_upper_case_globals)]
/// ד: Hebrew Letter Dalet
pub const SYMBOL_daleth: char = '\u{5d3}';
#[allow(unused, non_upper_case_globals)]
/// –: En Dash
pub const SYMBOL_dash_en: char = '\u{2013}';
#[allow(unused, non_upper_case_globals)]
/// —: Em Dash
pub const SYMBOL_dash_em: char = '\u{2014}';
#[allow(unused, non_upper_case_globals)]
/// ⸺: Two Em Dash
pub const SYMBOL_dash_em_two: char = '\u{2e3a}';
#[allow(unused, non_upper_case_globals)]
/// ⸻: Three Em Dash
pub const SYMBOL_dash_em_three: char = '\u{2e3b}';
#[allow(unused, non_upper_case_globals)]
/// ‒: Figure Dash
pub const SYMBOL_dash_fig: char = '\u{2012}';
#[allow(unused, non_upper_case_globals)]
/// ∹: Excess
pub const SYMBOL_dash_colon: char = '\u{2239}';
#[allow(unused, non_upper_case_globals)]
/// ⊝: Circled Dash
pub const SYMBOL_dash_o: char = '\u{229d}';
#[allow(unused, non_upper_case_globals)]
/// 〜: Wave Dash
pub const SYMBOL_dash_wave: char = '\u{301c}';
#[allow(unused, non_upper_case_globals)]
/// 〰︎: Wavy Dash
pub const SYMBOL_dash_wave_double: char = '\u{3030}';
#[allow(unused, non_upper_case_globals)]
/// °: Degree Sign
pub const SYMBOL_degree: char = '\u{b0}';
#[allow(unused, non_upper_case_globals)]
/// δ: Greek Small Letter Delta
pub const SYMBOL_delta: char = '\u{3b4}';
#[allow(unused, non_upper_case_globals)]
/// ¨: Diaeresis
pub const SYMBOL_diaer: char = '\u{a8}';
#[allow(unused, non_upper_case_globals)]
/// ⌀: Diameter Sign
pub const SYMBOL_diameter: char = '\u{2300}';
#[allow(unused, non_upper_case_globals)]
/// ◇: White Diamond
pub const SYMBOL_diamond_stroked: char = '\u{25c7}';
#[allow(unused, non_upper_case_globals)]
/// ⋄: Diamond Operator
pub const SYMBOL_diamond_stroked_small: char = '\u{22c4}';
#[allow(unused, non_upper_case_globals)]
/// ⬦: White Medium Diamond
pub const SYMBOL_diamond_stroked_medium: char = '\u{2b26}';
#[allow(unused, non_upper_case_globals)]
/// ⟐: White Diamond With Centred Dot
pub const SYMBOL_diamond_stroked_dot: char = '\u{27d0}';
#[allow(unused, non_upper_case_globals)]
/// ◆: Black Diamond
pub const SYMBOL_diamond_filled: char = '\u{25c6}';
#[allow(unused, non_upper_case_globals)]
/// ⬥: Black Medium Diamond
pub const SYMBOL_diamond_filled_medium: char = '\u{2b25}';
#[allow(unused, non_upper_case_globals)]
/// ⬩: Black Small Diamond
pub const SYMBOL_diamond_filled_small: char = '\u{2b29}';
#[allow(unused, non_upper_case_globals)]
/// ⚅: Die Face 6
pub const SYMBOL_die_six: char = '\u{2685}';
#[allow(unused, non_upper_case_globals)]
/// ⚄: Die Face 5
pub const SYMBOL_die_five: char = '\u{2684}';
#[allow(unused, non_upper_case_globals)]
/// ⚃: Die Face 4
pub const SYMBOL_die_four: char = '\u{2683}';
#[allow(unused, non_upper_case_globals)]
/// ⚂: Die Face 3
pub const SYMBOL_die_three: char = '\u{2682}';
#[allow(unused, non_upper_case_globals)]
/// ⚁: Die Face 2
pub const SYMBOL_die_two: char = '\u{2681}';
#[allow(unused, non_upper_case_globals)]
/// ⚀: Die Face 1
pub const SYMBOL_die_one: char = '\u{2680}';
#[allow(unused, non_upper_case_globals)]
/// ϝ: Greek Small Letter Digamma
pub const SYMBOL_digamma: char = '\u{3dd}';
#[allow(unused, non_upper_case_globals)]
/// ÷: Division Sign
pub const SYMBOL_div: char = '\u{f7}';
#[allow(unused, non_upper_case_globals)]
/// ⨸: Circled Division Sign
pub const SYMBOL_div_o: char = '\u{2a38}';
#[allow(unused, non_upper_case_globals)]
/// ⦼: Circled Anticlockwise Rotated Division Sign
pub const SYMBOL_div_slanted_o: char = '\u{29bc}';
#[allow(unused, non_upper_case_globals)]
/// ∣: Divides
pub const SYMBOL_divides: char = '\u{2223}';
#[allow(unused, non_upper_case_globals)]
/// ∤: Does Not Divide
pub const SYMBOL_divides_not: char = '\u{2224}';
#[allow(unused, non_upper_case_globals)]
/// ⫮: Does Not Divide With Reversed Negation Slash
pub const SYMBOL_divides_not_rev: char = '\u{2aee}';
#[allow(unused, non_upper_case_globals)]
/// ⟊: Vertical Bar With Horizontal Stroke
pub const SYMBOL_divides_struck: char = '\u{27ca}';
#[allow(unused, non_upper_case_globals)]
/// $: Dollar Sign
pub const SYMBOL_dollar: char = '\u{24}';
#[allow(unused, non_upper_case_globals)]
/// ₫: Dong Sign
pub const SYMBOL_dong: char = '\u{20ab}';
#[allow(unused, non_upper_case_globals)]
/// ߾: Nko Dorome Sign
pub const SYMBOL_dorome: char = '\u{7fe}';
#[allow(unused, non_upper_case_globals)]
/// ⋅: Dot Operator
pub const SYMBOL_dot_op: char = '\u{22c5}';
#[allow(unused, non_upper_case_globals)]
/// .: Full Stop
pub const SYMBOL_dot_basic: char = '\u{2e}';
#[allow(unused, non_upper_case_globals)]
/// ·: Middle Dot
pub const SYMBOL_dot_c: char = '\u{b7}';
#[allow(unused, non_upper_case_globals)]
/// ⊙: Circled Dot Operator
pub const SYMBOL_dot_o: char = '\u{2299}';
#[allow(unused, non_upper_case_globals)]
/// ⨀: N Ary Circled Dot Operator
pub const SYMBOL_dot_o_big: char = '\u{2a00}';
#[allow(unused, non_upper_case_globals)]
/// ⊡: Squared Dot Operator
pub const SYMBOL_dot_square: char = '\u{22a1}';
#[allow(unused, non_upper_case_globals)]
/// ¨: Diaeresis
pub const SYMBOL_dot_double: char = '\u{a8}';
#[allow(unused, non_upper_case_globals)]
/// ⃛: Combining Three Dots Above
pub const SYMBOL_dot_triple: char = '\u{20db}';
#[allow(unused, non_upper_case_globals)]
/// ⃜: Combining Four Dots Above
pub const SYMBOL_dot_quad: char = '\u{20dc}';
#[allow(unused, non_upper_case_globals)]
/// ı: Latin Small Letter Dotless I
pub const SYMBOL_dotless_i: char = '\u{131}';
#[allow(unused, non_upper_case_globals)]
/// ȷ: Latin Small Letter Dotless J
pub const SYMBOL_dotless_j: char = '\u{237}';
#[allow(unused, non_upper_case_globals)]
/// ⋯: Midline Horizontal Ellipsis
pub const SYMBOL_dots_h_c: char = '\u{22ef}';
#[allow(unused, non_upper_case_globals)]
/// …: Horizontal Ellipsis
pub const SYMBOL_dots_h: char = '\u{2026}';
#[allow(unused, non_upper_case_globals)]
/// ⋮: Vertical Ellipsis
pub const SYMBOL_dots_v: char = '\u{22ee}';
#[allow(unused, non_upper_case_globals)]
/// ⋱: Down Right Diagonal Ellipsis
pub const SYMBOL_dots_down: char = '\u{22f1}';
#[allow(unused, non_upper_case_globals)]
/// ⋰: Up Right Diagonal Ellipsis
pub const SYMBOL_dots_up: char = '\u{22f0}';
#[allow(unused, non_upper_case_globals)]
/// ֏: Armenian Dram Sign
pub const SYMBOL_dram: char = '\u{58f}';
#[allow(unused, non_upper_case_globals)]
/// 🜨: Alchemical Symbol For Verdigris
pub const SYMBOL_earth: char = '\u{1f728}';
#[allow(unused, non_upper_case_globals)]
/// ♁: Earth
pub const SYMBOL_earth_alt: char = '\u{2641}';
#[allow(unused, non_upper_case_globals)]
/// ℓ: Script Small L
pub const SYMBOL_ell: char = '\u{2113}';
#[allow(unused, non_upper_case_globals)]
/// ⬭: White Horizontal Ellipse
pub const SYMBOL_ellipse_stroked_h: char = '\u{2b2d}';
#[allow(unused, non_upper_case_globals)]
/// ⬯: White Vertical Ellipse
pub const SYMBOL_ellipse_stroked_v: char = '\u{2b2f}';
#[allow(unused, non_upper_case_globals)]
/// ⬬: Black Horizontal Ellipse
pub const SYMBOL_ellipse_filled_h: char = '\u{2b2c}';
#[allow(unused, non_upper_case_globals)]
/// ⬮: Black Vertical Ellipse
pub const SYMBOL_ellipse_filled_v: char = '\u{2b2e}';
#[allow(unused, non_upper_case_globals)]
/// ∅: Empty Set
pub const SYMBOL_emptyset: char = '\u{2205}';
#[allow(unused, non_upper_case_globals)]
/// ∅︀: Empty Set
pub const SYMBOL_emptyset_zero: char = '\u{2205}';
#[allow(unused, non_upper_case_globals)]
/// ⦳: Empty Set With Right Arrow Above
pub const SYMBOL_emptyset_arrow_r: char = '\u{29b3}';
#[allow(unused, non_upper_case_globals)]
/// ⦴: Empty Set With Left Arrow Above
pub const SYMBOL_emptyset_arrow_l: char = '\u{29b4}';
#[allow(unused, non_upper_case_globals)]
/// ⦱: Empty Set With Overbar
pub const SYMBOL_emptyset_bar: char = '\u{29b1}';
#[allow(unused, non_upper_case_globals)]
/// ⦲: Empty Set With Small Circle Above
pub const SYMBOL_emptyset_circle: char = '\u{29b2}';
#[allow(unused, non_upper_case_globals)]
/// ⦰: Reversed Empty Set
pub const SYMBOL_emptyset_rev: char = '\u{29b0}';
#[allow(unused, non_upper_case_globals)]
/// ε: Greek Small Letter Epsilon
pub const SYMBOL_epsilon: char = '\u{3b5}';
#[allow(unused, non_upper_case_globals)]
/// ϵ: Greek Lunate Epsilon Symbol
pub const SYMBOL_epsilon_alt: char = '\u{3f5}';
#[allow(unused, non_upper_case_globals)]
/// ϶: Greek Reversed Lunate Epsilon Symbol
pub const SYMBOL_epsilon_alt_rev: char = '\u{3f6}';
#[allow(unused, non_upper_case_globals)]
/// =: Equals Sign
pub const SYMBOL_eq: char = '\u{3d}';
#[allow(unused, non_upper_case_globals)]
/// ⩮: Equals With Asterisk
pub const SYMBOL_eq_ast: char = '\u{2a6e}';
#[allow(unused, non_upper_case_globals)]
/// ≛: Star Equals
pub const SYMBOL_eq_star: char = '\u{225b}';
#[allow(unused, non_upper_case_globals)]
/// ⊜: Circled Equals
pub const SYMBOL_eq_o: char = '\u{229c}';
#[allow(unused, non_upper_case_globals)]
/// ≕: Equals Colon
pub const SYMBOL_eq_colon: char = '\u{2255}';
#[allow(unused, non_upper_case_globals)]
/// ≐: Approaches The Limit
pub const SYMBOL_eq_dot: char = '\u{2250}';
#[allow(unused, non_upper_case_globals)]
/// ≑: Geometrically Equal To
pub const SYMBOL_eq_dots: char = '\u{2251}';
#[allow(unused, non_upper_case_globals)]
/// ≒: Approximately Equal To Or The Image Of
pub const SYMBOL_eq_dots_down: char = '\u{2252}';
#[allow(unused, non_upper_case_globals)]
/// ≓: Image Of Or Approximately Equal To
pub const SYMBOL_eq_dots_up: char = '\u{2253}';
#[allow(unused, non_upper_case_globals)]
/// ≝: Equal To By Definition
pub const SYMBOL_eq_def: char = '\u{225d}';
#[allow(unused, non_upper_case_globals)]
/// ≜: Delta Equal To
pub const SYMBOL_eq_delta: char = '\u{225c}';
#[allow(unused, non_upper_case_globals)]
/// ≚: Equiangular To
pub const SYMBOL_eq_equi: char = '\u{225a}';
#[allow(unused, non_upper_case_globals)]
/// ≙: Estimates
pub const SYMBOL_eq_est: char = '\u{2259}';
#[allow(unused, non_upper_case_globals)]
/// ⋝: Equal To Or Greater Than
pub const SYMBOL_eq_gt: char = '\u{22dd}';
#[allow(unused, non_upper_case_globals)]
/// ⋜: Equal To Or Less Than
pub const SYMBOL_eq_lt: char = '\u{22dc}';
#[allow(unused, non_upper_case_globals)]
/// ≞: Measured By
pub const SYMBOL_eq_m: char = '\u{225e}';
#[allow(unused, non_upper_case_globals)]
/// ≠: Not Equal To
pub const SYMBOL_eq_not: char = '\u{2260}';
#[allow(unused, non_upper_case_globals)]
/// ⋞: Equal To Or Precedes
pub const SYMBOL_eq_prec: char = '\u{22de}';
#[allow(unused, non_upper_case_globals)]
/// ≟: Questioned Equal To
pub const SYMBOL_eq_quest: char = '\u{225f}';
#[allow(unused, non_upper_case_globals)]
/// ⋟: Equal To Or Succeeds
pub const SYMBOL_eq_succ: char = '\u{22df}';
#[allow(unused, non_upper_case_globals)]
/// ≡: Identical To
pub const SYMBOL_eq_triple: char = '\u{2261}';
#[allow(unused, non_upper_case_globals)]
/// ≢: Not Identical To
pub const SYMBOL_eq_triple_not: char = '\u{2262}';
#[allow(unused, non_upper_case_globals)]
/// ≣: Strictly Equivalent To
pub const SYMBOL_eq_quad: char = '\u{2263}';
#[allow(unused, non_upper_case_globals)]
/// ≡: Identical To
pub const SYMBOL_equiv: char = '\u{2261}';
#[allow(unused, non_upper_case_globals)]
/// ≢: Not Identical To
pub const SYMBOL_equiv_not: char = '\u{2262}';
#[allow(unused, non_upper_case_globals)]
/// ⧮: Error Barred White Square
pub const SYMBOL_errorbar_square_stroked: char = '\u{29ee}';
#[allow(unused, non_upper_case_globals)]
/// ⧯: Error Barred Black Square
pub const SYMBOL_errorbar_square_filled: char = '\u{29ef}';
#[allow(unused, non_upper_case_globals)]
/// ⧰: Error Barred White Diamond
pub const SYMBOL_errorbar_diamond_stroked: char = '\u{29f0}';
#[allow(unused, non_upper_case_globals)]
/// ⧱: Error Barred Black Diamond
pub const SYMBOL_errorbar_diamond_filled: char = '\u{29f1}';
#[allow(unused, non_upper_case_globals)]
/// ⧲: Error Barred White Circle
pub const SYMBOL_errorbar_circle_stroked: char = '\u{29f2}';
#[allow(unused, non_upper_case_globals)]
/// ⧳: Error Barred Black Circle
pub const SYMBOL_errorbar_circle_filled: char = '\u{29f3}';
#[allow(unused, non_upper_case_globals)]
/// η: Greek Small Letter Eta
pub const SYMBOL_eta: char = '\u{3b7}';
#[allow(unused, non_upper_case_globals)]
/// €: Euro Sign
pub const SYMBOL_euro: char = '\u{20ac}';
#[allow(unused, non_upper_case_globals)]
/// !: Exclamation Mark
pub const SYMBOL_excl: char = '\u{21}';
#[allow(unused, non_upper_case_globals)]
/// ‼︎: Double Exclamation Mark
pub const SYMBOL_excl_double: char = '\u{203c}';
#[allow(unused, non_upper_case_globals)]
/// ¡: Inverted Exclamation Mark
pub const SYMBOL_excl_inv: char = '\u{a1}';
#[allow(unused, non_upper_case_globals)]
/// ⁉︎: Exclamation Question Mark
pub const SYMBOL_excl_quest: char = '\u{2049}';
#[allow(unused, non_upper_case_globals)]
/// ∃: There Exists
pub const SYMBOL_exists: char = '\u{2203}';
#[allow(unused, non_upper_case_globals)]
/// ∄: There Does Not Exist
pub const SYMBOL_exists_not: char = '\u{2204}';
#[allow(unused, non_upper_case_globals)]
/// ⧘: Left Wiggly Fence
pub const SYMBOL_fence_l: char = '\u{29d8}';
#[allow(unused, non_upper_case_globals)]
/// ⧚: Left Double Wiggly Fence
pub const SYMBOL_fence_l_double: char = '\u{29da}';
#[allow(unused, non_upper_case_globals)]
/// ⧙: Right Wiggly Fence
pub const SYMBOL_fence_r: char = '\u{29d9}';
#[allow(unused, non_upper_case_globals)]
/// ⧛: Right Double Wiggly Fence
pub const SYMBOL_fence_r_double: char = '\u{29db}';
#[allow(unused, non_upper_case_globals)]
/// ⦙: Dotted Fence
pub const SYMBOL_fence_dotted: char = '\u{2999}';
#[allow(unused, non_upper_case_globals)]
/// ♭: Music Flat Sign
pub const SYMBOL_flat: char = '\u{266d}';
#[allow(unused, non_upper_case_globals)]
/// 𝄬: Musical Symbol Flat Up
pub const SYMBOL_flat_t: char = '\u{1d12c}';
#[allow(unused, non_upper_case_globals)]
/// 𝄭: Musical Symbol Flat Down
pub const SYMBOL_flat_b: char = '\u{1d12d}';
#[allow(unused, non_upper_case_globals)]
/// 𝄫: Musical Symbol Double Flat
pub const SYMBOL_flat_double: char = '\u{1d12b}';
#[allow(unused, non_upper_case_globals)]
/// 𝄳: Musical Symbol Quarter Tone Flat
pub const SYMBOL_flat_quarter: char = '\u{1d133}';
#[allow(unused, non_upper_case_globals)]
/// ⌊: Left Floor
pub const SYMBOL_floor_l: char = '\u{230a}';
#[allow(unused, non_upper_case_globals)]
/// ⌋: Right Floor
pub const SYMBOL_floor_r: char = '\u{230b}';
#[allow(unused, non_upper_case_globals)]
/// ❦: Floral Heart
pub const SYMBOL_floral: char = '\u{2766}';
#[allow(unused, non_upper_case_globals)]
/// ☙: Reversed Rotated Floral Heart Bullet
pub const SYMBOL_floral_l: char = '\u{2619}';
#[allow(unused, non_upper_case_globals)]
/// ❧: Rotated Floral Heart Bullet
pub const SYMBOL_floral_r: char = '\u{2767}';
#[allow(unused, non_upper_case_globals)]
/// ∀: For All
pub const SYMBOL_forall: char = '\u{2200}';
#[allow(unused, non_upper_case_globals)]
/// ⊩: Forces
pub const SYMBOL_forces: char = '\u{22a9}';
#[allow(unused, non_upper_case_globals)]
/// ⊮: Does Not Force
pub const SYMBOL_forces_not: char = '\u{22ae}';
#[allow(unused, non_upper_case_globals)]
/// ⌢: Frown
pub const SYMBOL_frown: char = '\u{2322}';
#[allow(unused, non_upper_case_globals)]
/// γ: Greek Small Letter Gamma
pub const SYMBOL_gamma: char = '\u{3b3}';
#[allow(unused, non_upper_case_globals)]
/// ♀︎: Female Sign
pub const SYMBOL_gender_female: char = '\u{2640}';
#[allow(unused, non_upper_case_globals)]
/// ⚢: Doubled Female Sign
pub const SYMBOL_gender_female_double: char = '\u{26a2}';
#[allow(unused, non_upper_case_globals)]
/// ⚤: Interlocked Female And Male Sign
pub const SYMBOL_gender_female_male: char = '\u{26a4}';
#[allow(unused, non_upper_case_globals)]
/// ⚥: Male And Female Sign
pub const SYMBOL_gender_intersex: char = '\u{26a5}';
#[allow(unused, non_upper_case_globals)]
/// ♂︎: Male Sign
pub const SYMBOL_gender_male: char = '\u{2642}';
#[allow(unused, non_upper_case_globals)]
/// ⚣: Doubled Male Sign
pub const SYMBOL_gender_male_double: char = '\u{26a3}';
#[allow(unused, non_upper_case_globals)]
/// ⚤: Interlocked Female And Male Sign
pub const SYMBOL_gender_male_female: char = '\u{26a4}';
#[allow(unused, non_upper_case_globals)]
/// ⚦: Male With Stroke Sign
pub const SYMBOL_gender_male_stroke: char = '\u{26a6}';
#[allow(unused, non_upper_case_globals)]
/// ⚨: Vertical Male With Stroke Sign
pub const SYMBOL_gender_male_stroke_t: char = '\u{26a8}';
#[allow(unused, non_upper_case_globals)]
/// ⚩: Horizontal Male With Stroke Sign
pub const SYMBOL_gender_male_stroke_r: char = '\u{26a9}';
#[allow(unused, non_upper_case_globals)]
/// ⚲: Neuter
pub const SYMBOL_gender_neuter: char = '\u{26b2}';
#[allow(unused, non_upper_case_globals)]
/// ⚧︎: Male With Stroke And Male And Female Sign
pub const SYMBOL_gender_trans: char = '\u{26a7}';
#[allow(unused, non_upper_case_globals)]
/// ג: Hebrew Letter Gimel
pub const SYMBOL_gimel: char = '\u{5d2}';
#[allow(unused, non_upper_case_globals)]
/// ∇: Nabla
pub const SYMBOL_gradient: char = '\u{2207}';
#[allow(unused, non_upper_case_globals)]
/// `: Grave Accent
pub const SYMBOL_grave: char = '\u{60}';
#[allow(unused, non_upper_case_globals)]
/// >: Greater Than Sign
pub const SYMBOL_gt: char = '\u{3e}';
#[allow(unused, non_upper_case_globals)]
/// ⧁: Circled Greater Than
pub const SYMBOL_gt_o: char = '\u{29c1}';
#[allow(unused, non_upper_case_globals)]
/// ⋗: Greater Than With Dot
pub const SYMBOL_gt_dot: char = '\u{22d7}';
#[allow(unused, non_upper_case_globals)]
/// ⩼: Greater Than With Question Mark Above
pub const SYMBOL_gt_quest: char = '\u{2a7c}';
#[allow(unused, non_upper_case_globals)]
/// ⪆: Greater Than Or Approximate
pub const SYMBOL_gt_approx: char = '\u{2a86}';
#[allow(unused, non_upper_case_globals)]
/// ⪧: Greater Than Closed By Curve
pub const SYMBOL_gt_arc: char = '\u{2aa7}';
#[allow(unused, non_upper_case_globals)]
/// ⪩: Greater Than Closed By Curve Above Slanted Equal
pub const SYMBOL_gt_arc_eq: char = '\u{2aa9}';
#[allow(unused, non_upper_case_globals)]
/// ⊳: Contains As Normal Subgroup
pub const SYMBOL_gt_closed: char = '\u{22b3}';
#[allow(unused, non_upper_case_globals)]
/// ⊵: Contains As Normal Subgroup Or Equal To
pub const SYMBOL_gt_closed_eq: char = '\u{22b5}';
#[allow(unused, non_upper_case_globals)]
/// ⋭: Does Not Contain As Normal Subgroup Or Equal
pub const SYMBOL_gt_closed_eq_not: char = '\u{22ed}';
#[allow(unused, non_upper_case_globals)]
/// ⋫: Does Not Contain As Normal Subgroup
pub const SYMBOL_gt_closed_not: char = '\u{22eb}';
#[allow(unused, non_upper_case_globals)]
/// ≫: Much Greater Than
pub const SYMBOL_gt_double: char = '\u{226b}';
#[allow(unused, non_upper_case_globals)]
/// ⪢: Double Nested Greater Than
pub const SYMBOL_gt_double_nested: char = '\u{2aa2}';
#[allow(unused, non_upper_case_globals)]
/// ≥: Greater Than Or Equal To
pub const SYMBOL_gt_eq: char = '\u{2265}';
#[allow(unused, non_upper_case_globals)]
/// ⩾: Greater Than Or Slanted Equal To
pub const SYMBOL_gt_eq_slant: char = '\u{2a7e}';
#[allow(unused, non_upper_case_globals)]
/// ⋛: Greater Than Equal To Or Less Than
pub const SYMBOL_gt_eq_lt: char = '\u{22db}';
#[allow(unused, non_upper_case_globals)]
/// ≱: Neither Greater Than Nor Equal To
pub const SYMBOL_gt_eq_not: char = '\u{2271}';
#[allow(unused, non_upper_case_globals)]
/// ≧: Greater Than Over Equal To
pub const SYMBOL_gt_equiv: char = '\u{2267}';
#[allow(unused, non_upper_case_globals)]
/// ≷: Greater Than Or Less Than
pub const SYMBOL_gt_lt: char = '\u{2277}';
#[allow(unused, non_upper_case_globals)]
/// ≹: Neither Greater Than Nor Less Than
pub const SYMBOL_gt_lt_not: char = '\u{2279}';
#[allow(unused, non_upper_case_globals)]
/// ⪈: Greater Than And Single Line Not Equal To
pub const SYMBOL_gt_neq: char = '\u{2a88}';
#[allow(unused, non_upper_case_globals)]
/// ⪊: Greater Than And Not Approximate
pub const SYMBOL_gt_napprox: char = '\u{2a8a}';
#[allow(unused, non_upper_case_globals)]
/// ≩: Greater Than But Not Equal To
pub const SYMBOL_gt_nequiv: char = '\u{2269}';
#[allow(unused, non_upper_case_globals)]
/// ≯: Not Greater Than
pub const SYMBOL_gt_not: char = '\u{226f}';
#[allow(unused, non_upper_case_globals)]
/// ⋧: Greater Than But Not Equivalent To
pub const SYMBOL_gt_ntilde: char = '\u{22e7}';
#[allow(unused, non_upper_case_globals)]
/// ≳: Greater Than Or Equivalent To
pub const SYMBOL_gt_tilde: char = '\u{2273}';
#[allow(unused, non_upper_case_globals)]
/// ≵: Neither Greater Than Nor Equivalent To
pub const SYMBOL_gt_tilde_not: char = '\u{2275}';
#[allow(unused, non_upper_case_globals)]
/// ⊳: Contains As Normal Subgroup
pub const SYMBOL_gt_tri: char = '\u{22b3}';
#[allow(unused, non_upper_case_globals)]
/// ⊵: Contains As Normal Subgroup Or Equal To
pub const SYMBOL_gt_tri_eq: char = '\u{22b5}';
#[allow(unused, non_upper_case_globals)]
/// ⋭: Does Not Contain As Normal Subgroup Or Equal
pub const SYMBOL_gt_tri_eq_not: char = '\u{22ed}';
#[allow(unused, non_upper_case_globals)]
/// ⋫: Does Not Contain As Normal Subgroup
pub const SYMBOL_gt_tri_not: char = '\u{22eb}';
#[allow(unused, non_upper_case_globals)]
/// ⋙: Very Much Greater Than
pub const SYMBOL_gt_triple: char = '\u{22d9}';
#[allow(unused, non_upper_case_globals)]
/// ⫸: Triple Nested Greater Than
pub const SYMBOL_gt_triple_nested: char = '\u{2af8}';
#[allow(unused, non_upper_case_globals)]
/// ₲: Guarani Sign
pub const SYMBOL_guarani: char = '\u{20b2}';
#[allow(unused, non_upper_case_globals)]
/// ⇀: Rightwards Harpoon With Barb Upwards
pub const SYMBOL_harpoon_rt: char = '\u{21c0}';
#[allow(unused, non_upper_case_globals)]
/// ⥛: Rightwards Harpoon With Barb Up From Bar
pub const SYMBOL_harpoon_rt_bar: char = '\u{295b}';
#[allow(unused, non_upper_case_globals)]
/// ⥓: Rightwards Harpoon With Barb Up To Bar
pub const SYMBOL_harpoon_rt_stop: char = '\u{2953}';
#[allow(unused, non_upper_case_globals)]
/// ⇁: Rightwards Harpoon With Barb Downwards
pub const SYMBOL_harpoon_rb: char = '\u{21c1}';
#[allow(unused, non_upper_case_globals)]
/// ⥟: Rightwards Harpoon With Barb Down From Bar
pub const SYMBOL_harpoon_rb_bar: char = '\u{295f}';
#[allow(unused, non_upper_case_globals)]
/// ⥗: Rightwards Harpoon With Barb Down To Bar
pub const SYMBOL_harpoon_rb_stop: char = '\u{2957}';
#[allow(unused, non_upper_case_globals)]
/// ↼: Leftwards Harpoon With Barb Upwards
pub const SYMBOL_harpoon_lt: char = '\u{21bc}';
#[allow(unused, non_upper_case_globals)]
/// ⥚: Leftwards Harpoon With Barb Up From Bar
pub const SYMBOL_harpoon_lt_bar: char = '\u{295a}';
#[allow(unused, non_upper_case_globals)]
/// ⥒: Leftwards Harpoon With Barb Up To Bar
pub const SYMBOL_harpoon_lt_stop: char = '\u{2952}';
#[allow(unused, non_upper_case_globals)]
/// ↽: Leftwards Harpoon With Barb Downwards
pub const SYMBOL_harpoon_lb: char = '\u{21bd}';
#[allow(unused, non_upper_case_globals)]
/// ⥞: Leftwards Harpoon With Barb Down From Bar
pub const SYMBOL_harpoon_lb_bar: char = '\u{295e}';
#[allow(unused, non_upper_case_globals)]
/// ⥖: Leftwards Harpoon With Barb Down To Bar
pub const SYMBOL_harpoon_lb_stop: char = '\u{2956}';
#[allow(unused, non_upper_case_globals)]
/// ↿: Upwards Harpoon With Barb Leftwards
pub const SYMBOL_harpoon_tl: char = '\u{21bf}';
#[allow(unused, non_upper_case_globals)]
/// ⥠: Upwards Harpoon With Barb Left From Bar
pub const SYMBOL_harpoon_tl_bar: char = '\u{2960}';
#[allow(unused, non_upper_case_globals)]
/// ⥘: Upwards Harpoon With Barb Left To Bar
pub const SYMBOL_harpoon_tl_stop: char = '\u{2958}';
#[allow(unused, non_upper_case_globals)]
/// ↾: Upwards Harpoon With Barb Rightwards
pub const SYMBOL_harpoon_tr: char = '\u{21be}';
#[allow(unused, non_upper_case_globals)]
/// ⥜: Upwards Harpoon With Barb Right From Bar
pub const SYMBOL_harpoon_tr_bar: char = '\u{295c}';
#[allow(unused, non_upper_case_globals)]
/// ⥔: Upwards Harpoon With Barb Right To Bar
pub const SYMBOL_harpoon_tr_stop: char = '\u{2954}';
#[allow(unused, non_upper_case_globals)]
/// ⇃: Downwards Harpoon With Barb Leftwards
pub const SYMBOL_harpoon_bl: char = '\u{21c3}';
#[allow(unused, non_upper_case_globals)]
/// ⥡: Downwards Harpoon With Barb Left From Bar
pub const SYMBOL_harpoon_bl_bar: char = '\u{2961}';
#[allow(unused, non_upper_case_globals)]
/// ⥙: Downwards Harpoon With Barb Left To Bar
pub const SYMBOL_harpoon_bl_stop: char = '\u{2959}';
#[allow(unused, non_upper_case_globals)]
/// ⇂: Downwards Harpoon With Barb Rightwards
pub const SYMBOL_harpoon_br: char = '\u{21c2}';
#[allow(unused, non_upper_case_globals)]
/// ⥝: Downwards Harpoon With Barb Right From Bar
pub const SYMBOL_harpoon_br_bar: char = '\u{295d}';
#[allow(unused, non_upper_case_globals)]
/// ⥕: Downwards Harpoon With Barb Right To Bar
pub const SYMBOL_harpoon_br_stop: char = '\u{2955}';
#[allow(unused, non_upper_case_globals)]
/// ⥎: Left Barb Up Right Barb Up Harpoon
pub const SYMBOL_harpoon_lt_rt: char = '\u{294e}';
#[allow(unused, non_upper_case_globals)]
/// ⥐: Left Barb Down Right Barb Down Harpoon
pub const SYMBOL_harpoon_lb_rb: char = '\u{2950}';
#[allow(unused, non_upper_case_globals)]
/// ⥋: Left Barb Down Right Barb Up Harpoon
pub const SYMBOL_harpoon_lb_rt: char = '\u{294b}';
#[allow(unused, non_upper_case_globals)]
/// ⥊: Left Barb Up Right Barb Down Harpoon
pub const SYMBOL_harpoon_lt_rb: char = '\u{294a}';
#[allow(unused, non_upper_case_globals)]
/// ⥑: Up Barb Left Down Barb Left Harpoon
pub const SYMBOL_harpoon_tl_bl: char = '\u{2951}';
#[allow(unused, non_upper_case_globals)]
/// ⥏: Up Barb Right Down Barb Right Harpoon
pub const SYMBOL_harpoon_tr_br: char = '\u{294f}';
#[allow(unused, non_upper_case_globals)]
/// ⥍: Up Barb Left Down Barb Right Harpoon
pub const SYMBOL_harpoon_tl_br: char = '\u{294d}';
#[allow(unused, non_upper_case_globals)]
/// ⥌: Up Barb Right Down Barb Left Harpoon
pub const SYMBOL_harpoon_tr_bl: char = '\u{294c}';
#[allow(unused, non_upper_case_globals)]
/// ⥤: Rightwards Harpoon With Barb Up Above Rightwards Harpoon With Barb Down
pub const SYMBOL_harpoons_rtrb: char = '\u{2964}';
#[allow(unused, non_upper_case_globals)]
/// ⥥: Downwards Harpoon With Barb Left Beside Downwards Harpoon With Barb Right
pub const SYMBOL_harpoons_blbr: char = '\u{2965}';
#[allow(unused, non_upper_case_globals)]
/// ⥯: Downwards Harpoon With Barb Left Beside Upwards Harpoon With Barb Right
pub const SYMBOL_harpoons_bltr: char = '\u{296f}';
#[allow(unused, non_upper_case_globals)]
/// ⥧: Leftwards Harpoon With Barb Down Above Rightwards Harpoon With Barb Down
pub const SYMBOL_harpoons_lbrb: char = '\u{2967}';
#[allow(unused, non_upper_case_globals)]
/// ⥢: Leftwards Harpoon With Barb Up Above Leftwards Harpoon With Barb Down
pub const SYMBOL_harpoons_ltlb: char = '\u{2962}';
#[allow(unused, non_upper_case_globals)]
/// ⇋: Leftwards Harpoon Over Rightwards Harpoon
pub const SYMBOL_harpoons_ltrb: char = '\u{21cb}';
#[allow(unused, non_upper_case_globals)]
/// ⥦: Leftwards Harpoon With Barb Up Above Rightwards Harpoon With Barb Up
pub const SYMBOL_harpoons_ltrt: char = '\u{2966}';
#[allow(unused, non_upper_case_globals)]
/// ⥩: Rightwards Harpoon With Barb Down Above Leftwards Harpoon With Barb Down
pub const SYMBOL_harpoons_rblb: char = '\u{2969}';
#[allow(unused, non_upper_case_globals)]
/// ⇌: Rightwards Harpoon Over Leftwards Harpoon
pub const SYMBOL_harpoons_rtlb: char = '\u{21cc}';
#[allow(unused, non_upper_case_globals)]
/// ⥨: Rightwards Harpoon With Barb Up Above Leftwards Harpoon With Barb Up
pub const SYMBOL_harpoons_rtlt: char = '\u{2968}';
#[allow(unused, non_upper_case_globals)]
/// ⥮: Upwards Harpoon With Barb Left Beside Downwards Harpoon With Barb Right
pub const SYMBOL_harpoons_tlbr: char = '\u{296e}';
#[allow(unused, non_upper_case_globals)]
/// ⥣: Upwards Harpoon With Barb Left Beside Upwards Harpoon With Barb Right
pub const SYMBOL_harpoons_tltr: char = '\u{2963}';
#[allow(unused, non_upper_case_globals)]
/// #︎: Number Sign
pub const SYMBOL_hash: char = '\u{23}';
#[allow(unused, non_upper_case_globals)]
/// ^: Circumflex Accent
pub const SYMBOL_hat: char = '\u{5e}';
#[allow(unused, non_upper_case_globals)]
/// ⬡: White Hexagon
pub const SYMBOL_hexa_stroked: char = '\u{2b21}';
#[allow(unused, non_upper_case_globals)]
/// ⬢: Black Hexagon
pub const SYMBOL_hexa_filled: char = '\u{2b22}';
#[allow(unused, non_upper_case_globals)]
/// ⧖: White Hourglass
pub const SYMBOL_hourglass_stroked: char = '\u{29d6}';
#[allow(unused, non_upper_case_globals)]
/// ⧗: Black Hourglass
pub const SYMBOL_hourglass_filled: char = '\u{29d7}';
#[allow(unused, non_upper_case_globals)]
/// ₴: Hryvnia Sign
pub const SYMBOL_hryvnia: char = '\u{20b4}';
#[allow(unused, non_upper_case_globals)]
/// ‐: Hyphen
pub const SYMBOL_hyph: char = '\u{2010}';
#[allow(unused, non_upper_case_globals)]
/// -: Hyphen Minus
pub const SYMBOL_hyph_minus: char = '\u{2d}';
#[allow(unused, non_upper_case_globals)]
/// ‑: Non Breaking Hyphen
pub const SYMBOL_hyph_nobreak: char = '\u{2011}';
#[allow(unused, non_upper_case_globals)]
/// ‧: Hyphenation Point
pub const SYMBOL_hyph_point: char = '\u{2027}';
#[allow(unused, non_upper_case_globals)]
/// ­: Soft Hyphen
pub const SYMBOL_hyph_soft: char = '\u{ad}';
#[allow(unused, non_upper_case_globals)]
/// ⊷: Image Of
pub const SYMBOL_image: char = '\u{22b7}';
#[allow(unused, non_upper_case_globals)]
/// ∈: Element Of
pub const SYMBOL_in: char = '\u{2208}';
#[allow(unused, non_upper_case_globals)]
/// ∉: Not An Element Of
pub const SYMBOL_in_not: char = '\u{2209}';
#[allow(unused, non_upper_case_globals)]
/// ∋: Contains As Member
pub const SYMBOL_in_rev: char = '\u{220b}';
#[allow(unused, non_upper_case_globals)]
/// ∌: Does Not Contain As Member
pub const SYMBOL_in_rev_not: char = '\u{220c}';
#[allow(unused, non_upper_case_globals)]
/// ∍: Small Contains As Member
pub const SYMBOL_in_rev_small: char = '\u{220d}';
#[allow(unused, non_upper_case_globals)]
/// ∊: Small Element Of
pub const SYMBOL_in_small: char = '\u{220a}';
#[allow(unused, non_upper_case_globals)]
/// ∞: Infinity
pub const SYMBOL_infinity: char = '\u{221e}';
#[allow(unused, non_upper_case_globals)]
/// ⧞: Infinity Negated With Vertical Bar
pub const SYMBOL_infinity_bar: char = '\u{29de}';
#[allow(unused, non_upper_case_globals)]
/// ⧜: Incomplete Infinity
pub const SYMBOL_infinity_incomplete: char = '\u{29dc}';
#[allow(unused, non_upper_case_globals)]
/// ⧝: Tie Over Infinity
pub const SYMBOL_infinity_tie: char = '\u{29dd}';
#[allow(unused, non_upper_case_globals)]
/// ∫: Integral
pub const SYMBOL_integral: char = '\u{222b}';
#[allow(unused, non_upper_case_globals)]
/// ⨗: Integral With Leftwards Arrow With Hook
pub const SYMBOL_integral_arrow_hook: char = '\u{2a17}';
#[allow(unused, non_upper_case_globals)]
/// ⨑: Anticlockwise Integration
pub const SYMBOL_integral_ccw: char = '\u{2a11}';
#[allow(unused, non_upper_case_globals)]
/// ∮: Contour Integral
pub const SYMBOL_integral_cont: char = '\u{222e}';
#[allow(unused, non_upper_case_globals)]
/// ∳: Anticlockwise Contour Integral
pub const SYMBOL_integral_cont_ccw: char = '\u{2233}';
#[allow(unused, non_upper_case_globals)]
/// ∲: Clockwise Contour Integral
pub const SYMBOL_integral_cont_cw: char = '\u{2232}';
#[allow(unused, non_upper_case_globals)]
/// ∱: Clockwise Integral
pub const SYMBOL_integral_cw: char = '\u{2231}';
#[allow(unused, non_upper_case_globals)]
/// ⨍: Finite Part Integral
pub const SYMBOL_integral_dash: char = '\u{2a0d}';
#[allow(unused, non_upper_case_globals)]
/// ⨎: Integral With Double Stroke
pub const SYMBOL_integral_dash_double: char = '\u{2a0e}';
#[allow(unused, non_upper_case_globals)]
/// ∬: Double Integral
pub const SYMBOL_integral_double: char = '\u{222c}';
#[allow(unused, non_upper_case_globals)]
/// ⨌: Quadruple Integral Operator
pub const SYMBOL_integral_quad: char = '\u{2a0c}';
#[allow(unused, non_upper_case_globals)]
/// ⨙: Integral With Intersection
pub const SYMBOL_integral_inter: char = '\u{2a19}';
#[allow(unused, non_upper_case_globals)]
/// ⨏: Integral Average With Slash
pub const SYMBOL_integral_slash: char = '\u{2a0f}';
#[allow(unused, non_upper_case_globals)]
/// ⨖: Quaternion Integral Operator
pub const SYMBOL_integral_square: char = '\u{2a16}';
#[allow(unused, non_upper_case_globals)]
/// ∯: Surface Integral
pub const SYMBOL_integral_surf: char = '\u{222f}';
#[allow(unused, non_upper_case_globals)]
/// ⨘: Integral With Times Sign
pub const SYMBOL_integral_times: char = '\u{2a18}';
#[allow(unused, non_upper_case_globals)]
/// ∭: Triple Integral
pub const SYMBOL_integral_triple: char = '\u{222d}';
#[allow(unused, non_upper_case_globals)]
/// ⨚: Integral With Union
pub const SYMBOL_integral_union: char = '\u{2a1a}';
#[allow(unused, non_upper_case_globals)]
/// ∰: Volume Integral
pub const SYMBOL_integral_vol: char = '\u{2230}';
#[allow(unused, non_upper_case_globals)]
/// ∩: Intersection
pub const SYMBOL_inter: char = '\u{2229}';
#[allow(unused, non_upper_case_globals)]
/// ∩︀: Intersection
pub const SYMBOL_inter_serif: char = '\u{2229}';
#[allow(unused, non_upper_case_globals)]
/// ⩄: Intersection With Logical And
pub const SYMBOL_inter_and: char = '\u{2a44}';
#[allow(unused, non_upper_case_globals)]
/// ⋂: N Ary Intersection
pub const SYMBOL_inter_big: char = '\u{22c2}';
#[allow(unused, non_upper_case_globals)]
/// ⩀: Intersection With Dot
pub const SYMBOL_inter_dot: char = '\u{2a40}';
#[allow(unused, non_upper_case_globals)]
/// ⋒: Double Intersection
pub const SYMBOL_inter_double: char = '\u{22d2}';
#[allow(unused, non_upper_case_globals)]
/// ⊓: Square Cap
pub const SYMBOL_inter_sq: char = '\u{2293}';
#[allow(unused, non_upper_case_globals)]
/// ⊓︀: Square Cap
pub const SYMBOL_inter_sq_serif: char = '\u{2293}';
#[allow(unused, non_upper_case_globals)]
/// ⨅: N Ary Square Intersection Operator
pub const SYMBOL_inter_sq_big: char = '\u{2a05}';
#[allow(unused, non_upper_case_globals)]
/// ⩎: Double Square Intersection
pub const SYMBOL_inter_sq_double: char = '\u{2a4e}';
#[allow(unused, non_upper_case_globals)]
/// ⫴: Triple Vertical Bar Binary Relation
pub const SYMBOL_interleave: char = '\u{2af4}';
#[allow(unused, non_upper_case_globals)]
/// ⫼: Large Triple Vertical Bar Operator
pub const SYMBOL_interleave_big: char = '\u{2afc}';
#[allow(unused, non_upper_case_globals)]
/// ⫵: Triple Vertical Bar With Horizontal Stroke
pub const SYMBOL_interleave_struck: char = '\u{2af5}';
#[allow(unused, non_upper_case_globals)]
/// ‽: Interrobang
pub const SYMBOL_interrobang: char = '\u{203d}';
#[allow(unused, non_upper_case_globals)]
/// ⸘: Inverted Interrobang
pub const SYMBOL_interrobang_inv: char = '\u{2e18}';
#[allow(unused, non_upper_case_globals)]
/// ι: Greek Small Letter Iota
pub const SYMBOL_iota: char = '\u{3b9}';
#[allow(unused, non_upper_case_globals)]
/// ℩: Turned Greek Small Letter Iota
pub const SYMBOL_iota_inv: char = '\u{2129}';
#[allow(unused, non_upper_case_globals)]
/// ⨝: Join
pub const SYMBOL_join: char = '\u{2a1d}';
#[allow(unused, non_upper_case_globals)]
/// ⟖: Right Outer Join
pub const SYMBOL_join_r: char = '\u{27d6}';
#[allow(unused, non_upper_case_globals)]
/// ⟕: Left Outer Join
pub const SYMBOL_join_l: char = '\u{27d5}';
#[allow(unused, non_upper_case_globals)]
/// ⟗: Full Outer Join
pub const SYMBOL_join_l_r: char = '\u{27d7}';
#[allow(unused, non_upper_case_globals)]
/// ♃: Jupiter
pub const SYMBOL_jupiter: char = '\u{2643}';
#[allow(unused, non_upper_case_globals)]
/// κ: Greek Small Letter Kappa
pub const SYMBOL_kappa: char = '\u{3ba}';
#[allow(unused, non_upper_case_globals)]
/// ϰ: Greek Kappa Symbol
pub const SYMBOL_kappa_alt: char = '\u{3f0}';
#[allow(unused, non_upper_case_globals)]
/// ₭: Kip Sign
pub const SYMBOL_kip: char = '\u{20ad}';
#[allow(unused, non_upper_case_globals)]
/// λ: Greek Small Letter Lamda
pub const SYMBOL_lambda: char = '\u{3bb}';
#[allow(unused, non_upper_case_globals)]
/// ∆: Increment
pub const SYMBOL_laplace: char = '\u{2206}';
#[allow(unused, non_upper_case_globals)]
/// ₾: Lari Sign
pub const SYMBOL_lari: char = '\u{20be}';
#[allow(unused, non_upper_case_globals)]
/// ⪫: Larger Than
pub const SYMBOL_lat: char = '\u{2aab}';
#[allow(unused, non_upper_case_globals)]
/// ⪭: Larger Than Or Equal To
pub const SYMBOL_lat_eq: char = '\u{2aad}';
#[allow(unused, non_upper_case_globals)]
/// ₺: Turkish Lira Sign
pub const SYMBOL_lira: char = '\u{20ba}';
#[allow(unused, non_upper_case_globals)]
/// ◊: Lozenge
pub const SYMBOL_lozenge_stroked: char = '\u{25ca}';
#[allow(unused, non_upper_case_globals)]
/// ⬫: White Small Lozenge
pub const SYMBOL_lozenge_stroked_small: char = '\u{2b2b}';
#[allow(unused, non_upper_case_globals)]
/// ⬨: White Medium Lozenge
pub const SYMBOL_lozenge_stroked_medium: char = '\u{2b28}';
#[allow(unused, non_upper_case_globals)]
/// ⧫: Black Lozenge
pub const SYMBOL_lozenge_filled: char = '\u{29eb}';
#[allow(unused, non_upper_case_globals)]
/// ⬪: Black Small Lozenge
pub const SYMBOL_lozenge_filled_small: char = '\u{2b2a}';
#[allow(unused, non_upper_case_globals)]
/// ⬧: Black Medium Lozenge
pub const SYMBOL_lozenge_filled_medium: char = '\u{2b27}';
#[allow(unused, non_upper_case_globals)]
/// ‎: Left To Right Mark
pub const SYMBOL_lrm: char = '\u{200e}';
#[allow(unused, non_upper_case_globals)]
/// <: Less Than Sign
pub const SYMBOL_lt: char = '\u{3c}';
#[allow(unused, non_upper_case_globals)]
/// ⧀: Circled Less Than
pub const SYMBOL_lt_o: char = '\u{29c0}';
#[allow(unused, non_upper_case_globals)]
/// ⋖: Less Than With Dot
pub const SYMBOL_lt_dot: char = '\u{22d6}';
#[allow(unused, non_upper_case_globals)]
/// ⩻: Less Than With Question Mark Above
pub const SYMBOL_lt_quest: char = '\u{2a7b}';
#[allow(unused, non_upper_case_globals)]
/// ⪅: Less Than Or Approximate
pub const SYMBOL_lt_approx: char = '\u{2a85}';
#[allow(unused, non_upper_case_globals)]
/// ⪦: Less Than Closed By Curve
pub const SYMBOL_lt_arc: char = '\u{2aa6}';
#[allow(unused, non_upper_case_globals)]
/// ⪨: Less Than Closed By Curve Above Slanted Equal
pub const SYMBOL_lt_arc_eq: char = '\u{2aa8}';
#[allow(unused, non_upper_case_globals)]
/// ⊲: Normal Subgroup Of
pub const SYMBOL_lt_closed: char = '\u{22b2}';
#[allow(unused, non_upper_case_globals)]
/// ⊴: Normal Subgroup Of Or Equal To
pub const SYMBOL_lt_closed_eq: char = '\u{22b4}';
#[allow(unused, non_upper_case_globals)]
/// ⋬: Not Normal Subgroup Of Or Equal To
pub const SYMBOL_lt_closed_eq_not: char = '\u{22ec}';
#[allow(unused, non_upper_case_globals)]
/// ⋪: Not Normal Subgroup Of
pub const SYMBOL_lt_closed_not: char = '\u{22ea}';
#[allow(unused, non_upper_case_globals)]
/// ≪: Much Less Than
pub const SYMBOL_lt_double: char = '\u{226a}';
#[allow(unused, non_upper_case_globals)]
/// ⪡: Double Nested Less Than
pub const SYMBOL_lt_double_nested: char = '\u{2aa1}';
#[allow(unused, non_upper_case_globals)]
/// ≤: Less Than Or Equal To
pub const SYMBOL_lt_eq: char = '\u{2264}';
#[allow(unused, non_upper_case_globals)]
/// ⩽: Less Than Or Slanted Equal To
pub const SYMBOL_lt_eq_slant: char = '\u{2a7d}';
#[allow(unused, non_upper_case_globals)]
/// ⋚: Less Than Equal To Or Greater Than
pub const SYMBOL_lt_eq_gt: char = '\u{22da}';
#[allow(unused, non_upper_case_globals)]
/// ≰: Neither Less Than Nor Equal To
pub const SYMBOL_lt_eq_not: char = '\u{2270}';
#[allow(unused, non_upper_case_globals)]
/// ≦: Less Than Over Equal To
pub const SYMBOL_lt_equiv: char = '\u{2266}';
#[allow(unused, non_upper_case_globals)]
/// ≶: Less Than Or Greater Than
pub const SYMBOL_lt_gt: char = '\u{2276}';
#[allow(unused, non_upper_case_globals)]
/// ≸: Neither Less Than Nor Greater Than
pub const SYMBOL_lt_gt_not: char = '\u{2278}';
#[allow(unused, non_upper_case_globals)]
/// ⪇: Less Than And Single Line Not Equal To
pub const SYMBOL_lt_neq: char = '\u{2a87}';
#[allow(unused, non_upper_case_globals)]
/// ⪉: Less Than And Not Approximate
pub const SYMBOL_lt_napprox: char = '\u{2a89}';
#[allow(unused, non_upper_case_globals)]
/// ≨: Less Than But Not Equal To
pub const SYMBOL_lt_nequiv: char = '\u{2268}';
#[allow(unused, non_upper_case_globals)]
/// ≮: Not Less Than
pub const SYMBOL_lt_not: char = '\u{226e}';
#[allow(unused, non_upper_case_globals)]
/// ⋦: Less Than But Not Equivalent To
pub const SYMBOL_lt_ntilde: char = '\u{22e6}';
#[allow(unused, non_upper_case_globals)]
/// ≲: Less Than Or Equivalent To
pub const SYMBOL_lt_tilde: char = '\u{2272}';
#[allow(unused, non_upper_case_globals)]
/// ≴: Neither Less Than Nor Equivalent To
pub const SYMBOL_lt_tilde_not: char = '\u{2274}';
#[allow(unused, non_upper_case_globals)]
/// ⊲: Normal Subgroup Of
pub const SYMBOL_lt_tri: char = '\u{22b2}';
#[allow(unused, non_upper_case_globals)]
/// ⊴: Normal Subgroup Of Or Equal To
pub const SYMBOL_lt_tri_eq: char = '\u{22b4}';
#[allow(unused, non_upper_case_globals)]
/// ⋬: Not Normal Subgroup Of Or Equal To
pub const SYMBOL_lt_tri_eq_not: char = '\u{22ec}';
#[allow(unused, non_upper_case_globals)]
/// ⋪: Not Normal Subgroup Of
pub const SYMBOL_lt_tri_not: char = '\u{22ea}';
#[allow(unused, non_upper_case_globals)]
/// ⋘: Very Much Less Than
pub const SYMBOL_lt_triple: char = '\u{22d8}';
#[allow(unused, non_upper_case_globals)]
/// ⫷: Triple Nested Less Than
pub const SYMBOL_lt_triple_nested: char = '\u{2af7}';
#[allow(unused, non_upper_case_globals)]
/// ¯: Macron
pub const SYMBOL_macron: char = '\u{af}';
#[allow(unused, non_upper_case_globals)]
/// ✠: Maltese Cross
pub const SYMBOL_maltese: char = '\u{2720}';
#[allow(unused, non_upper_case_globals)]
/// ₼: Manat Sign
pub const SYMBOL_manat: char = '\u{20bc}';
#[allow(unused, non_upper_case_globals)]
/// ↤: Leftwards Arrow From Bar
pub const SYMBOL_mapsfrom: char = '\u{21a4}';
#[allow(unused, non_upper_case_globals)]
/// ⟻: Long Leftwards Arrow From Bar
pub const SYMBOL_mapsfrom_long: char = '\u{27fb}';
#[allow(unused, non_upper_case_globals)]
/// ↦: Rightwards Arrow From Bar
pub const SYMBOL_mapsto: char = '\u{21a6}';
#[allow(unused, non_upper_case_globals)]
/// ⟼: Long Rightwards Arrow From Bar
pub const SYMBOL_mapsto_long: char = '\u{27fc}';
#[allow(unused, non_upper_case_globals)]
/// ♂︎: Male Sign
pub const SYMBOL_mars: char = '\u{2642}';
#[allow(unused, non_upper_case_globals)]
/// ☿: Mercury
pub const SYMBOL_mercury: char = '\u{263f}';
#[allow(unused, non_upper_case_globals)]
/// −: Minus Sign
pub const SYMBOL_minus: char = '\u{2212}';
#[allow(unused, non_upper_case_globals)]
/// ⊖: Circled Minus
pub const SYMBOL_minus_o: char = '\u{2296}';
#[allow(unused, non_upper_case_globals)]
/// ∸: Dot Minus
pub const SYMBOL_minus_dot: char = '\u{2238}';
#[allow(unused, non_upper_case_globals)]
/// ∓: Minus Or Plus Sign
pub const SYMBOL_minus_plus: char = '\u{2213}';
#[allow(unused, non_upper_case_globals)]
/// ⊟: Squared Minus
pub const SYMBOL_minus_square: char = '\u{229f}';
#[allow(unused, non_upper_case_globals)]
/// ≂: Minus Tilde
pub const SYMBOL_minus_tilde: char = '\u{2242}';
#[allow(unused, non_upper_case_globals)]
/// ⨺: Minus Sign In Triangle
pub const SYMBOL_minus_triangle: char = '\u{2a3a}';
#[allow(unused, non_upper_case_globals)]
/// ⧿: Miny
pub const SYMBOL_miny: char = '\u{29ff}';
#[allow(unused, non_upper_case_globals)]
/// ⊧: Models
pub const SYMBOL_models: char = '\u{22a7}';
#[allow(unused, non_upper_case_globals)]
/// μ: Greek Small Letter Mu
pub const SYMBOL_mu: char = '\u{3bc}';
#[allow(unused, non_upper_case_globals)]
/// ⊸: Multimap
pub const SYMBOL_multimap: char = '\u{22b8}';
#[allow(unused, non_upper_case_globals)]
/// ⧟: Double Ended Multimap
pub const SYMBOL_multimap_double: char = '\u{29df}';
#[allow(unused, non_upper_case_globals)]
/// ⎰: Upper Left Or Lower Right Curly Bracket Section
pub const SYMBOL_mustache_l: char = '\u{23b0}';
#[allow(unused, non_upper_case_globals)]
/// ⎱: Upper Right Or Lower Left Curly Bracket Section
pub const SYMBOL_mustache_r: char = '\u{23b1}';
#[allow(unused, non_upper_case_globals)]
/// ∇: Nabla
pub const SYMBOL_nabla: char = '\u{2207}';
#[allow(unused, non_upper_case_globals)]
/// ₦: Naira Sign
pub const SYMBOL_naira: char = '\u{20a6}';
#[allow(unused, non_upper_case_globals)]
/// ♮: Music Natural Sign
pub const SYMBOL_natural: char = '\u{266e}';
#[allow(unused, non_upper_case_globals)]
/// 𝄮: Musical Symbol Natural Up
pub const SYMBOL_natural_t: char = '\u{1d12e}';
#[allow(unused, non_upper_case_globals)]
/// 𝄯: Musical Symbol Natural Down
pub const SYMBOL_natural_b: char = '\u{1d12f}';
#[allow(unused, non_upper_case_globals)]
/// ♆: Neptune
pub const SYMBOL_neptune: char = '\u{2646}';
#[allow(unused, non_upper_case_globals)]
/// ⯉: Neptune Form Two
pub const SYMBOL_neptune_alt: char = '\u{2bc9}';
#[allow(unused, non_upper_case_globals)]
/// ¬: Not Sign
pub const SYMBOL_not: char = '\u{ac}';
#[allow(unused, non_upper_case_globals)]
/// 🎜: Beamed Ascending Musical Notes
pub const SYMBOL_note_up: char = '\u{1f39c}';
#[allow(unused, non_upper_case_globals)]
/// 🎝: Beamed Descending Musical Notes
pub const SYMBOL_note_down: char = '\u{1f39d}';
#[allow(unused, non_upper_case_globals)]
/// 𝅝: Musical Symbol Whole Note
pub const SYMBOL_note_whole: char = '\u{1d15d}';
#[allow(unused, non_upper_case_globals)]
/// 𝅗𝅥: Musical Symbol Half Note
pub const SYMBOL_note_half: char = '\u{1d15e}';
#[allow(unused, non_upper_case_globals)]
/// 𝅘𝅥: Musical Symbol Quarter Note
pub const SYMBOL_note_quarter: char = '\u{1d15f}';
#[allow(unused, non_upper_case_globals)]
/// ♩: Quarter Note
pub const SYMBOL_note_quarter_alt: char = '\u{2669}';
#[allow(unused, non_upper_case_globals)]
/// 𝅘𝅥𝅮: Musical Symbol Eighth Note
pub const SYMBOL_note_eighth: char = '\u{1d160}';
#[allow(unused, non_upper_case_globals)]
/// ♪: Eighth Note
pub const SYMBOL_note_eighth_alt: char = '\u{266a}';
#[allow(unused, non_upper_case_globals)]
/// ♫: Beamed Eighth Notes
pub const SYMBOL_note_eighth_beamed: char = '\u{266b}';
#[allow(unused, non_upper_case_globals)]
/// 𝅘𝅥𝅯: Musical Symbol Sixteenth Note
pub const SYMBOL_note_sixteenth: char = '\u{1d161}';
#[allow(unused, non_upper_case_globals)]
/// ♬: Beamed Sixteenth Notes
pub const SYMBOL_note_sixteenth_beamed: char = '\u{266c}';
#[allow(unused, non_upper_case_globals)]
/// 𝆕: Musical Symbol Grace Note No Slash
pub const SYMBOL_note_grace: char = '\u{1d195}';
#[allow(unused, non_upper_case_globals)]
/// 𝆔: Musical Symbol Grace Note Slash
pub const SYMBOL_note_grace_slash: char = '\u{1d194}';
#[allow(unused, non_upper_case_globals)]
/// ∅: Empty Set
pub const SYMBOL_nothing: char = '\u{2205}';
#[allow(unused, non_upper_case_globals)]
/// ∅︀: Empty Set
pub const SYMBOL_nothing_zero: char = '\u{2205}';
#[allow(unused, non_upper_case_globals)]
/// ⦳: Empty Set With Right Arrow Above
pub const SYMBOL_nothing_arrow_r: char = '\u{29b3}';
#[allow(unused, non_upper_case_globals)]
/// ⦴: Empty Set With Left Arrow Above
pub const SYMBOL_nothing_arrow_l: char = '\u{29b4}';
#[allow(unused, non_upper_case_globals)]
/// ⦱: Empty Set With Overbar
pub const SYMBOL_nothing_bar: char = '\u{29b1}';
#[allow(unused, non_upper_case_globals)]
/// ⦲: Empty Set With Small Circle Above
pub const SYMBOL_nothing_circle: char = '\u{29b2}';
#[allow(unused, non_upper_case_globals)]
/// ⦰: Reversed Empty Set
pub const SYMBOL_nothing_rev: char = '\u{29b0}';
#[allow(unused, non_upper_case_globals)]
/// ν: Greek Small Letter Nu
pub const SYMBOL_nu: char = '\u{3bd}';
#[allow(unused, non_upper_case_globals)]
/// №: Numero Sign
pub const SYMBOL_numero: char = '\u{2116}';
#[allow(unused, non_upper_case_globals)]
/// ω: Greek Small Letter Omega
pub const SYMBOL_omega: char = '\u{3c9}';
#[allow(unused, non_upper_case_globals)]
/// ο: Greek Small Letter Omicron
pub const SYMBOL_omicron: char = '\u{3bf}';
#[allow(unused, non_upper_case_globals)]
/// ∞: Infinity
pub const SYMBOL_oo: char = '\u{221e}';
#[allow(unused, non_upper_case_globals)]
/// ∨: Logical Or
pub const SYMBOL_or: char = '\u{2228}';
#[allow(unused, non_upper_case_globals)]
/// ⋁: N Ary Logical Or
pub const SYMBOL_or_big: char = '\u{22c1}';
#[allow(unused, non_upper_case_globals)]
/// ⋎: Curly Logical Or
pub const SYMBOL_or_curly: char = '\u{22ce}';
#[allow(unused, non_upper_case_globals)]
/// ⟇: Or With Dot Inside
pub const SYMBOL_or_dot: char = '\u{27c7}';
#[allow(unused, non_upper_case_globals)]
/// ⩔: Double Logical Or
pub const SYMBOL_or_double: char = '\u{2a54}';
#[allow(unused, non_upper_case_globals)]
/// ⊶: Original Of
pub const SYMBOL_original: char = '\u{22b6}';
#[allow(unused, non_upper_case_globals)]
/// ∥: Parallel To
pub const SYMBOL_parallel: char = '\u{2225}';
#[allow(unused, non_upper_case_globals)]
/// ⫲: Parallel With Horizontal Stroke
pub const SYMBOL_parallel_struck: char = '\u{2af2}';
#[allow(unused, non_upper_case_globals)]
/// ⦷: Circled Parallel
pub const SYMBOL_parallel_o: char = '\u{29b7}';
#[allow(unused, non_upper_case_globals)]
/// ⋕: Equal And Parallel To
pub const SYMBOL_parallel_eq: char = '\u{22d5}';
#[allow(unused, non_upper_case_globals)]
/// ⩨: Triple Horizontal Bar With Double Vertical Stroke
pub const SYMBOL_parallel_equiv: char = '\u{2a68}';
#[allow(unused, non_upper_case_globals)]
/// ∦: Not Parallel To
pub const SYMBOL_parallel_not: char = '\u{2226}';
#[allow(unused, non_upper_case_globals)]
/// ⧣: Equals Sign And Slanted Parallel
pub const SYMBOL_parallel_slanted_eq: char = '\u{29e3}';
#[allow(unused, non_upper_case_globals)]
/// ⧤: Equals Sign And Slanted Parallel With Tilde Above
pub const SYMBOL_parallel_slanted_eq_tilde: char = '\u{29e4}';
#[allow(unused, non_upper_case_globals)]
/// ⧥: Identical To And Slanted Parallel
pub const SYMBOL_parallel_slanted_equiv: char = '\u{29e5}';
#[allow(unused, non_upper_case_globals)]
/// ⫳: Parallel With Tilde Operator
pub const SYMBOL_parallel_tilde: char = '\u{2af3}';
#[allow(unused, non_upper_case_globals)]
/// ▱: White Parallelogram
pub const SYMBOL_parallelogram_stroked: char = '\u{25b1}';
#[allow(unused, non_upper_case_globals)]
/// ▰: Black Parallelogram
pub const SYMBOL_parallelogram_filled: char = '\u{25b0}';
#[allow(unused, non_upper_case_globals)]
/// (: Left Parenthesis
pub const SYMBOL_paren_l: char = '\u{28}';
#[allow(unused, non_upper_case_globals)]
/// ⟮: Mathematical Left Flattened Parenthesis
pub const SYMBOL_paren_l_flat: char = '\u{27ee}';
#[allow(unused, non_upper_case_globals)]
/// ⦇: Z Notation Left Image Bracket
pub const SYMBOL_paren_l_closed: char = '\u{2987}';
#[allow(unused, non_upper_case_globals)]
/// ⦅: Left White Parenthesis
pub const SYMBOL_paren_l_stroked: char = '\u{2985}';
#[allow(unused, non_upper_case_globals)]
/// ): Right Parenthesis
pub const SYMBOL_paren_r: char = '\u{29}';
#[allow(unused, non_upper_case_globals)]
/// ⟯: Mathematical Right Flattened Parenthesis
pub const SYMBOL_paren_r_flat: char = '\u{27ef}';
#[allow(unused, non_upper_case_globals)]
/// ⦈: Z Notation Right Image Bracket
pub const SYMBOL_paren_r_closed: char = '\u{2988}';
#[allow(unused, non_upper_case_globals)]
/// ⦆: Right White Parenthesis
pub const SYMBOL_paren_r_stroked: char = '\u{2986}';
#[allow(unused, non_upper_case_globals)]
/// ⏜: Top Parenthesis
pub const SYMBOL_paren_t: char = '\u{23dc}';
#[allow(unused, non_upper_case_globals)]
/// ⏝: Bottom Parenthesis
pub const SYMBOL_paren_b: char = '\u{23dd}';
#[allow(unused, non_upper_case_globals)]
/// ∂: Partial Differential
pub const SYMBOL_partial: char = '\u{2202}';
#[allow(unused, non_upper_case_globals)]
/// $: Dollar Sign
pub const SYMBOL_pataca: char = '\u{24}';
#[allow(unused, non_upper_case_globals)]
/// ℘: Script Capital P
pub const SYMBOL_pee: char = '\u{2118}';
#[allow(unused, non_upper_case_globals)]
/// ⬠: White Pentagon
pub const SYMBOL_penta_stroked: char = '\u{2b20}';
#[allow(unused, non_upper_case_globals)]
/// ⬟: Black Pentagon
pub const SYMBOL_penta_filled: char = '\u{2b1f}';
#[allow(unused, non_upper_case_globals)]
/// %: Percent Sign
pub const SYMBOL_percent: char = '\u{25}';
#[allow(unused, non_upper_case_globals)]
/// ‰: Per Mille Sign
pub const SYMBOL_permille: char = '\u{2030}';
#[allow(unused, non_upper_case_globals)]
/// ‱: Per Ten Thousand Sign
pub const SYMBOL_permyriad: char = '\u{2031}';
#[allow(unused, non_upper_case_globals)]
/// ⟂: Perpendicular
pub const SYMBOL_perp: char = '\u{27c2}';
#[allow(unused, non_upper_case_globals)]
/// ⦹: Circled Perpendicular
pub const SYMBOL_perp_o: char = '\u{29b9}';
#[allow(unused, non_upper_case_globals)]
/// $: Dollar Sign
pub const SYMBOL_peso: char = '\u{24}';
#[allow(unused, non_upper_case_globals)]
/// ₱: Peso Sign
pub const SYMBOL_peso_philippine: char = '\u{20b1}';
#[allow(unused, non_upper_case_globals)]
/// φ: Greek Small Letter Phi
pub const SYMBOL_phi: char = '\u{3c6}';
#[allow(unused, non_upper_case_globals)]
/// ϕ: Greek Phi Symbol
pub const SYMBOL_phi_alt: char = '\u{3d5}';
#[allow(unused, non_upper_case_globals)]
/// π: Greek Small Letter Pi
pub const SYMBOL_pi: char = '\u{3c0}';
#[allow(unused, non_upper_case_globals)]
/// ϖ: Greek Pi Symbol
pub const SYMBOL_pi_alt: char = '\u{3d6}';
#[allow(unused, non_upper_case_globals)]
/// ¶: Pilcrow Sign
pub const SYMBOL_pilcrow: char = '\u{b6}';
#[allow(unused, non_upper_case_globals)]
/// ⁋: Reversed Pilcrow Sign
pub const SYMBOL_pilcrow_rev: char = '\u{204b}';
#[allow(unused, non_upper_case_globals)]
/// ħ: Latin Small Letter H With Stroke
pub const SYMBOL_planck: char = '\u{127}';
#[allow(unused, non_upper_case_globals)]
/// +: Plus Sign
pub const SYMBOL_plus: char = '\u{2b}';
#[allow(unused, non_upper_case_globals)]
/// ⊕: Circled Plus
pub const SYMBOL_plus_o: char = '\u{2295}';
#[allow(unused, non_upper_case_globals)]
/// ⨭: Plus Sign In Left Half Circle
pub const SYMBOL_plus_o_l: char = '\u{2a2d}';
#[allow(unused, non_upper_case_globals)]
/// ⨮: Plus Sign In Right Half Circle
pub const SYMBOL_plus_o_r: char = '\u{2a2e}';
#[allow(unused, non_upper_case_globals)]
/// ⟴: Right Arrow With Circled Plus
pub const SYMBOL_plus_o_arrow: char = '\u{27f4}';
#[allow(unused, non_upper_case_globals)]
/// ⨁: N Ary Circled Plus Operator
pub const SYMBOL_plus_o_big: char = '\u{2a01}';
#[allow(unused, non_upper_case_globals)]
/// ∔: Dot Plus
pub const SYMBOL_plus_dot: char = '\u{2214}';
#[allow(unused, non_upper_case_globals)]
/// ⧺: Double Plus
pub const SYMBOL_plus_double: char = '\u{29fa}';
#[allow(unused, non_upper_case_globals)]
/// ±: Plus Minus Sign
pub const SYMBOL_plus_minus: char = '\u{b1}';
#[allow(unused, non_upper_case_globals)]
/// ⊞: Squared Plus
pub const SYMBOL_plus_square: char = '\u{229e}';
#[allow(unused, non_upper_case_globals)]
/// ⨹: Plus Sign In Triangle
pub const SYMBOL_plus_triangle: char = '\u{2a39}';
#[allow(unused, non_upper_case_globals)]
/// ⧻: Triple Plus
pub const SYMBOL_plus_triple: char = '\u{29fb}';
#[allow(unused, non_upper_case_globals)]
/// ⨣: Plus Sign With Circumflex Accent Above
pub const SYMBOL_plus_hat: char = '\u{2a23}';
#[allow(unused, non_upper_case_globals)]
/// £: Pound Sign
pub const SYMBOL_pound: char = '\u{a3}';
#[allow(unused, non_upper_case_globals)]
/// ⏻: Power Symbol
pub const SYMBOL_power_standby: char = '\u{23fb}';
#[allow(unused, non_upper_case_globals)]
/// ⏽: Power On Symbol
pub const SYMBOL_power_on: char = '\u{23fd}';
#[allow(unused, non_upper_case_globals)]
/// ⭘: Heavy Circle
pub const SYMBOL_power_off: char = '\u{2b58}';
#[allow(unused, non_upper_case_globals)]
/// ⏼: Power On Off Symbol
pub const SYMBOL_power_on_off: char = '\u{23fc}';
#[allow(unused, non_upper_case_globals)]
/// ⏾: Power Sleep Symbol
pub const SYMBOL_power_sleep: char = '\u{23fe}';
#[allow(unused, non_upper_case_globals)]
/// ≺: Precedes
pub const SYMBOL_prec: char = '\u{227a}';
#[allow(unused, non_upper_case_globals)]
/// ⪷: Precedes Above Almost Equal To
pub const SYMBOL_prec_approx: char = '\u{2ab7}';
#[allow(unused, non_upper_case_globals)]
/// ≼: Precedes Or Equal To
pub const SYMBOL_prec_curly_eq: char = '\u{227c}';
#[allow(unused, non_upper_case_globals)]
/// ⋠: Does Not Precede Or Equal
pub const SYMBOL_prec_curly_eq_not: char = '\u{22e0}';
#[allow(unused, non_upper_case_globals)]
/// ⪻: Double Precedes
pub const SYMBOL_prec_double: char = '\u{2abb}';
#[allow(unused, non_upper_case_globals)]
/// ⪯: Precedes Above Single Line Equals Sign
pub const SYMBOL_prec_eq: char = '\u{2aaf}';
#[allow(unused, non_upper_case_globals)]
/// ⪳: Precedes Above Equals Sign
pub const SYMBOL_prec_equiv: char = '\u{2ab3}';
#[allow(unused, non_upper_case_globals)]
/// ⪹: Precedes Above Not Almost Equal To
pub const SYMBOL_prec_napprox: char = '\u{2ab9}';
#[allow(unused, non_upper_case_globals)]
/// ⪱: Precedes Above Single Line Not Equal To
pub const SYMBOL_prec_neq: char = '\u{2ab1}';
#[allow(unused, non_upper_case_globals)]
/// ⪵: Precedes Above Not Equal To
pub const SYMBOL_prec_nequiv: char = '\u{2ab5}';
#[allow(unused, non_upper_case_globals)]
/// ⊀: Does Not Precede
pub const SYMBOL_prec_not: char = '\u{2280}';
#[allow(unused, non_upper_case_globals)]
/// ⋨: Precedes But Not Equivalent To
pub const SYMBOL_prec_ntilde: char = '\u{22e8}';
#[allow(unused, non_upper_case_globals)]
/// ≾: Precedes Or Equivalent To
pub const SYMBOL_prec_tilde: char = '\u{227e}';
#[allow(unused, non_upper_case_globals)]
/// ′: Prime
pub const SYMBOL_prime: char = '\u{2032}';
#[allow(unused, non_upper_case_globals)]
/// ‵: Reversed Prime
pub const SYMBOL_prime_rev: char = '\u{2035}';
#[allow(unused, non_upper_case_globals)]
/// ″: Double Prime
pub const SYMBOL_prime_double: char = '\u{2033}';
#[allow(unused, non_upper_case_globals)]
/// ‶: Reversed Double Prime
pub const SYMBOL_prime_double_rev: char = '\u{2036}';
#[allow(unused, non_upper_case_globals)]
/// ‴: Triple Prime
pub const SYMBOL_prime_triple: char = '\u{2034}';
#[allow(unused, non_upper_case_globals)]
/// ‷: Reversed Triple Prime
pub const SYMBOL_prime_triple_rev: char = '\u{2037}';
#[allow(unused, non_upper_case_globals)]
/// ⁗: Quadruple Prime
pub const SYMBOL_prime_quad: char = '\u{2057}';
#[allow(unused, non_upper_case_globals)]
/// ∏: N Ary Product
pub const SYMBOL_product: char = '\u{220f}';
#[allow(unused, non_upper_case_globals)]
/// ∐: N Ary Coproduct
pub const SYMBOL_product_co: char = '\u{2210}';
#[allow(unused, non_upper_case_globals)]
/// ∝: Proportional To
pub const SYMBOL_prop: char = '\u{221d}';
#[allow(unused, non_upper_case_globals)]
/// ψ: Greek Small Letter Psi
pub const SYMBOL_psi: char = '\u{3c8}';
#[allow(unused, non_upper_case_globals)]
/// ∎: End Of Proof
pub const SYMBOL_qed: char = '\u{220e}';
#[allow(unused, non_upper_case_globals)]
/// ?: Question Mark
pub const SYMBOL_quest: char = '\u{3f}';
#[allow(unused, non_upper_case_globals)]
/// ⁇: Double Question Mark
pub const SYMBOL_quest_double: char = '\u{2047}';
#[allow(unused, non_upper_case_globals)]
/// ⁈: Question Exclamation Mark
pub const SYMBOL_quest_excl: char = '\u{2048}';
#[allow(unused, non_upper_case_globals)]
/// ¿: Inverted Question Mark
pub const SYMBOL_quest_inv: char = '\u{bf}';
#[allow(unused, non_upper_case_globals)]
/// ": Quotation Mark
pub const SYMBOL_quote_double: char = '\u{22}';
#[allow(unused, non_upper_case_globals)]
/// ': Apostrophe
pub const SYMBOL_quote_single: char = '\u{27}';
#[allow(unused, non_upper_case_globals)]
/// “: Left Double Quotation Mark
pub const SYMBOL_quote_l_double: char = '\u{201c}';
#[allow(unused, non_upper_case_globals)]
/// ‘: Left Single Quotation Mark
pub const SYMBOL_quote_l_single: char = '\u{2018}';
#[allow(unused, non_upper_case_globals)]
/// ”: Right Double Quotation Mark
pub const SYMBOL_quote_r_double: char = '\u{201d}';
#[allow(unused, non_upper_case_globals)]
/// ’: Right Single Quotation Mark
pub const SYMBOL_quote_r_single: char = '\u{2019}';
#[allow(unused, non_upper_case_globals)]
/// «: Left Pointing Double Angle Quotation Mark
pub const SYMBOL_quote_chevron_l_double: char = '\u{ab}';
#[allow(unused, non_upper_case_globals)]
/// ‹: Single Left Pointing Angle Quotation Mark
pub const SYMBOL_quote_chevron_l_single: char = '\u{2039}';
#[allow(unused, non_upper_case_globals)]
/// »: Right Pointing Double Angle Quotation Mark
pub const SYMBOL_quote_chevron_r_double: char = '\u{bb}';
#[allow(unused, non_upper_case_globals)]
/// ›: Single Right Pointing Angle Quotation Mark
pub const SYMBOL_quote_chevron_r_single: char = '\u{203a}';
#[allow(unused, non_upper_case_globals)]
/// ‟: Double High Reversed 9 Quotation Mark
pub const SYMBOL_quote_high_double: char = '\u{201f}';
#[allow(unused, non_upper_case_globals)]
/// ‛: Single High Reversed 9 Quotation Mark
pub const SYMBOL_quote_high_single: char = '\u{201b}';
#[allow(unused, non_upper_case_globals)]
/// „: Double Low 9 Quotation Mark
pub const SYMBOL_quote_low_double: char = '\u{201e}';
#[allow(unused, non_upper_case_globals)]
/// ‚: Single Low 9 Quotation Mark
pub const SYMBOL_quote_low_single: char = '\u{201a}';
#[allow(unused, non_upper_case_globals)]
/// ∶: Ratio
pub const SYMBOL_ratio: char = '\u{2236}';
#[allow(unused, non_upper_case_globals)]
/// ▭: White Rectangle
pub const SYMBOL_rect_stroked_h: char = '\u{25ad}';
#[allow(unused, non_upper_case_globals)]
/// ▯: White Vertical Rectangle
pub const SYMBOL_rect_stroked_v: char = '\u{25af}';
#[allow(unused, non_upper_case_globals)]
/// ▬: Black Rectangle
pub const SYMBOL_rect_filled_h: char = '\u{25ac}';
#[allow(unused, non_upper_case_globals)]
/// ▮: Black Vertical Rectangle
pub const SYMBOL_rect_filled_v: char = '\u{25ae}';
#[allow(unused, non_upper_case_globals)]
/// ※: Reference Mark
pub const SYMBOL_refmark: char = '\u{203b}';
#[allow(unused, non_upper_case_globals)]
/// 𝄻: Musical Symbol Whole Rest
pub const SYMBOL_rest_whole: char = '\u{1d13b}';
#[allow(unused, non_upper_case_globals)]
/// 𝄺: Musical Symbol Multi Rest
pub const SYMBOL_rest_multiple: char = '\u{1d13a}';
#[allow(unused, non_upper_case_globals)]
/// 𝄩: Musical Symbol Multiple Measure Rest
pub const SYMBOL_rest_multiple_measure: char = '\u{1d129}';
#[allow(unused, non_upper_case_globals)]
/// 𝄼: Musical Symbol Half Rest
pub const SYMBOL_rest_half: char = '\u{1d13c}';
#[allow(unused, non_upper_case_globals)]
/// 𝄽: Musical Symbol Quarter Rest
pub const SYMBOL_rest_quarter: char = '\u{1d13d}';
#[allow(unused, non_upper_case_globals)]
/// 𝄾: Musical Symbol Eighth Rest
pub const SYMBOL_rest_eighth: char = '\u{1d13e}';
#[allow(unused, non_upper_case_globals)]
/// 𝄿: Musical Symbol Sixteenth Rest
pub const SYMBOL_rest_sixteenth: char = '\u{1d13f}';
#[allow(unused, non_upper_case_globals)]
/// ρ: Greek Small Letter Rho
pub const SYMBOL_rho: char = '\u{3c1}';
#[allow(unused, non_upper_case_globals)]
/// ϱ: Greek Rho Symbol
pub const SYMBOL_rho_alt: char = '\u{3f1}';
#[allow(unused, non_upper_case_globals)]
/// ៛: Khmer Currency Symbol Riel
pub const SYMBOL_riel: char = '\u{17db}';
#[allow(unused, non_upper_case_globals)]
/// ⃁: null
pub const SYMBOL_riyal: char = '\u{20c1}';
#[allow(unused, non_upper_case_globals)]
/// ‏: Right To Left Mark
pub const SYMBOL_rlm: char = '\u{200f}';
#[allow(unused, non_upper_case_globals)]
/// ₽: Ruble Sign
pub const SYMBOL_ruble: char = '\u{20bd}';
#[allow(unused, non_upper_case_globals)]
/// ₹: Indian Rupee Sign
pub const SYMBOL_rupee_indian: char = '\u{20b9}';
#[allow(unused, non_upper_case_globals)]
/// ₨: Rupee Sign
pub const SYMBOL_rupee_generic: char = '\u{20a8}';
#[allow(unused, non_upper_case_globals)]
/// ௹: Tamil Rupee Sign
pub const SYMBOL_rupee_tamil: char = '\u{bf9}';
#[allow(unused, non_upper_case_globals)]
/// 𞋿: Wancho Ngun Sign
pub const SYMBOL_rupee_wancho: char = '\u{1e2ff}';
#[allow(unused, non_upper_case_globals)]
/// ♄: Saturn
pub const SYMBOL_saturn: char = '\u{2644}';
#[allow(unused, non_upper_case_globals)]
/// §: Section Sign
pub const SYMBOL_section: char = '\u{a7}';
#[allow(unused, non_upper_case_globals)]
/// ;: Semicolon
pub const SYMBOL_semi: char = '\u{3b}';
#[allow(unused, non_upper_case_globals)]
/// ⸵: Turned Semicolon
pub const SYMBOL_semi_inv: char = '\u{2e35}';
#[allow(unused, non_upper_case_globals)]
/// ⁏: Reversed Semicolon
pub const SYMBOL_semi_rev: char = '\u{204f}';
#[allow(unused, non_upper_case_globals)]
/// ш: Cyrillic Small Letter Sha
pub const SYMBOL_sha: char = '\u{448}';
#[allow(unused, non_upper_case_globals)]
/// ♯: Music Sharp Sign
pub const SYMBOL_sharp: char = '\u{266f}';
#[allow(unused, non_upper_case_globals)]
/// 𝄰: Musical Symbol Sharp Up
pub const SYMBOL_sharp_t: char = '\u{1d130}';
#[allow(unused, non_upper_case_globals)]
/// 𝄱: Musical Symbol Sharp Down
pub const SYMBOL_sharp_b: char = '\u{1d131}';
#[allow(unused, non_upper_case_globals)]
/// 𝄪: Musical Symbol Double Sharp
pub const SYMBOL_sharp_double: char = '\u{1d12a}';
#[allow(unused, non_upper_case_globals)]
/// 𝄲: Musical Symbol Quarter Tone Sharp
pub const SYMBOL_sharp_quarter: char = '\u{1d132}';
#[allow(unused, non_upper_case_globals)]
/// ₪: New Sheqel Sign
pub const SYMBOL_shekel: char = '\u{20aa}';
#[allow(unused, non_upper_case_globals)]
/// ❲: Light Left Tortoise Shell Bracket Ornament
pub const SYMBOL_shell_l: char = '\u{2772}';
#[allow(unused, non_upper_case_globals)]
/// ⟬: Mathematical Left White Tortoise Shell Bracket
pub const SYMBOL_shell_l_stroked: char = '\u{27ec}';
#[allow(unused, non_upper_case_globals)]
/// ⦗: Left Black Tortoise Shell Bracket
pub const SYMBOL_shell_l_filled: char = '\u{2997}';
#[allow(unused, non_upper_case_globals)]
/// ❳: Light Right Tortoise Shell Bracket Ornament
pub const SYMBOL_shell_r: char = '\u{2773}';
#[allow(unused, non_upper_case_globals)]
/// ⟭: Mathematical Right White Tortoise Shell Bracket
pub const SYMBOL_shell_r_stroked: char = '\u{27ed}';
#[allow(unused, non_upper_case_globals)]
/// ⦘: Right Black Tortoise Shell Bracket
pub const SYMBOL_shell_r_filled: char = '\u{2998}';
#[allow(unused, non_upper_case_globals)]
/// ⏠: Top Tortoise Shell Bracket
pub const SYMBOL_shell_t: char = '\u{23e0}';
#[allow(unused, non_upper_case_globals)]
/// ⏡: Bottom Tortoise Shell Bracket
pub const SYMBOL_shell_b: char = '\u{23e1}';
#[allow(unused, non_upper_case_globals)]
/// σ: Greek Small Letter Sigma
pub const SYMBOL_sigma: char = '\u{3c3}';
#[allow(unused, non_upper_case_globals)]
/// ς: Greek Small Letter Final Sigma
pub const SYMBOL_sigma_alt: char = '\u{3c2}';
#[allow(unused, non_upper_case_globals)]
/// /: Solidus
pub const SYMBOL_slash: char = '\u{2f}';
#[allow(unused, non_upper_case_globals)]
/// ⊘: Circled Division Slash
pub const SYMBOL_slash_o: char = '\u{2298}';
#[allow(unused, non_upper_case_globals)]
/// ⫽: Double Solidus Operator
pub const SYMBOL_slash_double: char = '\u{2afd}';
#[allow(unused, non_upper_case_globals)]
/// ⫻: Triple Solidus Binary Relation
pub const SYMBOL_slash_triple: char = '\u{2afb}';
#[allow(unused, non_upper_case_globals)]
/// ⧸: Big Solidus
pub const SYMBOL_slash_big: char = '\u{29f8}';
#[allow(unused, non_upper_case_globals)]
/// ⨳: Smash Product
pub const SYMBOL_smash: char = '\u{2a33}';
#[allow(unused, non_upper_case_globals)]
/// ⌣: Smile
pub const SYMBOL_smile: char = '\u{2323}';
#[allow(unused, non_upper_case_globals)]
/// ⪪: Smaller Than
pub const SYMBOL_smt: char = '\u{2aaa}';
#[allow(unused, non_upper_case_globals)]
/// ⪬: Smaller Than Or Equal To
pub const SYMBOL_smt_eq: char = '\u{2aac}';
#[allow(unused, non_upper_case_globals)]
/// ⃀: Som Sign
pub const SYMBOL_som: char = '\u{20c0}';
#[allow(unused, non_upper_case_globals)]
///  : Space
pub const SYMBOL_space: char = '\u{20}';
#[allow(unused, non_upper_case_globals)]
///  : No Break Space
pub const SYMBOL_space_nobreak: char = '\u{a0}';
#[allow(unused, non_upper_case_globals)]
///  : Narrow No Break Space
pub const SYMBOL_space_nobreak_narrow: char = '\u{202f}';
#[allow(unused, non_upper_case_globals)]
///  : En Space
pub const SYMBOL_space_en: char = '\u{2002}';
#[allow(unused, non_upper_case_globals)]
///  : Em Space
pub const SYMBOL_space_quad: char = '\u{2003}';
#[allow(unused, non_upper_case_globals)]
///  : Three Per Em Space
pub const SYMBOL_space_third: char = '\u{2004}';
#[allow(unused, non_upper_case_globals)]
///  : Four Per Em Space
pub const SYMBOL_space_quarter: char = '\u{2005}';
#[allow(unused, non_upper_case_globals)]
///  : Six Per Em Space
pub const SYMBOL_space_sixth: char = '\u{2006}';
#[allow(unused, non_upper_case_globals)]
///  : Medium Mathematical Space
pub const SYMBOL_space_med: char = '\u{205f}';
#[allow(unused, non_upper_case_globals)]
///  : Figure Space
pub const SYMBOL_space_fig: char = '\u{2007}';
#[allow(unused, non_upper_case_globals)]
///  : Punctuation Space
pub const SYMBOL_space_punct: char = '\u{2008}';
#[allow(unused, non_upper_case_globals)]
///  : Thin Space
pub const SYMBOL_space_thin: char = '\u{2009}';
#[allow(unused, non_upper_case_globals)]
///  : Hair Space
pub const SYMBOL_space_hair: char = '\u{200a}';
#[allow(unused, non_upper_case_globals)]
/// ␣: Open Box
pub const SYMBOL_spacebar: char = '\u{2423}';
#[allow(unused, non_upper_case_globals)]
/// □: White Square
pub const SYMBOL_square_stroked: char = '\u{25a1}';
#[allow(unused, non_upper_case_globals)]
/// ▫︎: White Small Square
pub const SYMBOL_square_stroked_tiny: char = '\u{25ab}';
#[allow(unused, non_upper_case_globals)]
/// ◽︎: White Medium Small Square
pub const SYMBOL_square_stroked_small: char = '\u{25fd}';
#[allow(unused, non_upper_case_globals)]
/// ◻︎: White Medium Square
pub const SYMBOL_square_stroked_medium: char = '\u{25fb}';
#[allow(unused, non_upper_case_globals)]
/// ⬜︎: White Large Square
pub const SYMBOL_square_stroked_big: char = '\u{2b1c}';
#[allow(unused, non_upper_case_globals)]
/// ⬚: Dotted Square
pub const SYMBOL_square_stroked_dotted: char = '\u{2b1a}';
#[allow(unused, non_upper_case_globals)]
/// ▢: White Square With Rounded Corners
pub const SYMBOL_square_stroked_rounded: char = '\u{25a2}';
#[allow(unused, non_upper_case_globals)]
/// ■: Black Square
pub const SYMBOL_square_filled: char = '\u{25a0}';
#[allow(unused, non_upper_case_globals)]
/// ▪︎: Black Small Square
pub const SYMBOL_square_filled_tiny: char = '\u{25aa}';
#[allow(unused, non_upper_case_globals)]
/// ◾︎: Black Medium Small Square
pub const SYMBOL_square_filled_small: char = '\u{25fe}';
#[allow(unused, non_upper_case_globals)]
/// ◼︎: Black Medium Square
pub const SYMBOL_square_filled_medium: char = '\u{25fc}';
#[allow(unused, non_upper_case_globals)]
/// ⬛︎: Black Large Square
pub const SYMBOL_square_filled_big: char = '\u{2b1b}';
#[allow(unused, non_upper_case_globals)]
/// ⋆: Star Operator
pub const SYMBOL_star_op: char = '\u{22c6}';
#[allow(unused, non_upper_case_globals)]
/// ☆: White Star
pub const SYMBOL_star_stroked: char = '\u{2606}';
#[allow(unused, non_upper_case_globals)]
/// ★: Black Star
pub const SYMBOL_star_filled: char = '\u{2605}';
#[allow(unused, non_upper_case_globals)]
/// ⊂: Subset Of
pub const SYMBOL_subset: char = '\u{2282}';
#[allow(unused, non_upper_case_globals)]
/// ⫉: Subset Of Above Almost Equal To
pub const SYMBOL_subset_approx: char = '\u{2ac9}';
#[allow(unused, non_upper_case_globals)]
/// ⫏: Closed Subset
pub const SYMBOL_subset_closed: char = '\u{2acf}';
#[allow(unused, non_upper_case_globals)]
/// ⫑: Closed Subset Or Equal To
pub const SYMBOL_subset_closed_eq: char = '\u{2ad1}';
#[allow(unused, non_upper_case_globals)]
/// ⪽: Subset With Dot
pub const SYMBOL_subset_dot: char = '\u{2abd}';
#[allow(unused, non_upper_case_globals)]
/// ⋐: Double Subset
pub const SYMBOL_subset_double: char = '\u{22d0}';
#[allow(unused, non_upper_case_globals)]
/// ⊆: Subset Of Or Equal To
pub const SYMBOL_subset_eq: char = '\u{2286}';
#[allow(unused, non_upper_case_globals)]
/// ⫃: Subset Of Or Equal To With Dot Above
pub const SYMBOL_subset_eq_dot: char = '\u{2ac3}';
#[allow(unused, non_upper_case_globals)]
/// ⊈: Neither A Subset Of Nor Equal To
pub const SYMBOL_subset_eq_not: char = '\u{2288}';
#[allow(unused, non_upper_case_globals)]
/// ⊑: Square Image Of Or Equal To
pub const SYMBOL_subset_eq_sq: char = '\u{2291}';
#[allow(unused, non_upper_case_globals)]
/// ⋢: Not Square Image Of Or Equal To
pub const SYMBOL_subset_eq_sq_not: char = '\u{22e2}';
#[allow(unused, non_upper_case_globals)]
/// ⫅: Subset Of Above Equals Sign
pub const SYMBOL_subset_equiv: char = '\u{2ac5}';
#[allow(unused, non_upper_case_globals)]
/// ⊊: Subset Of With Not Equal To
pub const SYMBOL_subset_neq: char = '\u{228a}';
#[allow(unused, non_upper_case_globals)]
/// ⫋: Subset Of Above Not Equal To
pub const SYMBOL_subset_nequiv: char = '\u{2acb}';
#[allow(unused, non_upper_case_globals)]
/// ⊄: Not A Subset Of
pub const SYMBOL_subset_not: char = '\u{2284}';
#[allow(unused, non_upper_case_globals)]
/// ⪿: Subset With Plus Sign Below
pub const SYMBOL_subset_plus: char = '\u{2abf}';
#[allow(unused, non_upper_case_globals)]
/// ⊏: Square Image Of
pub const SYMBOL_subset_sq: char = '\u{228f}';
#[allow(unused, non_upper_case_globals)]
/// ⋤: Square Image Of Or Not Equal To
pub const SYMBOL_subset_sq_neq: char = '\u{22e4}';
#[allow(unused, non_upper_case_globals)]
/// ⫇: Subset Of Above Tilde Operator
pub const SYMBOL_subset_tilde: char = '\u{2ac7}';
#[allow(unused, non_upper_case_globals)]
/// ⫁: Subset With Multiplication Sign Below
pub const SYMBOL_subset_times: char = '\u{2ac1}';
#[allow(unused, non_upper_case_globals)]
/// ≻: Succeeds
pub const SYMBOL_succ: char = '\u{227b}';
#[allow(unused, non_upper_case_globals)]
/// ⪸: Succeeds Above Almost Equal To
pub const SYMBOL_succ_approx: char = '\u{2ab8}';
#[allow(unused, non_upper_case_globals)]
/// ≽: Succeeds Or Equal To
pub const SYMBOL_succ_curly_eq: char = '\u{227d}';
#[allow(unused, non_upper_case_globals)]
/// ⋡: Does Not Succeed Or Equal
pub const SYMBOL_succ_curly_eq_not: char = '\u{22e1}';
#[allow(unused, non_upper_case_globals)]
/// ⪼: Double Succeeds
pub const SYMBOL_succ_double: char = '\u{2abc}';
#[allow(unused, non_upper_case_globals)]
/// ⪰: Succeeds Above Single Line Equals Sign
pub const SYMBOL_succ_eq: char = '\u{2ab0}';
#[allow(unused, non_upper_case_globals)]
/// ⪴: Succeeds Above Equals Sign
pub const SYMBOL_succ_equiv: char = '\u{2ab4}';
#[allow(unused, non_upper_case_globals)]
/// ⪺: Succeeds Above Not Almost Equal To
pub const SYMBOL_succ_napprox: char = '\u{2aba}';
#[allow(unused, non_upper_case_globals)]
/// ⪲: Succeeds Above Single Line Not Equal To
pub const SYMBOL_succ_neq: char = '\u{2ab2}';
#[allow(unused, non_upper_case_globals)]
/// ⪶: Succeeds Above Not Equal To
pub const SYMBOL_succ_nequiv: char = '\u{2ab6}';
#[allow(unused, non_upper_case_globals)]
/// ⊁: Does Not Succeed
pub const SYMBOL_succ_not: char = '\u{2281}';
#[allow(unused, non_upper_case_globals)]
/// ⋩: Succeeds But Not Equivalent To
pub const SYMBOL_succ_ntilde: char = '\u{22e9}';
#[allow(unused, non_upper_case_globals)]
/// ≿: Succeeds Or Equivalent To
pub const SYMBOL_succ_tilde: char = '\u{227f}';
#[allow(unused, non_upper_case_globals)]
/// ♣︎: Black Club Suit
pub const SYMBOL_suit_club_filled: char = '\u{2663}';
#[allow(unused, non_upper_case_globals)]
/// ♧: White Club Suit
pub const SYMBOL_suit_club_stroked: char = '\u{2667}';
#[allow(unused, non_upper_case_globals)]
/// ♦︎: Black Diamond Suit
pub const SYMBOL_suit_diamond_filled: char = '\u{2666}';
#[allow(unused, non_upper_case_globals)]
/// ♢: White Diamond Suit
pub const SYMBOL_suit_diamond_stroked: char = '\u{2662}';
#[allow(unused, non_upper_case_globals)]
/// ♥︎: Black Heart Suit
pub const SYMBOL_suit_heart_filled: char = '\u{2665}';
#[allow(unused, non_upper_case_globals)]
/// ♡: White Heart Suit
pub const SYMBOL_suit_heart_stroked: char = '\u{2661}';
#[allow(unused, non_upper_case_globals)]
/// ♠︎: Black Spade Suit
pub const SYMBOL_suit_spade_filled: char = '\u{2660}';
#[allow(unused, non_upper_case_globals)]
/// ♤: White Spade Suit
pub const SYMBOL_suit_spade_stroked: char = '\u{2664}';
#[allow(unused, non_upper_case_globals)]
/// ∑: N Ary Summation
pub const SYMBOL_sum: char = '\u{2211}';
#[allow(unused, non_upper_case_globals)]
/// ⨋: Summation With Integral
pub const SYMBOL_sum_integral: char = '\u{2a0b}';
#[allow(unused, non_upper_case_globals)]
/// ☉: Sun
pub const SYMBOL_sun: char = '\u{2609}';
#[allow(unused, non_upper_case_globals)]
/// ⊃: Superset Of
pub const SYMBOL_supset: char = '\u{2283}';
#[allow(unused, non_upper_case_globals)]
/// ⫊: Superset Of Above Almost Equal To
pub const SYMBOL_supset_approx: char = '\u{2aca}';
#[allow(unused, non_upper_case_globals)]
/// ⫐: Closed Superset
pub const SYMBOL_supset_closed: char = '\u{2ad0}';
#[allow(unused, non_upper_case_globals)]
/// ⫒: Closed Superset Or Equal To
pub const SYMBOL_supset_closed_eq: char = '\u{2ad2}';
#[allow(unused, non_upper_case_globals)]
/// ⪾: Superset With Dot
pub const SYMBOL_supset_dot: char = '\u{2abe}';
#[allow(unused, non_upper_case_globals)]
/// ⋑: Double Superset
pub const SYMBOL_supset_double: char = '\u{22d1}';
#[allow(unused, non_upper_case_globals)]
/// ⊇: Superset Of Or Equal To
pub const SYMBOL_supset_eq: char = '\u{2287}';
#[allow(unused, non_upper_case_globals)]
/// ⫄: Superset Of Or Equal To With Dot Above
pub const SYMBOL_supset_eq_dot: char = '\u{2ac4}';
#[allow(unused, non_upper_case_globals)]
/// ⊉: Neither A Superset Of Nor Equal To
pub const SYMBOL_supset_eq_not: char = '\u{2289}';
#[allow(unused, non_upper_case_globals)]
/// ⊒: Square Original Of Or Equal To
pub const SYMBOL_supset_eq_sq: char = '\u{2292}';
#[allow(unused, non_upper_case_globals)]
/// ⋣: Not Square Original Of Or Equal To
pub const SYMBOL_supset_eq_sq_not: char = '\u{22e3}';
#[allow(unused, non_upper_case_globals)]
/// ⫆: Superset Of Above Equals Sign
pub const SYMBOL_supset_equiv: char = '\u{2ac6}';
#[allow(unused, non_upper_case_globals)]
/// ⊋: Superset Of With Not Equal To
pub const SYMBOL_supset_neq: char = '\u{228b}';
#[allow(unused, non_upper_case_globals)]
/// ⫌: Superset Of Above Not Equal To
pub const SYMBOL_supset_nequiv: char = '\u{2acc}';
#[allow(unused, non_upper_case_globals)]
/// ⊅: Not A Superset Of
pub const SYMBOL_supset_not: char = '\u{2285}';
#[allow(unused, non_upper_case_globals)]
/// ⫀: Superset With Plus Sign Below
pub const SYMBOL_supset_plus: char = '\u{2ac0}';
#[allow(unused, non_upper_case_globals)]
/// ⊐: Square Original Of
pub const SYMBOL_supset_sq: char = '\u{2290}';
#[allow(unused, non_upper_case_globals)]
/// ⋥: Square Original Of Or Not Equal To
pub const SYMBOL_supset_sq_neq: char = '\u{22e5}';
#[allow(unused, non_upper_case_globals)]
/// ⫈: Superset Of Above Tilde Operator
pub const SYMBOL_supset_tilde: char = '\u{2ac8}';
#[allow(unused, non_upper_case_globals)]
/// ⫂: Superset With Multiplication Sign Below
pub const SYMBOL_supset_times: char = '\u{2ac2}';
#[allow(unused, non_upper_case_globals)]
/// ⊢: Right Tack
pub const SYMBOL_tack_r: char = '\u{22a2}';
#[allow(unused, non_upper_case_globals)]
/// ⊬: Does Not Prove
pub const SYMBOL_tack_r_not: char = '\u{22ac}';
#[allow(unused, non_upper_case_globals)]
/// ⟝: Long Right Tack
pub const SYMBOL_tack_r_long: char = '\u{27dd}';
#[allow(unused, non_upper_case_globals)]
/// ⊦: Assertion
pub const SYMBOL_tack_r_short: char = '\u{22a6}';
#[allow(unused, non_upper_case_globals)]
/// ⊨: True
pub const SYMBOL_tack_r_double: char = '\u{22a8}';
#[allow(unused, non_upper_case_globals)]
/// ⊨: True
pub const SYMBOL_tack_rr: char = '\u{22a8}';
#[allow(unused, non_upper_case_globals)]
/// ⊭: Not True
pub const SYMBOL_tack_r_double_not: char = '\u{22ad}';
#[allow(unused, non_upper_case_globals)]
/// ⊭: Not True
pub const SYMBOL_tack_rr_not: char = '\u{22ad}';
#[allow(unused, non_upper_case_globals)]
/// ⫢: Vertical Bar Triple Right Turnstile
pub const SYMBOL_tack_rrr: char = '\u{2ae2}';
#[allow(unused, non_upper_case_globals)]
/// ⊣: Left Tack
pub const SYMBOL_tack_l: char = '\u{22a3}';
#[allow(unused, non_upper_case_globals)]
/// ⟞: Long Left Tack
pub const SYMBOL_tack_l_long: char = '\u{27de}';
#[allow(unused, non_upper_case_globals)]
/// ⫞: Short Left Tack
pub const SYMBOL_tack_l_short: char = '\u{2ade}';
#[allow(unused, non_upper_case_globals)]
/// ⫤: Vertical Bar Double Left Turnstile
pub const SYMBOL_tack_l_double: char = '\u{2ae4}';
#[allow(unused, non_upper_case_globals)]
/// ⫤: Vertical Bar Double Left Turnstile
pub const SYMBOL_tack_ll: char = '\u{2ae4}';
#[allow(unused, non_upper_case_globals)]
/// ⊥: Up Tack
pub const SYMBOL_tack_t: char = '\u{22a5}';
#[allow(unused, non_upper_case_globals)]
/// ⟘: Large Up Tack
pub const SYMBOL_tack_t_big: char = '\u{27d8}';
#[allow(unused, non_upper_case_globals)]
/// ⫫: Double Up Tack
pub const SYMBOL_tack_t_double: char = '\u{2aeb}';
#[allow(unused, non_upper_case_globals)]
/// ⫫: Double Up Tack
pub const SYMBOL_tack_tt: char = '\u{2aeb}';
#[allow(unused, non_upper_case_globals)]
/// ⫠: Short Up Tack
pub const SYMBOL_tack_t_short: char = '\u{2ae0}';
#[allow(unused, non_upper_case_globals)]
/// ⊤: Down Tack
pub const SYMBOL_tack_b: char = '\u{22a4}';
#[allow(unused, non_upper_case_globals)]
/// ⟙: Large Down Tack
pub const SYMBOL_tack_b_big: char = '\u{27d9}';
#[allow(unused, non_upper_case_globals)]
/// ⫪: Double Down Tack
pub const SYMBOL_tack_b_double: char = '\u{2aea}';
#[allow(unused, non_upper_case_globals)]
/// ⫪: Double Down Tack
pub const SYMBOL_tack_bb: char = '\u{2aea}';
#[allow(unused, non_upper_case_globals)]
/// ⫟: Short Down Tack
pub const SYMBOL_tack_b_short: char = '\u{2adf}';
#[allow(unused, non_upper_case_globals)]
/// ⟛: Left And Right Tack
pub const SYMBOL_tack_l_r: char = '\u{27db}';
#[allow(unused, non_upper_case_globals)]
/// ৳: Bengali Rupee Sign
pub const SYMBOL_taka: char = '\u{9f3}';
#[allow(unused, non_upper_case_globals)]
/// ߿: Nko Taman Sign
pub const SYMBOL_taman: char = '\u{7ff}';
#[allow(unused, non_upper_case_globals)]
/// τ: Greek Small Letter Tau
pub const SYMBOL_tau: char = '\u{3c4}';
#[allow(unused, non_upper_case_globals)]
/// ₸: Tenge Sign
pub const SYMBOL_tenge: char = '\u{20b8}';
#[allow(unused, non_upper_case_globals)]
/// ∴: Therefore
pub const SYMBOL_therefore: char = '\u{2234}';
#[allow(unused, non_upper_case_globals)]
/// θ: Greek Small Letter Theta
pub const SYMBOL_theta: char = '\u{3b8}';
#[allow(unused, non_upper_case_globals)]
/// ϑ: Greek Theta Symbol
pub const SYMBOL_theta_alt: char = '\u{3d1}';
#[allow(unused, non_upper_case_globals)]
/// ∼: Tilde Operator
pub const SYMBOL_tilde_op: char = '\u{223c}';
#[allow(unused, non_upper_case_globals)]
/// ~: Tilde
pub const SYMBOL_tilde_basic: char = '\u{7e}';
#[allow(unused, non_upper_case_globals)]
/// ⩪: Tilde Operator With Dot Above
pub const SYMBOL_tilde_dot: char = '\u{2a6a}';
#[allow(unused, non_upper_case_globals)]
/// ≃: Asymptotically Equal To
pub const SYMBOL_tilde_eq: char = '\u{2243}';
#[allow(unused, non_upper_case_globals)]
/// ≄: Not Asymptotically Equal To
pub const SYMBOL_tilde_eq_not: char = '\u{2244}';
#[allow(unused, non_upper_case_globals)]
/// ⋍: Reversed Tilde Equals
pub const SYMBOL_tilde_eq_rev: char = '\u{22cd}';
#[allow(unused, non_upper_case_globals)]
/// ≅: Approximately Equal To
pub const SYMBOL_tilde_equiv: char = '\u{2245}';
#[allow(unused, non_upper_case_globals)]
/// ≇: Neither Approximately Nor Actually Equal To
pub const SYMBOL_tilde_equiv_not: char = '\u{2247}';
#[allow(unused, non_upper_case_globals)]
/// ≆: Approximately But Not Actually Equal To
pub const SYMBOL_tilde_nequiv: char = '\u{2246}';
#[allow(unused, non_upper_case_globals)]
/// ≁: Not Tilde
pub const SYMBOL_tilde_not: char = '\u{2241}';
#[allow(unused, non_upper_case_globals)]
/// ∽: Reversed Tilde
pub const SYMBOL_tilde_rev: char = '\u{223d}';
#[allow(unused, non_upper_case_globals)]
/// ≌: All Equal To
pub const SYMBOL_tilde_rev_equiv: char = '\u{224c}';
#[allow(unused, non_upper_case_globals)]
/// ≋: Triple Tilde
pub const SYMBOL_tilde_triple: char = '\u{224b}';
#[allow(unused, non_upper_case_globals)]
/// ×: Multiplication Sign
pub const SYMBOL_times: char = '\u{d7}';
#[allow(unused, non_upper_case_globals)]
/// ⨉: N Ary Times Operator
pub const SYMBOL_times_big: char = '\u{2a09}';
#[allow(unused, non_upper_case_globals)]
/// ⊗: Circled Times
pub const SYMBOL_times_o: char = '\u{2297}';
#[allow(unused, non_upper_case_globals)]
/// ⨴: Multiplication Sign In Left Half Circle
pub const SYMBOL_times_o_l: char = '\u{2a34}';
#[allow(unused, non_upper_case_globals)]
/// ⨵: Multiplication Sign In Right Half Circle
pub const SYMBOL_times_o_r: char = '\u{2a35}';
#[allow(unused, non_upper_case_globals)]
/// ⨶: Circled Multiplication Sign With Circumflex Accent
pub const SYMBOL_times_o_hat: char = '\u{2a36}';
#[allow(unused, non_upper_case_globals)]
/// ⨂: N Ary Circled Times Operator
pub const SYMBOL_times_o_big: char = '\u{2a02}';
#[allow(unused, non_upper_case_globals)]
/// ⋇: Division Times
pub const SYMBOL_times_div: char = '\u{22c7}';
#[allow(unused, non_upper_case_globals)]
/// ⋋: Left Semidirect Product
pub const SYMBOL_times_three_l: char = '\u{22cb}';
#[allow(unused, non_upper_case_globals)]
/// ⋌: Right Semidirect Product
pub const SYMBOL_times_three_r: char = '\u{22cc}';
#[allow(unused, non_upper_case_globals)]
/// ⋉: Left Normal Factor Semidirect Product
pub const SYMBOL_times_l: char = '\u{22c9}';
#[allow(unused, non_upper_case_globals)]
/// ⋊: Right Normal Factor Semidirect Product
pub const SYMBOL_times_r: char = '\u{22ca}';
#[allow(unused, non_upper_case_globals)]
/// ⊠: Squared Times
pub const SYMBOL_times_square: char = '\u{22a0}';
#[allow(unused, non_upper_case_globals)]
/// ⨻: Multiplication Sign In Triangle
pub const SYMBOL_times_triangle: char = '\u{2a3b}';
#[allow(unused, non_upper_case_globals)]
/// ⧾: Tiny
pub const SYMBOL_tiny: char = '\u{29fe}';
#[allow(unused, non_upper_case_globals)]
/// ₮: Tugrik Sign
pub const SYMBOL_togrog: char = '\u{20ae}';
#[allow(unused, non_upper_case_globals)]
/// ⊤: Down Tack
pub const SYMBOL_top: char = '\u{22a4}';
#[allow(unused, non_upper_case_globals)]
/// ™︎: Trade Mark Sign
pub const SYMBOL_trademark: char = '\u{2122}';
#[allow(unused, non_upper_case_globals)]
/// ®︎: Registered Sign
pub const SYMBOL_trademark_registered: char = '\u{ae}';
#[allow(unused, non_upper_case_globals)]
/// ℠: Service Mark
pub const SYMBOL_trademark_service: char = '\u{2120}';
#[allow(unused, non_upper_case_globals)]
/// △: White Up Pointing Triangle
pub const SYMBOL_triangle_stroked_t: char = '\u{25b3}';
#[allow(unused, non_upper_case_globals)]
/// ▽: White Down Pointing Triangle
pub const SYMBOL_triangle_stroked_b: char = '\u{25bd}';
#[allow(unused, non_upper_case_globals)]
/// ▷: White Right Pointing Triangle
pub const SYMBOL_triangle_stroked_r: char = '\u{25b7}';
#[allow(unused, non_upper_case_globals)]
/// ◁: White Left Pointing Triangle
pub const SYMBOL_triangle_stroked_l: char = '\u{25c1}';
#[allow(unused, non_upper_case_globals)]
/// ◺: Lower Left Triangle
pub const SYMBOL_triangle_stroked_bl: char = '\u{25fa}';
#[allow(unused, non_upper_case_globals)]
/// ◿: Lower Right Triangle
pub const SYMBOL_triangle_stroked_br: char = '\u{25ff}';
#[allow(unused, non_upper_case_globals)]
/// ◸: Upper Left Triangle
pub const SYMBOL_triangle_stroked_tl: char = '\u{25f8}';
#[allow(unused, non_upper_case_globals)]
/// ◹: Upper Right Triangle
pub const SYMBOL_triangle_stroked_tr: char = '\u{25f9}';
#[allow(unused, non_upper_case_globals)]
/// ▵: White Up Pointing Small Triangle
pub const SYMBOL_triangle_stroked_small_t: char = '\u{25b5}';
#[allow(unused, non_upper_case_globals)]
/// ▿: White Down Pointing Small Triangle
pub const SYMBOL_triangle_stroked_small_b: char = '\u{25bf}';
#[allow(unused, non_upper_case_globals)]
/// ▹: White Right Pointing Small Triangle
pub const SYMBOL_triangle_stroked_small_r: char = '\u{25b9}';
#[allow(unused, non_upper_case_globals)]
/// ◃: White Left Pointing Small Triangle
pub const SYMBOL_triangle_stroked_small_l: char = '\u{25c3}';
#[allow(unused, non_upper_case_globals)]
/// 🛆: Triangle With Rounded Corners
pub const SYMBOL_triangle_stroked_rounded: char = '\u{1f6c6}';
#[allow(unused, non_upper_case_globals)]
/// ⟁: White Triangle Containing Small White Triangle
pub const SYMBOL_triangle_stroked_nested: char = '\u{27c1}';
#[allow(unused, non_upper_case_globals)]
/// ◬: White Up Pointing Triangle With Dot
pub const SYMBOL_triangle_stroked_dot: char = '\u{25ec}';
#[allow(unused, non_upper_case_globals)]
/// ▲: Black Up Pointing Triangle
pub const SYMBOL_triangle_filled_t: char = '\u{25b2}';
#[allow(unused, non_upper_case_globals)]
/// ▼: Black Down Pointing Triangle
pub const SYMBOL_triangle_filled_b: char = '\u{25bc}';
#[allow(unused, non_upper_case_globals)]
/// ▶︎: Black Right Pointing Triangle
pub const SYMBOL_triangle_filled_r: char = '\u{25b6}';
#[allow(unused, non_upper_case_globals)]
/// ◀︎: Black Left Pointing Triangle
pub const SYMBOL_triangle_filled_l: char = '\u{25c0}';
#[allow(unused, non_upper_case_globals)]
/// ◣: Black Lower Left Triangle
pub const SYMBOL_triangle_filled_bl: char = '\u{25e3}';
#[allow(unused, non_upper_case_globals)]
/// ◢: Black Lower Right Triangle
pub const SYMBOL_triangle_filled_br: char = '\u{25e2}';
#[allow(unused, non_upper_case_globals)]
/// ◤: Black Upper Left Triangle
pub const SYMBOL_triangle_filled_tl: char = '\u{25e4}';
#[allow(unused, non_upper_case_globals)]
/// ◥: Black Upper Right Triangle
pub const SYMBOL_triangle_filled_tr: char = '\u{25e5}';
#[allow(unused, non_upper_case_globals)]
/// ▴: Black Up Pointing Small Triangle
pub const SYMBOL_triangle_filled_small_t: char = '\u{25b4}';
#[allow(unused, non_upper_case_globals)]
/// ▾: Black Down Pointing Small Triangle
pub const SYMBOL_triangle_filled_small_b: char = '\u{25be}';
#[allow(unused, non_upper_case_globals)]
/// ▸: Black Right Pointing Small Triangle
pub const SYMBOL_triangle_filled_small_r: char = '\u{25b8}';
#[allow(unused, non_upper_case_globals)]
/// ◂: Black Left Pointing Small Triangle
pub const SYMBOL_triangle_filled_small_l: char = '\u{25c2}';
#[allow(unused, non_upper_case_globals)]
/// _: Low Line
pub const SYMBOL_underscore: char = '\u{5f}';
#[allow(unused, non_upper_case_globals)]
/// ∪: Union
pub const SYMBOL_union: char = '\u{222a}';
#[allow(unused, non_upper_case_globals)]
/// ∪︀: Union
pub const SYMBOL_union_serif: char = '\u{222a}';
#[allow(unused, non_upper_case_globals)]
/// ⊌: Multiset
pub const SYMBOL_union_arrow: char = '\u{228c}';
#[allow(unused, non_upper_case_globals)]
/// ⋃: N Ary Union
pub const SYMBOL_union_big: char = '\u{22c3}';
#[allow(unused, non_upper_case_globals)]
/// ⊍: Multiset Multiplication
pub const SYMBOL_union_dot: char = '\u{228d}';
#[allow(unused, non_upper_case_globals)]
/// ⨃: N Ary Union Operator With Dot
pub const SYMBOL_union_dot_big: char = '\u{2a03}';
#[allow(unused, non_upper_case_globals)]
/// ⋓: Double Union
pub const SYMBOL_union_double: char = '\u{22d3}';
#[allow(unused, non_upper_case_globals)]
/// ⩁: Union With Minus Sign
pub const SYMBOL_union_minus: char = '\u{2a41}';
#[allow(unused, non_upper_case_globals)]
/// ⩅: Union With Logical Or
pub const SYMBOL_union_or: char = '\u{2a45}';
#[allow(unused, non_upper_case_globals)]
/// ⊎: Multiset Union
pub const SYMBOL_union_plus: char = '\u{228e}';
#[allow(unused, non_upper_case_globals)]
/// ⨄: N Ary Union Operator With Plus
pub const SYMBOL_union_plus_big: char = '\u{2a04}';
#[allow(unused, non_upper_case_globals)]
/// ⊔: Square Cup
pub const SYMBOL_union_sq: char = '\u{2294}';
#[allow(unused, non_upper_case_globals)]
/// ⊔︀: Square Cup
pub const SYMBOL_union_sq_serif: char = '\u{2294}';
#[allow(unused, non_upper_case_globals)]
/// ⨆: N Ary Square Union Operator
pub const SYMBOL_union_sq_big: char = '\u{2a06}';
#[allow(unused, non_upper_case_globals)]
/// ⩏: Double Square Union
pub const SYMBOL_union_sq_double: char = '\u{2a4f}';
#[allow(unused, non_upper_case_globals)]
/// υ: Greek Small Letter Upsilon
pub const SYMBOL_upsilon: char = '\u{3c5}';
#[allow(unused, non_upper_case_globals)]
/// ⛢: Astronomical Symbol For Uranus
pub const SYMBOL_uranus: char = '\u{26e2}';
#[allow(unused, non_upper_case_globals)]
/// ♅: Uranus
pub const SYMBOL_uranus_alt: char = '\u{2645}';
#[allow(unused, non_upper_case_globals)]
/// ♀︎: Female Sign
pub const SYMBOL_venus: char = '\u{2640}';
#[allow(unused, non_upper_case_globals)]
/// ∖: Set Minus
pub const SYMBOL_without: char = '\u{2216}';
#[allow(unused, non_upper_case_globals)]
/// ⁠: Word Joiner
pub const SYMBOL_wj: char = '\u{2060}';
#[allow(unused, non_upper_case_globals)]
/// ₩: Won Sign
pub const SYMBOL_won: char = '\u{20a9}';
#[allow(unused, non_upper_case_globals)]
/// ≀: Wreath Product
pub const SYMBOL_wreath: char = '\u{2240}';
#[allow(unused, non_upper_case_globals)]
/// ξ: Greek Small Letter Xi
pub const SYMBOL_xi: char = '\u{3be}';
#[allow(unused, non_upper_case_globals)]
/// ⊕: Circled Plus
pub const SYMBOL_xor: char = '\u{2295}';
#[allow(unused, non_upper_case_globals)]
/// ⨁: N Ary Circled Plus Operator
pub const SYMBOL_xor_big: char = '\u{2a01}';
#[allow(unused, non_upper_case_globals)]
/// ¥: Yen Sign
pub const SYMBOL_yen: char = '\u{a5}';
#[allow(unused, non_upper_case_globals)]
/// ¥: Yen Sign
pub const SYMBOL_yuan: char = '\u{a5}';
#[allow(unused, non_upper_case_globals)]
/// 0︎: Digit Zero
pub const SYMBOL_zero: char = '\u{30}';
#[allow(unused, non_upper_case_globals)]
/// 0︀: Digit Zero
pub const SYMBOL_zero_slashed: char = '\u{30}';
#[allow(unused, non_upper_case_globals)]
/// ζ: Greek Small Letter Zeta
pub const SYMBOL_zeta: char = '\u{3b6}';
#[allow(unused, non_upper_case_globals)]
/// ‍: Zero Width Joiner
pub const SYMBOL_zwj: char = '\u{200d}';
#[allow(unused, non_upper_case_globals)]
/// ‌: Zero Width Non Joiner
pub const SYMBOL_zwnj: char = '\u{200c}';
#[allow(unused, non_upper_case_globals)]
/// ​: Zero Width Space
pub const SYMBOL_zws: char = '\u{200b}';
