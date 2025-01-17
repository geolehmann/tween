use crate::{Tween, TweenTime, TweenValue};
use core::ops::RangeInclusive;

const MAGIC: f64 = 7.5625;
const STAGE_ZERO: f64 = 1.0 / 2.75;
const STAGE_ONE: f64 = 2.0 / 2.75;
const STAGE_TWO: f64 = 2.5 / 2.75;

declare_tween!(
    /// An bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInBounce) for a visual demonstration.
    pub struct BounceIn;

    fn run(&mut self, new_time: T) -> V {
        let v = {
            let t = T::percent(self.duration, self.duration.sub(new_time));

            let multip = if t < STAGE_ZERO {
                MAGIC * t * t
            } else if t < STAGE_ONE {
                let t = t - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if t < STAGE_TWO {
                let t = t - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = t - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            self.value_delta.scale(multip)
        };

        TweenValue::calculate_delta(self.value_delta, v).add(*self.range.start())
    }
);

declare_tween!(
    /// An bouncy tween, similar to gravity. Go [here](https://easings.net/#easeOutBounce) for a visual demonstration.
    pub struct BounceOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        let multip = if t < STAGE_ZERO {
            MAGIC * t * t
        } else if t < STAGE_ONE {
            let t = t - 1.5 / 2.75;
            MAGIC * t * t + 0.75
        } else if t < STAGE_TWO {
            let t = t - 2.25 / 2.75;
            MAGIC * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;

            MAGIC * t * t + 0.984375
        };

        self.value_delta.scale(multip).add(*self.range.start())
    }
);

declare_tween!(
    /// An bouncy tween, similar to gravity. Go [here](https://easings.net/#easeInOutBounce) for a visual demonstration.
    pub struct BounceInOut;

    fn run(&mut self, new_time: T) -> V {
        let t = T::percent(self.duration, new_time);

        if t < 0.5 {
            let t = T::percent(self.duration, self.duration.sub(new_time.scale(2.0)));

            let v = {
                let multip = if t < STAGE_ZERO {
                    MAGIC * t * t
                } else if t < STAGE_ONE {
                    let t = t - 1.5 / 2.75;
                    MAGIC * t * t + 0.75
                } else if t < STAGE_TWO {
                    let t = t - 2.25 / 2.75;
                    MAGIC * t * t + 0.9375
                } else {
                    let t = t - 2.625 / 2.75;

                    MAGIC * t * t + 0.984375
                };

                self.value_delta.scale(multip)
            };

            TweenValue::calculate_delta(self.value_delta, v)
                .scale(0.5)
                .add(*self.range.start())
        } else {
            let t = T::percent(self.duration, new_time.scale(2.0).sub(self.duration));

            let multip = if t < STAGE_ZERO {
                MAGIC * t * t
            } else if t < STAGE_ONE {
                let t = t - 1.5 / 2.75;
                MAGIC * t * t + 0.75
            } else if t < STAGE_TWO {
                let t = t - 2.25 / 2.75;
                MAGIC * t * t + 0.9375
            } else {
                let t = t - 2.625 / 2.75;

                MAGIC * t * t + 0.984375
            };

            self.value_delta
                .scale(multip)
                .scale(0.5)
                .add(self.value_delta.scale(0.5))
                .add(*self.range.start())
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Bounce, Easing};

    #[test]
    fn tween_in() {
        let mut tweener = BounceIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.run(time);
            let o = Bounce::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_out() {
        let mut tweener = BounceOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let v = tweener.run(time);
            let o = Bounce::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn tween_in_out() {
        let mut tweener = BounceInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f64;

            let our_value = tweener.run(time);
            let easer = Bounce::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
