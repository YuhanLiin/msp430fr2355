#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Port 8 Input"]
    pub p8in: P8IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x03 - Port 8 Output"]
    pub p8out: P8OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x05 - Port 8 Direction"]
    pub p8dir: P8DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x07 - Port 8 Resistor Enable"]
    pub p8ren: P8REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0b - Port 8 Select 0"]
    pub p8sel0: P8SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0d - Port 8 Select 1"]
    pub p8sel1: P8SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x17 - Port 8 Complement Select"]
    pub p8selc: P8SELC,
    _reserved7: [u8; 1usize],
    #[doc = "0x19 - Port 8 Interrupt Edge Select"]
    pub p8ies: P8IES,
    _reserved8: [u8; 1usize],
    #[doc = "0x1b - Port 8 Interrupt Enable"]
    pub p8ie: P8IE,
    _reserved9: [u8; 1usize],
    #[doc = "0x1d - Port 8 Interrupt Flag"]
    pub p8ifg: P8IFG,
    #[doc = "0x1e - Port 8 Interrupt Vector Register"]
    pub p8iv: P8IV,
}
#[doc = "Port 8 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8in](p8in) module"]
pub type P8IN = crate::Reg<u8, _P8IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IN;
#[doc = "`read()` method returns [p8in::R](p8in::R) reader structure"]
impl crate::Readable for P8IN {}
#[doc = "`write(|w| ..)` method takes [p8in::W](p8in::W) writer structure"]
impl crate::Writable for P8IN {}
#[doc = "Port 8 Input"]
pub mod p8in;
#[doc = "Port 8 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8out](p8out) module"]
pub type P8OUT = crate::Reg<u8, _P8OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8OUT;
#[doc = "`read()` method returns [p8out::R](p8out::R) reader structure"]
impl crate::Readable for P8OUT {}
#[doc = "`write(|w| ..)` method takes [p8out::W](p8out::W) writer structure"]
impl crate::Writable for P8OUT {}
#[doc = "Port 8 Output"]
pub mod p8out;
#[doc = "Port 8 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8dir](p8dir) module"]
pub type P8DIR = crate::Reg<u8, _P8DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8DIR;
#[doc = "`read()` method returns [p8dir::R](p8dir::R) reader structure"]
impl crate::Readable for P8DIR {}
#[doc = "`write(|w| ..)` method takes [p8dir::W](p8dir::W) writer structure"]
impl crate::Writable for P8DIR {}
#[doc = "Port 8 Direction"]
pub mod p8dir;
#[doc = "Port 8 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8ren](p8ren) module"]
pub type P8REN = crate::Reg<u8, _P8REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8REN;
#[doc = "`read()` method returns [p8ren::R](p8ren::R) reader structure"]
impl crate::Readable for P8REN {}
#[doc = "`write(|w| ..)` method takes [p8ren::W](p8ren::W) writer structure"]
impl crate::Writable for P8REN {}
#[doc = "Port 8 Resistor Enable"]
pub mod p8ren;
#[doc = "Port 8 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8sel0](p8sel0) module"]
pub type P8SEL0 = crate::Reg<u8, _P8SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8SEL0;
#[doc = "`read()` method returns [p8sel0::R](p8sel0::R) reader structure"]
impl crate::Readable for P8SEL0 {}
#[doc = "`write(|w| ..)` method takes [p8sel0::W](p8sel0::W) writer structure"]
impl crate::Writable for P8SEL0 {}
#[doc = "Port 8 Select 0"]
pub mod p8sel0;
#[doc = "Port 8 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8sel1](p8sel1) module"]
pub type P8SEL1 = crate::Reg<u8, _P8SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8SEL1;
#[doc = "`read()` method returns [p8sel1::R](p8sel1::R) reader structure"]
impl crate::Readable for P8SEL1 {}
#[doc = "`write(|w| ..)` method takes [p8sel1::W](p8sel1::W) writer structure"]
impl crate::Writable for P8SEL1 {}
#[doc = "Port 8 Select 1"]
pub mod p8sel1;
#[doc = "Port 8 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8selc](p8selc) module"]
pub type P8SELC = crate::Reg<u8, _P8SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8SELC;
#[doc = "`read()` method returns [p8selc::R](p8selc::R) reader structure"]
impl crate::Readable for P8SELC {}
#[doc = "`write(|w| ..)` method takes [p8selc::W](p8selc::W) writer structure"]
impl crate::Writable for P8SELC {}
#[doc = "Port 8 Complement Select"]
pub mod p8selc;
#[doc = "Port 8 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8ies](p8ies) module"]
pub type P8IES = crate::Reg<u8, _P8IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IES;
#[doc = "`read()` method returns [p8ies::R](p8ies::R) reader structure"]
impl crate::Readable for P8IES {}
#[doc = "`write(|w| ..)` method takes [p8ies::W](p8ies::W) writer structure"]
impl crate::Writable for P8IES {}
#[doc = "Port 8 Interrupt Edge Select"]
pub mod p8ies;
#[doc = "Port 8 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8ie](p8ie) module"]
pub type P8IE = crate::Reg<u8, _P8IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IE;
#[doc = "`read()` method returns [p8ie::R](p8ie::R) reader structure"]
impl crate::Readable for P8IE {}
#[doc = "`write(|w| ..)` method takes [p8ie::W](p8ie::W) writer structure"]
impl crate::Writable for P8IE {}
#[doc = "Port 8 Interrupt Enable"]
pub mod p8ie;
#[doc = "Port 8 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8ifg](p8ifg) module"]
pub type P8IFG = crate::Reg<u8, _P8IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IFG;
#[doc = "`read()` method returns [p8ifg::R](p8ifg::R) reader structure"]
impl crate::Readable for P8IFG {}
#[doc = "`write(|w| ..)` method takes [p8ifg::W](p8ifg::W) writer structure"]
impl crate::Writable for P8IFG {}
#[doc = "Port 8 Interrupt Flag"]
pub mod p8ifg;
#[doc = "Port 8 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p8iv](p8iv) module"]
pub type P8IV = crate::Reg<u16, _P8IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P8IV;
#[doc = "`read()` method returns [p8iv::R](p8iv::R) reader structure"]
impl crate::Readable for P8IV {}
#[doc = "Port 8 Interrupt Vector Register"]
pub mod p8iv;
