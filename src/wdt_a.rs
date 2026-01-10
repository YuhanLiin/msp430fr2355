#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdtctl: Wdtctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn wdtctl(&self) -> &Wdtctl {
        &self.wdtctl
    }
}
#[doc = "WDTCTL (rw) register accessor: Watchdog Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtctl`] module"]
#[doc(alias = "WDTCTL")]
pub type Wdtctl = crate::Reg<wdtctl::WdtctlSpec>;
#[doc = "Watchdog Timer Control Register"]
pub mod wdtctl;
