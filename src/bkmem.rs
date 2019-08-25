#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup Memory registers. Backup Memory 0."]
    pub bakmem0: BAKMEM0,
    #[doc = "0x02 - Backup Memory 1."]
    pub bakmem1: BAKMEM1,
    #[doc = "0x04 - Backup Memory 2."]
    pub bakmem2: BAKMEM2,
    #[doc = "0x06 - Backup Memory 3."]
    pub bakmem3: BAKMEM3,
    #[doc = "0x08 - Backup Memory 4."]
    pub bakmem4: BAKMEM4,
    #[doc = "0x0a - Backup Memory 5."]
    pub bakmem5: BAKMEM5,
    #[doc = "0x0c - Backup Memory 6."]
    pub bakmem6: BAKMEM6,
    #[doc = "0x0e - Backup Memory 7."]
    pub bakmem7: BAKMEM7,
    #[doc = "0x10 - Backup Memory 8."]
    pub bakmem8: BAKMEM8,
    #[doc = "0x12 - Backup Memory 9."]
    pub bakmem9: BAKMEM9,
    #[doc = "0x14 - Backup Memory registers. Backup Memory 10."]
    pub bakmem10: BAKMEM10,
    #[doc = "0x16 - Backup Memory 11."]
    pub bakmem11: BAKMEM11,
    #[doc = "0x18 - Backup Memory 12."]
    pub bakmem12: BAKMEM12,
    #[doc = "0x1a - Backup Memory 13."]
    pub bakmem13: BAKMEM13,
    #[doc = "0x1c - Backup Memory 14."]
    pub bakmem14: BAKMEM14,
    #[doc = "0x1e - Backup Memory 15."]
    pub bakmem15: BAKMEM15,
}
#[doc = "Backup Memory registers. Backup Memory 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem0](bakmem0) module"]
pub type BAKMEM0 = crate::Reg<u16, _BAKMEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM0;
#[doc = "`read()` method returns [bakmem0::R](bakmem0::R) reader structure"]
impl crate::Readable for BAKMEM0 {}
#[doc = "`write(|w| ..)` method takes [bakmem0::W](bakmem0::W) writer structure"]
impl crate::Writable for BAKMEM0 {}
#[doc = "Backup Memory registers. Backup Memory 0."]
pub mod bakmem0;
#[doc = "Backup Memory 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem1](bakmem1) module"]
pub type BAKMEM1 = crate::Reg<u16, _BAKMEM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM1;
#[doc = "`read()` method returns [bakmem1::R](bakmem1::R) reader structure"]
impl crate::Readable for BAKMEM1 {}
#[doc = "`write(|w| ..)` method takes [bakmem1::W](bakmem1::W) writer structure"]
impl crate::Writable for BAKMEM1 {}
#[doc = "Backup Memory 1."]
pub mod bakmem1;
#[doc = "Backup Memory 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem2](bakmem2) module"]
pub type BAKMEM2 = crate::Reg<u16, _BAKMEM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM2;
#[doc = "`read()` method returns [bakmem2::R](bakmem2::R) reader structure"]
impl crate::Readable for BAKMEM2 {}
#[doc = "`write(|w| ..)` method takes [bakmem2::W](bakmem2::W) writer structure"]
impl crate::Writable for BAKMEM2 {}
#[doc = "Backup Memory 2."]
pub mod bakmem2;
#[doc = "Backup Memory 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem3](bakmem3) module"]
pub type BAKMEM3 = crate::Reg<u16, _BAKMEM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM3;
#[doc = "`read()` method returns [bakmem3::R](bakmem3::R) reader structure"]
impl crate::Readable for BAKMEM3 {}
#[doc = "`write(|w| ..)` method takes [bakmem3::W](bakmem3::W) writer structure"]
impl crate::Writable for BAKMEM3 {}
#[doc = "Backup Memory 3."]
pub mod bakmem3;
#[doc = "Backup Memory 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem4](bakmem4) module"]
pub type BAKMEM4 = crate::Reg<u16, _BAKMEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM4;
#[doc = "`read()` method returns [bakmem4::R](bakmem4::R) reader structure"]
impl crate::Readable for BAKMEM4 {}
#[doc = "`write(|w| ..)` method takes [bakmem4::W](bakmem4::W) writer structure"]
impl crate::Writable for BAKMEM4 {}
#[doc = "Backup Memory 4."]
pub mod bakmem4;
#[doc = "Backup Memory 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem5](bakmem5) module"]
pub type BAKMEM5 = crate::Reg<u16, _BAKMEM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM5;
#[doc = "`read()` method returns [bakmem5::R](bakmem5::R) reader structure"]
impl crate::Readable for BAKMEM5 {}
#[doc = "`write(|w| ..)` method takes [bakmem5::W](bakmem5::W) writer structure"]
impl crate::Writable for BAKMEM5 {}
#[doc = "Backup Memory 5."]
pub mod bakmem5;
#[doc = "Backup Memory 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem6](bakmem6) module"]
pub type BAKMEM6 = crate::Reg<u16, _BAKMEM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM6;
#[doc = "`read()` method returns [bakmem6::R](bakmem6::R) reader structure"]
impl crate::Readable for BAKMEM6 {}
#[doc = "`write(|w| ..)` method takes [bakmem6::W](bakmem6::W) writer structure"]
impl crate::Writable for BAKMEM6 {}
#[doc = "Backup Memory 6."]
pub mod bakmem6;
#[doc = "Backup Memory 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem7](bakmem7) module"]
pub type BAKMEM7 = crate::Reg<u16, _BAKMEM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM7;
#[doc = "`read()` method returns [bakmem7::R](bakmem7::R) reader structure"]
impl crate::Readable for BAKMEM7 {}
#[doc = "`write(|w| ..)` method takes [bakmem7::W](bakmem7::W) writer structure"]
impl crate::Writable for BAKMEM7 {}
#[doc = "Backup Memory 7."]
pub mod bakmem7;
#[doc = "Backup Memory 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem8](bakmem8) module"]
pub type BAKMEM8 = crate::Reg<u16, _BAKMEM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM8;
#[doc = "`read()` method returns [bakmem8::R](bakmem8::R) reader structure"]
impl crate::Readable for BAKMEM8 {}
#[doc = "`write(|w| ..)` method takes [bakmem8::W](bakmem8::W) writer structure"]
impl crate::Writable for BAKMEM8 {}
#[doc = "Backup Memory 8."]
pub mod bakmem8;
#[doc = "Backup Memory 9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem9](bakmem9) module"]
pub type BAKMEM9 = crate::Reg<u16, _BAKMEM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM9;
#[doc = "`read()` method returns [bakmem9::R](bakmem9::R) reader structure"]
impl crate::Readable for BAKMEM9 {}
#[doc = "`write(|w| ..)` method takes [bakmem9::W](bakmem9::W) writer structure"]
impl crate::Writable for BAKMEM9 {}
#[doc = "Backup Memory 9."]
pub mod bakmem9;
#[doc = "Backup Memory registers. Backup Memory 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem10](bakmem10) module"]
pub type BAKMEM10 = crate::Reg<u16, _BAKMEM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM10;
#[doc = "`read()` method returns [bakmem10::R](bakmem10::R) reader structure"]
impl crate::Readable for BAKMEM10 {}
#[doc = "`write(|w| ..)` method takes [bakmem10::W](bakmem10::W) writer structure"]
impl crate::Writable for BAKMEM10 {}
#[doc = "Backup Memory registers. Backup Memory 10."]
pub mod bakmem10;
#[doc = "Backup Memory 11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem11](bakmem11) module"]
pub type BAKMEM11 = crate::Reg<u16, _BAKMEM11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM11;
#[doc = "`read()` method returns [bakmem11::R](bakmem11::R) reader structure"]
impl crate::Readable for BAKMEM11 {}
#[doc = "`write(|w| ..)` method takes [bakmem11::W](bakmem11::W) writer structure"]
impl crate::Writable for BAKMEM11 {}
#[doc = "Backup Memory 11."]
pub mod bakmem11;
#[doc = "Backup Memory 12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem12](bakmem12) module"]
pub type BAKMEM12 = crate::Reg<u16, _BAKMEM12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM12;
#[doc = "`read()` method returns [bakmem12::R](bakmem12::R) reader structure"]
impl crate::Readable for BAKMEM12 {}
#[doc = "`write(|w| ..)` method takes [bakmem12::W](bakmem12::W) writer structure"]
impl crate::Writable for BAKMEM12 {}
#[doc = "Backup Memory 12."]
pub mod bakmem12;
#[doc = "Backup Memory 13.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem13](bakmem13) module"]
pub type BAKMEM13 = crate::Reg<u16, _BAKMEM13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM13;
#[doc = "`read()` method returns [bakmem13::R](bakmem13::R) reader structure"]
impl crate::Readable for BAKMEM13 {}
#[doc = "`write(|w| ..)` method takes [bakmem13::W](bakmem13::W) writer structure"]
impl crate::Writable for BAKMEM13 {}
#[doc = "Backup Memory 13."]
pub mod bakmem13;
#[doc = "Backup Memory 14.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem14](bakmem14) module"]
pub type BAKMEM14 = crate::Reg<u16, _BAKMEM14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM14;
#[doc = "`read()` method returns [bakmem14::R](bakmem14::R) reader structure"]
impl crate::Readable for BAKMEM14 {}
#[doc = "`write(|w| ..)` method takes [bakmem14::W](bakmem14::W) writer structure"]
impl crate::Writable for BAKMEM14 {}
#[doc = "Backup Memory 14."]
pub mod bakmem14;
#[doc = "Backup Memory 15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem15](bakmem15) module"]
pub type BAKMEM15 = crate::Reg<u16, _BAKMEM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM15;
#[doc = "`read()` method returns [bakmem15::R](bakmem15::R) reader structure"]
impl crate::Readable for BAKMEM15 {}
#[doc = "`write(|w| ..)` method takes [bakmem15::W](bakmem15::W) writer structure"]
impl crate::Writable for BAKMEM15 {}
#[doc = "Backup Memory 15."]
pub mod bakmem15;
