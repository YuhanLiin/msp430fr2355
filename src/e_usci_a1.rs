#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uca1ctlw: [u8; 0x02],
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub uca1ctlw1: UCA1CTLW1,
    _reserved2: [u8; 0x02],
    _reserved_2_uca1: [u8; 0x02],
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub uca1mctlw: UCA1MCTLW,
    _reserved_4_uca1: [u8; 0x02],
    _reserved_5_uca1: [u8; 0x02],
    _reserved_6_uca1: [u8; 0x02],
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca1abctl: UCA1ABCTL,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub uca1irctl: UCA1IRCTL,
    _reserved9: [u8; 0x06],
    _reserved_9_uca1: [u8; 0x02],
    _reserved_10_uca1: [u8; 0x02],
    _reserved_11_uca1: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca1ctlw0_spi(&self) -> &UCA1CTLW0_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA1CTLW0_SPI) }
    }
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    #[inline(always)]
    pub fn uca1ctlw0(&self) -> &UCA1CTLW0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCA1CTLW0) }
    }
    #[doc = "0x06 - eUSCI_Ax Bit Rate Control Register 1"]
    #[inline(always)]
    pub fn uca1brw_spi(&self) -> &UCA1BRW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA1BRW_SPI) }
    }
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    #[inline(always)]
    pub fn uca1brw(&self) -> &UCA1BRW {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA1BRW) }
    }
    #[doc = "0x0a - UCA1STATW_SPI"]
    #[inline(always)]
    pub fn uca1statw_spi(&self) -> &UCA1STATW_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA1STATW_SPI) }
    }
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    #[inline(always)]
    pub fn uca1statw(&self) -> &UCA1STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const UCA1STATW) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca1rxbuf_spi(&self) -> &UCA1RXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA1RXBUF_SPI) }
    }
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    #[inline(always)]
    pub fn uca1rxbuf(&self) -> &UCA1RXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const UCA1RXBUF) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf_spi(&self) -> &UCA1TXBUF_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA1TXBUF_SPI) }
    }
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf(&self) -> &UCA1TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const UCA1TXBUF) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca1ie_spi(&self) -> &UCA1IE_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA1IE_SPI) }
    }
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    #[inline(always)]
    pub fn uca1ie(&self) -> &UCA1IE {
        unsafe { &*(((self as *const Self) as *const u8).add(26usize) as *const UCA1IE) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca1ifg_spi(&self) -> &UCA1IFG_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA1IFG_SPI) }
    }
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    #[inline(always)]
    pub fn uca1ifg(&self) -> &UCA1IFG {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const UCA1IFG) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv_spi(&self) -> &UCA1IV_SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA1IV_SPI) }
    }
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv(&self) -> &UCA1IV {
        unsafe { &*(((self as *const Self) as *const u8).add(30usize) as *const UCA1IV) }
    }
}
#[doc = "UCA1CTLW0 (rw) register accessor: an alias for `Reg<UCA1CTLW0_SPEC>`"]
pub type UCA1CTLW0 = crate::Reg<uca1ctlw0::UCA1CTLW0_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca1ctlw0;
#[doc = "UCA1CTLW0_SPI (rw) register accessor: an alias for `Reg<UCA1CTLW0_SPI_SPEC>`"]
pub type UCA1CTLW0_SPI = crate::Reg<uca1ctlw0_spi::UCA1CTLW0_SPI_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod uca1ctlw0_spi;
#[doc = "UCA1CTLW1 (rw) register accessor: an alias for `Reg<UCA1CTLW1_SPEC>`"]
pub type UCA1CTLW1 = crate::Reg<uca1ctlw1::UCA1CTLW1_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod uca1ctlw1;
#[doc = "UCA1BRW (rw) register accessor: an alias for `Reg<UCA1BRW_SPEC>`"]
pub type UCA1BRW = crate::Reg<uca1brw::UCA1BRW_SPEC>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod uca1brw;
#[doc = "UCA1BRW_SPI (rw) register accessor: an alias for `Reg<UCA1BRW_SPI_SPEC>`"]
pub type UCA1BRW_SPI = crate::Reg<uca1brw_spi::UCA1BRW_SPI_SPEC>;
#[doc = "eUSCI_Ax Bit Rate Control Register 1"]
pub mod uca1brw_spi;
#[doc = "UCA1MCTLW (rw) register accessor: an alias for `Reg<UCA1MCTLW_SPEC>`"]
pub type UCA1MCTLW = crate::Reg<uca1mctlw::UCA1MCTLW_SPEC>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca1mctlw;
#[doc = "UCA1STATW (rw) register accessor: an alias for `Reg<UCA1STATW_SPEC>`"]
pub type UCA1STATW = crate::Reg<uca1statw::UCA1STATW_SPEC>;
#[doc = "eUSCI_Ax Status Register"]
pub mod uca1statw;
#[doc = "UCA1STATW_SPI (rw) register accessor: an alias for `Reg<UCA1STATW_SPI_SPEC>`"]
pub type UCA1STATW_SPI = crate::Reg<uca1statw_spi::UCA1STATW_SPI_SPEC>;
#[doc = "UCA1STATW_SPI"]
pub mod uca1statw_spi;
#[doc = "UCA1RXBUF (rw) register accessor: an alias for `Reg<UCA1RXBUF_SPEC>`"]
pub type UCA1RXBUF = crate::Reg<uca1rxbuf::UCA1RXBUF_SPEC>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca1rxbuf;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: an alias for `Reg<UCA1RXBUF_SPI_SPEC>`"]
pub type UCA1RXBUF_SPI = crate::Reg<uca1rxbuf_spi::UCA1RXBUF_SPI_SPEC>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF (rw) register accessor: an alias for `Reg<UCA1TXBUF_SPEC>`"]
pub type UCA1TXBUF = crate::Reg<uca1txbuf::UCA1TXBUF_SPEC>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca1txbuf;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: an alias for `Reg<UCA1TXBUF_SPI_SPEC>`"]
pub type UCA1TXBUF_SPI = crate::Reg<uca1txbuf_spi::UCA1TXBUF_SPI_SPEC>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca1txbuf_spi;
#[doc = "UCA1ABCTL (rw) register accessor: an alias for `Reg<UCA1ABCTL_SPEC>`"]
pub type UCA1ABCTL = crate::Reg<uca1abctl::UCA1ABCTL_SPEC>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca1abctl;
#[doc = "UCA1IRCTL (rw) register accessor: an alias for `Reg<UCA1IRCTL_SPEC>`"]
pub type UCA1IRCTL = crate::Reg<uca1irctl::UCA1IRCTL_SPEC>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca1irctl;
#[doc = "UCA1IE (rw) register accessor: an alias for `Reg<UCA1IE_SPEC>`"]
pub type UCA1IE = crate::Reg<uca1ie::UCA1IE_SPEC>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca1ie;
#[doc = "UCA1IE_SPI (rw) register accessor: an alias for `Reg<UCA1IE_SPI_SPEC>`"]
pub type UCA1IE_SPI = crate::Reg<uca1ie_spi::UCA1IE_SPI_SPEC>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "UCA1IFG (rw) register accessor: an alias for `Reg<UCA1IFG_SPEC>`"]
pub type UCA1IFG = crate::Reg<uca1ifg::UCA1IFG_SPEC>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca1ifg;
#[doc = "UCA1IFG_SPI (rw) register accessor: an alias for `Reg<UCA1IFG_SPI_SPEC>`"]
pub type UCA1IFG_SPI = crate::Reg<uca1ifg_spi::UCA1IFG_SPI_SPEC>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod uca1ifg_spi;
#[doc = "UCA1IV (rw) register accessor: an alias for `Reg<UCA1IV_SPEC>`"]
pub type UCA1IV = crate::Reg<uca1iv::UCA1IV_SPEC>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca1iv;
#[doc = "UCA1IV_SPI (rw) register accessor: an alias for `Reg<UCA1IV_SPI_SPEC>`"]
pub type UCA1IV_SPI = crate::Reg<uca1iv_spi::UCA1IV_SPI_SPEC>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca1iv_spi;
