#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock System Control 0"]
    pub csctl0: CSCTL0,
    #[doc = "0x02 - Clock System Control 1"]
    pub csctl1: CSCTL1,
    #[doc = "0x04 - Clock System Control 2"]
    pub csctl2: CSCTL2,
    #[doc = "0x06 - Clock System Control 3"]
    pub csctl3: CSCTL3,
    #[doc = "0x08 - Clock System Control 4"]
    pub csctl4: CSCTL4,
    #[doc = "0x0a - Clock System Control 5"]
    pub csctl5: CSCTL5,
    #[doc = "0x0c - Clock System Control 6"]
    pub csctl6: CSCTL6,
    #[doc = "0x0e - Clock System Control Register 7"]
    pub csctl7: CSCTL7,
    #[doc = "0x10 - Clock System Control Register 8"]
    pub csctl8: CSCTL8,
}
#[doc = "Clock System Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl0](csctl0) module"]
pub type CSCTL0 = crate::Reg<u16, _CSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL0;
#[doc = "`read()` method returns [csctl0::R](csctl0::R) reader structure"]
impl crate::Readable for CSCTL0 {}
#[doc = "`write(|w| ..)` method takes [csctl0::W](csctl0::W) writer structure"]
impl crate::Writable for CSCTL0 {}
#[doc = "Clock System Control 0"]
pub mod csctl0;
#[doc = "Clock System Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl1](csctl1) module"]
pub type CSCTL1 = crate::Reg<u16, _CSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL1;
#[doc = "`read()` method returns [csctl1::R](csctl1::R) reader structure"]
impl crate::Readable for CSCTL1 {}
#[doc = "`write(|w| ..)` method takes [csctl1::W](csctl1::W) writer structure"]
impl crate::Writable for CSCTL1 {}
#[doc = "Clock System Control 1"]
pub mod csctl1;
#[doc = "Clock System Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl2](csctl2) module"]
pub type CSCTL2 = crate::Reg<u16, _CSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL2;
#[doc = "`read()` method returns [csctl2::R](csctl2::R) reader structure"]
impl crate::Readable for CSCTL2 {}
#[doc = "`write(|w| ..)` method takes [csctl2::W](csctl2::W) writer structure"]
impl crate::Writable for CSCTL2 {}
#[doc = "Clock System Control 2"]
pub mod csctl2;
#[doc = "Clock System Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl3](csctl3) module"]
pub type CSCTL3 = crate::Reg<u16, _CSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL3;
#[doc = "`read()` method returns [csctl3::R](csctl3::R) reader structure"]
impl crate::Readable for CSCTL3 {}
#[doc = "`write(|w| ..)` method takes [csctl3::W](csctl3::W) writer structure"]
impl crate::Writable for CSCTL3 {}
#[doc = "Clock System Control 3"]
pub mod csctl3;
#[doc = "Clock System Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl4](csctl4) module"]
pub type CSCTL4 = crate::Reg<u16, _CSCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL4;
#[doc = "`read()` method returns [csctl4::R](csctl4::R) reader structure"]
impl crate::Readable for CSCTL4 {}
#[doc = "`write(|w| ..)` method takes [csctl4::W](csctl4::W) writer structure"]
impl crate::Writable for CSCTL4 {}
#[doc = "Clock System Control 4"]
pub mod csctl4;
#[doc = "Clock System Control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl5](csctl5) module"]
pub type CSCTL5 = crate::Reg<u16, _CSCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL5;
#[doc = "`read()` method returns [csctl5::R](csctl5::R) reader structure"]
impl crate::Readable for CSCTL5 {}
#[doc = "`write(|w| ..)` method takes [csctl5::W](csctl5::W) writer structure"]
impl crate::Writable for CSCTL5 {}
#[doc = "Clock System Control 5"]
pub mod csctl5;
#[doc = "Clock System Control 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl6](csctl6) module"]
pub type CSCTL6 = crate::Reg<u16, _CSCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL6;
#[doc = "`read()` method returns [csctl6::R](csctl6::R) reader structure"]
impl crate::Readable for CSCTL6 {}
#[doc = "`write(|w| ..)` method takes [csctl6::W](csctl6::W) writer structure"]
impl crate::Writable for CSCTL6 {}
#[doc = "Clock System Control 6"]
pub mod csctl6;
#[doc = "Clock System Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl7](csctl7) module"]
pub type CSCTL7 = crate::Reg<u16, _CSCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL7;
#[doc = "`read()` method returns [csctl7::R](csctl7::R) reader structure"]
impl crate::Readable for CSCTL7 {}
#[doc = "`write(|w| ..)` method takes [csctl7::W](csctl7::W) writer structure"]
impl crate::Writable for CSCTL7 {}
#[doc = "Clock System Control Register 7"]
pub mod csctl7;
#[doc = "Clock System Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl8](csctl8) module"]
pub type CSCTL8 = crate::Reg<u16, _CSCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL8;
#[doc = "`read()` method returns [csctl8::R](csctl8::R) reader structure"]
impl crate::Readable for CSCTL8 {}
#[doc = "`write(|w| ..)` method takes [csctl8::W](csctl8::W) writer structure"]
impl crate::Writable for CSCTL8 {}
#[doc = "Clock System Control Register 8"]
pub mod csctl8;
