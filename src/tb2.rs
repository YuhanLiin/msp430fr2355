#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb2ctl: TB2CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb2cctl0: TB2CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb2cctl1: TB2CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb2cctl2: TB2CCTL2,
    _reserved4: [u8; 8usize],
    #[doc = "0x10 - Timer_B count register"]
    pub tb2r: TB2R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb2ccr0: TB2CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb2ccr1: TB2CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb2ccr2: TB2CCR2,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb2ex0: TB2EX0,
    _reserved9: [u8; 12usize],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb2iv: TB2IV,
}
#[doc = "Timer_B Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2ctl](tb2ctl) module"]
pub type TB2CTL = crate::Reg<u16, _TB2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CTL;
#[doc = "`read()` method returns [tb2ctl::R](tb2ctl::R) reader structure"]
impl crate::Readable for TB2CTL {}
#[doc = "`write(|w| ..)` method takes [tb2ctl::W](tb2ctl::W) writer structure"]
impl crate::Writable for TB2CTL {}
#[doc = "Timer_B Control Register"]
pub mod tb2ctl;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2cctl0](tb2cctl0) module"]
pub type TB2CCTL0 = crate::Reg<u16, _TB2CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCTL0;
#[doc = "`read()` method returns [tb2cctl0::R](tb2cctl0::R) reader structure"]
impl crate::Readable for TB2CCTL0 {}
#[doc = "`write(|w| ..)` method takes [tb2cctl0::W](tb2cctl0::W) writer structure"]
impl crate::Writable for TB2CCTL0 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl0;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2cctl1](tb2cctl1) module"]
pub type TB2CCTL1 = crate::Reg<u16, _TB2CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCTL1;
#[doc = "`read()` method returns [tb2cctl1::R](tb2cctl1::R) reader structure"]
impl crate::Readable for TB2CCTL1 {}
#[doc = "`write(|w| ..)` method takes [tb2cctl1::W](tb2cctl1::W) writer structure"]
impl crate::Writable for TB2CCTL1 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl1;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2cctl2](tb2cctl2) module"]
pub type TB2CCTL2 = crate::Reg<u16, _TB2CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCTL2;
#[doc = "`read()` method returns [tb2cctl2::R](tb2cctl2::R) reader structure"]
impl crate::Readable for TB2CCTL2 {}
#[doc = "`write(|w| ..)` method takes [tb2cctl2::W](tb2cctl2::W) writer structure"]
impl crate::Writable for TB2CCTL2 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb2cctl2;
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2r](tb2r) module"]
pub type TB2R = crate::Reg<u16, _TB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2R;
#[doc = "`read()` method returns [tb2r::R](tb2r::R) reader structure"]
impl crate::Readable for TB2R {}
#[doc = "`write(|w| ..)` method takes [tb2r::W](tb2r::W) writer structure"]
impl crate::Writable for TB2R {}
#[doc = "Timer_B count register"]
pub mod tb2r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2ccr0](tb2ccr0) module"]
pub type TB2CCR0 = crate::Reg<u16, _TB2CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCR0;
#[doc = "`read()` method returns [tb2ccr0::R](tb2ccr0::R) reader structure"]
impl crate::Readable for TB2CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb2ccr0::W](tb2ccr0::W) writer structure"]
impl crate::Writable for TB2CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2ccr1](tb2ccr1) module"]
pub type TB2CCR1 = crate::Reg<u16, _TB2CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCR1;
#[doc = "`read()` method returns [tb2ccr1::R](tb2ccr1::R) reader structure"]
impl crate::Readable for TB2CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb2ccr1::W](tb2ccr1::W) writer structure"]
impl crate::Writable for TB2CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr1;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2ccr2](tb2ccr2) module"]
pub type TB2CCR2 = crate::Reg<u16, _TB2CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCR2;
#[doc = "`read()` method returns [tb2ccr2::R](tb2ccr2::R) reader structure"]
impl crate::Readable for TB2CCR2 {}
#[doc = "`write(|w| ..)` method takes [tb2ccr2::W](tb2ccr2::W) writer structure"]
impl crate::Writable for TB2CCR2 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr2;
#[doc = "Timer_Bx Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2ex0](tb2ex0) module"]
pub type TB2EX0 = crate::Reg<u16, _TB2EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2EX0;
#[doc = "`read()` method returns [tb2ex0::R](tb2ex0::R) reader structure"]
impl crate::Readable for TB2EX0 {}
#[doc = "`write(|w| ..)` method takes [tb2ex0::W](tb2ex0::W) writer structure"]
impl crate::Writable for TB2EX0 {}
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb2ex0;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb2iv](tb2iv) module"]
pub type TB2IV = crate::Reg<u16, _TB2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2IV;
#[doc = "`read()` method returns [tb2iv::R](tb2iv::R) reader structure"]
impl crate::Readable for TB2IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb2iv;
