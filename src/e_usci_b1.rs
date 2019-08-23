#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb1: [u8; 2usize],
    _reserved1: [u8; 4usize],
    _reserved_1_ucb1: [u8; 2usize],
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucb1i2coa0: UCB1I2COA0,
    _reserved3: [u8; 8usize],
    #[doc = "0x16 - eUSCI_Bx I2C Address Mask Register"]
    pub ucb1addmask: UCB1ADDMASK,
    _reserved4: [u8; 14usize],
    _reserved_4_ucb1: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - UCB1STATW_SPI"]
    #[inline(always)]
    pub fn ucb1statw(&self) -> &UCB1STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB1STATW) }
    }
    #[doc = "0x00 - UCB1STATW_SPI"]
    #[inline(always)]
    pub fn ucb1statw_mut(&self) -> &mut UCB1STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB1STATW) }
    }
    #[doc = "0x06 - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf(&self) -> &UCB1TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB1TXBUF) }
    }
    #[doc = "0x06 - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf_mut(&self) -> &mut UCB1TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB1TXBUF) }
    }
    #[doc = "0x26 - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv(&self) -> &UCB1IV {
        unsafe { &*(((self as *const Self) as *const u8).add(38usize) as *const UCB1IV) }
    }
    #[doc = "0x26 - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv_mut(&self) -> &mut UCB1IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(38usize) as *mut UCB1IV) }
    }
}
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb1statw](ucb1statw) module"]
pub type UCB1STATW = crate::Reg<u16, _UCB1STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1STATW;
#[doc = "`read()` method returns [ucb1statw::R](ucb1statw::R) reader structure"]
impl crate::Readable for UCB1STATW {}
#[doc = "`write(|w| ..)` method takes [ucb1statw::W](ucb1statw::W) writer structure"]
impl crate::Writable for UCB1STATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb1statw;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb1txbuf](ucb1txbuf) module"]
pub type UCB1TXBUF = crate::Reg<u16, _UCB1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TXBUF;
#[doc = "`read()` method returns [ucb1txbuf::R](ucb1txbuf::R) reader structure"]
impl crate::Readable for UCB1TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb1txbuf::W](ucb1txbuf::W) writer structure"]
impl crate::Writable for UCB1TXBUF {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb1txbuf;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb1i2coa0](ucb1i2coa0) module"]
pub type UCB1I2COA0 = crate::Reg<u16, _UCB1I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA0;
#[doc = "`read()` method returns [ucb1i2coa0::R](ucb1i2coa0::R) reader structure"]
impl crate::Readable for UCB1I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa0::W](ucb1i2coa0::W) writer structure"]
impl crate::Writable for UCB1I2COA0 {}
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb1i2coa0;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb1addmask](ucb1addmask) module"]
pub type UCB1ADDMASK = crate::Reg<u16, _UCB1ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1ADDMASK;
#[doc = "`read()` method returns [ucb1addmask::R](ucb1addmask::R) reader structure"]
impl crate::Readable for UCB1ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb1addmask::W](ucb1addmask::W) writer structure"]
impl crate::Writable for UCB1ADDMASK {}
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb1addmask;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb1iv](ucb1iv) module"]
pub type UCB1IV = crate::Reg<u16, _UCB1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IV;
#[doc = "`read()` method returns [ucb1iv::R](ucb1iv::R) reader structure"]
impl crate::Readable for UCB1IV {}
#[doc = "`write(|w| ..)` method takes [ucb1iv::W](ucb1iv::W) writer structure"]
impl crate::Writable for UCB1IV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv;
