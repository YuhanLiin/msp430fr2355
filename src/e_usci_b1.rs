#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb1ctlw: [u8; 2usize],
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucb1ctlw1: UCB1CTLW1,
    _reserved2: [u8; 2usize],
    _reserved_2_ucb1: [u8; 2usize],
    _reserved_3_ucb1: [u8; 2usize],
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucb1tbcnt: UCB1TBCNT,
    _reserved_5_ucb1: [u8; 2usize],
    _reserved_6_ucb1: [u8; 2usize],
    _reserved7: [u8; 4usize],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucb1i2coa0: UCB1I2COA0,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucb1i2coa1: UCB1I2COA1,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucb1i2coa2: UCB1I2COA2,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucb1i2coa3: UCB1I2COA3,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucb1addrx: UCB1ADDRX,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucb1addmask: UCB1ADDMASK,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucb1i2csa: UCB1I2CSA,
    _reserved14: [u8; 8usize],
    _reserved_14_ucb1: [u8; 2usize],
    _reserved_15_ucb1: [u8; 2usize],
    _reserved_16_ucb1: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb1ctlw0_spi(&self) -> &UCB1CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB1CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb1ctlw0_spi_mut(&self) -> &mut UCB1CTLW0_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB1CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb1ctlw0(&self) -> &UCB1CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB1CTLW0) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb1ctlw0_mut(&self) -> &mut UCB1CTLW0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB1CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb1brw_spi(&self) -> &UCB1BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB1BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb1brw_spi_mut(&self) -> &mut UCB1BRW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB1BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb1brw(&self) -> &UCB1BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB1BRW) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb1brw_mut(&self) -> &mut UCB1BRW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB1BRW) }
    }
    #[doc = "0x08 - UCB1STATW_SPI"]
    #[inline(always)]
    pub fn ucb1statw_spi(&self) -> &UCB1STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB1STATW_SPI) }
    }
    #[doc = "0x08 - UCB1STATW_SPI"]
    #[inline(always)]
    pub fn ucb1statw_spi_mut(&self) -> &mut UCB1STATW_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB1STATW_SPI) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb1statw(&self) -> &UCB1STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB1STATW) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb1statw_mut(&self) -> &mut UCB1STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut UCB1STATW) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb1rxbuf_spi(&self) -> &UCB1RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB1RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb1rxbuf_spi_mut(&self) -> &mut UCB1RXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB1RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb1rxbuf(&self) -> &UCB1RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB1RXBUF) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb1rxbuf_mut(&self) -> &mut UCB1RXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut UCB1RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf_spi(&self) -> &UCB1TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB1TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf_spi_mut(&self) -> &mut UCB1TXBUF_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB1TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf(&self) -> &UCB1TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB1TXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf_mut(&self) -> &mut UCB1TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(14usize) as *mut UCB1TXBUF) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_spi(&self) -> &UCB1IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB1IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_spi_mut(&self) -> &mut UCB1IE_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB1IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie(&self) -> &UCB1IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB1IE) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_mut(&self) -> &mut UCB1IE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(42usize) as *mut UCB1IE) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb1ifg_spi(&self) -> &UCB1IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB1IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb1ifg_spi_mut(&self) -> &mut UCB1IFG_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB1IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb1ifg(&self) -> &UCB1IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB1IFG) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb1ifg_mut(&self) -> &mut UCB1IFG {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(44usize) as *mut UCB1IFG) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv_spi(&self) -> &UCB1IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB1IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv_spi_mut(&self) -> &mut UCB1IV_SPI {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB1IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv(&self) -> &UCB1IV {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB1IV) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv_mut(&self) -> &mut UCB1IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(46usize) as *mut UCB1IV) }
    }
}
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctlw0](ucb1ctlw0) module"]
pub type UCB1CTLW0 = crate::Reg<u16, _UCB1CTLW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTLW0;
#[doc = "`read()` method returns [ucb1ctlw0::R](ucb1ctlw0::R) reader structure"]
impl crate::Readable for UCB1CTLW0 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctlw0::W](ucb1ctlw0::W) writer structure"]
impl crate::Writable for UCB1CTLW0 {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb1ctlw0;
#[doc = "eUSCI_Bx Control Word Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctlw0_spi](ucb1ctlw0_spi) module"]
pub type UCB1CTLW0_SPI = crate::Reg<u16, _UCB1CTLW0_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTLW0_SPI;
#[doc = "`read()` method returns [ucb1ctlw0_spi::R](ucb1ctlw0_spi::R) reader structure"]
impl crate::Readable for UCB1CTLW0_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ctlw0_spi::W](ucb1ctlw0_spi::W) writer structure"]
impl crate::Writable for UCB1CTLW0_SPI {}
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb1ctlw0_spi;
#[doc = "eUSCI_Bx Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ctlw1](ucb1ctlw1) module"]
pub type UCB1CTLW1 = crate::Reg<u16, _UCB1CTLW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1CTLW1;
#[doc = "`read()` method returns [ucb1ctlw1::R](ucb1ctlw1::R) reader structure"]
impl crate::Readable for UCB1CTLW1 {}
#[doc = "`write(|w| ..)` method takes [ucb1ctlw1::W](ucb1ctlw1::W) writer structure"]
impl crate::Writable for UCB1CTLW1 {}
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb1ctlw1;
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1brw](ucb1brw) module"]
pub type UCB1BRW = crate::Reg<u16, _UCB1BRW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BRW;
#[doc = "`read()` method returns [ucb1brw::R](ucb1brw::R) reader structure"]
impl crate::Readable for UCB1BRW {}
#[doc = "`write(|w| ..)` method takes [ucb1brw::W](ucb1brw::W) writer structure"]
impl crate::Writable for UCB1BRW {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb1brw;
#[doc = "eUSCI_Bx Bit Rate Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1brw_spi](ucb1brw_spi) module"]
pub type UCB1BRW_SPI = crate::Reg<u16, _UCB1BRW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1BRW_SPI;
#[doc = "`read()` method returns [ucb1brw_spi::R](ucb1brw_spi::R) reader structure"]
impl crate::Readable for UCB1BRW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1brw_spi::W](ucb1brw_spi::W) writer structure"]
impl crate::Writable for UCB1BRW_SPI {}
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb1brw_spi;
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1statw](ucb1statw) module"]
pub type UCB1STATW = crate::Reg<u16, _UCB1STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1STATW;
#[doc = "`read()` method returns [ucb1statw::R](ucb1statw::R) reader structure"]
impl crate::Readable for UCB1STATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb1statw;
#[doc = "UCB1STATW_SPI\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1statw_spi](ucb1statw_spi) module"]
pub type UCB1STATW_SPI = crate::Reg<u16, _UCB1STATW_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1STATW_SPI;
#[doc = "`read()` method returns [ucb1statw_spi::R](ucb1statw_spi::R) reader structure"]
impl crate::Readable for UCB1STATW_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1statw_spi::W](ucb1statw_spi::W) writer structure"]
impl crate::Writable for UCB1STATW_SPI {}
#[doc = "UCB1STATW_SPI"]
pub mod ucb1statw_spi;
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1tbcnt](ucb1tbcnt) module"]
pub type UCB1TBCNT = crate::Reg<u16, _UCB1TBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TBCNT;
#[doc = "`read()` method returns [ucb1tbcnt::R](ucb1tbcnt::R) reader structure"]
impl crate::Readable for UCB1TBCNT {}
#[doc = "`write(|w| ..)` method takes [ucb1tbcnt::W](ucb1tbcnt::W) writer structure"]
impl crate::Writable for UCB1TBCNT {}
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb1tbcnt;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1rxbuf](ucb1rxbuf) module"]
pub type UCB1RXBUF = crate::Reg<u16, _UCB1RXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1RXBUF;
#[doc = "`read()` method returns [ucb1rxbuf::R](ucb1rxbuf::R) reader structure"]
impl crate::Readable for UCB1RXBUF {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb1rxbuf;
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1rxbuf_spi](ucb1rxbuf_spi) module"]
pub type UCB1RXBUF_SPI = crate::Reg<u16, _UCB1RXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1RXBUF_SPI;
#[doc = "`read()` method returns [ucb1rxbuf_spi::R](ucb1rxbuf_spi::R) reader structure"]
impl crate::Readable for UCB1RXBUF_SPI {}
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb1rxbuf_spi;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1txbuf](ucb1txbuf) module"]
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
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1txbuf_spi](ucb1txbuf_spi) module"]
pub type UCB1TXBUF_SPI = crate::Reg<u16, _UCB1TXBUF_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1TXBUF_SPI;
#[doc = "`read()` method returns [ucb1txbuf_spi::R](ucb1txbuf_spi::R) reader structure"]
impl crate::Readable for UCB1TXBUF_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1txbuf_spi::W](ucb1txbuf_spi::W) writer structure"]
impl crate::Writable for UCB1TXBUF_SPI {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb1txbuf_spi;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa0](ucb1i2coa0) module"]
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
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa1](ucb1i2coa1) module"]
pub type UCB1I2COA1 = crate::Reg<u16, _UCB1I2COA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA1;
#[doc = "`read()` method returns [ucb1i2coa1::R](ucb1i2coa1::R) reader structure"]
impl crate::Readable for UCB1I2COA1 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa1::W](ucb1i2coa1::W) writer structure"]
impl crate::Writable for UCB1I2COA1 {}
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb1i2coa1;
#[doc = "eUSCI_Bx I2C Own Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa2](ucb1i2coa2) module"]
pub type UCB1I2COA2 = crate::Reg<u16, _UCB1I2COA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA2;
#[doc = "`read()` method returns [ucb1i2coa2::R](ucb1i2coa2::R) reader structure"]
impl crate::Readable for UCB1I2COA2 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa2::W](ucb1i2coa2::W) writer structure"]
impl crate::Writable for UCB1I2COA2 {}
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb1i2coa2;
#[doc = "eUSCI_Bx I2C Own Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2coa3](ucb1i2coa3) module"]
pub type UCB1I2COA3 = crate::Reg<u16, _UCB1I2COA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2COA3;
#[doc = "`read()` method returns [ucb1i2coa3::R](ucb1i2coa3::R) reader structure"]
impl crate::Readable for UCB1I2COA3 {}
#[doc = "`write(|w| ..)` method takes [ucb1i2coa3::W](ucb1i2coa3::W) writer structure"]
impl crate::Writable for UCB1I2COA3 {}
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb1i2coa3;
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1addrx](ucb1addrx) module"]
pub type UCB1ADDRX = crate::Reg<u16, _UCB1ADDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1ADDRX;
#[doc = "`read()` method returns [ucb1addrx::R](ucb1addrx::R) reader structure"]
impl crate::Readable for UCB1ADDRX {}
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb1addrx;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1addmask](ucb1addmask) module"]
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
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1i2csa](ucb1i2csa) module"]
pub type UCB1I2CSA = crate::Reg<u16, _UCB1I2CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1I2CSA;
#[doc = "`read()` method returns [ucb1i2csa::R](ucb1i2csa::R) reader structure"]
impl crate::Readable for UCB1I2CSA {}
#[doc = "`write(|w| ..)` method takes [ucb1i2csa::W](ucb1i2csa::W) writer structure"]
impl crate::Writable for UCB1I2CSA {}
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb1i2csa;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie](ucb1ie) module"]
pub type UCB1IE = crate::Reg<u16, _UCB1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IE;
#[doc = "`read()` method returns [ucb1ie::R](ucb1ie::R) reader structure"]
impl crate::Readable for UCB1IE {}
#[doc = "`write(|w| ..)` method takes [ucb1ie::W](ucb1ie::W) writer structure"]
impl crate::Writable for UCB1IE {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "eUSCI_Bx Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ie_spi](ucb1ie_spi) module"]
pub type UCB1IE_SPI = crate::Reg<u16, _UCB1IE_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IE_SPI;
#[doc = "`read()` method returns [ucb1ie_spi::R](ucb1ie_spi::R) reader structure"]
impl crate::Readable for UCB1IE_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ie_spi::W](ucb1ie_spi::W) writer structure"]
impl crate::Writable for UCB1IE_SPI {}
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb1ie_spi;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg](ucb1ifg) module"]
pub type UCB1IFG = crate::Reg<u16, _UCB1IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IFG;
#[doc = "`read()` method returns [ucb1ifg::R](ucb1ifg::R) reader structure"]
impl crate::Readable for UCB1IFG {}
#[doc = "`write(|w| ..)` method takes [ucb1ifg::W](ucb1ifg::W) writer structure"]
impl crate::Writable for UCB1IFG {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb1ifg;
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1ifg_spi](ucb1ifg_spi) module"]
pub type UCB1IFG_SPI = crate::Reg<u16, _UCB1IFG_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IFG_SPI;
#[doc = "`read()` method returns [ucb1ifg_spi::R](ucb1ifg_spi::R) reader structure"]
impl crate::Readable for UCB1IFG_SPI {}
#[doc = "`write(|w| ..)` method takes [ucb1ifg_spi::W](ucb1ifg_spi::W) writer structure"]
impl crate::Writable for UCB1IFG_SPI {}
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb1ifg_spi;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1iv](ucb1iv) module"]
pub type UCB1IV = crate::Reg<u16, _UCB1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IV;
#[doc = "`read()` method returns [ucb1iv::R](ucb1iv::R) reader structure"]
impl crate::Readable for UCB1IV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb1iv_spi](ucb1iv_spi) module"]
pub type UCB1IV_SPI = crate::Reg<u16, _UCB1IV_SPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB1IV_SPI;
#[doc = "`read()` method returns [ucb1iv_spi::R](ucb1iv_spi::R) reader structure"]
impl crate::Readable for UCB1IV_SPI {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv_spi;
