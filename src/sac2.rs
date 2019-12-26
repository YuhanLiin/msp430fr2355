#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC OA Control Register"]
    pub sac2oa: SAC2OA,
    #[doc = "0x02 - SAC PGA Control Register"]
    pub sac2pga: SAC2PGA,
    #[doc = "0x04 - SAC DAC Control Register"]
    pub sac2dac: SAC2DAC,
    #[doc = "0x06 - SAC DAC Data Register"]
    pub sac2dat: SAC2DAT,
    #[doc = "0x08 - SAC DAC Status Register"]
    pub sac2dacsts: SAC2DACSTS,
    #[doc = "0x0a - SAC Interrupt Vector Register"]
    pub sac2iv: SAC2IV,
}
#[doc = "SAC OA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2oa](sac2oa) module"]
pub type SAC2OA = crate::Reg<u16, _SAC2OA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC2OA;
#[doc = "`read()` method returns [sac2oa::R](sac2oa::R) reader structure"]
impl crate::Readable for SAC2OA {}
#[doc = "`write(|w| ..)` method takes [sac2oa::W](sac2oa::W) writer structure"]
impl crate::Writable for SAC2OA {}
#[doc = "SAC OA Control Register"]
pub mod sac2oa;
#[doc = "SAC PGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2pga](sac2pga) module"]
pub type SAC2PGA = crate::Reg<u16, _SAC2PGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC2PGA;
#[doc = "`read()` method returns [sac2pga::R](sac2pga::R) reader structure"]
impl crate::Readable for SAC2PGA {}
#[doc = "`write(|w| ..)` method takes [sac2pga::W](sac2pga::W) writer structure"]
impl crate::Writable for SAC2PGA {}
#[doc = "SAC PGA Control Register"]
pub mod sac2pga;
#[doc = "SAC DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2dac](sac2dac) module"]
pub type SAC2DAC = crate::Reg<u16, _SAC2DAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC2DAC;
#[doc = "`read()` method returns [sac2dac::R](sac2dac::R) reader structure"]
impl crate::Readable for SAC2DAC {}
#[doc = "`write(|w| ..)` method takes [sac2dac::W](sac2dac::W) writer structure"]
impl crate::Writable for SAC2DAC {}
#[doc = "SAC DAC Control Register"]
pub mod sac2dac;
#[doc = "SAC DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2dat](sac2dat) module"]
pub type SAC2DAT = crate::Reg<u16, _SAC2DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC2DAT;
#[doc = "`read()` method returns [sac2dat::R](sac2dat::R) reader structure"]
impl crate::Readable for SAC2DAT {}
#[doc = "`write(|w| ..)` method takes [sac2dat::W](sac2dat::W) writer structure"]
impl crate::Writable for SAC2DAT {}
#[doc = "SAC DAC Data Register"]
pub mod sac2dat;
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2dacsts](sac2dacsts) module"]
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
#[doc = "SAC Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac2iv](sac2iv) module"]
pub type SAC2IV = crate::Reg<u16, _SAC2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC2IV;
#[doc = "`read()` method returns [sac2iv::R](sac2iv::R) reader structure"]
impl crate::Readable for SAC2IV {}
#[doc = "SAC Interrupt Vector Register"]
pub mod sac2iv;
