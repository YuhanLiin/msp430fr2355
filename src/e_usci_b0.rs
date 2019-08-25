#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb0ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucb0ctlw1: UCB0CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_ucb0: [u8; 2usize],
    _reserved_3_ucb0: [u8; 2usize],
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucb0tbcnt: UCB0TBCNT,
    _reserved_5_ucb0: [u8; 2usize],
    _reserved_6_ucb0: [u8; 2usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucb0i2coa0: UCB0I2COA0,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucb0i2coa1: UCB0I2COA1,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucb0i2coa2: UCB0I2COA2,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucb0i2coa3: UCB0I2COA3,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucb0addrx: UCB0ADDRX,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucb0addmask: UCB0ADDMASK,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucb0i2csa: UCB0I2CSA,
    _reserved14: [u8; 8usize],
    _reserved_14_ucb0: [u8; 2usize],
    _reserved_15_ucb0: [u8; 2usize],
    _reserved_16_ucb0: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb0ctlw0_spi(&self) -> &UCB0CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB0CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb0ctlw0_spi_mut(&self) -> &mut UCB0CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB0CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb0ctlw0(&self) -> &UCB0CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB0CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb0ctlw0_mut(&self) -> &mut UCB0CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB0CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb0brw_spi(&self) -> &UCB0BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB0BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb0brw_spi_mut(&self) -> &mut UCB0BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB0BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb0brw(&self) -> &UCB0BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB0BRW) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb0brw_mut(&self) -> &mut UCB0BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB0BRW) }
    }
    #[doc = "0x08 - UCB0STATW_SPI"]
    #[inline(always)]
    pub fn ucb0statw_spi(&self) -> &UCB0STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB0STATW_SPI) }
    }
    #[doc = "0x08 - UCB0STATW_SPI"]
    #[inline(always)]
    pub fn ucb0statw_spi_mut(&self) -> &mut UCB0STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB0STATW_SPI) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb0statw(&self) -> &UCB0STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB0STATW) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb0statw_mut(&self) -> &mut UCB0STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB0STATW) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb0rxbuf_spi(&self) -> &UCB0RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB0RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb0rxbuf_spi_mut(&self) -> &mut UCB0RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB0RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb0rxbuf(&self) -> &UCB0RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB0RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb0rxbuf_mut(&self) -> &mut UCB0RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB0RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf_spi(&self) -> &UCB0TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB0TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf_spi_mut(&self) -> &mut UCB0TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB0TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> &UCB0TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB0TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf_mut(&self) -> &mut UCB0TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB0TXBUF) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_spi(&self) -> &UCB0IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB0IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_spi_mut(&self) -> &mut UCB0IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB0IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie(&self) -> &UCB0IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB0IE) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_mut(&self) -> &mut UCB0IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB0IE) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb0ifg_spi(&self) -> &UCB0IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB0IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb0ifg_spi_mut(&self) -> &mut UCB0IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB0IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb0ifg(&self) -> &UCB0IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB0IFG) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb0ifg_mut(&self) -> &mut UCB0IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB0IFG) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv_spi(&self) -> &UCB0IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB0IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv_spi_mut(&self) -> &mut UCB0IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB0IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv(&self) -> &UCB0IV {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB0IV) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv_mut(&self) -> &mut UCB0IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB0IV) }
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ctlw0](ucb0ctlw0) module"]
pub type UCB0CTLW0 = crate::Reg<u16, _UCB0CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTLW0;
#[doc = "`read()` method returns [ucb0ctlw0::R](ucb0ctlw0::R) reader structure"]
impl crate::Readable for UCB0CTLW0 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctlw0::W](ucb0ctlw0::W) writer structure"]
impl crate::Writable for UCB0CTLW0 {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb0ctlw0;
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ctlw0_spi](ucb0ctlw0_spi) module"]
pub type UCB0CTLW0_SPI = crate::Reg<u16, _UCB0CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTLW0_SPI;
#[doc = "`read()` method returns [ucb0ctlw0_spi::R](ucb0ctlw0_spi::R) reader structure"]
impl crate::Readable for UCB0CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ctlw0_spi::W](ucb0ctlw0_spi::W) writer structure"]
impl crate::Writable for UCB0CTLW0_SPI {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb0ctlw0_spi;
#[doc = "eUSCI_Bx Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ctlw1](ucb0ctlw1) module"]
pub type UCB0CTLW1 = crate::Reg<u16, _UCB0CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0CTLW1;
#[doc = "`read()` method returns [ucb0ctlw1::R](ucb0ctlw1::R) reader structure"]
impl crate::Readable for UCB0CTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucb0ctlw1::W](ucb0ctlw1::W) writer structure"]
impl crate::Writable for UCB0CTLW1 {}
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0brw](ucb0brw) module"]
pub type UCB0BRW = crate::Reg<u16, _UCB0BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BRW;
#[doc = "`read()` method returns [ucb0brw::R](ucb0brw::R) reader structure"]
impl crate::Readable for UCB0BRW {}
#[doc = "`write(|w| ..)` method takes [ucb0brw::W](ucb0brw::W) writer structure"]
impl crate::Writable for UCB0BRW {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb0brw;
#[doc = "eUSCI_Bx Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0brw_spi](ucb0brw_spi) module"]
pub type UCB0BRW_SPI = crate::Reg<u16, _UCB0BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0BRW_SPI;
#[doc = "`read()` method returns [ucb0brw_spi::R](ucb0brw_spi::R) reader structure"]
impl crate::Readable for UCB0BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0brw_spi::W](ucb0brw_spi::W) writer structure"]
impl crate::Writable for UCB0BRW_SPI {}
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb0brw_spi;
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0statw](ucb0statw) module"]
pub type UCB0STATW = crate::Reg<u16, _UCB0STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0STATW;
#[doc = "`read()` method returns [ucb0statw::R](ucb0statw::R) reader structure"]
impl crate::Readable for UCB0STATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb0statw;
#[doc = "UCB0STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0statw_spi](ucb0statw_spi) module"]
pub type UCB0STATW_SPI = crate::Reg<u16, _UCB0STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0STATW_SPI;
#[doc = "`read()` method returns [ucb0statw_spi::R](ucb0statw_spi::R) reader structure"]
impl crate::Readable for UCB0STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0statw_spi::W](ucb0statw_spi::W) writer structure"]
impl crate::Writable for UCB0STATW_SPI {}
#[doc = "UCB0STATW_SPI"]
pub mod ucb0statw_spi;
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0tbcnt](ucb0tbcnt) module"]
pub type UCB0TBCNT = crate::Reg<u16, _UCB0TBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TBCNT;
#[doc = "`read()` method returns [ucb0tbcnt::R](ucb0tbcnt::R) reader structure"]
impl crate::Readable for UCB0TBCNT {}
#[doc = "`write(|w| ..)` method takes [ucb0tbcnt::W](ucb0tbcnt::W) writer structure"]
impl crate::Writable for UCB0TBCNT {}
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0rxbuf](ucb0rxbuf) module"]
pub type UCB0RXBUF = crate::Reg<u16, _UCB0RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0RXBUF;
#[doc = "`read()` method returns [ucb0rxbuf::R](ucb0rxbuf::R) reader structure"]
impl crate::Readable for UCB0RXBUF {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb0rxbuf;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0rxbuf_spi](ucb0rxbuf_spi) module"]
pub type UCB0RXBUF_SPI = crate::Reg<u16, _UCB0RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0RXBUF_SPI;
#[doc = "`read()` method returns [ucb0rxbuf_spi::R](ucb0rxbuf_spi::R) reader structure"]
impl crate::Readable for UCB0RXBUF_SPI {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb0rxbuf_spi;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0txbuf](ucb0txbuf) module"]
pub type UCB0TXBUF = crate::Reg<u16, _UCB0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TXBUF;
#[doc = "`read()` method returns [ucb0txbuf::R](ucb0txbuf::R) reader structure"]
impl crate::Readable for UCB0TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf::W](ucb0txbuf::W) writer structure"]
impl crate::Writable for UCB0TXBUF {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0txbuf_spi](ucb0txbuf_spi) module"]
pub type UCB0TXBUF_SPI = crate::Reg<u16, _UCB0TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TXBUF_SPI;
#[doc = "`read()` method returns [ucb0txbuf_spi::R](ucb0txbuf_spi::R) reader structure"]
impl crate::Readable for UCB0TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf_spi::W](ucb0txbuf_spi::W) writer structure"]
impl crate::Writable for UCB0TXBUF_SPI {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf_spi;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0i2coa0](ucb0i2coa0) module"]
pub type UCB0I2COA0 = crate::Reg<u16, _UCB0I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA0;
#[doc = "`read()` method returns [ucb0i2coa0::R](ucb0i2coa0::R) reader structure"]
impl crate::Readable for UCB0I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa0::W](ucb0i2coa0::W) writer structure"]
impl crate::Writable for UCB0I2COA0 {}
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb0i2coa0;
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0i2coa1](ucb0i2coa1) module"]
pub type UCB0I2COA1 = crate::Reg<u16, _UCB0I2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA1;
#[doc = "`read()` method returns [ucb0i2coa1::R](ucb0i2coa1::R) reader structure"]
impl crate::Readable for UCB0I2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa1::W](ucb0i2coa1::W) writer structure"]
impl crate::Writable for UCB0I2COA1 {}
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb0i2coa1;
#[doc = "eUSCI_Bx I2C Own Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0i2coa2](ucb0i2coa2) module"]
pub type UCB0I2COA2 = crate::Reg<u16, _UCB0I2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA2;
#[doc = "`read()` method returns [ucb0i2coa2::R](ucb0i2coa2::R) reader structure"]
impl crate::Readable for UCB0I2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa2::W](ucb0i2coa2::W) writer structure"]
impl crate::Writable for UCB0I2COA2 {}
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb0i2coa2;
#[doc = "eUSCI_Bx I2C Own Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0i2coa3](ucb0i2coa3) module"]
pub type UCB0I2COA3 = crate::Reg<u16, _UCB0I2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA3;
#[doc = "`read()` method returns [ucb0i2coa3::R](ucb0i2coa3::R) reader structure"]
impl crate::Readable for UCB0I2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa3::W](ucb0i2coa3::W) writer structure"]
impl crate::Writable for UCB0I2COA3 {}
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb0i2coa3;
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0addrx](ucb0addrx) module"]
pub type UCB0ADDRX = crate::Reg<u16, _UCB0ADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0ADDRX;
#[doc = "`read()` method returns [ucb0addrx::R](ucb0addrx::R) reader structure"]
impl crate::Readable for UCB0ADDRX {}
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb0addrx;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0addmask](ucb0addmask) module"]
pub type UCB0ADDMASK = crate::Reg<u16, _UCB0ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0ADDMASK;
#[doc = "`read()` method returns [ucb0addmask::R](ucb0addmask::R) reader structure"]
impl crate::Readable for UCB0ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb0addmask::W](ucb0addmask::W) writer structure"]
impl crate::Writable for UCB0ADDMASK {}
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb0addmask;
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0i2csa](ucb0i2csa) module"]
pub type UCB0I2CSA = crate::Reg<u16, _UCB0I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2CSA;
#[doc = "`read()` method returns [ucb0i2csa::R](ucb0i2csa::R) reader structure"]
impl crate::Readable for UCB0I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb0i2csa::W](ucb0i2csa::W) writer structure"]
impl crate::Writable for UCB0I2CSA {}
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb0i2csa;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ie](ucb0ie) module"]
pub type UCB0IE = crate::Reg<u16, _UCB0IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IE;
#[doc = "`read()` method returns [ucb0ie::R](ucb0ie::R) reader structure"]
impl crate::Readable for UCB0IE {}
#[doc = "`write(|w| ..)` method takes [ucb0ie::W](ucb0ie::W) writer structure"]
impl crate::Writable for UCB0IE {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ie_spi](ucb0ie_spi) module"]
pub type UCB0IE_SPI = crate::Reg<u16, _UCB0IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IE_SPI;
#[doc = "`read()` method returns [ucb0ie_spi::R](ucb0ie_spi::R) reader structure"]
impl crate::Readable for UCB0IE_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ie_spi::W](ucb0ie_spi::W) writer structure"]
impl crate::Writable for UCB0IE_SPI {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ifg](ucb0ifg) module"]
pub type UCB0IFG = crate::Reg<u16, _UCB0IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IFG;
#[doc = "`read()` method returns [ucb0ifg::R](ucb0ifg::R) reader structure"]
impl crate::Readable for UCB0IFG {}
#[doc = "`write(|w| ..)` method takes [ucb0ifg::W](ucb0ifg::W) writer structure"]
impl crate::Writable for UCB0IFG {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb0ifg;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0ifg_spi](ucb0ifg_spi) module"]
pub type UCB0IFG_SPI = crate::Reg<u16, _UCB0IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IFG_SPI;
#[doc = "`read()` method returns [ucb0ifg_spi::R](ucb0ifg_spi::R) reader structure"]
impl crate::Readable for UCB0IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb0ifg_spi::W](ucb0ifg_spi::W) writer structure"]
impl crate::Writable for UCB0IFG_SPI {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb0ifg_spi;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0iv](ucb0iv) module"]
pub type UCB0IV = crate::Reg<u16, _UCB0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IV;
#[doc = "`read()` method returns [ucb0iv::R](ucb0iv::R) reader structure"]
impl crate::Readable for UCB0IV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0iv_spi](ucb0iv_spi) module"]
pub type UCB0IV_SPI = crate::Reg<u16, _UCB0IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IV_SPI;
#[doc = "`read()` method returns [ucb0iv_spi::R](ucb0iv_spi::R) reader structure"]
impl crate::Readable for UCB0IV_SPI {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv_spi;
