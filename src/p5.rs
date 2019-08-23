#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    pub p5in: P5IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 5 Output"]
    pub p5out: P5OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 5 Direction"]
    pub p5dir: P5DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 5 Resistor Enable"]
    pub p5ren: P5REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 5 Select 0"]
    pub p5sel0: P5SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 5 Select 1"]
    pub p5sel1: P5SEL1,
    _reserved6: [u8; 1usize],
    #[doc = "0x0e - Port 5 Interrupt Vector Register"]
    pub p5iv: P5IV,
    _reserved7: [u8; 6usize],
    #[doc = "0x16 - Port 5 Complement Select"]
    pub p5selc: P5SELC,
    _reserved8: [u8; 1usize],
    #[doc = "0x18 - Port 5 Interrupt Edge Select"]
    pub p5ies: P5IES,
    _reserved9: [u8; 1usize],
    #[doc = "0x1a - Port 5 Interrupt Enable"]
    pub p5ie: P5IE,
    _reserved10: [u8; 1usize],
    #[doc = "0x1c - Port 5 Interrupt Flag"]
    pub p5ifg: P5IFG,
}
#[doc = "Port 5 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5in](p5in) module"]
pub type P5IN = crate::Reg<u8, _P5IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IN;
#[doc = "`read()` method returns [p5in::R](p5in::R) reader structure"]
impl crate::Readable for P5IN {}
#[doc = "`write(|w| ..)` method takes [p5in::W](p5in::W) writer structure"]
impl crate::Writable for P5IN {}
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "Port 5 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5out](p5out) module"]
pub type P5OUT = crate::Reg<u8, _P5OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5OUT;
#[doc = "`read()` method returns [p5out::R](p5out::R) reader structure"]
impl crate::Readable for P5OUT {}
#[doc = "`write(|w| ..)` method takes [p5out::W](p5out::W) writer structure"]
impl crate::Writable for P5OUT {}
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "Port 5 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5dir](p5dir) module"]
pub type P5DIR = crate::Reg<u8, _P5DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5DIR;
#[doc = "`read()` method returns [p5dir::R](p5dir::R) reader structure"]
impl crate::Readable for P5DIR {}
#[doc = "`write(|w| ..)` method takes [p5dir::W](p5dir::W) writer structure"]
impl crate::Writable for P5DIR {}
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "Port 5 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5ren](p5ren) module"]
pub type P5REN = crate::Reg<u8, _P5REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5REN;
#[doc = "`read()` method returns [p5ren::R](p5ren::R) reader structure"]
impl crate::Readable for P5REN {}
#[doc = "`write(|w| ..)` method takes [p5ren::W](p5ren::W) writer structure"]
impl crate::Writable for P5REN {}
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "Port 5 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5sel0](p5sel0) module"]
pub type P5SEL0 = crate::Reg<u8, _P5SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5SEL0;
#[doc = "`read()` method returns [p5sel0::R](p5sel0::R) reader structure"]
impl crate::Readable for P5SEL0 {}
#[doc = "`write(|w| ..)` method takes [p5sel0::W](p5sel0::W) writer structure"]
impl crate::Writable for P5SEL0 {}
#[doc = "Port 5 Select 0"]
pub mod p5sel0;
#[doc = "Port 5 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5sel1](p5sel1) module"]
pub type P5SEL1 = crate::Reg<u8, _P5SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5SEL1;
#[doc = "`read()` method returns [p5sel1::R](p5sel1::R) reader structure"]
impl crate::Readable for P5SEL1 {}
#[doc = "`write(|w| ..)` method takes [p5sel1::W](p5sel1::W) writer structure"]
impl crate::Writable for P5SEL1 {}
#[doc = "Port 5 Select 1"]
pub mod p5sel1;
#[doc = "Port 5 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5selc](p5selc) module"]
pub type P5SELC = crate::Reg<u8, _P5SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5SELC;
#[doc = "`read()` method returns [p5selc::R](p5selc::R) reader structure"]
impl crate::Readable for P5SELC {}
#[doc = "`write(|w| ..)` method takes [p5selc::W](p5selc::W) writer structure"]
impl crate::Writable for P5SELC {}
#[doc = "Port 5 Complement Select"]
pub mod p5selc;
#[doc = "Port 5 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5ies](p5ies) module"]
pub type P5IES = crate::Reg<u8, _P5IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IES;
#[doc = "`read()` method returns [p5ies::R](p5ies::R) reader structure"]
impl crate::Readable for P5IES {}
#[doc = "`write(|w| ..)` method takes [p5ies::W](p5ies::W) writer structure"]
impl crate::Writable for P5IES {}
#[doc = "Port 5 Interrupt Edge Select"]
pub mod p5ies;
#[doc = "Port 5 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5ie](p5ie) module"]
pub type P5IE = crate::Reg<u8, _P5IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IE;
#[doc = "`read()` method returns [p5ie::R](p5ie::R) reader structure"]
impl crate::Readable for P5IE {}
#[doc = "`write(|w| ..)` method takes [p5ie::W](p5ie::W) writer structure"]
impl crate::Writable for P5IE {}
#[doc = "Port 5 Interrupt Enable"]
pub mod p5ie;
#[doc = "Port 5 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5ifg](p5ifg) module"]
pub type P5IFG = crate::Reg<u8, _P5IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IFG;
#[doc = "`read()` method returns [p5ifg::R](p5ifg::R) reader structure"]
impl crate::Readable for P5IFG {}
#[doc = "`write(|w| ..)` method takes [p5ifg::W](p5ifg::W) writer structure"]
impl crate::Writable for P5IFG {}
#[doc = "Port 5 Interrupt Flag"]
pub mod p5ifg;
#[doc = "Port 5 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p5iv](p5iv) module"]
pub type P5IV = crate::Reg<u16, _P5IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P5IV;
#[doc = "`read()` method returns [p5iv::R](p5iv::R) reader structure"]
impl crate::Readable for P5IV {}
#[doc = "`write(|w| ..)` method takes [p5iv::W](p5iv::W) writer structure"]
impl crate::Writable for P5IV {}
#[doc = "Port 5 Interrupt Vector Register"]
pub mod p5iv;
