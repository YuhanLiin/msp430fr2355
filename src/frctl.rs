#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FRAM Controller Control Register 0"]
    pub frctl0: FRCTL0,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - General Control Register 0"]
    pub gcctl0: GCCTL0,
    #[doc = "0x06 - General Control Register 1"]
    pub gcctl1: GCCTL1,
}
#[doc = "FRAM Controller Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frctl0](frctl0) module"]
pub type FRCTL0 = crate::Reg<u16, _FRCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRCTL0;
#[doc = "`read()` method returns [frctl0::R](frctl0::R) reader structure"]
impl crate::Readable for FRCTL0 {}
#[doc = "`write(|w| ..)` method takes [frctl0::W](frctl0::W) writer structure"]
impl crate::Writable for FRCTL0 {}
#[doc = "FRAM Controller Control Register 0"]
pub mod frctl0;
#[doc = "General Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcctl0](gcctl0) module"]
pub type GCCTL0 = crate::Reg<u16, _GCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCTL0;
#[doc = "`read()` method returns [gcctl0::R](gcctl0::R) reader structure"]
impl crate::Readable for GCCTL0 {}
#[doc = "`write(|w| ..)` method takes [gcctl0::W](gcctl0::W) writer structure"]
impl crate::Writable for GCCTL0 {}
#[doc = "General Control Register 0"]
pub mod gcctl0;
#[doc = "General Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcctl1](gcctl1) module"]
pub type GCCTL1 = crate::Reg<u16, _GCCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCTL1;
#[doc = "`read()` method returns [gcctl1::R](gcctl1::R) reader structure"]
impl crate::Readable for GCCTL1 {}
#[doc = "`write(|w| ..)` method takes [gcctl1::W](gcctl1::W) writer structure"]
impl crate::Writable for GCCTL1 {}
#[doc = "General Control Register 1"]
pub mod gcctl1;
