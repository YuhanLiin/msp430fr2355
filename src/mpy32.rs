#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 16-bit operand one multiply"]
    pub mpy: MPY,
    #[doc = "0x02 - 16-bit operand one signed multiply"]
    pub mpys: MPYS,
    #[doc = "0x04 - 16-bit operand one multiply accumulate"]
    pub mac: MAC,
    #[doc = "0x06 - 16-bit operand one signed multiply accumulate"]
    pub macs: MACS,
    #[doc = "0x08 - 16-bit operand two"]
    pub op2: OP2,
    #[doc = "0x0a - 16x16-bit result low word"]
    pub reslo: RESLO,
    #[doc = "0x0c - 16x16-bit result high word"]
    pub reshi: RESHI,
    #[doc = "0x0e - 16x16-bit sum extension register"]
    pub sumext: SUMEXT,
    #[doc = "0x10 - 32-bit operand 1 multiply low word"]
    pub mpy32l: MPY32L,
    #[doc = "0x12 - 32-bit operand 1 multiply high word"]
    pub mpy32h: MPY32H,
    #[doc = "0x14 - 32-bit operand 1 signed multiply low word"]
    pub mpys32l: MPYS32L,
    #[doc = "0x16 - 32-bit operand 1 signed multiply high word"]
    pub mpys32h: MPYS32H,
    #[doc = "0x18 - 32-bit operand 1 multiply accumulate low word"]
    pub mac32l: MAC32L,
    #[doc = "0x1a - 32-bit operand 1 multiply accumulate high word"]
    pub mac32h: MAC32H,
    #[doc = "0x1c - 32-bit operand 1 signed multiply accumulate low word"]
    pub macs32l: MACS32L,
    #[doc = "0x1e - 32-bit operand 1 signed multiply accumulate high word"]
    pub macs32h: MACS32H,
    #[doc = "0x20 - 32-bit operand 2 low word"]
    pub op2l: OP2L,
    #[doc = "0x22 - 32-bit operand 2 high word"]
    pub op2h: OP2H,
    #[doc = "0x24 - 32x32-bit result 0 least significant word"]
    pub res0: RES0,
    #[doc = "0x26 - 32x32-bit result 1"]
    pub res1: RES1,
    #[doc = "0x28 - 32x32-bit result 2"]
    pub res2: RES2,
    #[doc = "0x2a - 32x32-bit result 3 most significant word"]
    pub res3: RES3,
    #[doc = "0x2c - MPY32 control register 0"]
    pub mpy32ctl0: MPY32CTL0,
}
#[doc = "16-bit operand one multiply\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy](mpy) module"]
pub type MPY = crate::Reg<u16, _MPY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY;
#[doc = "`read()` method returns [mpy::R](mpy::R) reader structure"]
impl crate::Readable for MPY {}
#[doc = "`write(|w| ..)` method takes [mpy::W](mpy::W) writer structure"]
impl crate::Writable for MPY {}
#[doc = "16-bit operand one multiply"]
pub mod mpy;
#[doc = "16-bit operand one signed multiply\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpys](mpys) module"]
pub type MPYS = crate::Reg<u16, _MPYS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPYS;
#[doc = "`read()` method returns [mpys::R](mpys::R) reader structure"]
impl crate::Readable for MPYS {}
#[doc = "`write(|w| ..)` method takes [mpys::W](mpys::W) writer structure"]
impl crate::Writable for MPYS {}
#[doc = "16-bit operand one signed multiply"]
pub mod mpys;
#[doc = "16-bit operand one multiply accumulate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac](mac) module"]
pub type MAC = crate::Reg<u16, _MAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC;
#[doc = "`read()` method returns [mac::R](mac::R) reader structure"]
impl crate::Readable for MAC {}
#[doc = "`write(|w| ..)` method takes [mac::W](mac::W) writer structure"]
impl crate::Writable for MAC {}
#[doc = "16-bit operand one multiply accumulate"]
pub mod mac;
#[doc = "16-bit operand one signed multiply accumulate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs](macs) module"]
pub type MACS = crate::Reg<u16, _MACS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACS;
#[doc = "`read()` method returns [macs::R](macs::R) reader structure"]
impl crate::Readable for MACS {}
#[doc = "`write(|w| ..)` method takes [macs::W](macs::W) writer structure"]
impl crate::Writable for MACS {}
#[doc = "16-bit operand one signed multiply accumulate"]
pub mod macs;
#[doc = "16-bit operand two\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op2](op2) module"]
pub type OP2 = crate::Reg<u16, _OP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP2;
#[doc = "`read()` method returns [op2::R](op2::R) reader structure"]
impl crate::Readable for OP2 {}
#[doc = "`write(|w| ..)` method takes [op2::W](op2::W) writer structure"]
impl crate::Writable for OP2 {}
#[doc = "16-bit operand two"]
pub mod op2;
#[doc = "16x16-bit result low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reslo](reslo) module"]
pub type RESLO = crate::Reg<u16, _RESLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESLO;
#[doc = "`read()` method returns [reslo::R](reslo::R) reader structure"]
impl crate::Readable for RESLO {}
#[doc = "`write(|w| ..)` method takes [reslo::W](reslo::W) writer structure"]
impl crate::Writable for RESLO {}
#[doc = "16x16-bit result low word"]
pub mod reslo;
#[doc = "16x16-bit result high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reshi](reshi) module"]
pub type RESHI = crate::Reg<u16, _RESHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESHI;
#[doc = "`read()` method returns [reshi::R](reshi::R) reader structure"]
impl crate::Readable for RESHI {}
#[doc = "`write(|w| ..)` method takes [reshi::W](reshi::W) writer structure"]
impl crate::Writable for RESHI {}
#[doc = "16x16-bit result high word"]
pub mod reshi;
#[doc = "16x16-bit sum extension register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sumext](sumext) module"]
pub type SUMEXT = crate::Reg<u16, _SUMEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUMEXT;
#[doc = "`read()` method returns [sumext::R](sumext::R) reader structure"]
impl crate::Readable for SUMEXT {}
#[doc = "`write(|w| ..)` method takes [sumext::W](sumext::W) writer structure"]
impl crate::Writable for SUMEXT {}
#[doc = "16x16-bit sum extension register"]
pub mod sumext;
#[doc = "32-bit operand 1 multiply low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32l](mpy32l) module"]
pub type MPY32L = crate::Reg<u16, _MPY32L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY32L;
#[doc = "`read()` method returns [mpy32l::R](mpy32l::R) reader structure"]
impl crate::Readable for MPY32L {}
#[doc = "`write(|w| ..)` method takes [mpy32l::W](mpy32l::W) writer structure"]
impl crate::Writable for MPY32L {}
#[doc = "32-bit operand 1 multiply low word"]
pub mod mpy32l;
#[doc = "32-bit operand 1 multiply high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32h](mpy32h) module"]
pub type MPY32H = crate::Reg<u16, _MPY32H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY32H;
#[doc = "`read()` method returns [mpy32h::R](mpy32h::R) reader structure"]
impl crate::Readable for MPY32H {}
#[doc = "`write(|w| ..)` method takes [mpy32h::W](mpy32h::W) writer structure"]
impl crate::Writable for MPY32H {}
#[doc = "32-bit operand 1 multiply high word"]
pub mod mpy32h;
#[doc = "32-bit operand 1 signed multiply low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpys32l](mpys32l) module"]
pub type MPYS32L = crate::Reg<u16, _MPYS32L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPYS32L;
#[doc = "`read()` method returns [mpys32l::R](mpys32l::R) reader structure"]
impl crate::Readable for MPYS32L {}
#[doc = "`write(|w| ..)` method takes [mpys32l::W](mpys32l::W) writer structure"]
impl crate::Writable for MPYS32L {}
#[doc = "32-bit operand 1 signed multiply low word"]
pub mod mpys32l;
#[doc = "32-bit operand 1 signed multiply high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpys32h](mpys32h) module"]
pub type MPYS32H = crate::Reg<u16, _MPYS32H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPYS32H;
#[doc = "`read()` method returns [mpys32h::R](mpys32h::R) reader structure"]
impl crate::Readable for MPYS32H {}
#[doc = "`write(|w| ..)` method takes [mpys32h::W](mpys32h::W) writer structure"]
impl crate::Writable for MPYS32H {}
#[doc = "32-bit operand 1 signed multiply high word"]
pub mod mpys32h;
#[doc = "32-bit operand 1 multiply accumulate low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac32l](mac32l) module"]
pub type MAC32L = crate::Reg<u16, _MAC32L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC32L;
#[doc = "`read()` method returns [mac32l::R](mac32l::R) reader structure"]
impl crate::Readable for MAC32L {}
#[doc = "`write(|w| ..)` method takes [mac32l::W](mac32l::W) writer structure"]
impl crate::Writable for MAC32L {}
#[doc = "32-bit operand 1 multiply accumulate low word"]
pub mod mac32l;
#[doc = "32-bit operand 1 multiply accumulate high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac32h](mac32h) module"]
pub type MAC32H = crate::Reg<u16, _MAC32H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC32H;
#[doc = "`read()` method returns [mac32h::R](mac32h::R) reader structure"]
impl crate::Readable for MAC32H {}
#[doc = "`write(|w| ..)` method takes [mac32h::W](mac32h::W) writer structure"]
impl crate::Writable for MAC32H {}
#[doc = "32-bit operand 1 multiply accumulate high word"]
pub mod mac32h;
#[doc = "32-bit operand 1 signed multiply accumulate low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs32l](macs32l) module"]
pub type MACS32L = crate::Reg<u16, _MACS32L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACS32L;
#[doc = "`read()` method returns [macs32l::R](macs32l::R) reader structure"]
impl crate::Readable for MACS32L {}
#[doc = "`write(|w| ..)` method takes [macs32l::W](macs32l::W) writer structure"]
impl crate::Writable for MACS32L {}
#[doc = "32-bit operand 1 signed multiply accumulate low word"]
pub mod macs32l;
#[doc = "32-bit operand 1 signed multiply accumulate high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macs32h](macs32h) module"]
pub type MACS32H = crate::Reg<u16, _MACS32H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACS32H;
#[doc = "`read()` method returns [macs32h::R](macs32h::R) reader structure"]
impl crate::Readable for MACS32H {}
#[doc = "`write(|w| ..)` method takes [macs32h::W](macs32h::W) writer structure"]
impl crate::Writable for MACS32H {}
#[doc = "32-bit operand 1 signed multiply accumulate high word"]
pub mod macs32h;
#[doc = "32-bit operand 2 low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op2l](op2l) module"]
pub type OP2L = crate::Reg<u16, _OP2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP2L;
#[doc = "`read()` method returns [op2l::R](op2l::R) reader structure"]
impl crate::Readable for OP2L {}
#[doc = "`write(|w| ..)` method takes [op2l::W](op2l::W) writer structure"]
impl crate::Writable for OP2L {}
#[doc = "32-bit operand 2 low word"]
pub mod op2l;
#[doc = "32-bit operand 2 high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op2h](op2h) module"]
pub type OP2H = crate::Reg<u16, _OP2H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP2H;
#[doc = "`read()` method returns [op2h::R](op2h::R) reader structure"]
impl crate::Readable for OP2H {}
#[doc = "`write(|w| ..)` method takes [op2h::W](op2h::W) writer structure"]
impl crate::Writable for OP2H {}
#[doc = "32-bit operand 2 high word"]
pub mod op2h;
#[doc = "32x32-bit result 0 least significant word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res0](res0) module"]
pub type RES0 = crate::Reg<u16, _RES0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES0;
#[doc = "`read()` method returns [res0::R](res0::R) reader structure"]
impl crate::Readable for RES0 {}
#[doc = "`write(|w| ..)` method takes [res0::W](res0::W) writer structure"]
impl crate::Writable for RES0 {}
#[doc = "32x32-bit result 0 least significant word"]
pub mod res0;
#[doc = "32x32-bit result 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res1](res1) module"]
pub type RES1 = crate::Reg<u16, _RES1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES1;
#[doc = "`read()` method returns [res1::R](res1::R) reader structure"]
impl crate::Readable for RES1 {}
#[doc = "`write(|w| ..)` method takes [res1::W](res1::W) writer structure"]
impl crate::Writable for RES1 {}
#[doc = "32x32-bit result 1"]
pub mod res1;
#[doc = "32x32-bit result 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res2](res2) module"]
pub type RES2 = crate::Reg<u16, _RES2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES2;
#[doc = "`read()` method returns [res2::R](res2::R) reader structure"]
impl crate::Readable for RES2 {}
#[doc = "`write(|w| ..)` method takes [res2::W](res2::W) writer structure"]
impl crate::Writable for RES2 {}
#[doc = "32x32-bit result 2"]
pub mod res2;
#[doc = "32x32-bit result 3 most significant word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res3](res3) module"]
pub type RES3 = crate::Reg<u16, _RES3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES3;
#[doc = "`read()` method returns [res3::R](res3::R) reader structure"]
impl crate::Readable for RES3 {}
#[doc = "`write(|w| ..)` method takes [res3::W](res3::W) writer structure"]
impl crate::Writable for RES3 {}
#[doc = "32x32-bit result 3 most significant word"]
pub mod res3;
#[doc = "MPY32 control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy32ctl0](mpy32ctl0) module"]
pub type MPY32CTL0 = crate::Reg<u16, _MPY32CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY32CTL0;
#[doc = "`read()` method returns [mpy32ctl0::R](mpy32ctl0::R) reader structure"]
impl crate::Readable for MPY32CTL0 {}
#[doc = "`write(|w| ..)` method takes [mpy32ctl0::W](mpy32ctl0::W) writer structure"]
impl crate::Writable for MPY32CTL0 {}
#[doc = "MPY32 control register 0"]
pub mod mpy32ctl0;
