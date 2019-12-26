#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb0ctl: TB0CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl0: TB0CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl1: TB0CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb0cctl2: TB0CCTL2,
    _reserved4: [u8; 8usize],
    #[doc = "0x10 - Timer_B count register"]
    pub tb0r: TB0R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb0ccr0: TB0CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb0ccr1: TB0CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb0ccr2: TB0CCR2,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb0ex0: TB0EX0,
    _reserved9: [u8; 12usize],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb0iv: TB0IV,
}
#[doc = "Timer_B Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ctl](tb0ctl) module"]
pub type TB0CTL = crate::Reg<u16, _TB0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CTL;
#[doc = "`read()` method returns [tb0ctl::R](tb0ctl::R) reader structure"]
impl crate::Readable for TB0CTL {}
#[doc = "`write(|w| ..)` method takes [tb0ctl::W](tb0ctl::W) writer structure"]
impl crate::Writable for TB0CTL {}
#[doc = "Timer_B Control Register"]
pub mod tb0ctl;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl0](tb0cctl0) module"]
pub type TB0CCTL0 = crate::Reg<u16, _TB0CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL0;
#[doc = "`read()` method returns [tb0cctl0::R](tb0cctl0::R) reader structure"]
impl crate::Readable for TB0CCTL0 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl0::W](tb0cctl0::W) writer structure"]
impl crate::Writable for TB0CCTL0 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl0;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl1](tb0cctl1) module"]
pub type TB0CCTL1 = crate::Reg<u16, _TB0CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL1;
#[doc = "`read()` method returns [tb0cctl1::R](tb0cctl1::R) reader structure"]
impl crate::Readable for TB0CCTL1 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl1::W](tb0cctl1::W) writer structure"]
impl crate::Writable for TB0CCTL1 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl1;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0cctl2](tb0cctl2) module"]
pub type TB0CCTL2 = crate::Reg<u16, _TB0CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCTL2;
#[doc = "`read()` method returns [tb0cctl2::R](tb0cctl2::R) reader structure"]
impl crate::Readable for TB0CCTL2 {}
#[doc = "`write(|w| ..)` method takes [tb0cctl2::W](tb0cctl2::W) writer structure"]
impl crate::Writable for TB0CCTL2 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb0cctl2;
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0r](tb0r) module"]
pub type TB0R = crate::Reg<u16, _TB0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0R;
#[doc = "`read()` method returns [tb0r::R](tb0r::R) reader structure"]
impl crate::Readable for TB0R {}
#[doc = "`write(|w| ..)` method takes [tb0r::W](tb0r::W) writer structure"]
impl crate::Writable for TB0R {}
#[doc = "Timer_B count register"]
pub mod tb0r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr0](tb0ccr0) module"]
pub type TB0CCR0 = crate::Reg<u16, _TB0CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR0;
#[doc = "`read()` method returns [tb0ccr0::R](tb0ccr0::R) reader structure"]
impl crate::Readable for TB0CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr0::W](tb0ccr0::W) writer structure"]
impl crate::Writable for TB0CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr1](tb0ccr1) module"]
pub type TB0CCR1 = crate::Reg<u16, _TB0CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR1;
#[doc = "`read()` method returns [tb0ccr1::R](tb0ccr1::R) reader structure"]
impl crate::Readable for TB0CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr1::W](tb0ccr1::W) writer structure"]
impl crate::Writable for TB0CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr1;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ccr2](tb0ccr2) module"]
pub type TB0CCR2 = crate::Reg<u16, _TB0CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR2;
#[doc = "`read()` method returns [tb0ccr2::R](tb0ccr2::R) reader structure"]
impl crate::Readable for TB0CCR2 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr2::W](tb0ccr2::W) writer structure"]
impl crate::Writable for TB0CCR2 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr2;
#[doc = "Timer_Bx Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0ex0](tb0ex0) module"]
pub type TB0EX0 = crate::Reg<u16, _TB0EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0EX0;
#[doc = "`read()` method returns [tb0ex0::R](tb0ex0::R) reader structure"]
impl crate::Readable for TB0EX0 {}
#[doc = "`write(|w| ..)` method takes [tb0ex0::W](tb0ex0::W) writer structure"]
impl crate::Writable for TB0EX0 {}
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb0ex0;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb0iv](tb0iv) module"]
pub type TB0IV = crate::Reg<u16, _TB0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0IV;
#[doc = "`read()` method returns [tb0iv::R](tb0iv::R) reader structure"]
impl crate::Readable for TB0IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb0iv;
