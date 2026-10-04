#[doc = "Register `CP0DACCTL` reader"]
pub type R = crate::R<Cp0dacctlSpec>;
#[doc = "Register `CP0DACCTL` writer"]
pub type W = crate::W<Cp0dacctlSpec>;
#[doc = "This bit is only valid when CPDACBUFS is set to 1.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpdacsw {
    #[doc = "0: CPDACBUF1 selected"]
    Cpdacsw0 = 0,
    #[doc = "1: CPDACBUF2 selected"]
    Cpdacsw1 = 1,
}
impl From<Cpdacsw> for bool {
    #[inline(always)]
    fn from(variant: Cpdacsw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPDACSW` reader - This bit is only valid when CPDACBUFS is set to 1."]
pub type CpdacswR = crate::BitReader<Cpdacsw>;
impl CpdacswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpdacsw {
        match self.bits {
            false => Cpdacsw::Cpdacsw0,
            true => Cpdacsw::Cpdacsw1,
        }
    }
    #[doc = "CPDACBUF1 selected"]
    #[inline(always)]
    pub fn is_cpdacsw_0(&self) -> bool {
        *self == Cpdacsw::Cpdacsw0
    }
    #[doc = "CPDACBUF2 selected"]
    #[inline(always)]
    pub fn is_cpdacsw_1(&self) -> bool {
        *self == Cpdacsw::Cpdacsw1
    }
}
#[doc = "Field `CPDACSW` writer - This bit is only valid when CPDACBUFS is set to 1."]
pub type CpdacswW<'a, REG> = crate::BitWriter<'a, REG, Cpdacsw>;
impl<'a, REG> CpdacswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPDACBUF1 selected"]
    #[inline(always)]
    pub fn cpdacsw_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacsw::Cpdacsw0)
    }
    #[doc = "CPDACBUF2 selected"]
    #[inline(always)]
    pub fn cpdacsw_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacsw::Cpdacsw1)
    }
}
#[doc = "Comparator built-in DAC buffer controlled source selection.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpdacbufs {
    #[doc = "0: Comparator output is selected as the buffer control source"]
    Cpdacbufs0 = 0,
    #[doc = "1: CPDACSW bit is selected as the buffer control source"]
    Cpdacbufs1 = 1,
}
impl From<Cpdacbufs> for bool {
    #[inline(always)]
    fn from(variant: Cpdacbufs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPDACBUFS` reader - Comparator built-in DAC buffer controlled source selection."]
pub type CpdacbufsR = crate::BitReader<Cpdacbufs>;
impl CpdacbufsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpdacbufs {
        match self.bits {
            false => Cpdacbufs::Cpdacbufs0,
            true => Cpdacbufs::Cpdacbufs1,
        }
    }
    #[doc = "Comparator output is selected as the buffer control source"]
    #[inline(always)]
    pub fn is_cpdacbufs_0(&self) -> bool {
        *self == Cpdacbufs::Cpdacbufs0
    }
    #[doc = "CPDACSW bit is selected as the buffer control source"]
    #[inline(always)]
    pub fn is_cpdacbufs_1(&self) -> bool {
        *self == Cpdacbufs::Cpdacbufs1
    }
}
#[doc = "Field `CPDACBUFS` writer - Comparator built-in DAC buffer controlled source selection."]
pub type CpdacbufsW<'a, REG> = crate::BitWriter<'a, REG, Cpdacbufs>;
impl<'a, REG> CpdacbufsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator output is selected as the buffer control source"]
    #[inline(always)]
    pub fn cpdacbufs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbufs::Cpdacbufs0)
    }
    #[doc = "CPDACSW bit is selected as the buffer control source"]
    #[inline(always)]
    pub fn cpdacbufs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbufs::Cpdacbufs1)
    }
}
#[doc = "Comparator built-in DAC reference voltage selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpdacrefs {
    #[doc = "0: VDD selected"]
    Cpdacrefs0 = 0,
    #[doc = "1: on-chip VREF selected"]
    Cpdacrefs1 = 1,
}
impl From<Cpdacrefs> for bool {
    #[inline(always)]
    fn from(variant: Cpdacrefs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPDACREFS` reader - Comparator built-in DAC reference voltage selection"]
pub type CpdacrefsR = crate::BitReader<Cpdacrefs>;
impl CpdacrefsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpdacrefs {
        match self.bits {
            false => Cpdacrefs::Cpdacrefs0,
            true => Cpdacrefs::Cpdacrefs1,
        }
    }
    #[doc = "VDD selected"]
    #[inline(always)]
    pub fn is_cpdacrefs_0(&self) -> bool {
        *self == Cpdacrefs::Cpdacrefs0
    }
    #[doc = "on-chip VREF selected"]
    #[inline(always)]
    pub fn is_cpdacrefs_1(&self) -> bool {
        *self == Cpdacrefs::Cpdacrefs1
    }
}
#[doc = "Field `CPDACREFS` writer - Comparator built-in DAC reference voltage selection"]
pub type CpdacrefsW<'a, REG> = crate::BitWriter<'a, REG, Cpdacrefs>;
impl<'a, REG> CpdacrefsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDD selected"]
    #[inline(always)]
    pub fn cpdacrefs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacrefs::Cpdacrefs0)
    }
    #[doc = "on-chip VREF selected"]
    #[inline(always)]
    pub fn cpdacrefs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacrefs::Cpdacrefs1)
    }
}
#[doc = "Comparator built-in DAC output control bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpdacen {
    #[doc = "0: DAC output is disabled."]
    Cpdacen0 = 0,
    #[doc = "1: DAC output is enabled."]
    Cpdacen1 = 1,
}
impl From<Cpdacen> for bool {
    #[inline(always)]
    fn from(variant: Cpdacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPDACEN` reader - Comparator built-in DAC output control bit."]
pub type CpdacenR = crate::BitReader<Cpdacen>;
impl CpdacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpdacen {
        match self.bits {
            false => Cpdacen::Cpdacen0,
            true => Cpdacen::Cpdacen1,
        }
    }
    #[doc = "DAC output is disabled."]
    #[inline(always)]
    pub fn is_cpdacen_0(&self) -> bool {
        *self == Cpdacen::Cpdacen0
    }
    #[doc = "DAC output is enabled."]
    #[inline(always)]
    pub fn is_cpdacen_1(&self) -> bool {
        *self == Cpdacen::Cpdacen1
    }
}
#[doc = "Field `CPDACEN` writer - Comparator built-in DAC output control bit."]
pub type CpdacenW<'a, REG> = crate::BitWriter<'a, REG, Cpdacen>;
impl<'a, REG> CpdacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC output is disabled."]
    #[inline(always)]
    pub fn cpdacen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacen::Cpdacen0)
    }
    #[doc = "DAC output is enabled."]
    #[inline(always)]
    pub fn cpdacen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacen::Cpdacen1)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is only valid when CPDACBUFS is set to 1."]
    #[inline(always)]
    pub fn cpdacsw(&self) -> CpdacswR {
        CpdacswR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator built-in DAC buffer controlled source selection."]
    #[inline(always)]
    pub fn cpdacbufs(&self) -> CpdacbufsR {
        CpdacbufsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator built-in DAC reference voltage selection"]
    #[inline(always)]
    pub fn cpdacrefs(&self) -> CpdacrefsR {
        CpdacrefsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator built-in DAC output control bit."]
    #[inline(always)]
    pub fn cpdacen(&self) -> CpdacenR {
        CpdacenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is only valid when CPDACBUFS is set to 1."]
    #[inline(always)]
    pub fn cpdacsw(&mut self) -> CpdacswW<'_, Cp0dacctlSpec> {
        CpdacswW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator built-in DAC buffer controlled source selection."]
    #[inline(always)]
    pub fn cpdacbufs(&mut self) -> CpdacbufsW<'_, Cp0dacctlSpec> {
        CpdacbufsW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator built-in DAC reference voltage selection"]
    #[inline(always)]
    pub fn cpdacrefs(&mut self) -> CpdacrefsW<'_, Cp0dacctlSpec> {
        CpdacrefsW::new(self, 2)
    }
    #[doc = "Bit 7 - Comparator built-in DAC output control bit."]
    #[inline(always)]
    pub fn cpdacen(&mut self) -> CpdacenW<'_, Cp0dacctlSpec> {
        CpdacenW::new(self, 7)
    }
}
#[doc = "6-bit Comparator built-in DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cp0dacctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cp0dacctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cp0dacctlSpec;
impl crate::RegisterSpec for Cp0dacctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cp0dacctl::R`](R) reader structure"]
impl crate::Readable for Cp0dacctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cp0dacctl::W`](W) writer structure"]
impl crate::Writable for Cp0dacctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CP0DACCTL to value 0"]
impl crate::Resettable for Cp0dacctlSpec {}
