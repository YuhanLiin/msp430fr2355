#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control"]
    pub sysctl: SYSCTL,
    #[doc = "0x02 - Bootstrap Loader Configuration Register"]
    pub sysbslc: SYSBSLC,
    _reserved2: [u8; 2usize],
    #[doc = "0x06 - JTAG Mailbox Control"]
    pub sysjmbc: SYSJMBC,
    #[doc = "0x08 - JTAG Mailbox Input"]
    pub sysjmbi0: SYSJMBI0,
    #[doc = "0x0a - JTAG Mailbox Input 1 Register"]
    pub sysjmbi1: SYSJMBI1,
    #[doc = "0x0c - JTAG Mailbox Output"]
    pub sysjmbo0: SYSJMBO0,
    #[doc = "0x0e - JTAG Mailbox Output 1 Register"]
    pub sysjmbo1: SYSJMBO1,
    _reserved7: [u8; 10usize],
    #[doc = "0x1a - User NMI Vector Generator"]
    pub sysuniv: SYSUNIV,
    #[doc = "0x1c - System NMI Vector Generator"]
    pub syssniv: SYSSNIV,
    #[doc = "0x1e - Reset Vector Generator"]
    pub sysrstiv: SYSRSTIV,
    #[doc = "0x20 - System Configuration Register 0"]
    pub syscfg0: SYSCFG0,
    #[doc = "0x22 - System Configuration Register 1"]
    pub syscfg1: SYSCFG1,
    #[doc = "0x24 - System Configuration Register 2"]
    pub syscfg2: SYSCFG2,
    #[doc = "0x26 - System Configuration Register 3"]
    pub syscfg3: SYSCFG3,
}
#[doc = "System Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysctl](sysctl) module"]
pub type SYSCTL = crate::Reg<u16, _SYSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCTL;
#[doc = "`read()` method returns [sysctl::R](sysctl::R) reader structure"]
impl crate::Readable for SYSCTL {}
#[doc = "`write(|w| ..)` method takes [sysctl::W](sysctl::W) writer structure"]
impl crate::Writable for SYSCTL {}
#[doc = "System Control"]
pub mod sysctl;
#[doc = "Bootstrap Loader Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysbslc](sysbslc) module"]
pub type SYSBSLC = crate::Reg<u16, _SYSBSLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSBSLC;
#[doc = "`read()` method returns [sysbslc::R](sysbslc::R) reader structure"]
impl crate::Readable for SYSBSLC {}
#[doc = "`write(|w| ..)` method takes [sysbslc::W](sysbslc::W) writer structure"]
impl crate::Writable for SYSBSLC {}
#[doc = "Bootstrap Loader Configuration Register"]
pub mod sysbslc;
#[doc = "JTAG Mailbox Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysjmbc](sysjmbc) module"]
pub type SYSJMBC = crate::Reg<u16, _SYSJMBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBC;
#[doc = "`read()` method returns [sysjmbc::R](sysjmbc::R) reader structure"]
impl crate::Readable for SYSJMBC {}
#[doc = "`write(|w| ..)` method takes [sysjmbc::W](sysjmbc::W) writer structure"]
impl crate::Writable for SYSJMBC {}
#[doc = "JTAG Mailbox Control"]
pub mod sysjmbc;
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
#[doc = "JTAG Mailbox Input 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysjmbi1](sysjmbi1) module"]
pub type SYSJMBI1 = crate::Reg<u16, _SYSJMBI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBI1;
#[doc = "`read()` method returns [sysjmbi1::R](sysjmbi1::R) reader structure"]
impl crate::Readable for SYSJMBI1 {}
#[doc = "`write(|w| ..)` method takes [sysjmbi1::W](sysjmbi1::W) writer structure"]
impl crate::Writable for SYSJMBI1 {}
#[doc = "JTAG Mailbox Input 1 Register"]
pub mod sysjmbi1;
#[doc = "JTAG Mailbox Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysjmbo0](sysjmbo0) module"]
pub type SYSJMBO0 = crate::Reg<u16, _SYSJMBO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSJMBO0;
#[doc = "`read()` method returns [sysjmbo0::R](sysjmbo0::R) reader structure"]
impl crate::Readable for SYSJMBO0 {}
#[doc = "`write(|w| ..)` method takes [sysjmbo0::W](sysjmbo0::W) writer structure"]
impl crate::Writable for SYSJMBO0 {}
#[doc = "JTAG Mailbox Output"]
pub mod sysjmbo0;
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
#[doc = "User NMI Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysuniv](sysuniv) module"]
pub type SYSUNIV = crate::Reg<u16, _SYSUNIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSUNIV;
#[doc = "`read()` method returns [sysuniv::R](sysuniv::R) reader structure"]
impl crate::Readable for SYSUNIV {}
#[doc = "User NMI Vector Generator"]
pub mod sysuniv;
#[doc = "System NMI Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syssniv](syssniv) module"]
pub type SYSSNIV = crate::Reg<u16, _SYSSNIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSSNIV;
#[doc = "`read()` method returns [syssniv::R](syssniv::R) reader structure"]
impl crate::Readable for SYSSNIV {}
#[doc = "System NMI Vector Generator"]
pub mod syssniv;
#[doc = "Reset Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sysrstiv](sysrstiv) module"]
pub type SYSRSTIV = crate::Reg<u16, _SYSRSTIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSRSTIV;
#[doc = "`read()` method returns [sysrstiv::R](sysrstiv::R) reader structure"]
impl crate::Readable for SYSRSTIV {}
#[doc = "Reset Vector Generator"]
pub mod sysrstiv;
#[doc = "System Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscfg0](syscfg0) module"]
pub type SYSCFG0 = crate::Reg<u16, _SYSCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG0;
#[doc = "`read()` method returns [syscfg0::R](syscfg0::R) reader structure"]
impl crate::Readable for SYSCFG0 {}
#[doc = "`write(|w| ..)` method takes [syscfg0::W](syscfg0::W) writer structure"]
impl crate::Writable for SYSCFG0 {}
#[doc = "System Configuration Register 0"]
pub mod syscfg0;
#[doc = "System Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscfg1](syscfg1) module"]
pub type SYSCFG1 = crate::Reg<u16, _SYSCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG1;
#[doc = "`read()` method returns [syscfg1::R](syscfg1::R) reader structure"]
impl crate::Readable for SYSCFG1 {}
#[doc = "`write(|w| ..)` method takes [syscfg1::W](syscfg1::W) writer structure"]
impl crate::Writable for SYSCFG1 {}
#[doc = "System Configuration Register 1"]
pub mod syscfg1;
#[doc = "System Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscfg2](syscfg2) module"]
pub type SYSCFG2 = crate::Reg<u16, _SYSCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG2;
#[doc = "`read()` method returns [syscfg2::R](syscfg2::R) reader structure"]
impl crate::Readable for SYSCFG2 {}
#[doc = "`write(|w| ..)` method takes [syscfg2::W](syscfg2::W) writer structure"]
impl crate::Writable for SYSCFG2 {}
#[doc = "System Configuration Register 2"]
pub mod syscfg2;
#[doc = "System Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syscfg3](syscfg3) module"]
pub type SYSCFG3 = crate::Reg<u16, _SYSCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG3;
#[doc = "`read()` method returns [syscfg3::R](syscfg3::R) reader structure"]
impl crate::Readable for SYSCFG3 {}
#[doc = "`write(|w| ..)` method takes [syscfg3::W](syscfg3::W) writer structure"]
impl crate::Writable for SYSCFG3 {}
#[doc = "System Configuration Register 3"]
pub mod syscfg3;
