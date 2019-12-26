#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cp1ctl0: CP1CTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cp1ctl1: CP1CTL1,
    _reserved2: [u8; 2usize],
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    pub cp1int: CP1INT,
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    pub cp1iv: CP1IV,
    _reserved4: [u8; 6usize],
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    pub cp1dacctl: CP1DACCTL,
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    pub cp1dacdata: CP1DACDATA,
}
#[doc = "Comparator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1ctl0](cp1ctl0) module"]
pub type CP1CTL0 = crate::Reg<u16, _CP1CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP1CTL0;
#[doc = "`read()` method returns [cp1ctl0::R](cp1ctl0::R) reader structure"]
impl crate::Readable for CP1CTL0 {}
#[doc = "`write(|w| ..)` method takes [cp1ctl0::W](cp1ctl0::W) writer structure"]
impl crate::Writable for CP1CTL0 {}
#[doc = "Comparator Control Register 0"]
pub mod cp1ctl0;
#[doc = "Comparator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1ctl1](cp1ctl1) module"]
pub type CP1CTL1 = crate::Reg<u16, _CP1CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP1CTL1;
#[doc = "`read()` method returns [cp1ctl1::R](cp1ctl1::R) reader structure"]
impl crate::Readable for CP1CTL1 {}
#[doc = "`write(|w| ..)` method takes [cp1ctl1::W](cp1ctl1::W) writer structure"]
impl crate::Writable for CP1CTL1 {}
#[doc = "Comparator Control Register 1"]
pub mod cp1ctl1;
#[doc = "Comparator Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1int](cp1int) module"]
pub type CP1INT = crate::Reg<u16, _CP1INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP1INT;
#[doc = "`read()` method returns [cp1int::R](cp1int::R) reader structure"]
impl crate::Readable for CP1INT {}
#[doc = "`write(|w| ..)` method takes [cp1int::W](cp1int::W) writer structure"]
impl crate::Writable for CP1INT {}
#[doc = "Comparator Interrupt Control Register"]
pub mod cp1int;
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1iv](cp1iv) module"]
pub type CP1IV = crate::Reg<u16, _CP1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP1IV;
#[doc = "`read()` method returns [cp1iv::R](cp1iv::R) reader structure"]
impl crate::Readable for CP1IV {}
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cp1iv;
#[doc = "6-bit Comparator built-in DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1dacctl](cp1dacctl) module"]
pub type CP1DACCTL = crate::Reg<u16, _CP1DACCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP1DACCTL;
#[doc = "`read()` method returns [cp1dacctl::R](cp1dacctl::R) reader structure"]
impl crate::Readable for CP1DACCTL {}
#[doc = "`write(|w| ..)` method takes [cp1dacctl::W](cp1dacctl::W) writer structure"]
impl crate::Writable for CP1DACCTL {}
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cp1dacctl;
#[doc = "6-bit Comparator built-in DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1dacdata](cp1dacdata) module"]
pub type CP1DACDATA = crate::Reg<u16, _CP1DACDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CP1DACDATA;
#[doc = "`read()` method returns [cp1dacdata::R](cp1dacdata::R) reader structure"]
impl crate::Readable for CP1DACDATA {}
#[doc = "`write(|w| ..)` method takes [cp1dacdata::W](cp1dacdata::W) writer structure"]
impl crate::Writable for CP1DACDATA {}
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cp1dacdata;
