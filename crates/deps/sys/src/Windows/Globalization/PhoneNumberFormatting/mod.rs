#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPhoneNumberFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberFormatterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneNumberInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberFormat(pub i32);
impl PhoneNumberFormat {
    pub const E164: Self = Self(0i32);
    pub const International: Self = Self(1i32);
    pub const National: Self = Self(2i32);
    pub const Rfc3966: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PhoneNumberFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneNumberMatchResult(pub i32);
impl PhoneNumberMatchResult {
    pub const NoMatch: Self = Self(0i32);
    pub const ShortNationalSignificantNumberMatch: Self = Self(1i32);
    pub const NationalSignificantNumberMatch: Self = Self(2i32);
    pub const ExactMatch: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PhoneNumberParseResult(pub i32);
impl PhoneNumberParseResult {
    pub const Valid: Self = Self(0i32);
    pub const NotANumber: Self = Self(1i32);
    pub const InvalidCountryCode: Self = Self(2i32);
    pub const TooShort: Self = Self(3i32);
    pub const TooLong: Self = Self(4i32);
}
#[repr(transparent)]
pub struct PredictedPhoneNumberKind(pub i32);
impl PredictedPhoneNumberKind {
    pub const FixedLine: Self = Self(0i32);
    pub const Mobile: Self = Self(1i32);
    pub const FixedLineOrMobile: Self = Self(2i32);
    pub const TollFree: Self = Self(3i32);
    pub const PremiumRate: Self = Self(4i32);
    pub const SharedCost: Self = Self(5i32);
    pub const Voip: Self = Self(6i32);
    pub const PersonalNumber: Self = Self(7i32);
    pub const Pager: Self = Self(8i32);
    pub const UniversalAccountNumber: Self = Self(9i32);
    pub const Voicemail: Self = Self(10i32);
    pub const Unknown: Self = Self(11i32);
}