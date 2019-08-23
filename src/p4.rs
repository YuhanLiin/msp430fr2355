#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Port 4 Input"]
    pub p4in: P4IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x03 - Port 4 Output"]
    pub p4out: P4OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x05 - Port 4 Direction"]
    pub p4dir: P4DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x07 - Port 4 Resistor Enable"]
    pub p4ren: P4REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0b - Port 4 Select 0"]
    pub p4sel0: P4SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0d - Port 4 Select 1"]
    pub p4sel1: P4SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x17 - Port 4 Complement Select"]
    pub p4selc: P4SELC,
    _reserved7: [u8; 1usize],
    #[doc = "0x19 - Port 4 Interrupt Edge Select"]
    pub p4ies: P4IES,
    _reserved8: [u8; 1usize],
    #[doc = "0x1b - Port 4 Interrupt Enable"]
    pub p4ie: P4IE,
    _reserved9: [u8; 1usize],
    #[doc = "0x1d - Port 4 Interrupt Flag"]
    pub p4ifg: P4IFG,
    #[doc = "0x1e - Port 4 Interrupt Vector Register"]
    pub p4iv: P4IV,
}
#[doc = "Port 4 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4in](p4in) module"]
pub type P4IN = crate::Reg<u8, _P4IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IN;
#[doc = "`read()` method returns [p4in::R](p4in::R) reader structure"]
impl crate::Readable for P4IN {}
#[doc = "`write(|w| ..)` method takes [p4in::W](p4in::W) writer structure"]
impl crate::Writable for P4IN {}
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "Port 4 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4out](p4out) module"]
pub type P4OUT = crate::Reg<u8, _P4OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4OUT;
#[doc = "`read()` method returns [p4out::R](p4out::R) reader structure"]
impl crate::Readable for P4OUT {}
#[doc = "`write(|w| ..)` method takes [p4out::W](p4out::W) writer structure"]
impl crate::Writable for P4OUT {}
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "Port 4 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4dir](p4dir) module"]
pub type P4DIR = crate::Reg<u8, _P4DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4DIR;
#[doc = "`read()` method returns [p4dir::R](p4dir::R) reader structure"]
impl crate::Readable for P4DIR {}
#[doc = "`write(|w| ..)` method takes [p4dir::W](p4dir::W) writer structure"]
impl crate::Writable for P4DIR {}
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "Port 4 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4ren](p4ren) module"]
pub type P4REN = crate::Reg<u8, _P4REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4REN;
#[doc = "`read()` method returns [p4ren::R](p4ren::R) reader structure"]
impl crate::Readable for P4REN {}
#[doc = "`write(|w| ..)` method takes [p4ren::W](p4ren::W) writer structure"]
impl crate::Writable for P4REN {}
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "Port 4 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4sel0](p4sel0) module"]
pub type P4SEL0 = crate::Reg<u8, _P4SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SEL0;
#[doc = "`read()` method returns [p4sel0::R](p4sel0::R) reader structure"]
impl crate::Readable for P4SEL0 {}
#[doc = "`write(|w| ..)` method takes [p4sel0::W](p4sel0::W) writer structure"]
impl crate::Writable for P4SEL0 {}
#[doc = "Port 4 Select 0"]
pub mod p4sel0;
#[doc = "Port 4 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4sel1](p4sel1) module"]
pub type P4SEL1 = crate::Reg<u8, _P4SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SEL1;
#[doc = "`read()` method returns [p4sel1::R](p4sel1::R) reader structure"]
impl crate::Readable for P4SEL1 {}
#[doc = "`write(|w| ..)` method takes [p4sel1::W](p4sel1::W) writer structure"]
impl crate::Writable for P4SEL1 {}
#[doc = "Port 4 Select 1"]
pub mod p4sel1;
#[doc = "Port 4 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4selc](p4selc) module"]
pub type P4SELC = crate::Reg<u8, _P4SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4SELC;
#[doc = "`read()` method returns [p4selc::R](p4selc::R) reader structure"]
impl crate::Readable for P4SELC {}
#[doc = "`write(|w| ..)` method takes [p4selc::W](p4selc::W) writer structure"]
impl crate::Writable for P4SELC {}
#[doc = "Port 4 Complement Select"]
pub mod p4selc;
#[doc = "Port 4 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4ies](p4ies) module"]
pub type P4IES = crate::Reg<u8, _P4IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IES;
#[doc = "`read()` method returns [p4ies::R](p4ies::R) reader structure"]
impl crate::Readable for P4IES {}
#[doc = "`write(|w| ..)` method takes [p4ies::W](p4ies::W) writer structure"]
impl crate::Writable for P4IES {}
#[doc = "Port 4 Interrupt Edge Select"]
pub mod p4ies;
#[doc = "Port 4 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4ie](p4ie) module"]
pub type P4IE = crate::Reg<u8, _P4IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IE;
#[doc = "`read()` method returns [p4ie::R](p4ie::R) reader structure"]
impl crate::Readable for P4IE {}
#[doc = "`write(|w| ..)` method takes [p4ie::W](p4ie::W) writer structure"]
impl crate::Writable for P4IE {}
#[doc = "Port 4 Interrupt Enable"]
pub mod p4ie;
#[doc = "Port 4 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4ifg](p4ifg) module"]
pub type P4IFG = crate::Reg<u8, _P4IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IFG;
#[doc = "`read()` method returns [p4ifg::R](p4ifg::R) reader structure"]
impl crate::Readable for P4IFG {}
#[doc = "`write(|w| ..)` method takes [p4ifg::W](p4ifg::W) writer structure"]
impl crate::Writable for P4IFG {}
#[doc = "Port 4 Interrupt Flag"]
pub mod p4ifg;
#[doc = "Port 4 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p4iv](p4iv) module"]
pub type P4IV = crate::Reg<u16, _P4IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P4IV;
#[doc = "`read()` method returns [p4iv::R](p4iv::R) reader structure"]
impl crate::Readable for P4IV {}
#[doc = "`write(|w| ..)` method takes [p4iv::W](p4iv::W) writer structure"]
impl crate::Writable for P4IV {}
#[doc = "Port 4 Interrupt Vector Register"]
pub mod p4iv;
