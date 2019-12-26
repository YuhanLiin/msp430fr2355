#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICCSC"]
    pub iccsc: ICCSC,
    #[doc = "0x02 - ICCMVS"]
    pub iccmvs: ICCMVS,
    #[doc = "0x04 - ICCILSR0"]
    pub iccilsr0: ICCILSR0,
    #[doc = "0x06 - ICCILSR1"]
    pub iccilsr1: ICCILSR1,
    #[doc = "0x08 - ICCILSR2"]
    pub iccilsr2: ICCILSR2,
    #[doc = "0x0a - ICCILSR3"]
    pub iccilsr3: ICCILSR3,
}
#[doc = "ICCSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccsc](iccsc) module"]
pub type ICCSC = crate::Reg<u16, _ICCSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCSC;
#[doc = "`read()` method returns [iccsc::R](iccsc::R) reader structure"]
impl crate::Readable for ICCSC {}
#[doc = "`write(|w| ..)` method takes [iccsc::W](iccsc::W) writer structure"]
impl crate::Writable for ICCSC {}
#[doc = "ICCSC"]
pub mod iccsc;
#[doc = "ICCMVS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccmvs](iccmvs) module"]
pub type ICCMVS = crate::Reg<u16, _ICCMVS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCMVS;
#[doc = "`read()` method returns [iccmvs::R](iccmvs::R) reader structure"]
impl crate::Readable for ICCMVS {}
#[doc = "ICCMVS"]
pub mod iccmvs;
#[doc = "ICCILSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr0](iccilsr0) module"]
pub type ICCILSR0 = crate::Reg<u16, _ICCILSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCILSR0;
#[doc = "`read()` method returns [iccilsr0::R](iccilsr0::R) reader structure"]
impl crate::Readable for ICCILSR0 {}
#[doc = "`write(|w| ..)` method takes [iccilsr0::W](iccilsr0::W) writer structure"]
impl crate::Writable for ICCILSR0 {}
#[doc = "ICCILSR0"]
pub mod iccilsr0;
#[doc = "ICCILSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr1](iccilsr1) module"]
pub type ICCILSR1 = crate::Reg<u16, _ICCILSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCILSR1;
#[doc = "`read()` method returns [iccilsr1::R](iccilsr1::R) reader structure"]
impl crate::Readable for ICCILSR1 {}
#[doc = "`write(|w| ..)` method takes [iccilsr1::W](iccilsr1::W) writer structure"]
impl crate::Writable for ICCILSR1 {}
#[doc = "ICCILSR1"]
pub mod iccilsr1;
#[doc = "ICCILSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr2](iccilsr2) module"]
pub type ICCILSR2 = crate::Reg<u16, _ICCILSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCILSR2;
#[doc = "`read()` method returns [iccilsr2::R](iccilsr2::R) reader structure"]
impl crate::Readable for ICCILSR2 {}
#[doc = "`write(|w| ..)` method takes [iccilsr2::W](iccilsr2::W) writer structure"]
impl crate::Writable for ICCILSR2 {}
#[doc = "ICCILSR2"]
pub mod iccilsr2;
#[doc = "ICCILSR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr3](iccilsr3) module"]
pub type ICCILSR3 = crate::Reg<u16, _ICCILSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICCILSR3;
#[doc = "`read()` method returns [iccilsr3::R](iccilsr3::R) reader structure"]
impl crate::Readable for ICCILSR3 {}
#[doc = "`write(|w| ..)` method takes [iccilsr3::W](iccilsr3::W) writer structure"]
impl crate::Writable for ICCILSR3 {}
#[doc = "ICCILSR3"]
pub mod iccilsr3;
