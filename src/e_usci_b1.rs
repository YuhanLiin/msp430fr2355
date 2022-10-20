#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb1ctlw: [u8; 0x02],
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucb1ctlw1: UCB1CTLW1,
    _reserved2: [u8; 0x02],
    _reserved_2_ucb1: [u8; 0x02],
    _reserved_3_ucb1: [u8; 0x02],
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucb1tbcnt: UCB1TBCNT,
    _reserved_5_ucb1: [u8; 0x02],
    _reserved_6_ucb1: [u8; 0x02],
    _reserved7: [u8; 0x04],
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
    _reserved14: [u8; 0x08],
    _reserved_14_ucb1: [u8; 0x02],
    _reserved_15_ucb1: [u8; 0x02],
    _reserved_16_ucb1: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb1ctlw0_spi(&self) -> &UCB1CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB1CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub fn ucb1ctlw0(&self) -> &UCB1CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB1CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn ucb1brw_spi(&self) -> &UCB1BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB1BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn ucb1brw(&self) -> &UCB1BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB1BRW) }
    }
    #[doc = "0x08 - UCB1STATW_SPI"]
    #[inline(always)]
    pub fn ucb1statw_spi(&self) -> &UCB1STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB1STATW_SPI) }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub fn ucb1statw(&self) -> &UCB1STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const UCB1STATW) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb1rxbuf_spi(&self) -> &UCB1RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB1RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub fn ucb1rxbuf(&self) -> &UCB1RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCB1RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf_spi(&self) -> &UCB1TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB1TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb1txbuf(&self) -> &UCB1TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCB1TXBUF) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie_spi(&self) -> &UCB1IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB1IE_SPI) }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub fn ucb1ie(&self) -> &UCB1IE {
        unsafe { &*(((self as *const Self) as *const u8).add(42usize) as *const UCB1IE) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb1ifg_spi(&self) -> &UCB1IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB1IFG_SPI) }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub fn ucb1ifg(&self) -> &UCB1IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const UCB1IFG) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv_spi(&self) -> &UCB1IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB1IV_SPI) }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb1iv(&self) -> &UCB1IV {
        unsafe { &*(((self as *const Self) as *const u8).add(46usize) as *const UCB1IV) }
    }
}
#[doc = "UCB1CTLW0 (rw) register accessor: an alias for `Reg<UCB1CTLW0_SPEC>`"]
pub type UCB1CTLW0 = crate::Reg<ucb1ctlw0::UCB1CTLW0_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb1ctlw0;
#[doc = "UCB1CTLW0_SPI (rw) register accessor: an alias for `Reg<UCB1CTLW0_SPI_SPEC>`"]
pub type UCB1CTLW0_SPI = crate::Reg<ucb1ctlw0_spi::UCB1CTLW0_SPI_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb1ctlw0_spi;
#[doc = "UCB1CTLW1 (rw) register accessor: an alias for `Reg<UCB1CTLW1_SPEC>`"]
pub type UCB1CTLW1 = crate::Reg<ucb1ctlw1::UCB1CTLW1_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb1ctlw1;
#[doc = "UCB1BRW (rw) register accessor: an alias for `Reg<UCB1BRW_SPEC>`"]
pub type UCB1BRW = crate::Reg<ucb1brw::UCB1BRW_SPEC>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb1brw;
#[doc = "UCB1BRW_SPI (rw) register accessor: an alias for `Reg<UCB1BRW_SPI_SPEC>`"]
pub type UCB1BRW_SPI = crate::Reg<ucb1brw_spi::UCB1BRW_SPI_SPEC>;
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb1brw_spi;
#[doc = "UCB1STATW (rw) register accessor: an alias for `Reg<UCB1STATW_SPEC>`"]
pub type UCB1STATW = crate::Reg<ucb1statw::UCB1STATW_SPEC>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb1statw;
#[doc = "UCB1STATW_SPI (rw) register accessor: an alias for `Reg<UCB1STATW_SPI_SPEC>`"]
pub type UCB1STATW_SPI = crate::Reg<ucb1statw_spi::UCB1STATW_SPI_SPEC>;
#[doc = "UCB1STATW_SPI"]
pub mod ucb1statw_spi;
#[doc = "UCB1TBCNT (rw) register accessor: an alias for `Reg<UCB1TBCNT_SPEC>`"]
pub type UCB1TBCNT = crate::Reg<ucb1tbcnt::UCB1TBCNT_SPEC>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb1tbcnt;
#[doc = "UCB1RXBUF (rw) register accessor: an alias for `Reg<UCB1RXBUF_SPEC>`"]
pub type UCB1RXBUF = crate::Reg<ucb1rxbuf::UCB1RXBUF_SPEC>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb1rxbuf;
#[doc = "UCB1RXBUF_SPI (rw) register accessor: an alias for `Reg<UCB1RXBUF_SPI_SPEC>`"]
pub type UCB1RXBUF_SPI = crate::Reg<ucb1rxbuf_spi::UCB1RXBUF_SPI_SPEC>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb1rxbuf_spi;
#[doc = "UCB1TXBUF (rw) register accessor: an alias for `Reg<UCB1TXBUF_SPEC>`"]
pub type UCB1TXBUF = crate::Reg<ucb1txbuf::UCB1TXBUF_SPEC>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb1txbuf;
#[doc = "UCB1TXBUF_SPI (rw) register accessor: an alias for `Reg<UCB1TXBUF_SPI_SPEC>`"]
pub type UCB1TXBUF_SPI = crate::Reg<ucb1txbuf_spi::UCB1TXBUF_SPI_SPEC>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb1txbuf_spi;
#[doc = "UCB1I2COA0 (rw) register accessor: an alias for `Reg<UCB1I2COA0_SPEC>`"]
pub type UCB1I2COA0 = crate::Reg<ucb1i2coa0::UCB1I2COA0_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb1i2coa0;
#[doc = "UCB1I2COA1 (rw) register accessor: an alias for `Reg<UCB1I2COA1_SPEC>`"]
pub type UCB1I2COA1 = crate::Reg<ucb1i2coa1::UCB1I2COA1_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb1i2coa1;
#[doc = "UCB1I2COA2 (rw) register accessor: an alias for `Reg<UCB1I2COA2_SPEC>`"]
pub type UCB1I2COA2 = crate::Reg<ucb1i2coa2::UCB1I2COA2_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb1i2coa2;
#[doc = "UCB1I2COA3 (rw) register accessor: an alias for `Reg<UCB1I2COA3_SPEC>`"]
pub type UCB1I2COA3 = crate::Reg<ucb1i2coa3::UCB1I2COA3_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb1i2coa3;
#[doc = "UCB1ADDRX (rw) register accessor: an alias for `Reg<UCB1ADDRX_SPEC>`"]
pub type UCB1ADDRX = crate::Reg<ucb1addrx::UCB1ADDRX_SPEC>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb1addrx;
#[doc = "UCB1ADDMASK (rw) register accessor: an alias for `Reg<UCB1ADDMASK_SPEC>`"]
pub type UCB1ADDMASK = crate::Reg<ucb1addmask::UCB1ADDMASK_SPEC>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb1addmask;
#[doc = "UCB1I2CSA (rw) register accessor: an alias for `Reg<UCB1I2CSA_SPEC>`"]
pub type UCB1I2CSA = crate::Reg<ucb1i2csa::UCB1I2CSA_SPEC>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb1i2csa;
#[doc = "UCB1IE (rw) register accessor: an alias for `Reg<UCB1IE_SPEC>`"]
pub type UCB1IE = crate::Reg<ucb1ie::UCB1IE_SPEC>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb1ie;
#[doc = "UCB1IE_SPI (rw) register accessor: an alias for `Reg<UCB1IE_SPI_SPEC>`"]
pub type UCB1IE_SPI = crate::Reg<ucb1ie_spi::UCB1IE_SPI_SPEC>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb1ie_spi;
#[doc = "UCB1IFG (rw) register accessor: an alias for `Reg<UCB1IFG_SPEC>`"]
pub type UCB1IFG = crate::Reg<ucb1ifg::UCB1IFG_SPEC>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb1ifg;
#[doc = "UCB1IFG_SPI (rw) register accessor: an alias for `Reg<UCB1IFG_SPI_SPEC>`"]
pub type UCB1IFG_SPI = crate::Reg<ucb1ifg_spi::UCB1IFG_SPI_SPEC>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb1ifg_spi;
#[doc = "UCB1IV (rw) register accessor: an alias for `Reg<UCB1IV_SPEC>`"]
pub type UCB1IV = crate::Reg<ucb1iv::UCB1IV_SPEC>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv;
#[doc = "UCB1IV_SPI (rw) register accessor: an alias for `Reg<UCB1IV_SPI_SPEC>`"]
pub type UCB1IV_SPI = crate::Reg<ucb1iv_spi::UCB1IV_SPI_SPEC>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb1iv_spi;
