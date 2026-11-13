//! 1€ (One Euro) Filter implementation for adaptive velocity-based tremor suppression.
//! Based on Casiez, Roussel, and Vogel (CHI 2012).
//!
//! Designed for zero-lag high-velocity tracking combined with high-stability low-velocity damping.

#![allow(dead_code)]

/// Low-pass filter single-step state.
#[derive(Debug, Clone, Copy)]
pub struct LowPassFilter {
    hat_x_prev: f64,
    has_first_value: bool,
}

impl LowPassFilter {
    pub const fn new() -> Self {
        Self {
            hat_x_prev: 0.0,
            has_first_value: false,
        }
    }

    /// Computes smoothing factor alpha from cutoff frequency and delta time.
    /// alpha = 1 / (1 + tau / dt), where tau = 1 / (2 * pi * fc)
    pub fn compute_alpha(rate_hz: f64, cutoff_hz: f64) -> f64 {
        if rate_hz <= 0.0 || cutoff_hz <= 0.0 {
            return 1.0;
        }
        let tau = 1.0 / (2.0 * core::f64::consts::PI * cutoff_hz);
        let te = 1.0 / rate_hz;
        1.0 / (1.0 + tau / te)
    }

    /// Filters raw input `x` with factor `alpha` in range [0, 1].
    pub fn filter(&mut self, x: f64, alpha: f64) -> f64 {
        if !self.has_first_value {
            self.has_first_value = true;
            self.hat_x_prev = x;
            return x;
        }
        let clamped_alpha = if alpha < 0.0 {
            0.0
        } else if alpha > 1.0 {
            1.0
        } else {
            alpha
        };
        let hat_x = clamped_alpha * x + (1.0 - clamped_alpha) * self.hat_x_prev;
        self.hat_x_prev = hat_x;
        hat_x
    }

    pub fn last_value(&self) -> f64 {
        self.hat_x_prev
    }

    pub fn reset(&mut self) {
        self.hat_x_prev = 0.0;
        self.has_first_value = false;
    }
}

/// A 1-Dimensional 1€ Filter.
#[derive(Debug, Clone, Copy)]
pub struct OneEuroFilter {
    min_cutoff: f64,
    beta: f64,
    d_cutoff: f64,
    x_filter: LowPassFilter,
    dx_filter: LowPassFilter,
    last_time_sec: f64,
}

impl OneEuroFilter {
    /// Creates a new 1€ filter.
    ///
    /// * `min_cutoff` - Minimum cutoff frequency in Hz (typically 0.5 to 1.5 Hz). Lower = more damping at rest.
    /// * `beta` - Speed coefficient (typically 0.001 to 0.05). Higher = less lag when moving fast.
    /// * `d_cutoff` - Derivative cutoff frequency in Hz (typically 1.0 Hz).
    pub const fn new(min_cutoff: f64, beta: f64, d_cutoff: f64) -> Self {
        Self {
            min_cutoff,
            beta,
            d_cutoff,
            x_filter: LowPassFilter::new(),
            dx_filter: LowPassFilter::new(),
            last_time_sec: -1.0,
        }
    }

    /// Filters an incoming sample `(x, timestamp_sec)`.
    pub fn filter(&mut self, x: f64, timestamp_sec: f64) -> f64 {
        if self.last_time_sec < 0.0 {
            self.last_time_sec = timestamp_sec;
            return self.x_filter.filter(x, 1.0);
        }

        let dt = timestamp_sec - self.last_time_sec;
        self.last_time_sec = timestamp_sec;

        // Prevent division by zero or negative time jumps
        if dt <= 1e-6 {
            return self.x_filter.last_value();
        }

        let rate_hz = 1.0 / dt;

        // Estimate derivative (speed) with low-pass filtering
        let dx = (x - self.x_filter.last_value()) * rate_hz;
        let d_alpha = LowPassFilter::compute_alpha(rate_hz, self.d_cutoff);
        let hat_dx = self.dx_filter.filter(dx, d_alpha);

        // Adaptive cutoff frequency
        let abs_hat_dx = if hat_dx < 0.0 { -hat_dx } else { hat_dx };
        let cutoff = self.min_cutoff + self.beta * abs_hat_dx;

        // Filter the actual signal
        let alpha = LowPassFilter::compute_alpha(rate_hz, cutoff);
        self.x_filter.filter(x, alpha)
    }

    pub fn reset(&mut self) {
        self.x_filter.reset();
        self.dx_filter.reset();
        self.last_time_sec = -1.0;
    }
}

/// A 2-Dimensional Point Filter (combining X and Y channels).
#[derive(Debug, Clone, Copy)]
pub struct Point2DFilter {
    x_filter: OneEuroFilter,
    y_filter: OneEuroFilter,
}

impl Point2DFilter {
    pub const fn new(min_cutoff: f64, beta: f64, d_cutoff: f64) -> Self {
        Self {
            x_filter: OneEuroFilter::new(min_cutoff, beta, d_cutoff),
            y_filter: OneEuroFilter::new(min_cutoff, beta, d_cutoff),
        }
    }

    pub fn filter(&mut self, x: f64, y: f64, timestamp_sec: f64) -> (f64, f64) {
        let filtered_x = self.x_filter.filter(x, timestamp_sec);
        let filtered_y = self.y_filter.filter(y, timestamp_sec);
        (filtered_x, filtered_y)
    }

    pub fn reset(&mut self) {
        self.x_filter.reset();
        self.y_filter.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_pass_initial_value() {
        let mut lpf = LowPassFilter::new();
        let val = lpf.filter(42.0, 0.5);
        assert_eq!(val, 42.0);
    }

    #[test]
    fn test_low_pass_smoothing() {
        let mut lpf = LowPassFilter::new();
        lpf.filter(0.0, 1.0);
        let smoothed = lpf.filter(100.0, 0.1);
        assert!((smoothed - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_tremor_damping_at_low_speed() {
        let mut filter = Point2DFilter::new(1.0, 0.005, 1.0);
        let dt = 1.0 / 60.0; // 60 FPS
        let mut t = 0.0;

        // Base point (100.0, 100.0) with an oscillating 8Hz tremor (hand shaking)
        let mut max_output_shake = 0.0;
        for i in 0..60 {
            t += dt;
            let tremor = 5.0 * ((i as f64 * 0.8).sin()); // Shake amplitude +/- 5px
            let input_x = 100.0 + tremor;
            let input_y = 100.0 + tremor;
            let (out_x, _) = filter.filter(input_x, input_y, t);

            if i > 10 {
                let diff = (out_x - 100.0).abs();
                if diff > max_output_shake {
                    max_output_shake = diff;
                }
            }
        }

        // Tremor should be significantly attenuated from 5.0px down to < 2.0px
        assert!(max_output_shake < 2.0, "Tremor was not sufficiently attenuated: {}", max_output_shake);
    }

    #[test]
    fn test_zero_lag_at_high_speed() {
        let mut filter = Point2DFilter::new(1.0, 0.1, 1.0);
        let dt = 1.0 / 60.0;
        let mut t = 0.0;

        // Rapid ballistic movement across screen
        let mut last_out_x = 0.0;
        for i in 0..10 {
            t += dt;
            let input_x = i as f64 * 50.0; // Moving 50px per frame (fast swipe)
            let (out_x, _) = filter.filter(input_x, 0.0, t);
            last_out_x = out_x;
        }

        // At frame 9 (target 450), the filtered value should be very close (minimal lag)
        assert!((450.0 - last_out_x).abs() < 25.0, "High speed lag was too high: {}", 450.0 - last_out_x);
    }
}
