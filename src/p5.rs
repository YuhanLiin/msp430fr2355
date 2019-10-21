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
    _reserved6: [u8; 9usize],
    #[doc = "0x16 - Port 5 Complement Select"]
    pub p5selc: P5SELC,
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
