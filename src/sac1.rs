#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC DAC Status Register"]
    pub sac1dacsts: SAC1DACSTS,
}
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sac1dacsts](sac1dacsts) module"]
pub type SAC1DACSTS = crate::Reg<u16, _SAC1DACSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC1DACSTS;
#[doc = "`read()` method returns [sac1dacsts::R](sac1dacsts::R) reader structure"]
impl crate::Readable for SAC1DACSTS {}
#[doc = "`write(|w| ..)` method takes [sac1dacsts::W](sac1dacsts::W) writer structure"]
impl crate::Writable for SAC1DACSTS {}
#[doc = "SAC DAC Status Register"]
pub mod sac1dacsts;
