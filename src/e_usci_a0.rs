#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uca0ctlw: [u8; 0x02],
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub uca0ctlw1: UCA0CTLW1,
    _reserved2: [u8; 0x02],
    _reserved_2_uca0: [u8; 0x02],
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub uca0mctlw: UCA0MCTLW,
    _reserved_4_uca0: [u8; 0x02],
    _reserved_5_uca0: [u8; 0x02],
    _reserved_6_uca0: [u8; 0x02],
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca0abctl: UCA0ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub uca0irctl: UCA0IRCTL,
    _reserved9: [u8; 0x06],
    _reserved_9_uca0: [u8; 0x02],
    _reserved_10_uca0: [u8; 0x02],
    _reserved_11_uca0: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca0ctlw0_spi(&self) -> &UCA0CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA0CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca0ctlw0(&self) -> &UCA0CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA0CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca0brw_spi(&self) -> &UCA0BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA0BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca0brw(&self) -> &UCA0BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA0BRW) }
    }
    #[doc = "0x0a - UCA0STATW_SPI"]
    #[inline(always)]
    pub fn uca0statw_spi(&self) -> &UCA0STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA0STATW_SPI) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca0statw(&self) -> &UCA0STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA0STATW) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca0rxbuf_spi(&self) -> &UCA0RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA0RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca0rxbuf(&self) -> &UCA0RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA0RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf_spi(&self) -> &UCA0TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA0TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> &UCA0TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA0TXBUF) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca0ie_spi(&self) -> &UCA0IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA0IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca0ie(&self) -> &UCA0IE {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA0IE) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca0ifg_spi(&self) -> &UCA0IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA0IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca0ifg(&self) -> &UCA0IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA0IFG) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv_spi(&self) -> &UCA0IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA0IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv(&self) -> &UCA0IV {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA0IV) }
    }
}
#[doc = "UCA0CTLW0 (rw) register accessor: an alias for `Reg<UCA0CTLW0_SPEC>`"]
pub type UCA0CTLW0 = crate::Reg<uca0ctlw0::UCA0CTLW0_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca0ctlw0;
#[doc = "UCA0CTLW0_SPI (rw) register accessor: an alias for `Reg<UCA0CTLW0_SPI_SPEC>`"]
pub type UCA0CTLW0_SPI = crate::Reg<uca0ctlw0_spi::UCA0CTLW0_SPI_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca0ctlw0_spi;
#[doc = "UCA0CTLW1 (rw) register accessor: an alias for `Reg<UCA0CTLW1_SPEC>`"]
pub type UCA0CTLW1 = crate::Reg<uca0ctlw1::UCA0CTLW1_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "UCA0BRW (rw) register accessor: an alias for `Reg<UCA0BRW_SPEC>`"]
pub type UCA0BRW = crate::Reg<uca0brw::UCA0BRW_SPEC>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca0brw;
#[doc = "UCA0BRW_SPI (rw) register accessor: an alias for `Reg<UCA0BRW_SPI_SPEC>`"]
pub type UCA0BRW_SPI = crate::Reg<uca0brw_spi::UCA0BRW_SPI_SPEC>;
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca0brw_spi;
#[doc = "UCA0MCTLW (rw) register accessor: an alias for `Reg<UCA0MCTLW_SPEC>`"]
pub type UCA0MCTLW = crate::Reg<uca0mctlw::UCA0MCTLW_SPEC>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca0mctlw;
#[doc = "UCA0STATW (rw) register accessor: an alias for `Reg<UCA0STATW_SPEC>`"]
pub type UCA0STATW = crate::Reg<uca0statw::UCA0STATW_SPEC>;
#[doc = "eUSCI_Ax Status Register"]
pub mod uca0statw;
#[doc = "UCA0STATW_SPI (rw) register accessor: an alias for `Reg<UCA0STATW_SPI_SPEC>`"]
pub type UCA0STATW_SPI = crate::Reg<uca0statw_spi::UCA0STATW_SPI_SPEC>;
#[doc = "UCA0STATW_SPI"]
pub mod uca0statw_spi;
#[doc = "UCA0RXBUF (rw) register accessor: an alias for `Reg<UCA0RXBUF_SPEC>`"]
pub type UCA0RXBUF = crate::Reg<uca0rxbuf::UCA0RXBUF_SPEC>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca0rxbuf;
#[doc = "UCA0RXBUF_SPI (rw) register accessor: an alias for `Reg<UCA0RXBUF_SPI_SPEC>`"]
pub type UCA0RXBUF_SPI = crate::Reg<uca0rxbuf_spi::UCA0RXBUF_SPI_SPEC>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca0rxbuf_spi;
#[doc = "UCA0TXBUF (rw) register accessor: an alias for `Reg<UCA0TXBUF_SPEC>`"]
pub type UCA0TXBUF = crate::Reg<uca0txbuf::UCA0TXBUF_SPEC>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf;
#[doc = "UCA0TXBUF_SPI (rw) register accessor: an alias for `Reg<UCA0TXBUF_SPI_SPEC>`"]
pub type UCA0TXBUF_SPI = crate::Reg<uca0txbuf_spi::UCA0TXBUF_SPI_SPEC>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf_spi;
#[doc = "UCA0ABCTL (rw) register accessor: an alias for `Reg<UCA0ABCTL_SPEC>`"]
pub type UCA0ABCTL = crate::Reg<uca0abctl::UCA0ABCTL_SPEC>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca0abctl;
#[doc = "UCA0IRCTL (rw) register accessor: an alias for `Reg<UCA0IRCTL_SPEC>`"]
pub type UCA0IRCTL = crate::Reg<uca0irctl::UCA0IRCTL_SPEC>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca0irctl;
#[doc = "UCA0IE (rw) register accessor: an alias for `Reg<UCA0IE_SPEC>`"]
pub type UCA0IE = crate::Reg<uca0ie::UCA0IE_SPEC>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca0ie;
#[doc = "UCA0IE_SPI (rw) register accessor: an alias for `Reg<UCA0IE_SPI_SPEC>`"]
pub type UCA0IE_SPI = crate::Reg<uca0ie_spi::UCA0IE_SPI_SPEC>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca0ie_spi;
#[doc = "UCA0IFG (rw) register accessor: an alias for `Reg<UCA0IFG_SPEC>`"]
pub type UCA0IFG = crate::Reg<uca0ifg::UCA0IFG_SPEC>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca0ifg;
#[doc = "UCA0IFG_SPI (rw) register accessor: an alias for `Reg<UCA0IFG_SPI_SPEC>`"]
pub type UCA0IFG_SPI = crate::Reg<uca0ifg_spi::UCA0IFG_SPI_SPEC>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca0ifg_spi;
#[doc = "UCA0IV (rw) register accessor: an alias for `Reg<UCA0IV_SPEC>`"]
pub type UCA0IV = crate::Reg<uca0iv::UCA0IV_SPEC>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv;
#[doc = "UCA0IV_SPI (rw) register accessor: an alias for `Reg<UCA0IV_SPI_SPEC>`"]
pub type UCA0IV_SPI = crate::Reg<uca0iv_spi::UCA0IV_SPI_SPEC>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv_spi;
