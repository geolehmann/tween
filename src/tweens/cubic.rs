use crate::{Tween, TweenTime, TweenValue};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubicIn<TValue = f32, TTime = f32> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> CubicIn<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        Self {
            range,
            value_delta: delta,
            duration,
        }
    }
}

impl<V, T> Tween for CubicIn<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time);
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubicOut<TValue = f32, TTime = f32> {
    range: RangeInclusive<TValue>,
    value_delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> CubicOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let delta = TValue::calculate_delta(*range.end(), *range.start());
        Self {
            range,
            value_delta: delta,
            duration,
        }
    }
}

impl<V, T> Tween for CubicOut<V, T>
where
    V: TweenValue,
    T: TweenTime,
{
    type Value = V;
    type Time = T;

    fn update(&mut self, new_time: T) -> V {
        let percent_time = T::percent(self.duration, new_time) - 1.0;
        let new_value = self
            .value_delta
            .scale(percent_time * percent_time * percent_time + 1.0);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<V> {
        &self.range
    }

    fn duration(&self) -> T {
        self.duration
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CubicInOut<TValue = f32, TTime = f32> {
    range: RangeInclusive<TValue>,
    half_delta: TValue,
    duration: TTime,
}

impl<TValue, TTime> CubicInOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    pub fn new(range: RangeInclusive<TValue>, duration: TTime) -> Self {
        let value_delta = TValue::calculate_delta(*range.end(), *range.start());
        let half_delta = TValue::scale(value_delta, 0.5);
        Self {
            range,
            half_delta,
            duration,
        }
    }
}

impl<TValue, TTime> Tween for CubicInOut<TValue, TTime>
where
    TValue: TweenValue,
    TTime: TweenTime,
{
    type Value = TValue;
    type Time = TTime;

    fn update(&mut self, new_time: TTime) -> TValue {
        let percent_time = TTime::percent(self.duration, new_time) * 2.0;

        let scalar = if percent_time < 1.0 {
            percent_time * percent_time * percent_time
        } else {
            let p = percent_time - 2.0;
            p * p * p + 2.0
        };
        let new_value = self.half_delta.scale(scalar);

        new_value.add(*self.range.start())
    }

    fn range(&self) -> &RangeInclusive<TValue> {
        &self.range
    }

    fn duration(&self) -> TTime {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;
    use easer::functions::{Cubic as EaseCubic, Easing};

    #[test]
    fn cubic_in() {
        let mut tweener = CubicIn::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.update(time);
            let o = EaseCubic::ease_in(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn cubic_out() {
        let mut tweener = CubicOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let v = tweener.update(time);
            let o = EaseCubic::ease_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(v, o);
        }
    }

    #[test]
    fn cubic_in_out() {
        let mut tweener = CubicInOut::new(0.0..=100.0, 10.0);

        for time in 0..=10 {
            let time = time as f32;

            let our_value = tweener.update(time);
            let easer = EaseCubic::ease_in_out(time, 0.0, 100.0, 10.0);

            assert_ulps_eq!(our_value, easer);
        }
    }
}
