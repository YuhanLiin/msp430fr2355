#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac3oa: SAC3OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac3pga: SAC3PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac3dac: SAC3DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac3dat: SAC3DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac3dacsts: SAC3DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac3iv: SAC3IV,
}
#[doc = "SAC OA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3oa](sac3oa) module"]
pub type SAC3OA = crate::Reg<u16, _SAC3OA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3OA;
#[doc = "`read()` method returns [sac3oa::R](sac3oa::R) reader structure"]
impl crate::Readable for SAC3OA {}
#[doc = "`write(|w| ..)` method takes [sac3oa::W](sac3oa::W) writer structure"]
impl crate::Writable for SAC3OA {}
#[doc = "SAC OA Control Register"]
pub mod sac3oa;
#[doc = "SAC PGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3pga](sac3pga) module"]
pub type SAC3PGA = crate::Reg<u16, _SAC3PGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3PGA;
#[doc = "`read()` method returns [sac3pga::R](sac3pga::R) reader structure"]
impl crate::Readable for SAC3PGA {}
#[doc = "`write(|w| ..)` method takes [sac3pga::W](sac3pga::W) writer structure"]
impl crate::Writable for SAC3PGA {}
#[doc = "SAC PGA Control Register"]
pub mod sac3pga;
#[doc = "SAC DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3dac](sac3dac) module"]
pub type SAC3DAC = crate::Reg<u16, _SAC3DAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3DAC;
#[doc = "`read()` method returns [sac3dac::R](sac3dac::R) reader structure"]
impl crate::Readable for SAC3DAC {}
#[doc = "`write(|w| ..)` method takes [sac3dac::W](sac3dac::W) writer structure"]
impl crate::Writable for SAC3DAC {}
#[doc = "SAC DAC Control Register"]
pub mod sac3dac;
#[doc = "SAC DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3dat](sac3dat) module"]
pub type SAC3DAT = crate::Reg<u16, _SAC3DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3DAT;
#[doc = "`read()` method returns [sac3dat::R](sac3dat::R) reader structure"]
impl crate::Readable for SAC3DAT {}
#[doc = "`write(|w| ..)` method takes [sac3dat::W](sac3dat::W) writer structure"]
impl crate::Writable for SAC3DAT {}
#[doc = "SAC DAC Data Register"]
pub mod sac3dat;
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3dacsts](sac3dacsts) module"]
pub type SAC3DACSTS = crate::Reg<u16, _SAC3DACSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3DACSTS;
#[doc = "`read()` method returns [sac3dacsts::R](sac3dacsts::R) reader structure"]
impl crate::Readable for SAC3DACSTS {}
#[doc = "`write(|w| ..)` method takes [sac3dacsts::W](sac3dacsts::W) writer structure"]
impl crate::Writable for SAC3DACSTS {}
#[doc = "SAC DAC Status Register"]
pub mod sac3dacsts;
#[doc = "SAC Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac3iv](sac3iv) module"]
pub type SAC3IV = crate::Reg<u16, _SAC3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3IV;
#[doc = "`read()` method returns [sac3iv::R](sac3iv::R) reader structure"]
impl crate::Readable for SAC3IV {}
#[doc = "SAC Interrupt Vector Register"]
pub mod sac3iv;
