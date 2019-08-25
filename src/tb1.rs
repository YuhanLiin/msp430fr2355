#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb1ctl: TB1CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb1cctl0: TB1CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb1cctl1: TB1CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb1cctl2: TB1CCTL2,
    _reserved4: [u8; 8usize],
    #[doc = "0x10 - Timer_B count register"]
    pub tb1r: TB1R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb1ccr0: TB1CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb1ccr1: TB1CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb1ccr2: TB1CCR2,
    _reserved8: [u8; 8usize],
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb1ex0: TB1EX0,
    _reserved9: [u8; 12usize],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb1iv: TB1IV,
}
#[doc = "Timer_B Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ctl](tb1ctl) module"]
pub type TB1CTL = crate::Reg<u16, _TB1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CTL;
#[doc = "`read()` method returns [tb1ctl::R](tb1ctl::R) reader structure"]
impl crate::Readable for TB1CTL {}
#[doc = "`write(|w| ..)` method takes [tb1ctl::W](tb1ctl::W) writer structure"]
impl crate::Writable for TB1CTL {}
#[doc = "Timer_B Control Register"]
pub mod tb1ctl;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1cctl0](tb1cctl0) module"]
pub type TB1CCTL0 = crate::Reg<u16, _TB1CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCTL0;
#[doc = "`read()` method returns [tb1cctl0::R](tb1cctl0::R) reader structure"]
impl crate::Readable for TB1CCTL0 {}
#[doc = "`write(|w| ..)` method takes [tb1cctl0::W](tb1cctl0::W) writer structure"]
impl crate::Writable for TB1CCTL0 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl0;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1cctl1](tb1cctl1) module"]
pub type TB1CCTL1 = crate::Reg<u16, _TB1CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCTL1;
#[doc = "`read()` method returns [tb1cctl1::R](tb1cctl1::R) reader structure"]
impl crate::Readable for TB1CCTL1 {}
#[doc = "`write(|w| ..)` method takes [tb1cctl1::W](tb1cctl1::W) writer structure"]
impl crate::Writable for TB1CCTL1 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl1;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1cctl2](tb1cctl2) module"]
pub type TB1CCTL2 = crate::Reg<u16, _TB1CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCTL2;
#[doc = "`read()` method returns [tb1cctl2::R](tb1cctl2::R) reader structure"]
impl crate::Readable for TB1CCTL2 {}
#[doc = "`write(|w| ..)` method takes [tb1cctl2::W](tb1cctl2::W) writer structure"]
impl crate::Writable for TB1CCTL2 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb1cctl2;
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1r](tb1r) module"]
pub type TB1R = crate::Reg<u16, _TB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1R;
#[doc = "`read()` method returns [tb1r::R](tb1r::R) reader structure"]
impl crate::Readable for TB1R {}
#[doc = "`write(|w| ..)` method takes [tb1r::W](tb1r::W) writer structure"]
impl crate::Writable for TB1R {}
#[doc = "Timer_B count register"]
pub mod tb1r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ccr0](tb1ccr0) module"]
pub type TB1CCR0 = crate::Reg<u16, _TB1CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCR0;
#[doc = "`read()` method returns [tb1ccr0::R](tb1ccr0::R) reader structure"]
impl crate::Readable for TB1CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb1ccr0::W](tb1ccr0::W) writer structure"]
impl crate::Writable for TB1CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ccr1](tb1ccr1) module"]
pub type TB1CCR1 = crate::Reg<u16, _TB1CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCR1;
#[doc = "`read()` method returns [tb1ccr1::R](tb1ccr1::R) reader structure"]
impl crate::Readable for TB1CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb1ccr1::W](tb1ccr1::W) writer structure"]
impl crate::Writable for TB1CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr1;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ccr2](tb1ccr2) module"]
pub type TB1CCR2 = crate::Reg<u16, _TB1CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCR2;
#[doc = "`read()` method returns [tb1ccr2::R](tb1ccr2::R) reader structure"]
impl crate::Readable for TB1CCR2 {}
#[doc = "`write(|w| ..)` method takes [tb1ccr2::W](tb1ccr2::W) writer structure"]
impl crate::Writable for TB1CCR2 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr2;
#[doc = "Timer_Bx Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ex0](tb1ex0) module"]
pub type TB1EX0 = crate::Reg<u16, _TB1EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1EX0;
#[doc = "`read()` method returns [tb1ex0::R](tb1ex0::R) reader structure"]
impl crate::Readable for TB1EX0 {}
#[doc = "`write(|w| ..)` method takes [tb1ex0::W](tb1ex0::W) writer structure"]
impl crate::Writable for TB1EX0 {}
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb1ex0;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1iv](tb1iv) module"]
pub type TB1IV = crate::Reg<u16, _TB1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1IV;
#[doc = "`read()` method returns [tb1iv::R](tb1iv::R) reader structure"]
impl crate::Readable for TB1IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb1iv;
