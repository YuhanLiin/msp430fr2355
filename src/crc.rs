#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Data In"]
    pub crcdi: CRCDI,
    #[doc = "0x02 - CRC Data In Reverse Byte"]
    pub crcdirb: CRCDIRB,
    #[doc = "0x04 - CRC Initialization and Result"]
    pub crcinires: CRCINIRES,
    #[doc = "0x06 - CRC Result Reverse"]
    pub crcresr: CRCRESR,
}
#[doc = "CRC Data In\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdi](crcdi) module"]
pub type CRCDI = crate::Reg<u16, _CRCDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCDI;
#[doc = "`read()` method returns [crcdi::R](crcdi::R) reader structure"]
impl crate::Readable for CRCDI {}
#[doc = "`write(|w| ..)` method takes [crcdi::W](crcdi::W) writer structure"]
impl crate::Writable for CRCDI {}
#[doc = "CRC Data In"]
pub mod crcdi;
#[doc = "CRC Data In Reverse Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdirb](crcdirb) module"]
pub type CRCDIRB = crate::Reg<u16, _CRCDIRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCDIRB;
#[doc = "`read()` method returns [crcdirb::R](crcdirb::R) reader structure"]
impl crate::Readable for CRCDIRB {}
#[doc = "`write(|w| ..)` method takes [crcdirb::W](crcdirb::W) writer structure"]
impl crate::Writable for CRCDIRB {}
#[doc = "CRC Data In Reverse Byte"]
pub mod crcdirb;
#[doc = "CRC Initialization and Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcinires](crcinires) module"]
pub type CRCINIRES = crate::Reg<u16, _CRCINIRES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCINIRES;
#[doc = "`read()` method returns [crcinires::R](crcinires::R) reader structure"]
impl crate::Readable for CRCINIRES {}
#[doc = "`write(|w| ..)` method takes [crcinires::W](crcinires::W) writer structure"]
impl crate::Writable for CRCINIRES {}
#[doc = "CRC Initialization and Result"]
pub mod crcinires;
#[doc = "CRC Result Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcresr](crcresr) module"]
pub type CRCRESR = crate::Reg<u16, _CRCRESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCRESR;
#[doc = "`read()` method returns [crcresr::R](crcresr::R) reader structure"]
impl crate::Readable for CRCRESR {}
#[doc = "`write(|w| ..)` method takes [crcresr::W](crcresr::W) writer structure"]
impl crate::Writable for CRCRESR {}
#[doc = "CRC Result Reverse"]
pub mod crcresr;
