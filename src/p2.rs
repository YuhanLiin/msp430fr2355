#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Port 2 Input"]
    pub p2in: P2IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x03 - Port 2 Output"]
    pub p2out: P2OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x05 - Port 2 Direction"]
    pub p2dir: P2DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x07 - Port 2 Resistor Enable"]
    pub p2ren: P2REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0b - Port 2 Select 0"]
    pub p2sel0: P2SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0d - Port 2 Select 1"]
    pub p2sel1: P2SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x17 - Port 2 Complement Select"]
    pub p2selc: P2SELC,
    _reserved7: [u8; 1usize],
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    pub p2ies: P2IES,
    _reserved8: [u8; 1usize],
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    pub p2ie: P2IE,
    _reserved9: [u8; 1usize],
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    pub p2ifg: P2IFG,
    #[doc = "0x1e - Port 2 Interrupt Vector Register"]
    pub p2iv: P2IV,
}
#[doc = "Port 2 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2in](p2in) module"]
pub type P2IN = crate::Reg<u8, _P2IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IN;
#[doc = "`read()` method returns [p2in::R](p2in::R) reader structure"]
impl crate::Readable for P2IN {}
#[doc = "`write(|w| ..)` method takes [p2in::W](p2in::W) writer structure"]
impl crate::Writable for P2IN {}
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "Port 2 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2out](p2out) module"]
pub type P2OUT = crate::Reg<u8, _P2OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2OUT;
#[doc = "`read()` method returns [p2out::R](p2out::R) reader structure"]
impl crate::Readable for P2OUT {}
#[doc = "`write(|w| ..)` method takes [p2out::W](p2out::W) writer structure"]
impl crate::Writable for P2OUT {}
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "Port 2 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2dir](p2dir) module"]
pub type P2DIR = crate::Reg<u8, _P2DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2DIR;
#[doc = "`read()` method returns [p2dir::R](p2dir::R) reader structure"]
impl crate::Readable for P2DIR {}
#[doc = "`write(|w| ..)` method takes [p2dir::W](p2dir::W) writer structure"]
impl crate::Writable for P2DIR {}
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "Port 2 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2ren](p2ren) module"]
pub type P2REN = crate::Reg<u8, _P2REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2REN;
#[doc = "`read()` method returns [p2ren::R](p2ren::R) reader structure"]
impl crate::Readable for P2REN {}
#[doc = "`write(|w| ..)` method takes [p2ren::W](p2ren::W) writer structure"]
impl crate::Writable for P2REN {}
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "Port 2 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2sel0](p2sel0) module"]
pub type P2SEL0 = crate::Reg<u8, _P2SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2SEL0;
#[doc = "`read()` method returns [p2sel0::R](p2sel0::R) reader structure"]
impl crate::Readable for P2SEL0 {}
#[doc = "`write(|w| ..)` method takes [p2sel0::W](p2sel0::W) writer structure"]
impl crate::Writable for P2SEL0 {}
#[doc = "Port 2 Select 0"]
pub mod p2sel0;
#[doc = "Port 2 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2sel1](p2sel1) module"]
pub type P2SEL1 = crate::Reg<u8, _P2SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2SEL1;
#[doc = "`read()` method returns [p2sel1::R](p2sel1::R) reader structure"]
impl crate::Readable for P2SEL1 {}
#[doc = "`write(|w| ..)` method takes [p2sel1::W](p2sel1::W) writer structure"]
impl crate::Writable for P2SEL1 {}
#[doc = "Port 2 Select 1"]
pub mod p2sel1;
#[doc = "Port 2 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2selc](p2selc) module"]
pub type P2SELC = crate::Reg<u8, _P2SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2SELC;
#[doc = "`read()` method returns [p2selc::R](p2selc::R) reader structure"]
impl crate::Readable for P2SELC {}
#[doc = "`write(|w| ..)` method takes [p2selc::W](p2selc::W) writer structure"]
impl crate::Writable for P2SELC {}
#[doc = "Port 2 Complement Select"]
pub mod p2selc;
#[doc = "Port 2 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2ies](p2ies) module"]
pub type P2IES = crate::Reg<u8, _P2IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IES;
#[doc = "`read()` method returns [p2ies::R](p2ies::R) reader structure"]
impl crate::Readable for P2IES {}
#[doc = "`write(|w| ..)` method takes [p2ies::W](p2ies::W) writer structure"]
impl crate::Writable for P2IES {}
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "Port 2 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2ie](p2ie) module"]
pub type P2IE = crate::Reg<u8, _P2IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IE;
#[doc = "`read()` method returns [p2ie::R](p2ie::R) reader structure"]
impl crate::Readable for P2IE {}
#[doc = "`write(|w| ..)` method takes [p2ie::W](p2ie::W) writer structure"]
impl crate::Writable for P2IE {}
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "Port 2 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2ifg](p2ifg) module"]
pub type P2IFG = crate::Reg<u8, _P2IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IFG;
#[doc = "`read()` method returns [p2ifg::R](p2ifg::R) reader structure"]
impl crate::Readable for P2IFG {}
#[doc = "`write(|w| ..)` method takes [p2ifg::W](p2ifg::W) writer structure"]
impl crate::Writable for P2IFG {}
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "Port 2 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p2iv](p2iv) module"]
pub type P2IV = crate::Reg<u16, _P2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P2IV;
#[doc = "`read()` method returns [p2iv::R](p2iv::R) reader structure"]
impl crate::Readable for P2IV {}
#[doc = "`write(|w| ..)` method takes [p2iv::W](p2iv::W) writer structure"]
impl crate::Writable for P2IV {}
#[doc = "Port 2 Interrupt Vector Register"]
pub mod p2iv;
