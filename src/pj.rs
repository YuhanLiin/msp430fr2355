#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    pub pjin: PJIN,
    #[doc = "0x02 - Port J Output"]
    pub pjout: PJOUT,
    #[doc = "0x04 - Port J Direction"]
    pub pjdir: PJDIR,
    #[doc = "0x06 - Port J Resistor Enable"]
    pub pjren: PJREN,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - Port J Select 0"]
    pub pjsel0: PJSEL0,
    #[doc = "0x0c - Port J Select 1"]
    pub pjsel1: PJSEL1,
    _reserved6: [u8; 8usize],
    #[doc = "0x16 - Port J Complement Select"]
    pub pjselc: PJSELC,
}
#[doc = "Port J Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjin](pjin) module"]
pub type PJIN = crate::Reg<u16, _PJIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJIN;
#[doc = "`read()` method returns [pjin::R](pjin::R) reader structure"]
impl crate::Readable for PJIN {}
#[doc = "`write(|w| ..)` method takes [pjin::W](pjin::W) writer structure"]
impl crate::Writable for PJIN {}
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "Port J Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjout](pjout) module"]
pub type PJOUT = crate::Reg<u16, _PJOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJOUT;
#[doc = "`read()` method returns [pjout::R](pjout::R) reader structure"]
impl crate::Readable for PJOUT {}
#[doc = "`write(|w| ..)` method takes [pjout::W](pjout::W) writer structure"]
impl crate::Writable for PJOUT {}
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "Port J Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjdir](pjdir) module"]
pub type PJDIR = crate::Reg<u16, _PJDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJDIR;
#[doc = "`read()` method returns [pjdir::R](pjdir::R) reader structure"]
impl crate::Readable for PJDIR {}
#[doc = "`write(|w| ..)` method takes [pjdir::W](pjdir::W) writer structure"]
impl crate::Writable for PJDIR {}
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "Port J Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjren](pjren) module"]
pub type PJREN = crate::Reg<u16, _PJREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJREN;
#[doc = "`read()` method returns [pjren::R](pjren::R) reader structure"]
impl crate::Readable for PJREN {}
#[doc = "`write(|w| ..)` method takes [pjren::W](pjren::W) writer structure"]
impl crate::Writable for PJREN {}
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "Port J Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjsel0](pjsel0) module"]
pub type PJSEL0 = crate::Reg<u16, _PJSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJSEL0;
#[doc = "`read()` method returns [pjsel0::R](pjsel0::R) reader structure"]
impl crate::Readable for PJSEL0 {}
#[doc = "`write(|w| ..)` method takes [pjsel0::W](pjsel0::W) writer structure"]
impl crate::Writable for PJSEL0 {}
#[doc = "Port J Select 0"]
pub mod pjsel0;
#[doc = "Port J Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjsel1](pjsel1) module"]
pub type PJSEL1 = crate::Reg<u16, _PJSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJSEL1;
#[doc = "`read()` method returns [pjsel1::R](pjsel1::R) reader structure"]
impl crate::Readable for PJSEL1 {}
#[doc = "`write(|w| ..)` method takes [pjsel1::W](pjsel1::W) writer structure"]
impl crate::Writable for PJSEL1 {}
#[doc = "Port J Select 1"]
pub mod pjsel1;
#[doc = "Port J Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pjselc](pjselc) module"]
pub type PJSELC = crate::Reg<u16, _PJSELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJSELC;
#[doc = "`read()` method returns [pjselc::R](pjselc::R) reader structure"]
impl crate::Readable for PJSELC {}
#[doc = "`write(|w| ..)` method takes [pjselc::W](pjselc::W) writer structure"]
impl crate::Writable for PJSELC {}
#[doc = "Port J Complement Select"]
pub mod pjselc;
