#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Control Register"]
    pub tb3ctl: TB3CTL,
    #[doc = "0x02 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl0: TB3CCTL0,
    #[doc = "0x04 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl1: TB3CCTL1,
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl2: TB3CCTL2,
    #[doc = "0x08 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl3: TB3CCTL3,
    #[doc = "0x0a - Timer_B Capture/Compare Control Register"]
    pub tb3cctl4: TB3CCTL4,
    #[doc = "0x0c - Timer_B Capture/Compare Control Register"]
    pub tb3cctl5: TB3CCTL5,
    #[doc = "0x0e - Timer_B Capture/Compare Control Register"]
    pub tb3cctl6: TB3CCTL6,
    #[doc = "0x10 - Timer_B count register"]
    pub tb3r: TB3R,
    #[doc = "0x12 - Timer_B Capture/Compare Register"]
    pub tb3ccr0: TB3CCR0,
    #[doc = "0x14 - Timer_B Capture/Compare Register"]
    pub tb3ccr1: TB3CCR1,
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb3ccr2: TB3CCR2,
    #[doc = "0x18 - Timer_B Capture/Compare Register"]
    pub tb3ccr3: TB3CCR3,
    #[doc = "0x1a - Timer_B Capture/Compare Register"]
    pub tb3ccr4: TB3CCR4,
    #[doc = "0x1c - Timer_B Capture/Compare Register"]
    pub tb3ccr5: TB3CCR5,
    #[doc = "0x1e - Timer_B Capture/Compare Register"]
    pub tb3ccr6: TB3CCR6,
    #[doc = "0x20 - Timer_Bx Expansion Register 0"]
    pub tb3ex0: TB3EX0,
    _reserved17: [u8; 12usize],
    #[doc = "0x2e - Timer_Bx Interrupt Vector Register"]
    pub tb3iv: TB3IV,
}
#[doc = "Timer_B Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ctl](tb3ctl) module"]
pub type TB3CTL = crate::Reg<u16, _TB3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CTL;
#[doc = "`read()` method returns [tb3ctl::R](tb3ctl::R) reader structure"]
impl crate::Readable for TB3CTL {}
#[doc = "`write(|w| ..)` method takes [tb3ctl::W](tb3ctl::W) writer structure"]
impl crate::Writable for TB3CTL {}
#[doc = "Timer_B Control Register"]
pub mod tb3ctl;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl0](tb3cctl0) module"]
pub type TB3CCTL0 = crate::Reg<u16, _TB3CCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL0;
#[doc = "`read()` method returns [tb3cctl0::R](tb3cctl0::R) reader structure"]
impl crate::Readable for TB3CCTL0 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl0::W](tb3cctl0::W) writer structure"]
impl crate::Writable for TB3CCTL0 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl0;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl1](tb3cctl1) module"]
pub type TB3CCTL1 = crate::Reg<u16, _TB3CCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL1;
#[doc = "`read()` method returns [tb3cctl1::R](tb3cctl1::R) reader structure"]
impl crate::Readable for TB3CCTL1 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl1::W](tb3cctl1::W) writer structure"]
impl crate::Writable for TB3CCTL1 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl1;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl2](tb3cctl2) module"]
pub type TB3CCTL2 = crate::Reg<u16, _TB3CCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL2;
#[doc = "`read()` method returns [tb3cctl2::R](tb3cctl2::R) reader structure"]
impl crate::Readable for TB3CCTL2 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl2::W](tb3cctl2::W) writer structure"]
impl crate::Writable for TB3CCTL2 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl2;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl3](tb3cctl3) module"]
pub type TB3CCTL3 = crate::Reg<u16, _TB3CCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL3;
#[doc = "`read()` method returns [tb3cctl3::R](tb3cctl3::R) reader structure"]
impl crate::Readable for TB3CCTL3 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl3::W](tb3cctl3::W) writer structure"]
impl crate::Writable for TB3CCTL3 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl3;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl4](tb3cctl4) module"]
pub type TB3CCTL4 = crate::Reg<u16, _TB3CCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL4;
#[doc = "`read()` method returns [tb3cctl4::R](tb3cctl4::R) reader structure"]
impl crate::Readable for TB3CCTL4 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl4::W](tb3cctl4::W) writer structure"]
impl crate::Writable for TB3CCTL4 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl4;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl5](tb3cctl5) module"]
pub type TB3CCTL5 = crate::Reg<u16, _TB3CCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL5;
#[doc = "`read()` method returns [tb3cctl5::R](tb3cctl5::R) reader structure"]
impl crate::Readable for TB3CCTL5 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl5::W](tb3cctl5::W) writer structure"]
impl crate::Writable for TB3CCTL5 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl5;
#[doc = "Timer_B Capture/Compare Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3cctl6](tb3cctl6) module"]
pub type TB3CCTL6 = crate::Reg<u16, _TB3CCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCTL6;
#[doc = "`read()` method returns [tb3cctl6::R](tb3cctl6::R) reader structure"]
impl crate::Readable for TB3CCTL6 {}
#[doc = "`write(|w| ..)` method takes [tb3cctl6::W](tb3cctl6::W) writer structure"]
impl crate::Writable for TB3CCTL6 {}
#[doc = "Timer_B Capture/Compare Control Register"]
pub mod tb3cctl6;
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3r](tb3r) module"]
pub type TB3R = crate::Reg<u16, _TB3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3R;
#[doc = "`read()` method returns [tb3r::R](tb3r::R) reader structure"]
impl crate::Readable for TB3R {}
#[doc = "`write(|w| ..)` method takes [tb3r::W](tb3r::W) writer structure"]
impl crate::Writable for TB3R {}
#[doc = "Timer_B count register"]
pub mod tb3r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr0](tb3ccr0) module"]
pub type TB3CCR0 = crate::Reg<u16, _TB3CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR0;
#[doc = "`read()` method returns [tb3ccr0::R](tb3ccr0::R) reader structure"]
impl crate::Readable for TB3CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr0::W](tb3ccr0::W) writer structure"]
impl crate::Writable for TB3CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr1](tb3ccr1) module"]
pub type TB3CCR1 = crate::Reg<u16, _TB3CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR1;
#[doc = "`read()` method returns [tb3ccr1::R](tb3ccr1::R) reader structure"]
impl crate::Readable for TB3CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr1::W](tb3ccr1::W) writer structure"]
impl crate::Writable for TB3CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr1;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr2](tb3ccr2) module"]
pub type TB3CCR2 = crate::Reg<u16, _TB3CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR2;
#[doc = "`read()` method returns [tb3ccr2::R](tb3ccr2::R) reader structure"]
impl crate::Readable for TB3CCR2 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr2::W](tb3ccr2::W) writer structure"]
impl crate::Writable for TB3CCR2 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr2;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr3](tb3ccr3) module"]
pub type TB3CCR3 = crate::Reg<u16, _TB3CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR3;
#[doc = "`read()` method returns [tb3ccr3::R](tb3ccr3::R) reader structure"]
impl crate::Readable for TB3CCR3 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr3::W](tb3ccr3::W) writer structure"]
impl crate::Writable for TB3CCR3 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr3;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr4](tb3ccr4) module"]
pub type TB3CCR4 = crate::Reg<u16, _TB3CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR4;
#[doc = "`read()` method returns [tb3ccr4::R](tb3ccr4::R) reader structure"]
impl crate::Readable for TB3CCR4 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr4::W](tb3ccr4::W) writer structure"]
impl crate::Writable for TB3CCR4 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr4;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr5](tb3ccr5) module"]
pub type TB3CCR5 = crate::Reg<u16, _TB3CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR5;
#[doc = "`read()` method returns [tb3ccr5::R](tb3ccr5::R) reader structure"]
impl crate::Readable for TB3CCR5 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr5::W](tb3ccr5::W) writer structure"]
impl crate::Writable for TB3CCR5 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr5;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ccr6](tb3ccr6) module"]
pub type TB3CCR6 = crate::Reg<u16, _TB3CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3CCR6;
#[doc = "`read()` method returns [tb3ccr6::R](tb3ccr6::R) reader structure"]
impl crate::Readable for TB3CCR6 {}
#[doc = "`write(|w| ..)` method takes [tb3ccr6::W](tb3ccr6::W) writer structure"]
impl crate::Writable for TB3CCR6 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb3ccr6;
#[doc = "Timer_Bx Expansion Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3ex0](tb3ex0) module"]
pub type TB3EX0 = crate::Reg<u16, _TB3EX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3EX0;
#[doc = "`read()` method returns [tb3ex0::R](tb3ex0::R) reader structure"]
impl crate::Readable for TB3EX0 {}
#[doc = "`write(|w| ..)` method takes [tb3ex0::W](tb3ex0::W) writer structure"]
impl crate::Writable for TB3EX0 {}
#[doc = "Timer_Bx Expansion Register 0"]
pub mod tb3ex0;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3iv](tb3iv) module"]
pub type TB3IV = crate::Reg<u16, _TB3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3IV;
#[doc = "`read()` method returns [tb3iv::R](tb3iv::R) reader structure"]
impl crate::Readable for TB3IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb3iv;
