#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    pub p1in: P1IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 1 Output"]
    pub p1out: P1OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 1 Direction"]
    pub p1dir: P1DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 1 Resistor Enable"]
    pub p1ren: P1REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 1 Select 0"]
    pub p1sel0: P1SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 1 Select 1"]
    pub p1sel1: P1SEL1,
    _reserved6: [u8; 1usize],
    #[doc = "0x0e - Port 1 Interrupt Vector Register"]
    pub p1iv: P1IV,
    _reserved7: [u8; 6usize],
    #[doc = "0x16 - Port 1 Complement Select"]
    pub p1selc: P1SELC,
    _reserved8: [u8; 1usize],
    #[doc = "0x18 - Port 1 Interrupt Edge Select"]
    pub p1ies: P1IES,
    _reserved9: [u8; 1usize],
    #[doc = "0x1a - Port 1 Interrupt Enable"]
    pub p1ie: P1IE,
    _reserved10: [u8; 1usize],
    #[doc = "0x1c - Port 1 Interrupt Flag"]
    pub p1ifg: P1IFG,
}
#[doc = "Port 1 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1in](p1in) module"]
pub type P1IN = crate::Reg<u8, _P1IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IN;
#[doc = "`read()` method returns [p1in::R](p1in::R) reader structure"]
impl crate::Readable for P1IN {}
#[doc = "`write(|w| ..)` method takes [p1in::W](p1in::W) writer structure"]
impl crate::Writable for P1IN {}
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "Port 1 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1out](p1out) module"]
pub type P1OUT = crate::Reg<u8, _P1OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1OUT;
#[doc = "`read()` method returns [p1out::R](p1out::R) reader structure"]
impl crate::Readable for P1OUT {}
#[doc = "`write(|w| ..)` method takes [p1out::W](p1out::W) writer structure"]
impl crate::Writable for P1OUT {}
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "Port 1 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1dir](p1dir) module"]
pub type P1DIR = crate::Reg<u8, _P1DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1DIR;
#[doc = "`read()` method returns [p1dir::R](p1dir::R) reader structure"]
impl crate::Readable for P1DIR {}
#[doc = "`write(|w| ..)` method takes [p1dir::W](p1dir::W) writer structure"]
impl crate::Writable for P1DIR {}
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "Port 1 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1ren](p1ren) module"]
pub type P1REN = crate::Reg<u8, _P1REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1REN;
#[doc = "`read()` method returns [p1ren::R](p1ren::R) reader structure"]
impl crate::Readable for P1REN {}
#[doc = "`write(|w| ..)` method takes [p1ren::W](p1ren::W) writer structure"]
impl crate::Writable for P1REN {}
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "Port 1 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1sel0](p1sel0) module"]
pub type P1SEL0 = crate::Reg<u8, _P1SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1SEL0;
#[doc = "`read()` method returns [p1sel0::R](p1sel0::R) reader structure"]
impl crate::Readable for P1SEL0 {}
#[doc = "`write(|w| ..)` method takes [p1sel0::W](p1sel0::W) writer structure"]
impl crate::Writable for P1SEL0 {}
#[doc = "Port 1 Select 0"]
pub mod p1sel0;
#[doc = "Port 1 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1sel1](p1sel1) module"]
pub type P1SEL1 = crate::Reg<u8, _P1SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1SEL1;
#[doc = "`read()` method returns [p1sel1::R](p1sel1::R) reader structure"]
impl crate::Readable for P1SEL1 {}
#[doc = "`write(|w| ..)` method takes [p1sel1::W](p1sel1::W) writer structure"]
impl crate::Writable for P1SEL1 {}
#[doc = "Port 1 Select 1"]
pub mod p1sel1;
#[doc = "Port 1 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1selc](p1selc) module"]
pub type P1SELC = crate::Reg<u8, _P1SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1SELC;
#[doc = "`read()` method returns [p1selc::R](p1selc::R) reader structure"]
impl crate::Readable for P1SELC {}
#[doc = "`write(|w| ..)` method takes [p1selc::W](p1selc::W) writer structure"]
impl crate::Writable for P1SELC {}
#[doc = "Port 1 Complement Select"]
pub mod p1selc;
#[doc = "Port 1 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1ies](p1ies) module"]
pub type P1IES = crate::Reg<u8, _P1IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IES;
#[doc = "`read()` method returns [p1ies::R](p1ies::R) reader structure"]
impl crate::Readable for P1IES {}
#[doc = "`write(|w| ..)` method takes [p1ies::W](p1ies::W) writer structure"]
impl crate::Writable for P1IES {}
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "Port 1 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1ie](p1ie) module"]
pub type P1IE = crate::Reg<u8, _P1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IE;
#[doc = "`read()` method returns [p1ie::R](p1ie::R) reader structure"]
impl crate::Readable for P1IE {}
#[doc = "`write(|w| ..)` method takes [p1ie::W](p1ie::W) writer structure"]
impl crate::Writable for P1IE {}
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "Port 1 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1ifg](p1ifg) module"]
pub type P1IFG = crate::Reg<u8, _P1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IFG;
#[doc = "`read()` method returns [p1ifg::R](p1ifg::R) reader structure"]
impl crate::Readable for P1IFG {}
#[doc = "`write(|w| ..)` method takes [p1ifg::W](p1ifg::W) writer structure"]
impl crate::Writable for P1IFG {}
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "Port 1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p1iv](p1iv) module"]
pub type P1IV = crate::Reg<u16, _P1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P1IV;
#[doc = "`read()` method returns [p1iv::R](p1iv::R) reader structure"]
impl crate::Readable for P1IV {}
#[doc = "`write(|w| ..)` method takes [p1iv::W](p1iv::W) writer structure"]
impl crate::Writable for P1IV {}
#[doc = "Port 1 Interrupt Vector Register"]
pub mod p1iv;
