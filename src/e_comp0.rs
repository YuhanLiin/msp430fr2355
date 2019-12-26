#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cpctl0: CPCTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cpctl1: CPCTL1,
    _reserved2: [u8; 2usize],
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    pub cpint: CPINT,
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    pub cpiv: CPIV,
    _reserved4: [u8; 6usize],
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    pub cpdacctl: CPDACCTL,
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    pub cpdacdata: CPDACDATA,
}
#[doc = "Comparator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpctl0](cpctl0) module"]
pub type CPCTL0 = crate::Reg<u16, _CPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPCTL0;
#[doc = "`read()` method returns [cpctl0::R](cpctl0::R) reader structure"]
impl crate::Readable for CPCTL0 {}
#[doc = "`write(|w| ..)` method takes [cpctl0::W](cpctl0::W) writer structure"]
impl crate::Writable for CPCTL0 {}
#[doc = "Comparator Control Register 0"]
pub mod cpctl0;
#[doc = "Comparator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpctl1](cpctl1) module"]
pub type CPCTL1 = crate::Reg<u16, _CPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPCTL1;
#[doc = "`read()` method returns [cpctl1::R](cpctl1::R) reader structure"]
impl crate::Readable for CPCTL1 {}
#[doc = "`write(|w| ..)` method takes [cpctl1::W](cpctl1::W) writer structure"]
impl crate::Writable for CPCTL1 {}
#[doc = "Comparator Control Register 1"]
pub mod cpctl1;
#[doc = "Comparator Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpint](cpint) module"]
pub type CPINT = crate::Reg<u16, _CPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPINT;
#[doc = "`read()` method returns [cpint::R](cpint::R) reader structure"]
impl crate::Readable for CPINT {}
#[doc = "`write(|w| ..)` method takes [cpint::W](cpint::W) writer structure"]
impl crate::Writable for CPINT {}
#[doc = "Comparator Interrupt Control Register"]
pub mod cpint;
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpiv](cpiv) module"]
pub type CPIV = crate::Reg<u16, _CPIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPIV;
#[doc = "`read()` method returns [cpiv::R](cpiv::R) reader structure"]
impl crate::Readable for CPIV {}
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cpiv;
#[doc = "6-bit Comparator built-in DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpdacctl](cpdacctl) module"]
pub type CPDACCTL = crate::Reg<u16, _CPDACCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPDACCTL;
#[doc = "`read()` method returns [cpdacctl::R](cpdacctl::R) reader structure"]
impl crate::Readable for CPDACCTL {}
#[doc = "`write(|w| ..)` method takes [cpdacctl::W](cpdacctl::W) writer structure"]
impl crate::Writable for CPDACCTL {}
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cpdacctl;
#[doc = "6-bit Comparator built-in DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpdacdata](cpdacdata) module"]
pub type CPDACDATA = crate::Reg<u16, _CPDACDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPDACDATA;
#[doc = "`read()` method returns [cpdacdata::R](cpdacdata::R) reader structure"]
impl crate::Readable for CPDACDATA {}
#[doc = "`write(|w| ..)` method takes [cpdacdata::W](cpdacdata::W) writer structure"]
impl crate::Writable for CPDACDATA {}
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cpdacdata;
