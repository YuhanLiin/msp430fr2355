#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl3: TB3CCTL3,
    _reserved1: [u8; 4usize],
    #[doc = "0x06 - Timer_B Capture/Compare Control Register"]
    pub tb3cctl6: TB3CCTL6,
    #[doc = "0x08 - Timer_B count register"]
    pub tb3r: TB3R,
    #[doc = "0x0a - Timer_B Capture/Compare Register"]
    pub tb3ccr0: TB3CCR0,
    #[doc = "0x0c - Timer_B Capture/Compare Register"]
    pub tb3ccr1: TB3CCR1,
    _reserved5: [u8; 8usize],
    #[doc = "0x16 - Timer_B Capture/Compare Register"]
    pub tb3ccr6: TB3CCR6,
    _reserved6: [u8; 14usize],
    #[doc = "0x26 - Timer_Bx Interrupt Vector Register"]
    pub tb3iv: TB3IV,
}
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
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb3iv](tb3iv) module"]
pub type TB3IV = crate::Reg<u16, _TB3IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB3IV;
#[doc = "`read()` method returns [tb3iv::R](tb3iv::R) reader structure"]
impl crate::Readable for TB3IV {}
#[doc = "`write(|w| ..)` method takes [tb3iv::W](tb3iv::W) writer structure"]
impl crate::Writable for TB3IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb3iv;
