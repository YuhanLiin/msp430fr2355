#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac1oa: SAC1OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac1pga: SAC1PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac1dac: SAC1DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac1dat: SAC1DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac1dacsts: SAC1DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac1iv: SAC1IV,
}
#[doc = "SAC OA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1oa](sac1oa) module"]
pub type SAC1OA = crate::Reg<u16, _SAC1OA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC1OA;
#[doc = "`read()` method returns [sac1oa::R](sac1oa::R) reader structure"]
impl crate::Readable for SAC1OA {}
#[doc = "`write(|w| ..)` method takes [sac1oa::W](sac1oa::W) writer structure"]
impl crate::Writable for SAC1OA {}
#[doc = "SAC OA Control Register"]
pub mod sac1oa;
#[doc = "SAC PGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1pga](sac1pga) module"]
pub type SAC1PGA = crate::Reg<u16, _SAC1PGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC1PGA;
#[doc = "`read()` method returns [sac1pga::R](sac1pga::R) reader structure"]
impl crate::Readable for SAC1PGA {}
#[doc = "`write(|w| ..)` method takes [sac1pga::W](sac1pga::W) writer structure"]
impl crate::Writable for SAC1PGA {}
#[doc = "SAC PGA Control Register"]
pub mod sac1pga;
#[doc = "SAC DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1dac](sac1dac) module"]
pub type SAC1DAC = crate::Reg<u16, _SAC1DAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC1DAC;
#[doc = "`read()` method returns [sac1dac::R](sac1dac::R) reader structure"]
impl crate::Readable for SAC1DAC {}
#[doc = "`write(|w| ..)` method takes [sac1dac::W](sac1dac::W) writer structure"]
impl crate::Writable for SAC1DAC {}
#[doc = "SAC DAC Control Register"]
pub mod sac1dac;
#[doc = "SAC DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1dat](sac1dat) module"]
pub type SAC1DAT = crate::Reg<u16, _SAC1DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC1DAT;
#[doc = "`read()` method returns [sac1dat::R](sac1dat::R) reader structure"]
impl crate::Readable for SAC1DAT {}
#[doc = "`write(|w| ..)` method takes [sac1dat::W](sac1dat::W) writer structure"]
impl crate::Writable for SAC1DAT {}
#[doc = "SAC DAC Data Register"]
pub mod sac1dat;
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1dacsts](sac1dacsts) module"]
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
#[doc = "SAC Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac1iv](sac1iv) module"]
pub type SAC1IV = crate::Reg<u16, _SAC1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC1IV;
#[doc = "`read()` method returns [sac1iv::R](sac1iv::R) reader structure"]
impl crate::Readable for SAC1IV {}
#[doc = "SAC Interrupt Vector Register"]
pub mod sac1iv;
