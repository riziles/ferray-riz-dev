// ferray-ufunc: Correctly-rounded math dispatch
//
// Native targets: uses INRIA's CORE-MATH library (via the `core-math` crate)
// for f64 and f32 transcendental functions. CORE-MATH guarantees
// correctly-rounded results (< 0.5 ULP from the mathematical truth),
// which is better than glibc, openlibm, and NumPy's backends.
//
// WASM targets: core-math-sys → bindgen → libclang does not resolve for
// wasm32-unknown-unknown, so we swap in the pure-Rust `libm` crate instead.
// libm provides faithful (not correctly-rounded) results — accuracy is
// within a few ULP, which is sufficient for browser-deployed workloads.
//
// For types other than f64/f32, falls back to num_traits::Float methods.

// Every method on the `CrMath` trait is a pure transcendental that returns
// a new `Self`; the contract is uniform and the `#[must_use]` decoration
// would be strictly redundant noise on every method.
#![allow(clippy::return_self_not_must_use)]

/// Trait providing correctly-rounded math operations.
///
/// Implemented for f64 and f32 using CORE-MATH (native) or libm (wasm32),
/// with a blanket fallback to `num_traits::Float` for other types.
pub trait CrMath: Copy {
    fn cr_sin(self) -> Self;
    fn cr_cos(self) -> Self;
    fn cr_tan(self) -> Self;
    fn cr_asin(self) -> Self;
    fn cr_acos(self) -> Self;
    fn cr_atan(self) -> Self;
    fn cr_atan2(self, other: Self) -> Self;
    fn cr_sinh(self) -> Self;
    fn cr_cosh(self) -> Self;
    fn cr_tanh(self) -> Self;
    fn cr_asinh(self) -> Self;
    fn cr_acosh(self) -> Self;
    fn cr_atanh(self) -> Self;
    fn cr_exp(self) -> Self;
    fn cr_exp2(self) -> Self;
    fn cr_exp_m1(self) -> Self;
    fn cr_ln(self) -> Self;
    fn cr_log2(self) -> Self;
    fn cr_log10(self) -> Self;
    fn cr_ln_1p(self) -> Self;
    fn cr_cbrt(self) -> Self;
    fn cr_hypot(self, other: Self) -> Self;
}

// ---------------------------------------------------------------------------
// Native targets: CORE-MATH (correctly-rounded, < 0.5 ULP)
// ---------------------------------------------------------------------------
#[cfg(not(target_arch = "wasm32"))]
impl CrMath for f64 {
    #[inline]
    fn cr_sin(self) -> Self {
        core_math::sin(self)
    }
    #[inline]
    fn cr_cos(self) -> Self {
        core_math::cos(self)
    }
    #[inline]
    fn cr_tan(self) -> Self {
        core_math::tan(self)
    }
    #[inline]
    fn cr_asin(self) -> Self {
        core_math::asin(self)
    }
    #[inline]
    fn cr_acos(self) -> Self {
        core_math::acos(self)
    }
    #[inline]
    fn cr_atan(self) -> Self {
        core_math::atan(self)
    }
    #[inline]
    fn cr_atan2(self, other: Self) -> Self {
        core_math::atan2(self, other)
    }
    #[inline]
    fn cr_sinh(self) -> Self {
        core_math::sinh(self)
    }
    #[inline]
    fn cr_cosh(self) -> Self {
        core_math::cosh(self)
    }
    #[inline]
    fn cr_tanh(self) -> Self {
        core_math::tanh(self)
    }
    #[inline]
    fn cr_asinh(self) -> Self {
        core_math::asinh(self)
    }
    #[inline]
    fn cr_acosh(self) -> Self {
        core_math::acosh(self)
    }
    #[inline]
    fn cr_atanh(self) -> Self {
        core_math::atanh(self)
    }
    #[inline]
    fn cr_exp(self) -> Self {
        core_math::exp(self)
    }
    #[inline]
    fn cr_exp2(self) -> Self {
        core_math::exp2(self)
    }
    #[inline]
    fn cr_exp_m1(self) -> Self {
        core_math::expm1(self)
    }
    #[inline]
    fn cr_ln(self) -> Self {
        core_math::log(self)
    }
    #[inline]
    fn cr_log2(self) -> Self {
        core_math::log2(self)
    }
    #[inline]
    fn cr_log10(self) -> Self {
        core_math::log10(self)
    }
    #[inline]
    fn cr_ln_1p(self) -> Self {
        core_math::log1p(self)
    }
    #[inline]
    fn cr_cbrt(self) -> Self {
        core_math::cbrt(self)
    }
    #[inline]
    fn cr_hypot(self, other: Self) -> Self {
        core_math::hypot(self, other)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl CrMath for f32 {
    #[inline]
    fn cr_sin(self) -> Self {
        core_math::sinf(self)
    }
    #[inline]
    fn cr_cos(self) -> Self {
        core_math::cosf(self)
    }
    #[inline]
    fn cr_tan(self) -> Self {
        core_math::tanf(self)
    }
    #[inline]
    fn cr_asin(self) -> Self {
        core_math::asinf(self)
    }
    #[inline]
    fn cr_acos(self) -> Self {
        core_math::acosf(self)
    }
    #[inline]
    fn cr_atan(self) -> Self {
        core_math::atanf(self)
    }
    #[inline]
    fn cr_atan2(self, other: Self) -> Self {
        core_math::atan2f(self, other)
    }
    #[inline]
    fn cr_sinh(self) -> Self {
        core_math::sinhf(self)
    }
    #[inline]
    fn cr_cosh(self) -> Self {
        core_math::coshf(self)
    }
    #[inline]
    fn cr_tanh(self) -> Self {
        core_math::tanhf(self)
    }
    #[inline]
    fn cr_asinh(self) -> Self {
        core_math::asinhf(self)
    }
    #[inline]
    fn cr_acosh(self) -> Self {
        core_math::acoshf(self)
    }
    #[inline]
    fn cr_atanh(self) -> Self {
        core_math::atanhf(self)
    }
    #[inline]
    fn cr_exp(self) -> Self {
        core_math::expf(self)
    }
    #[inline]
    fn cr_exp2(self) -> Self {
        core_math::exp2f(self)
    }
    #[inline]
    fn cr_exp_m1(self) -> Self {
        core_math::expm1f(self)
    }
    #[inline]
    fn cr_ln(self) -> Self {
        core_math::logf(self)
    }
    #[inline]
    fn cr_log2(self) -> Self {
        core_math::log2f(self)
    }
    #[inline]
    fn cr_log10(self) -> Self {
        core_math::log10f(self)
    }
    #[inline]
    fn cr_ln_1p(self) -> Self {
        core_math::log1pf(self)
    }
    #[inline]
    fn cr_cbrt(self) -> Self {
        core_math::cbrtf(self)
    }
    #[inline]
    fn cr_hypot(self, other: Self) -> Self {
        core_math::hypotf(self, other)
    }
}

// ---------------------------------------------------------------------------
// WASM targets: libm (pure-Rust, faithful rounding, a few ULP accuracy)
// ---------------------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
impl CrMath for f64 {
    #[inline]
    fn cr_sin(self) -> Self {
        libm::sin(self)
    }
    #[inline]
    fn cr_cos(self) -> Self {
        libm::cos(self)
    }
    #[inline]
    fn cr_tan(self) -> Self {
        libm::tan(self)
    }
    #[inline]
    fn cr_asin(self) -> Self {
        libm::asin(self)
    }
    #[inline]
    fn cr_acos(self) -> Self {
        libm::acos(self)
    }
    #[inline]
    fn cr_atan(self) -> Self {
        libm::atan(self)
    }
    #[inline]
    fn cr_atan2(self, other: Self) -> Self {
        libm::atan2(self, other)
    }
    #[inline]
    fn cr_sinh(self) -> Self {
        libm::sinh(self)
    }
    #[inline]
    fn cr_cosh(self) -> Self {
        libm::cosh(self)
    }
    #[inline]
    fn cr_tanh(self) -> Self {
        libm::tanh(self)
    }
    #[inline]
    fn cr_asinh(self) -> Self {
        libm::asinh(self)
    }
    #[inline]
    fn cr_acosh(self) -> Self {
        libm::acosh(self)
    }
    #[inline]
    fn cr_atanh(self) -> Self {
        libm::atanh(self)
    }
    #[inline]
    fn cr_exp(self) -> Self {
        libm::exp(self)
    }
    #[inline]
    fn cr_exp2(self) -> Self {
        libm::exp2(self)
    }
    #[inline]
    fn cr_exp_m1(self) -> Self {
        libm::expm1(self)
    }
    #[inline]
    fn cr_ln(self) -> Self {
        libm::log(self)
    }
    #[inline]
    fn cr_log2(self) -> Self {
        libm::log2(self)
    }
    #[inline]
    fn cr_log10(self) -> Self {
        libm::log10(self)
    }
    #[inline]
    fn cr_ln_1p(self) -> Self {
        libm::log1p(self)
    }
    #[inline]
    fn cr_cbrt(self) -> Self {
        libm::cbrt(self)
    }
    #[inline]
    fn cr_hypot(self, other: Self) -> Self {
        libm::hypot(self, other)
    }
}

#[cfg(target_arch = "wasm32")]
impl CrMath for f32 {
    #[inline]
    fn cr_sin(self) -> Self {
        libm::sinf(self)
    }
    #[inline]
    fn cr_cos(self) -> Self {
        libm::cosf(self)
    }
    #[inline]
    fn cr_tan(self) -> Self {
        libm::tanf(self)
    }
    #[inline]
    fn cr_asin(self) -> Self {
        libm::asinf(self)
    }
    #[inline]
    fn cr_acos(self) -> Self {
        libm::acosf(self)
    }
    #[inline]
    fn cr_atan(self) -> Self {
        libm::atanf(self)
    }
    #[inline]
    fn cr_atan2(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }
    #[inline]
    fn cr_sinh(self) -> Self {
        libm::sinhf(self)
    }
    #[inline]
    fn cr_cosh(self) -> Self {
        libm::coshf(self)
    }
    #[inline]
    fn cr_tanh(self) -> Self {
        libm::tanhf(self)
    }
    #[inline]
    fn cr_asinh(self) -> Self {
        libm::asinhf(self)
    }
    #[inline]
    fn cr_acosh(self) -> Self {
        libm::acoshf(self)
    }
    #[inline]
    fn cr_atanh(self) -> Self {
        libm::atanhf(self)
    }
    #[inline]
    fn cr_exp(self) -> Self {
        libm::expf(self)
    }
    #[inline]
    fn cr_exp2(self) -> Self {
        libm::exp2f(self)
    }
    #[inline]
    fn cr_exp_m1(self) -> Self {
        libm::expm1f(self)
    }
    #[inline]
    fn cr_ln(self) -> Self {
        libm::logf(self)
    }
    #[inline]
    fn cr_log2(self) -> Self {
        libm::log2f(self)
    }
    #[inline]
    fn cr_log10(self) -> Self {
        libm::log10f(self)
    }
    #[inline]
    fn cr_ln_1p(self) -> Self {
        libm::log1pf(self)
    }
    #[inline]
    fn cr_cbrt(self) -> Self {
        libm::cbrtf(self)
    }
    #[inline]
    fn cr_hypot(self, other: Self) -> Self {
        libm::hypotf(self, other)
    }
}
