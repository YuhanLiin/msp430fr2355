#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control 0"]
    pub adcctl0: ADCCTL0,
    #[doc = "0x02 - ADC Control 1"]
    pub adcctl1: ADCCTL1,
    #[doc = "0x04 - ADC Control 2"]
    pub adcctl2: ADCCTL2,
    #[doc = "0x06 - ADC Window Comparator Low Threshold Register"]
    pub adclo: ADCLO,
    #[doc = "0x08 - ADC Window Comparator High Threshold Register"]
    pub adchi: ADCHI,
    #[doc = "0x0a - ADC Conversion Memory Control Register"]
    pub adcmctl0: ADCMCTL0,
    _reserved6: [u8; 6usize],
    #[doc = "0x12 - ADC Conversion Memory Register"]
    pub adcmem0: ADCMEM0,
    _reserved7: [u8; 6usize],
    #[doc = "0x1a - ADC Interrupt Enable 0"]
    pub adcie: ADCIE,
    #[doc = "0x1c - ADC Interrupt Flag"]
    pub adcifg: ADCIFG,
    #[doc = "0x1e - ADC Interrupt Vector"]
    pub adciv: ADCIV,
}
#[doc = "ADC Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl0](adcctl0) module"]
pub type ADCCTL0 = crate::Reg<u16, _ADCCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTL0;
#[doc = "`read()` method returns [adcctl0::R](adcctl0::R) reader structure"]
impl crate::Readable for ADCCTL0 {}
#[doc = "`write(|w| ..)` method takes [adcctl0::W](adcctl0::W) writer structure"]
impl crate::Writable for ADCCTL0 {}
#[doc = "ADC Control 0"]
pub mod adcctl0;
#[doc = "ADC Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl1](adcctl1) module"]
pub type ADCCTL1 = crate::Reg<u16, _ADCCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTL1;
#[doc = "`read()` method returns [adcctl1::R](adcctl1::R) reader structure"]
impl crate::Readable for ADCCTL1 {}
#[doc = "`write(|w| ..)` method takes [adcctl1::W](adcctl1::W) writer structure"]
impl crate::Writable for ADCCTL1 {}
#[doc = "ADC Control 1"]
pub mod adcctl1;
#[doc = "ADC Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl2](adcctl2) module"]
pub type ADCCTL2 = crate::Reg<u16, _ADCCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTL2;
#[doc = "`read()` method returns [adcctl2::R](adcctl2::R) reader structure"]
impl crate::Readable for ADCCTL2 {}
#[doc = "`write(|w| ..)` method takes [adcctl2::W](adcctl2::W) writer structure"]
impl crate::Writable for ADCCTL2 {}
#[doc = "ADC Control 2"]
pub mod adcctl2;
#[doc = "ADC Window Comparator Low Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adclo](adclo) module"]
pub type ADCLO = crate::Reg<u16, _ADCLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCLO;
#[doc = "`read()` method returns [adclo::R](adclo::R) reader structure"]
impl crate::Readable for ADCLO {}
#[doc = "`write(|w| ..)` method takes [adclo::W](adclo::W) writer structure"]
impl crate::Writable for ADCLO {}
#[doc = "ADC Window Comparator Low Threshold Register"]
pub mod adclo;
#[doc = "ADC Window Comparator High Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adchi](adchi) module"]
pub type ADCHI = crate::Reg<u16, _ADCHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCHI;
#[doc = "`read()` method returns [adchi::R](adchi::R) reader structure"]
impl crate::Readable for ADCHI {}
#[doc = "`write(|w| ..)` method takes [adchi::W](adchi::W) writer structure"]
impl crate::Writable for ADCHI {}
#[doc = "ADC Window Comparator High Threshold Register"]
pub mod adchi;
#[doc = "ADC Conversion Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmctl0](adcmctl0) module"]
pub type ADCMCTL0 = crate::Reg<u16, _ADCMCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCMCTL0;
#[doc = "`read()` method returns [adcmctl0::R](adcmctl0::R) reader structure"]
impl crate::Readable for ADCMCTL0 {}
#[doc = "`write(|w| ..)` method takes [adcmctl0::W](adcmctl0::W) writer structure"]
impl crate::Writable for ADCMCTL0 {}
#[doc = "ADC Conversion Memory Control Register"]
pub mod adcmctl0;
#[doc = "ADC Conversion Memory Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcmem0](adcmem0) module"]
pub type ADCMEM0 = crate::Reg<u16, _ADCMEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCMEM0;
#[doc = "`read()` method returns [adcmem0::R](adcmem0::R) reader structure"]
impl crate::Readable for ADCMEM0 {}
#[doc = "`write(|w| ..)` method takes [adcmem0::W](adcmem0::W) writer structure"]
impl crate::Writable for ADCMEM0 {}
#[doc = "ADC Conversion Memory Register"]
pub mod adcmem0;
#[doc = "ADC Interrupt Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcie](adcie) module"]
pub type ADCIE = crate::Reg<u16, _ADCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCIE;
#[doc = "`read()` method returns [adcie::R](adcie::R) reader structure"]
impl crate::Readable for ADCIE {}
#[doc = "`write(|w| ..)` method takes [adcie::W](adcie::W) writer structure"]
impl crate::Writable for ADCIE {}
#[doc = "ADC Interrupt Enable 0"]
pub mod adcie;
#[doc = "ADC Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcifg](adcifg) module"]
pub type ADCIFG = crate::Reg<u16, _ADCIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCIFG;
#[doc = "`read()` method returns [adcifg::R](adcifg::R) reader structure"]
impl crate::Readable for ADCIFG {}
#[doc = "`write(|w| ..)` method takes [adcifg::W](adcifg::W) writer structure"]
impl crate::Writable for ADCIFG {}
#[doc = "ADC Interrupt Flag"]
pub mod adcifg;
#[doc = "ADC Interrupt Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adciv](adciv) module"]
pub type ADCIV = crate::Reg<u16, _ADCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCIV;
#[doc = "`read()` method returns [adciv::R](adciv::R) reader structure"]
impl crate::Readable for ADCIV {}
#[doc = "`write(|w| ..)` method takes [adciv::W](adciv::W) writer structure"]
impl crate::Writable for ADCIV {}
#[doc = "ADC Interrupt Vector"]
pub mod adciv;
