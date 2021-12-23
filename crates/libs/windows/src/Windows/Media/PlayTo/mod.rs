#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Media_PlayTo'*"]
#[repr(transparent)]
pub struct CurrentTimeChangeRequestedEventArgs(::windows::core::IUnknown);
impl CurrentTimeChangeRequestedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for CurrentTimeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentTimeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentTimeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for CurrentTimeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentTimeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentTimeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs;{99711324-edc7-4bf5-91f6-3c8627db59e5})");
}
unsafe impl ::windows::core::Interface for CurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99711324_edc7_4bf5_91f6_3c8627db59e5);
}
impl ::windows::core::RuntimeName for CurrentTimeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.CurrentTimeChangeRequestedEventArgs";
}
impl ::core::convert::From<CurrentTimeChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CurrentTimeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentTimeChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CurrentTimeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrentTimeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CurrentTimeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrentTimeChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CurrentTimeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentTimeChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CurrentTimeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CurrentTimeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CurrentTimeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentTimeChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentTimeChangeRequestedEventArgs {
    type Vtable = ICurrentTimeChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99711324_edc7_4bf5_91f6_3c8627db59e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentTimeChangeRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMuteChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4b4f5f6_af1f_4f1e_b437_7da32400e1d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMuteChangeRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnection(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnection {
    type Vtable = IPlayToConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x112fbfc8_f235_4fde_8d41_9bf27c9e9a40);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnectionErrorEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf5eada6_88e6_445f_9d40_d9b9f8939896);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionErrorEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionError) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnectionStateChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68c4b50f_0c20_4980_8602_58c62238d423);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionStateChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PlayToConnectionState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToConnectionTransferredEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae3193a_0683_47d9_8df0_18cbb48984d8);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToConnectionTransferredEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToManager(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToManager {
    type Vtable = IPlayToManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf56a206e_1b77_42ef_8f0d_b949f8d9b260);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToManagerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToManagerStatics {
    type Vtable = IPlayToManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e6a887_3982_4f3b_ba20_6155e435325b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlayToReceiver(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlayToReceiver {
    type Vtable = IPlayToReceiverVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac15cf47_a162_4aa6_af1b_3aa35f3b9069);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToReceiverVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: f64, mute: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currenttime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSource {
    type Vtable = IPlayToSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f138a08_fbb7_4b09_8356_aa5f4e335c31);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceDeferral(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4100891d_278e_4f29_859b_a9e501053e7d);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceRequest {
    type Vtable = IPlayToSourceRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8584665_64f4_44a0_ac0d_468d2b8fda83);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequestVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5cdc330_29df_4ec6_9da9_9fbdfcfc1b3e);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceSelectedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9d8511_5202_4dcb_8c67_abda12bb3c12);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceSelectedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
    #[cfg(feature = "deprecated")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))] usize,
);
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IPlayToSourceWithPreferredSourceUri(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IPlayToSourceWithPreferredSourceUri {
    type Vtable = IPlayToSourceWithPreferredSourceUriVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaab253eb_3301_4dc4_afba_b2f2ed9635a0);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToSourceWithPreferredSourceUriVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f5661ae_2c88_4cca_8540_d586095d13a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackRateChangeRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISourceChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3f3a96_7aa6_4a8b_86e7_54f6c6d34f64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceChangeRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVolumeChangeRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f026d5c_cf75_4c2b_913e_6d7c6c329179);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVolumeChangeRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Media_PlayTo'*"]
#[repr(transparent)]
pub struct MuteChangeRequestedEventArgs(::windows::core::IUnknown);
impl MuteChangeRequestedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Mute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MuteChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MuteChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MuteChangeRequestedEventArgs {}
impl ::core::fmt::Debug for MuteChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MuteChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MuteChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.MuteChangeRequestedEventArgs;{e4b4f5f6-af1f-4f1e-b437-7da32400e1d4})");
}
unsafe impl ::windows::core::Interface for MuteChangeRequestedEventArgs {
    type Vtable = IMuteChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4b4f5f6_af1f_4f1e_b437_7da32400e1d4);
}
impl ::windows::core::RuntimeName for MuteChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.MuteChangeRequestedEventArgs";
}
impl ::core::convert::From<MuteChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MuteChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MuteChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MuteChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MuteChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MuteChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MuteChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MuteChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MuteChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MuteChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MuteChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MuteChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnection(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnection {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn State(&self) -> ::windows::core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionState>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Transferred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionTransferredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveTransferred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Error<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToConnection, PlayToConnectionErrorEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveError<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnection {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnection").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnection;{112fbfc8-f235-4fde-8d41-9bf27c9e9a40})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnection {
    type Vtable = IPlayToConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x112fbfc8_f235_4fde_8d41_9bf27c9e9a40);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnection {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnection";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnection> for ::windows::core::IUnknown {
    fn from(value: PlayToConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnection> for ::windows::core::IUnknown {
    fn from(value: &PlayToConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnection> for ::windows::core::IInspectable {
    fn from(value: PlayToConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnection> for ::windows::core::IInspectable {
    fn from(value: &PlayToConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnection {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnection {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionError(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionError {
    pub const None: Self = Self(0i32);
    pub const DeviceNotResponding: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionError {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionError {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for PlayToConnectionError {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionError {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionError").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionError;i4)");
}
#[cfg(feature = "deprecated")]
impl ::windows::core::DefaultType for PlayToConnectionError {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionErrorEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionErrorEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Code(&self) -> ::windows::core::Result<PlayToConnectionError> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionError = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionError>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionErrorEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionErrorEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionErrorEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionErrorEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionErrorEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionErrorEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionErrorEventArgs;{bf5eada6-88e6-445f-9d40-d9b9f8939896})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnectionErrorEventArgs {
    type Vtable = IPlayToConnectionErrorEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf5eada6_88e6_445f_9d40_d9b9f8939896);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnectionErrorEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionErrorEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnectionErrorEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlayToConnectionErrorEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnectionErrorEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlayToConnectionErrorEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToConnectionErrorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToConnectionErrorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnectionErrorEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlayToConnectionErrorEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnectionErrorEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlayToConnectionErrorEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToConnectionErrorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToConnectionErrorEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionErrorEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionErrorEventArgs {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionState(pub i32);
#[cfg(feature = "deprecated")]
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for PlayToConnectionState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for PlayToConnectionState {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionState {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.PlayTo.PlayToConnectionState;i4)");
}
#[cfg(feature = "deprecated")]
impl ::windows::core::DefaultType for PlayToConnectionState {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionStateChangedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionStateChangedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn PreviousState(&self) -> ::windows::core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionState>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentState(&self) -> ::windows::core::Result<PlayToConnectionState> {
        let this = self;
        unsafe {
            let mut result__: PlayToConnectionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnectionState>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionStateChangedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs;{68c4b50f-0c20-4980-8602-58c62238d423})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnectionStateChangedEventArgs {
    type Vtable = IPlayToConnectionStateChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68c4b50f_0c20_4980_8602_58c62238d423);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionStateChangedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnectionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlayToConnectionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnectionStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlayToConnectionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnectionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlayToConnectionStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnectionStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlayToConnectionStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToConnectionStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionStateChangedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionStateChangedEventArgs {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToConnectionTransferredEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToConnectionTransferredEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn PreviousSource(&self) -> ::windows::core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSource>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentSource(&self) -> ::windows::core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSource>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToConnectionTransferredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToConnectionTransferredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToConnectionTransferredEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToConnectionTransferredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToConnectionTransferredEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToConnectionTransferredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs;{fae3193a-0683-47d9-8df0-18cbb48984d8})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToConnectionTransferredEventArgs {
    type Vtable = IPlayToConnectionTransferredEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae3193a_0683_47d9_8df0_18cbb48984d8);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToConnectionTransferredEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToConnectionTransferredEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnectionTransferredEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlayToConnectionTransferredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnectionTransferredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlayToConnectionTransferredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToConnectionTransferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToConnectionTransferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToConnectionTransferredEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlayToConnectionTransferredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToConnectionTransferredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlayToConnectionTransferredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToConnectionTransferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToConnectionTransferredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToConnectionTransferredEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToConnectionTransferredEventArgs {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToManager(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToManager {
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SourceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToManager, PlayToSourceSelectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSourceSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetDefaultSourceSelection(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn DefaultSourceSelection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn GetForCurrentView() -> ::windows::core::Result<PlayToManager> {
        Self::IPlayToManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToManager>(result__)
        })
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn ShowPlayToUI() -> ::windows::core::Result<()> {
        Self::IPlayToManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IPlayToManagerStatics<R, F: FnOnce(&IPlayToManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayToManager, IPlayToManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToManager {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToManager").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToManager;{f56a206e-1b77-42ef-8f0d-b949f8d9b260})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToManager {
    type Vtable = IPlayToManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf56a206e_1b77_42ef_8f0d_b949f8d9b260);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToManager {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToManager";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToManager> for ::windows::core::IUnknown {
    fn from(value: PlayToManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToManager> for ::windows::core::IUnknown {
    fn from(value: &PlayToManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToManager> for ::windows::core::IInspectable {
    fn from(value: PlayToManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToManager> for ::windows::core::IInspectable {
    fn from(value: &PlayToManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToManager {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToManager {}
#[doc = "*Required features: 'Media_PlayTo'*"]
#[repr(transparent)]
pub struct PlayToReceiver(::windows::core::IUnknown);
impl PlayToReceiver {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlayToReceiver, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PlayRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePauseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, SourceChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, PlaybackRateChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentTimeChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, CurrentTimeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentTimeChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MuteChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, MuteChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMuteChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn VolumeChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, VolumeChangeRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVolumeChangeRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimeUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PlayToReceiver, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyVolumeChange(&self, volume: f64, mute: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), volume, mute).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyRateChange(&self, rate: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), rate).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyLoadedMetadata(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyTimeUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, currenttime: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), currenttime.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyDurationChange<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, duration: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), duration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifySeeking(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifySeeked(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyPaused(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyPlaying(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyEnded(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyError(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn NotifyStopped(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SetFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SetSupportsImage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SupportsImage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SupportsAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn SupportsVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PlayToReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlayToReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlayToReceiver {}
impl ::core::fmt::Debug for PlayToReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlayToReceiver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToReceiver;{ac15cf47-a162-4aa6-af1b-3aa35f3b9069})");
}
unsafe impl ::windows::core::Interface for PlayToReceiver {
    type Vtable = IPlayToReceiverVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac15cf47_a162_4aa6_af1b_3aa35f3b9069);
}
impl ::windows::core::RuntimeName for PlayToReceiver {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToReceiver";
}
impl ::core::convert::From<PlayToReceiver> for ::windows::core::IUnknown {
    fn from(value: PlayToReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayToReceiver> for ::windows::core::IUnknown {
    fn from(value: &PlayToReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlayToReceiver> for ::windows::core::IInspectable {
    fn from(value: PlayToReceiver) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlayToReceiver> for ::windows::core::IInspectable {
    fn from(value: &PlayToReceiver) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToReceiver {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSource {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Connection(&self) -> ::windows::core::Result<PlayToConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToConnection>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Next(&self) -> ::windows::core::Result<PlayToSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSource>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetNext<'a, Param0: ::windows::core::IntoParam<'a, PlayToSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn PlayNext(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PreferredSourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetPreferredSourceUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPlayToSourceWithPreferredSourceUri>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSource;{7f138a08-fbb7-4b09-8356-aa5f4e335c31})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSource {
    type Vtable = IPlayToSourceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f138a08_fbb7_4b09_8356_aa5f4e335c31);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSource {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSource";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSource> for ::windows::core::IUnknown {
    fn from(value: PlayToSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSource> for ::windows::core::IUnknown {
    fn from(value: &PlayToSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSource> for ::windows::core::IInspectable {
    fn from(value: PlayToSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSource> for ::windows::core::IInspectable {
    fn from(value: &PlayToSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSource {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSource {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceDeferral(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceDeferral {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceDeferral {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceDeferral;{4100891d-278e-4f29-859b-a9e501053e7d})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceDeferral {
    type Vtable = IPlayToSourceDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4100891d_278e_4f29_859b_a9e501053e7d);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceDeferral {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceDeferral";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceDeferral> for ::windows::core::IUnknown {
    fn from(value: PlayToSourceDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceDeferral> for ::windows::core::IUnknown {
    fn from(value: &PlayToSourceDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToSourceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToSourceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceDeferral> for ::windows::core::IInspectable {
    fn from(value: PlayToSourceDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceDeferral> for ::windows::core::IInspectable {
    fn from(value: &PlayToSourceDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToSourceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToSourceDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceDeferral {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceDeferral {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceRequest(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceRequest {
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn DisplayErrorString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, errorstring: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), errorstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<PlayToSourceDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSourceDeferral>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, PlayToSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceRequest {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceRequest;{f8584665-64f4-44a0-ac0d-468d2b8fda83})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceRequest {
    type Vtable = IPlayToSourceRequestVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8584665_64f4_44a0_ac0d_468d2b8fda83);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceRequest {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequest";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceRequest> for ::windows::core::IUnknown {
    fn from(value: PlayToSourceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceRequest> for ::windows::core::IUnknown {
    fn from(value: &PlayToSourceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToSourceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToSourceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceRequest> for ::windows::core::IInspectable {
    fn from(value: PlayToSourceRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceRequest> for ::windows::core::IInspectable {
    fn from(value: &PlayToSourceRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToSourceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToSourceRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceRequest {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceRequest {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceRequestedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceRequestedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SourceRequest(&self) -> ::windows::core::Result<PlayToSourceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PlayToSourceRequest>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceRequestedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceRequestedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceRequestedEventArgs;{c5cdc330-29df-4ec6-9da9-9fbdfcfc1b3e})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceRequestedEventArgs {
    type Vtable = IPlayToSourceRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5cdc330_29df_4ec6_9da9_9fbdfcfc1b3e);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceRequestedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlayToSourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlayToSourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlayToSourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlayToSourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToSourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceRequestedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceRequestedEventArgs {}
#[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct PlayToSourceSelectedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl PlayToSourceSelectedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Storage_Streams', 'deprecated'*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsImage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'deprecated'*"]
    #[cfg(feature = "deprecated")]
    pub fn SupportsVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for PlayToSourceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for PlayToSourceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for PlayToSourceSelectedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for PlayToSourceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlayToSourceSelectedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for PlayToSourceSelectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlayToSourceSelectedEventArgs;{0c9d8511-5202-4dcb-8c67-abda12bb3c12})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for PlayToSourceSelectedEventArgs {
    type Vtable = IPlayToSourceSelectedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9d8511_5202_4dcb_8c67_abda12bb3c12);
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for PlayToSourceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlayToSourceSelectedEventArgs";
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlayToSourceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceSelectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlayToSourceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlayToSourceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlayToSourceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<PlayToSourceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlayToSourceSelectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::convert::From<&PlayToSourceSelectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlayToSourceSelectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlayToSourceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlayToSourceSelectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for PlayToSourceSelectedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for PlayToSourceSelectedEventArgs {}
#[doc = "*Required features: 'Media_PlayTo'*"]
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(::windows::core::IUnknown);
impl PlaybackRateChangeRequestedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Rate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackRateChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackRateChangeRequestedEventArgs {}
impl ::core::fmt::Debug for PlaybackRateChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackRateChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackRateChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs;{0f5661ae-2c88-4cca-8540-d586095d13a5})");
}
unsafe impl ::windows::core::Interface for PlaybackRateChangeRequestedEventArgs {
    type Vtable = IPlaybackRateChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f5661ae_2c88_4cca_8540_d586095d13a5);
}
impl ::windows::core::RuntimeName for PlaybackRateChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.PlaybackRateChangeRequestedEventArgs";
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PlaybackRateChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PlaybackRateChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PlaybackRateChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PlaybackRateChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PlaybackRateChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_PlayTo'*"]
#[repr(transparent)]
pub struct SourceChangeRequestedEventArgs(::windows::core::IUnknown);
impl SourceChangeRequestedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Album(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Genre(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Rating(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Media_PlayTo', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for SourceChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SourceChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SourceChangeRequestedEventArgs {}
impl ::core::fmt::Debug for SourceChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SourceChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SourceChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.SourceChangeRequestedEventArgs;{fb3f3a96-7aa6-4a8b-86e7-54f6c6d34f64})");
}
unsafe impl ::windows::core::Interface for SourceChangeRequestedEventArgs {
    type Vtable = ISourceChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb3f3a96_7aa6_4a8b_86e7_54f6c6d34f64);
}
impl ::windows::core::RuntimeName for SourceChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.SourceChangeRequestedEventArgs";
}
impl ::core::convert::From<SourceChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SourceChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SourceChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SourceChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SourceChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SourceChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SourceChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SourceChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SourceChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SourceChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SourceChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SourceChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Media_PlayTo'*"]
#[repr(transparent)]
pub struct VolumeChangeRequestedEventArgs(::windows::core::IUnknown);
impl VolumeChangeRequestedEventArgs {
    #[doc = "*Required features: 'Media_PlayTo'*"]
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
}
impl ::core::clone::Clone for VolumeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VolumeChangeRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VolumeChangeRequestedEventArgs {}
impl ::core::fmt::Debug for VolumeChangeRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VolumeChangeRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VolumeChangeRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.PlayTo.VolumeChangeRequestedEventArgs;{6f026d5c-cf75-4c2b-913e-6d7c6c329179})");
}
unsafe impl ::windows::core::Interface for VolumeChangeRequestedEventArgs {
    type Vtable = IVolumeChangeRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f026d5c_cf75_4c2b_913e_6d7c6c329179);
}
impl ::windows::core::RuntimeName for VolumeChangeRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.PlayTo.VolumeChangeRequestedEventArgs";
}
impl ::core::convert::From<VolumeChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VolumeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VolumeChangeRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VolumeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VolumeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VolumeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VolumeChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VolumeChangeRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VolumeChangeRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VolumeChangeRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VolumeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VolumeChangeRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
