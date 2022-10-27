#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cp1ctl0: CP1CTL0,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cp1ctl1: CP1CTL1,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - Comparator Interrupt Control Register"]
    pub cp1int: CP1INT,
    #[doc = "0x08 - Comparator Interrupt Vector Word Register"]
    pub cp1iv: CP1IV,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - 6-bit Comparator built-in DAC Control Register"]
    pub cp1dacctl: CP1DACCTL,
    #[doc = "0x12 - 6-bit Comparator built-in DAC Data Register"]
    pub cp1dacdata: CP1DACDATA,
}
#[doc = "CP1CTL0 (rw) register accessor: an alias for `Reg<CP1CTL0_SPEC>`"]
pub type CP1CTL0 = crate::Reg<cp1ctl0::CP1CTL0_SPEC>;
#[doc = "Comparator Control Register 0"]
pub mod cp1ctl0;
#[doc = "CP1CTL1 (rw) register accessor: an alias for `Reg<CP1CTL1_SPEC>`"]
pub type CP1CTL1 = crate::Reg<cp1ctl1::CP1CTL1_SPEC>;
#[doc = "Comparator Control Register 1"]
pub mod cp1ctl1;
#[doc = "CP1INT (rw) register accessor: an alias for `Reg<CP1INT_SPEC>`"]
pub type CP1INT = crate::Reg<cp1int::CP1INT_SPEC>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cp1int;
#[doc = "CP1IV (rw) register accessor: an alias for `Reg<CP1IV_SPEC>`"]
pub type CP1IV = crate::Reg<cp1iv::CP1IV_SPEC>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cp1iv;
#[doc = "CP1DACCTL (rw) register accessor: an alias for `Reg<CP1DACCTL_SPEC>`"]
pub type CP1DACCTL = crate::Reg<cp1dacctl::CP1DACCTL_SPEC>;
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cp1dacctl;
#[doc = "CP1DACDATA (rw) register accessor: an alias for `Reg<CP1DACDATA_SPEC>`"]
pub type CP1DACDATA = crate::Reg<cp1dacdata::CP1DACDATA_SPEC>;
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cp1dacdata;
