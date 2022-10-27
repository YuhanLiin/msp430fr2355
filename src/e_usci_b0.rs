#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb0ctlw: [u8; 0x02],
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucb0ctlw1: UCB0CTLW1,
    _reserved2: [u8; 0x02],
    _reserved_2_ucb0: [u8; 0x02],
    _reserved_3_ucb0: [u8; 0x02],
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucb0tbcnt: UCB0TBCNT,
    _reserved_5_ucb0: [u8; 0x02],
    _reserved_6_ucb0: [u8; 0x02],
    _reserved7: [u8; 0x04],
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
    _reserved14: [u8; 0x08],
    _reserved_14_ucb0: [u8; 0x02],
    _reserved_15_ucb0: [u8; 0x02],
    _reserved_16_ucb0: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb0ctlw0_spi(&self) -> &UCB0CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB0CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb0ctlw0(&self) -> &UCB0CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB0CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb0brw_spi(&self) -> &UCB0BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB0BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb0brw(&self) -> &UCB0BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB0BRW) }
    }
    #[doc = "0x08 - UCB0STATW_SPI"]
    #[inline(always)]
    pub fn ucb0statw_spi(&self) -> &UCB0STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB0STATW_SPI) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb0statw(&self) -> &UCB0STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB0STATW) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb0rxbuf_spi(&self) -> &UCB0RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB0RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb0rxbuf(&self) -> &UCB0RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB0RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf_spi(&self) -> &UCB0TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB0TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> &UCB0TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB0TXBUF) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie_spi(&self) -> &UCB0IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB0IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb0ie(&self) -> &UCB0IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB0IE) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb0ifg_spi(&self) -> &UCB0IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB0IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb0ifg(&self) -> &UCB0IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB0IFG) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv_spi(&self) -> &UCB0IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB0IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv(&self) -> &UCB0IV {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB0IV) }
    }
}
#[doc = "UCB0CTLW0 (rw) register accessor: an alias for `Reg<UCB0CTLW0_SPEC>`"]
pub type UCB0CTLW0 = crate::Reg<ucb0ctlw0::UCB0CTLW0_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb0ctlw0;
#[doc = "UCB0CTLW0_SPI (rw) register accessor: an alias for `Reg<UCB0CTLW0_SPI_SPEC>`"]
pub type UCB0CTLW0_SPI = crate::Reg<ucb0ctlw0_spi::UCB0CTLW0_SPI_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb0ctlw0_spi;
#[doc = "UCB0CTLW1 (rw) register accessor: an alias for `Reg<UCB0CTLW1_SPEC>`"]
pub type UCB0CTLW1 = crate::Reg<ucb0ctlw1::UCB0CTLW1_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "UCB0BRW (rw) register accessor: an alias for `Reg<UCB0BRW_SPEC>`"]
pub type UCB0BRW = crate::Reg<ucb0brw::UCB0BRW_SPEC>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb0brw;
#[doc = "UCB0BRW_SPI (rw) register accessor: an alias for `Reg<UCB0BRW_SPI_SPEC>`"]
pub type UCB0BRW_SPI = crate::Reg<ucb0brw_spi::UCB0BRW_SPI_SPEC>;
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb0brw_spi;
#[doc = "UCB0STATW (rw) register accessor: an alias for `Reg<UCB0STATW_SPEC>`"]
pub type UCB0STATW = crate::Reg<ucb0statw::UCB0STATW_SPEC>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb0statw;
#[doc = "UCB0STATW_SPI (rw) register accessor: an alias for `Reg<UCB0STATW_SPI_SPEC>`"]
pub type UCB0STATW_SPI = crate::Reg<ucb0statw_spi::UCB0STATW_SPI_SPEC>;
#[doc = "UCB0STATW_SPI"]
pub mod ucb0statw_spi;
#[doc = "UCB0TBCNT (rw) register accessor: an alias for `Reg<UCB0TBCNT_SPEC>`"]
pub type UCB0TBCNT = crate::Reg<ucb0tbcnt::UCB0TBCNT_SPEC>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
#[doc = "UCB0RXBUF (rw) register accessor: an alias for `Reg<UCB0RXBUF_SPEC>`"]
pub type UCB0RXBUF = crate::Reg<ucb0rxbuf::UCB0RXBUF_SPEC>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb0rxbuf;
#[doc = "UCB0RXBUF_SPI (rw) register accessor: an alias for `Reg<UCB0RXBUF_SPI_SPEC>`"]
pub type UCB0RXBUF_SPI = crate::Reg<ucb0rxbuf_spi::UCB0RXBUF_SPI_SPEC>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb0rxbuf_spi;
#[doc = "UCB0TXBUF (rw) register accessor: an alias for `Reg<UCB0TXBUF_SPEC>`"]
pub type UCB0TXBUF = crate::Reg<ucb0txbuf::UCB0TXBUF_SPEC>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf;
#[doc = "UCB0TXBUF_SPI (rw) register accessor: an alias for `Reg<UCB0TXBUF_SPI_SPEC>`"]
pub type UCB0TXBUF_SPI = crate::Reg<ucb0txbuf_spi::UCB0TXBUF_SPI_SPEC>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf_spi;
#[doc = "UCB0I2COA0 (rw) register accessor: an alias for `Reg<UCB0I2COA0_SPEC>`"]
pub type UCB0I2COA0 = crate::Reg<ucb0i2coa0::UCB0I2COA0_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb0i2coa0;
#[doc = "UCB0I2COA1 (rw) register accessor: an alias for `Reg<UCB0I2COA1_SPEC>`"]
pub type UCB0I2COA1 = crate::Reg<ucb0i2coa1::UCB0I2COA1_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb0i2coa1;
#[doc = "UCB0I2COA2 (rw) register accessor: an alias for `Reg<UCB0I2COA2_SPEC>`"]
pub type UCB0I2COA2 = crate::Reg<ucb0i2coa2::UCB0I2COA2_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb0i2coa2;
#[doc = "UCB0I2COA3 (rw) register accessor: an alias for `Reg<UCB0I2COA3_SPEC>`"]
pub type UCB0I2COA3 = crate::Reg<ucb0i2coa3::UCB0I2COA3_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb0i2coa3;
#[doc = "UCB0ADDRX (rw) register accessor: an alias for `Reg<UCB0ADDRX_SPEC>`"]
pub type UCB0ADDRX = crate::Reg<ucb0addrx::UCB0ADDRX_SPEC>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb0addrx;
#[doc = "UCB0ADDMASK (rw) register accessor: an alias for `Reg<UCB0ADDMASK_SPEC>`"]
pub type UCB0ADDMASK = crate::Reg<ucb0addmask::UCB0ADDMASK_SPEC>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb0addmask;
#[doc = "UCB0I2CSA (rw) register accessor: an alias for `Reg<UCB0I2CSA_SPEC>`"]
pub type UCB0I2CSA = crate::Reg<ucb0i2csa::UCB0I2CSA_SPEC>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb0i2csa;
#[doc = "UCB0IE (rw) register accessor: an alias for `Reg<UCB0IE_SPEC>`"]
pub type UCB0IE = crate::Reg<ucb0ie::UCB0IE_SPEC>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "UCB0IE_SPI (rw) register accessor: an alias for `Reg<UCB0IE_SPI_SPEC>`"]
pub type UCB0IE_SPI = crate::Reg<ucb0ie_spi::UCB0IE_SPI_SPEC>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb0ie_spi;
#[doc = "UCB0IFG (rw) register accessor: an alias for `Reg<UCB0IFG_SPEC>`"]
pub type UCB0IFG = crate::Reg<ucb0ifg::UCB0IFG_SPEC>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb0ifg;
#[doc = "UCB0IFG_SPI (rw) register accessor: an alias for `Reg<UCB0IFG_SPI_SPEC>`"]
pub type UCB0IFG_SPI = crate::Reg<ucb0ifg_spi::UCB0IFG_SPI_SPEC>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb0ifg_spi;
#[doc = "UCB0IV (rw) register accessor: an alias for `Reg<UCB0IV_SPEC>`"]
pub type UCB0IV = crate::Reg<ucb0iv::UCB0IV_SPEC>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv;
#[doc = "UCB0IV_SPI (rw) register accessor: an alias for `Reg<UCB0IV_SPI_SPEC>`"]
pub type UCB0IV_SPI = crate::Reg<ucb0iv_spi::UCB0IV_SPI_SPEC>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv_spi;
