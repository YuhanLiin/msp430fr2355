#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac0oa: SAC0OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac0pga: SAC0PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac0dac: SAC0DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac0dat: SAC0DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac0dacsts: SAC0DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac0iv: SAC0IV,
}
#[doc = "SAC OA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac0oa](sac0oa) module"]
pub type SAC0OA = crate::Reg<u16, _SAC0OA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0OA;
#[doc = "`read()` method returns [sac0oa::R](sac0oa::R) reader structure"]
impl crate::Readable for SAC0OA {}
#[doc = "`write(|w| ..)` method takes [sac0oa::W](sac0oa::W) writer structure"]
impl crate::Writable for SAC0OA {}
#[doc = "SAC OA Control Register"]
pub mod sac0oa;
#[doc = "SAC PGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac0pga](sac0pga) module"]
pub type SAC0PGA = crate::Reg<u16, _SAC0PGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0PGA;
#[doc = "`read()` method returns [sac0pga::R](sac0pga::R) reader structure"]
impl crate::Readable for SAC0PGA {}
#[doc = "`write(|w| ..)` method takes [sac0pga::W](sac0pga::W) writer structure"]
impl crate::Writable for SAC0PGA {}
#[doc = "SAC PGA Control Register"]
pub mod sac0pga;
#[doc = "SAC DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac0dac](sac0dac) module"]
pub type SAC0DAC = crate::Reg<u16, _SAC0DAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0DAC;
#[doc = "`read()` method returns [sac0dac::R](sac0dac::R) reader structure"]
impl crate::Readable for SAC0DAC {}
#[doc = "`write(|w| ..)` method takes [sac0dac::W](sac0dac::W) writer structure"]
impl crate::Writable for SAC0DAC {}
#[doc = "SAC DAC Control Register"]
pub mod sac0dac;
#[doc = "SAC DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac0dat](sac0dat) module"]
pub type SAC0DAT = crate::Reg<u16, _SAC0DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0DAT;
#[doc = "`read()` method returns [sac0dat::R](sac0dat::R) reader structure"]
impl crate::Readable for SAC0DAT {}
#[doc = "`write(|w| ..)` method takes [sac0dat::W](sac0dat::W) writer structure"]
impl crate::Writable for SAC0DAT {}
#[doc = "SAC DAC Data Register"]
pub mod sac0dat;
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac0dacsts](sac0dacsts) module"]
pub type SAC0DACSTS = crate::Reg<u16, _SAC0DACSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0DACSTS;
#[doc = "`read()` method returns [sac0dacsts::R](sac0dacsts::R) reader structure"]
impl crate::Readable for SAC0DACSTS {}
#[doc = "`write(|w| ..)` method takes [sac0dacsts::W](sac0dacsts::W) writer structure"]
impl crate::Writable for SAC0DACSTS {}
#[doc = "SAC DAC Status Register"]
pub mod sac0dacsts;
#[doc = "SAC Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac0iv](sac0iv) module"]
pub type SAC0IV = crate::Reg<u16, _SAC0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0IV;
#[doc = "`read()` method returns [sac0iv::R](sac0iv::R) reader structure"]
impl crate::Readable for SAC0IV {}
#[doc = "SAC Interrupt Vector Register"]
pub mod sac0iv;
