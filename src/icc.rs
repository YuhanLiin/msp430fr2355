#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICCILSR2"]
    pub iccilsr2: ICCILSR2,
}
#[doc = "ICCILSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iccilsr2](iccilsr2) module"]
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
