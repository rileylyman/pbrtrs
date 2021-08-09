use std::ops;

pub trait Numeric:
    Copy
    + Default
    + ops::Add<Output = Self>
    + ops::AddAssign
    + ops::Sub<Output = Self>
    + ops::SubAssign
    + ops::Mul<Output = Self>
    + ops::MulAssign
    + ops::Div<Output = Self>
    + ops::DivAssign
    + ops::Neg<Output = Self>
    + PartialEq
    + PartialOrd
{
    fn m_max_value() -> Self;
    fn m_min_value() -> Self;
    fn m_abs(self) -> Self;
    fn m_div_euclid(self, rhs: Self) -> Self;
    fn m_rem_euclid(self, rhs: Self) -> Self;
    fn m_signum(self) -> Self;
    fn m_max(self, other: Self) -> Self;
    fn m_min(self, other: Self) -> Self;
}

pub trait NumericFloat: Numeric {
    fn m_acos(self) -> Self;
    fn m_acosh(self) -> Self;
    fn m_asin(self) -> Self;
    fn m_asinh(self) -> Self;
    fn m_atan(self) -> Self;
    fn m_atan2(self, other: Self) -> Self;
    fn m_atanh(self) -> Self;
    fn m_ceil(self) -> Self;
    fn m_cbrt(self) -> Self;
    fn m_clamp(self, min: Self, max: Self) -> Self;
    fn m_classify(self) -> std::num::FpCategory;
    fn m_copysign(self, sign: Self) -> Self;
    fn m_cos(self) -> Self;
    fn m_cosh(self) -> Self;
    fn m_exp(self) -> Self;
    fn m_exp2(self) -> Self;
    fn m_exp_m1(self) -> Self;
    fn m_floor(self) -> Self;
    fn m_fract(self) -> Self;
    fn m_hypot(self, other: Self) -> Self;
    fn m_is_finite(self) -> bool;
    fn m_is_infinite(self) -> bool;
    fn m_is_nan(self) -> bool;
    fn m_is_normal(self) -> bool;
    fn m_is_sign_negative(self) -> bool;
    fn m_is_sign_positive(self) -> bool;
    fn m_is_subnormal(self) -> bool;
    fn m_ln(self) -> Self;
    fn m_ln_1p(self) -> Self;
    fn m_log10(self) -> Self;
    fn m_log2(self) -> Self;
    fn m_mul_add(self, a: Self, b: Self) -> Self;
    fn m_powf(self, n: Self) -> Self;
    fn m_recip(self) -> Self;
    fn m_round(self) -> Self;
    fn m_sin(self) -> Self;
    fn m_sin_cos(self) -> (Self, Self);
    fn m_sinh(self) -> Self;
    fn m_sqrt(self) -> Self;
    fn m_tan(self) -> Self;
    fn m_tanh(self) -> Self;
    fn m_to_degrees(self) -> Self;
    fn m_to_radians(self) -> Self;
    fn m_trunc(self) -> Self;
}

macro_rules! impl_numeric {
    ($t:ident) => {
        impl Numeric for $t {
            fn m_max_value() -> Self {
                $t::MAX
            }
            fn m_min_value() -> Self {
                $t::MIN
            }
            fn m_abs(self) -> Self {
                self.abs()
            }
            fn m_div_euclid(self, rhs: Self) -> Self {
                self.div_euclid(rhs)
            }
            fn m_rem_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
            fn m_signum(self) -> Self {
                self.signum()
            }
            fn m_max(self, other: Self) -> Self {
                self.max(other)
            }
            fn m_min(self, other: Self) -> Self {
                self.min(other)
            }
        }
    };
}

macro_rules! impl_numeric_float {
    ($t:ident) => {
        impl NumericFloat for $t {
            fn m_acos(self) -> Self {
                self.acos()
            }
            fn m_acosh(self) -> Self {
                self.acosh()
            }
            fn m_asin(self) -> Self {
                self.asin()
            }
            fn m_asinh(self) -> Self {
                self.asinh()
            }
            fn m_atan(self) -> Self {
                self.atan()
            }
            fn m_atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            fn m_atanh(self) -> Self {
                self.atanh()
            }
            fn m_ceil(self) -> Self {
                self.ceil()
            }
            fn m_cbrt(self) -> Self {
                self.cbrt()
            }
            fn m_clamp(self, min: Self, max: Self) -> Self {
                self.clamp(min, max)
            }
            fn m_classify(self) -> std::num::FpCategory {
                self.classify()
            }
            fn m_copysign(self, sign: Self) -> Self {
                self.copysign(sign)
            }
            fn m_cos(self) -> Self {
                self.cos()
            }
            fn m_cosh(self) -> Self {
                self.cosh()
            }
            fn m_exp(self) -> Self {
                self.exp()
            }
            fn m_exp2(self) -> Self {
                self.exp2()
            }
            fn m_exp_m1(self) -> Self {
                self.exp_m1()
            }
            fn m_floor(self) -> Self {
                self.floor()
            }
            fn m_fract(self) -> Self {
                self.fract()
            }
            fn m_hypot(self, other: Self) -> Self {
                self.hypot(other)
            }
            fn m_is_finite(self) -> bool {
                self.is_finite()
            }
            fn m_is_infinite(self) -> bool {
                self.is_infinite()
            }
            fn m_is_nan(self) -> bool {
                self.is_nan()
            }
            fn m_is_normal(self) -> bool {
                self.is_normal()
            }
            fn m_is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }
            fn m_is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }
            fn m_is_subnormal(self) -> bool {
                self.is_subnormal()
            }
            fn m_ln(self) -> Self {
                self.ln()
            }
            fn m_ln_1p(self) -> Self {
                self.ln_1p()
            }
            fn m_log10(self) -> Self {
                self.log10()
            }
            fn m_log2(self) -> Self {
                self.log2()
            }
            fn m_mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }
            fn m_powf(self, n: Self) -> Self {
                self.powf(n)
            }
            fn m_recip(self) -> Self {
                self.recip()
            }
            fn m_round(self) -> Self {
                self.round()
            }
            fn m_sin(self) -> Self {
                self.sin()
            }
            fn m_sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }
            fn m_sinh(self) -> Self {
                self.sinh()
            }
            fn m_sqrt(self) -> Self {
                self.sqrt()
            }
            fn m_tan(self) -> Self {
                self.tan()
            }
            fn m_tanh(self) -> Self {
                self.tanh()
            }
            fn m_to_degrees(self) -> Self {
                self.to_degrees()
            }
            fn m_to_radians(self) -> Self {
                self.to_radians()
            }
            fn m_trunc(self) -> Self {
                self.trunc()
            }
        }
    };
}

impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(f32);
impl_numeric!(f64);
impl_numeric_float!(f32);
impl_numeric_float!(f64);
