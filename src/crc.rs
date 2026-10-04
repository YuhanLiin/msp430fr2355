#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_crcdi: [u8; 0x02],
    _reserved_1_crcdirb: [u8; 0x02],
    crcinires: Crcinires,
    crcresr: Crcresr,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Data In Register, lower byte"]
    #[inline(always)]
    pub const fn crcdi_l(&self) -> &CrcdiL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - CRC Data In"]
    #[inline(always)]
    pub const fn crcdi(&self) -> &Crcdi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - CRC Data In Reverse Byte Register, lower byte"]
    #[inline(always)]
    pub const fn crcdirb_l(&self) -> &CrcdirbL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - CRC Data In Reverse Byte"]
    #[inline(always)]
    pub const fn crcdirb(&self) -> &Crcdirb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x04 - CRC Initialization and Result"]
    #[inline(always)]
    pub const fn crcinires(&self) -> &Crcinires {
        &self.crcinires
    }
    #[doc = "0x06 - CRC Result Reverse"]
    #[inline(always)]
    pub const fn crcresr(&self) -> &Crcresr {
        &self.crcresr
    }
}
#[doc = "CRCDI (rw) register accessor: CRC Data In\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdi`] module"]
#[doc(alias = "CRCDI")]
pub type Crcdi = crate::Reg<crcdi::CrcdiSpec>;
#[doc = "CRC Data In"]
pub mod crcdi;
#[doc = "CRCDI_L (rw) register accessor: CRC Data In Register, lower byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdi_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdi_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdi_l`] module"]
#[doc(alias = "CRCDI_L")]
pub type CrcdiL = crate::Reg<crcdi_l::CrcdiLSpec>;
#[doc = "CRC Data In Register, lower byte"]
pub mod crcdi_l;
#[doc = "CRCDIRB (rw) register accessor: CRC Data In Reverse Byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdirb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdirb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdirb`] module"]
#[doc(alias = "CRCDIRB")]
pub type Crcdirb = crate::Reg<crcdirb::CrcdirbSpec>;
#[doc = "CRC Data In Reverse Byte"]
pub mod crcdirb;
#[doc = "CRCDIRB_L (rw) register accessor: CRC Data In Reverse Byte Register, lower byte\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdirb_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdirb_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdirb_l`] module"]
#[doc(alias = "CRCDIRB_L")]
pub type CrcdirbL = crate::Reg<crcdirb_l::CrcdirbLSpec>;
#[doc = "CRC Data In Reverse Byte Register, lower byte"]
pub mod crcdirb_l;
#[doc = "CRCINIRES (rw) register accessor: CRC Initialization and Result\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinires::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinires::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinires`] module"]
#[doc(alias = "CRCINIRES")]
pub type Crcinires = crate::Reg<crcinires::CrciniresSpec>;
#[doc = "CRC Initialization and Result"]
pub mod crcinires;
#[doc = "CRCRESR (rw) register accessor: CRC Result Reverse\n\nYou can [`read`](crate::Reg::read) this register and get [`crcresr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcresr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcresr`] module"]
#[doc(alias = "CRCRESR")]
pub type Crcresr = crate::Reg<crcresr::CrcresrSpec>;
#[doc = "CRC Result Reverse"]
pub mod crcresr;
