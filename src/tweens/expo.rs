use crate::{Tween, TweenTime, TweenValue};
use core::ops::RangeInclusive;

declare_tween!(
    /// An exponenential tween in. See [here](https://easings.net/#easeInExpo)
    pub struct ExpoIn;

    fn run(&mut self, new_time: T) -> V {
        if new_time == T::ZERO {
            *self.range.start()
        } else {
            let percent_time = 10.0 * (T::percent(self.duration, new_time) - 1.0);
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, percent_time);

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(percent_time);

            let new_value = self.value_delta.scale(scalar);

            new_value.add(*self.range.start())
        }
    }
);

declare_tween!(
    /// An exponenential tween out. See [here](https://easings.net/#easeOutExpo)
    pub struct ExpoOut;

    fn run(&mut self, new_time: T) -> V {
        if new_time == self.duration {
            *self.range.end()
        } else {
            #[cfg(feature = "libm")]
            let powf = libm::pow(2.0, -10.0 * T::percent(self.duration, new_time));

            #[cfg(feature = "std")]
            let powf = 2.0f64.powf(-10.0 * T::percent(self.duration, new_time));

            let new_value = self.value_delta.scale(1.0 - powf);

            new_value.add(*self.range.start())
        }
    }
);

declare_tween!(
    /// An exponenential tween in and out. See [here](https://easings.net/#easeInOutExpo)
    pub struct ExpoInOut;

    fn run(&mut self, new_time: T) -> V {
        if new_time == T::ZERO {
            return *self.range.start();
        }

        if new_time == self.duration {
            return *self.range.end();
        }

        let t = T::percent(self.duration, new_time) * 2.0;

        let powf = if t < 1.0 {
            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, 10.0 * (t - 1.0));

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(10.0 * (t - 1.0));

            scalar / 2.0
        } else {
            let t = t - 1.0;

            #[cfg(feature = "libm")]
            let scalar = libm::pow(2.0, -10.0 * t);

            #[cfg(feature = "std")]
            let scalar = 2.0f64.powf(-10.0 * t);

            (2.0 - scalar) / 2.0
        };

        let new_value = self.value_delta.scale(powf);

        new_value.add(*self.range.start())
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Easing, Expo};

    #[test]
    fn tween_in() {
        let mut tweener = ExpoIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.run(time);
            let o = Expo::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = ExpoOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.run(time);
            let o = Expo::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = ExpoInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.run(time);
            let easer = Expo::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
