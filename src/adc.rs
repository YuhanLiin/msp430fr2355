#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adcctl0: Adcctl0,
    adcctl1: Adcctl1,
    adcctl2: Adcctl2,
    adclo: Adclo,
    adchi: Adchi,
    adcmctl0: Adcmctl0,
    _reserved6: [u8; 0x06],
    adcmem0: Adcmem0,
    _reserved7: [u8; 0x06],
    adcie: Adcie,
    adcifg: Adcifg,
    adciv: Adciv,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Control 0"]
    #[inline(always)]
    pub const fn adcctl0(&self) -> &Adcctl0 {
        &self.adcctl0
    }
    #[doc = "0x02 - ADC Control 1"]
    #[inline(always)]
    pub const fn adcctl1(&self) -> &Adcctl1 {
        &self.adcctl1
    }
    #[doc = "0x04 - ADC Control 2"]
    #[inline(always)]
    pub const fn adcctl2(&self) -> &Adcctl2 {
        &self.adcctl2
    }
    #[doc = "0x06 - ADC Window Comparator Low Threshold Register"]
    #[inline(always)]
    pub const fn adclo(&self) -> &Adclo {
        &self.adclo
    }
    #[doc = "0x08 - ADC Window Comparator High Threshold Register"]
    #[inline(always)]
    pub const fn adchi(&self) -> &Adchi {
        &self.adchi
    }
    #[doc = "0x0a - ADC Conversion Memory Control Register"]
    #[inline(always)]
    pub const fn adcmctl0(&self) -> &Adcmctl0 {
        &self.adcmctl0
    }
    #[doc = "0x12 - ADC Conversion Memory Register"]
    #[inline(always)]
    pub const fn adcmem0(&self) -> &Adcmem0 {
        &self.adcmem0
    }
    #[doc = "0x1a - ADC Interrupt Enable 0"]
    #[inline(always)]
    pub const fn adcie(&self) -> &Adcie {
        &self.adcie
    }
    #[doc = "0x1c - ADC Interrupt Flag"]
    #[inline(always)]
    pub const fn adcifg(&self) -> &Adcifg {
        &self.adcifg
    }
    #[doc = "0x1e - ADC Interrupt Vector"]
    #[inline(always)]
    pub const fn adciv(&self) -> &Adciv {
        &self.adciv
    }
}
#[doc = "ADCCTL0 (rw) register accessor: ADC Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl0`] module"]
#[doc(alias = "ADCCTL0")]
pub type Adcctl0 = crate::Reg<adcctl0::Adcctl0Spec>;
#[doc = "ADC Control 0"]
pub mod adcctl0;
#[doc = "ADCCTL1 (rw) register accessor: ADC Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl1`] module"]
#[doc(alias = "ADCCTL1")]
pub type Adcctl1 = crate::Reg<adcctl1::Adcctl1Spec>;
#[doc = "ADC Control 1"]
pub mod adcctl1;
#[doc = "ADCCTL2 (rw) register accessor: ADC Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl2`] module"]
#[doc(alias = "ADCCTL2")]
pub type Adcctl2 = crate::Reg<adcctl2::Adcctl2Spec>;
#[doc = "ADC Control 2"]
pub mod adcctl2;
#[doc = "ADCLO (rw) register accessor: ADC Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adclo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adclo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adclo`] module"]
#[doc(alias = "ADCLO")]
pub type Adclo = crate::Reg<adclo::AdcloSpec>;
#[doc = "ADC Window Comparator Low Threshold Register"]
pub mod adclo;
#[doc = "ADCHI (rw) register accessor: ADC Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adchi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adchi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adchi`] module"]
#[doc(alias = "ADCHI")]
pub type Adchi = crate::Reg<adchi::AdchiSpec>;
#[doc = "ADC Window Comparator High Threshold Register"]
pub mod adchi;
#[doc = "ADCMCTL0 (rw) register accessor: ADC Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmctl0`] module"]
#[doc(alias = "ADCMCTL0")]
pub type Adcmctl0 = crate::Reg<adcmctl0::Adcmctl0Spec>;
#[doc = "ADC Conversion Memory Control Register"]
pub mod adcmctl0;
#[doc = "ADCMEM0 (rw) register accessor: ADC Conversion Memory Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmem0`] module"]
#[doc(alias = "ADCMEM0")]
pub type Adcmem0 = crate::Reg<adcmem0::Adcmem0Spec>;
#[doc = "ADC Conversion Memory Register"]
pub mod adcmem0;
#[doc = "ADCIE (rw) register accessor: ADC Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcie`] module"]
#[doc(alias = "ADCIE")]
pub type Adcie = crate::Reg<adcie::AdcieSpec>;
#[doc = "ADC Interrupt Enable 0"]
pub mod adcie;
#[doc = "ADCIFG (rw) register accessor: ADC Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`adcifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcifg`] module"]
#[doc(alias = "ADCIFG")]
pub type Adcifg = crate::Reg<adcifg::AdcifgSpec>;
#[doc = "ADC Interrupt Flag"]
pub mod adcifg;
#[doc = "ADCIV (rw) register accessor: ADC Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`adciv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adciv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adciv`] module"]
#[doc(alias = "ADCIV")]
pub type Adciv = crate::Reg<adciv::AdcivSpec>;
#[doc = "ADC Interrupt Vector"]
pub mod adciv;
