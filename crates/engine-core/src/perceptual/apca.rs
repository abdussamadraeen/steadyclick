//! Accessible Perceptual Contrast Algorithm (APCA) implementation.
//! Candidate standard for W3C WCAG 3.0.
//!
//! Models physiological human contrast perception, polarity (light-on-dark vs dark-on-light),
//! and spatial frequency to replace the flawed legacy WCAG 2.x ratio.

#![allow(dead_code)]

/// Linearizes an 8-bit sRGB color channel into relative screen luminance space.
/// IEC 61966-2-1 power curve approximation (exponent = 2.4).
#[inline]
fn srgb_to_linear(val: u8) -> f64 {
    let normalized = (val as f64) / 255.0;
    pow_f64(normalized, 2.4)
}

/// Computes x^y for positive x.
#[inline]
pub fn pow_f64(base: f64, exp: f64) -> f64 {
    if base <= 0.0 {
        return 0.0;
    }
    base.powf(exp)
}

/// Computes relative luminance (Y) from sRGB (0-255).
pub fn srgb_to_luminance(r: u8, g: u8, b: u8) -> f64 {
    let r_lin = srgb_to_linear(r);
    let g_lin = srgb_to_linear(g);
    let b_lin = srgb_to_linear(b);

    // CIE 1931 standard Rec.709 primaries
    0.2126729 * r_lin + 0.7151522 * g_lin + 0.0721750 * b_lin
}

/// APCA Contrast Result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApcaResult {
    /// Lightness Contrast value (Lc).
    /// Positive values (+Lc) mean dark text on a light background.
    /// Negative values (-Lc) mean light text on a dark background.
    /// Values range from approximately -108 to +106.
    pub lc: f64,
    /// Absolute contrast magnitude.
    pub abs_lc: f64,
    /// Whether the text is darker than the background.
    pub is_dark_on_light: bool,
}

impl ApcaResult {
    /// Checks if this contrast level satisfies a given target criteria based on font size and weight.
    /// Recommended APCA minimums:
    /// - Body text (16px normal): Lc >= 75
    /// - Large text (24px normal / 18px bold): Lc >= 60
    /// - Heavy headlines (32px+ bold): Lc >= 45
    pub fn is_accessible_for_size(&self, font_size_px: f64, is_bold: bool) -> bool {
        let required_lc = if font_size_px >= 32.0 || (font_size_px >= 24.0 && is_bold) {
            45.0
        } else if font_size_px >= 20.0 || (font_size_px >= 16.0 && is_bold) {
            60.0
        } else if font_size_px >= 14.0 {
            75.0
        } else {
            90.0 // Small captions need very high contrast
        };

        self.abs_lc >= required_lc
    }
}

/// Calculates the APCA Lightness Contrast (Lc) between text and background.
pub fn calculate_apca(txt_r: u8, txt_g: u8, txt_b: u8, bg_r: u8, bg_g: u8, bg_b: u8) -> ApcaResult {
    let y_txt = srgb_to_luminance(txt_r, txt_g, txt_b);
    let y_bg = srgb_to_luminance(bg_r, bg_g, bg_b);

    // APCA 0.98G constants
    const SCALE_FACTOR: f64 = 1.14;
    const BLK_THRESH: f64 = 0.022;
    const BLK_CLMP: f64 = 1.414;

    // Apply soft black clamping for low-luminance flare reduction
    let y_txt_clamped = if y_txt > BLK_THRESH {
        y_txt
    } else {
        y_txt + pow_f64(BLK_THRESH - y_txt, BLK_CLMP)
    };

    let y_bg_clamped = if y_bg > BLK_THRESH {
        y_bg
    } else {
        y_bg + pow_f64(BLK_THRESH - y_bg, BLK_CLMP)
    };

    // Calculate contrast based on polarity
    let (lc, is_dark_on_light) = if y_bg_clamped >= y_txt_clamped {
        // Dark text on light background (Positive Lc)
        let sapc = (pow_f64(y_bg_clamped, 0.56) - pow_f64(y_txt_clamped, 0.57)) * SCALE_FACTOR;
        let score = if sapc < 0.1 {
            0.0
        } else {
            (sapc - 0.027) * 100.0
        };
        (score, true)
    } else {
        // Light text on dark background (Negative Lc)
        let sapc = (pow_f64(y_bg_clamped, 0.65) - pow_f64(y_txt_clamped, 0.62)) * SCALE_FACTOR;
        let score = if sapc > -0.1 {
            0.0
        } else {
            (sapc + 0.027) * 100.0
        };
        (score, false)
    };

    let abs_lc = if lc < 0.0 { -lc } else { lc };

    ApcaResult {
        lc,
        abs_lc,
        is_dark_on_light,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_on_white_contrast() {
        // Pure black on pure white should yield near maximum positive contrast (~106 Lc)
        let res = calculate_apca(0, 0, 0, 255, 255, 255);
        assert!(res.is_dark_on_light);
        assert!(res.lc > 100.0, "Black on white contrast should be > 100, got {}", res.lc);
        assert!(res.is_accessible_for_size(16.0, false));
    }

    #[test]
    fn test_white_on_black_contrast() {
        // Pure white on pure black should yield near maximum negative contrast (~ -106 Lc)
        let res = calculate_apca(255, 255, 255, 0, 0, 0);
        assert!(!res.is_dark_on_light);
        assert!(res.lc < -100.0, "White on black contrast should be < -100, got {}", res.lc);
        assert!(res.is_accessible_for_size(16.0, false));
    }

    #[test]
    fn test_identical_colors_zero_contrast() {
        let res = calculate_apca(128, 128, 128, 128, 128, 128);
        assert_eq!(res.abs_lc, 0.0);
        assert!(!res.is_accessible_for_size(16.0, false));
    }

    #[test]
    fn test_poor_contrast_flagged() {
        // Light gray text on white background: should fail body text check
        let res = calculate_apca(180, 180, 180, 255, 255, 255);
        assert!(!res.is_accessible_for_size(16.0, false));
    }
}
