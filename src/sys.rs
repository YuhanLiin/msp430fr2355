#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JTAG Mailbox Input"]
    pub sysjmbi0: SYSJMBI0,
    _reserved1: [u8; 4usize],
    #[doc = "0x06 - JTAG Mailbox Output 1 Register"]
    pub sysjmbo1: SYSJMBO1,
    _reserved2: [u8; 14usize],
    #[doc = "0x16 - Reset Vector Generator"]
    pub sysrstiv: SYSRSTIV,
}
#[doc = "JTAG Mailbox Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysjmbi0](sysjmbi0) module"]
pub type SYSJMBI0 = crate::Reg<u16, _SYSJMBI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBI0;
#[doc = "`read()` method returns [sysjmbi0::R](sysjmbi0::R) reader structure"]
impl crate::Readable for SYSJMBI0 {}
#[doc = "`write(|w| ..)` method takes [sysjmbi0::W](sysjmbi0::W) writer structure"]
impl crate::Writable for SYSJMBI0 {}
#[doc = "JTAG Mailbox Input"]
pub mod sysjmbi0;
#[doc = "JTAG Mailbox Output 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysjmbo1](sysjmbo1) module"]
pub type SYSJMBO1 = crate::Reg<u16, _SYSJMBO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBO1;
#[doc = "`read()` method returns [sysjmbo1::R](sysjmbo1::R) reader structure"]
impl crate::Readable for SYSJMBO1 {}
#[doc = "`write(|w| ..)` method takes [sysjmbo1::W](sysjmbo1::W) writer structure"]
impl crate::Writable for SYSJMBO1 {}
#[doc = "JTAG Mailbox Output 1 Register"]
pub mod sysjmbo1;
#[doc = "Reset Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrstiv](sysrstiv) module"]
pub type SYSRSTIV = crate::Reg<u16, _SYSRSTIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSRSTIV;
#[doc = "`read()` method returns [sysrstiv::R](sysrstiv::R) reader structure"]
impl crate::Readable for SYSRSTIV {}
#[doc = "`write(|w| ..)` method takes [sysrstiv::W](sysrstiv::W) writer structure"]
impl crate::Writable for SYSRSTIV {}
#[doc = "Reset Vector Generator"]
pub mod sysrstiv;
