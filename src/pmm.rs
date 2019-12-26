#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Management Module control register 0"]
    pub pmmctl0: PMMCTL0,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - Power Management Module Control Register 2"]
    pub pmmctl2: PMMCTL2,
    _reserved2: [u8; 4usize],
    #[doc = "0x0a - PMM interrupt flag register"]
    pub pmmifg: PMMIFG,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Power mode 5 control register 0"]
    pub pm5ctl0: PM5CTL0,
}
#[doc = "Power Management Module control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](pmmctl0) module"]
pub type PMMCTL0 = crate::Reg<u16, _PMMCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL0;
#[doc = "`read()` method returns [pmmctl0::R](pmmctl0::R) reader structure"]
impl crate::Readable for PMMCTL0 {}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](pmmctl0::W) writer structure"]
impl crate::Writable for PMMCTL0 {}
#[doc = "Power Management Module control register 0"]
pub mod pmmctl0;
#[doc = "Power Management Module Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl2](pmmctl2) module"]
pub type PMMCTL2 = crate::Reg<u16, _PMMCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMCTL2;
#[doc = "`read()` method returns [pmmctl2::R](pmmctl2::R) reader structure"]
impl crate::Readable for PMMCTL2 {}
#[doc = "`write(|w| ..)` method takes [pmmctl2::W](pmmctl2::W) writer structure"]
impl crate::Writable for PMMCTL2 {}
#[doc = "Power Management Module Control Register 2"]
pub mod pmmctl2;
#[doc = "PMM interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmifg](pmmifg) module"]
pub type PMMIFG = crate::Reg<u16, _PMMIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMIFG;
#[doc = "`read()` method returns [pmmifg::R](pmmifg::R) reader structure"]
impl crate::Readable for PMMIFG {}
#[doc = "`write(|w| ..)` method takes [pmmifg::W](pmmifg::W) writer structure"]
impl crate::Writable for PMMIFG {}
#[doc = "PMM interrupt flag register"]
pub mod pmmifg;
#[doc = "Power mode 5 control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pm5ctl0](pm5ctl0) module"]
pub type PM5CTL0 = crate::Reg<u16, _PM5CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PM5CTL0;
#[doc = "`read()` method returns [pm5ctl0::R](pm5ctl0::R) reader structure"]
impl crate::Readable for PM5CTL0 {}
#[doc = "`write(|w| ..)` method takes [pm5ctl0::W](pm5ctl0::W) writer structure"]
impl crate::Writable for PM5CTL0 {}
#[doc = "Power mode 5 control register 0"]
pub mod pm5ctl0;
