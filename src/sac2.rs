#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC DAC Status Register"]
    pub sac2dacsts: SAC2DACSTS,
}
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sac2dacsts](sac2dacsts) module"]
pub type SAC2DACSTS = crate::Reg<u16, _SAC2DACSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC2DACSTS;
#[doc = "`read()` method returns [sac2dacsts::R](sac2dacsts::R) reader structure"]
impl crate::Readable for SAC2DACSTS {}
#[doc = "`write(|w| ..)` method takes [sac2dacsts::W](sac2dacsts::W) writer structure"]
impl crate::Writable for SAC2DACSTS {}
#[doc = "SAC DAC Status Register"]
pub mod sac2dacsts;
