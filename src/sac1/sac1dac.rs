#[doc = "Register `SAC1DAC` reader"]
pub type R = crate::R<Sac1dacSpec>;
#[doc = "Register `SAC1DAC` writer"]
pub type W = crate::W<Sac1dacSpec>;
#[doc = "SAC DAC enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacen {
    #[doc = "0: Disabled"]
    Dacen0 = 0,
    #[doc = "1: Enabled"]
    Dacen1 = 1,
}
impl From<Dacen> for bool {
    #[inline(always)]
    fn from(variant: Dacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - SAC DAC enable"]
pub type DacenR = crate::BitReader<Dacen>;
impl DacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacen {
        match self.bits {
            false => Dacen::Dacen0,
            true => Dacen::Dacen1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dacen_0(&self) -> bool {
        *self == Dacen::Dacen0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dacen_1(&self) -> bool {
        *self == Dacen::Dacen1
    }
}
#[doc = "Field `DACEN` writer - SAC DAC enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG, Dacen>;
impl<'a, REG> DacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dacen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::Dacen0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dacen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::Dacen1)
    }
}
#[doc = "SAC DAC interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacie {
    #[doc = "0: Disabled"]
    Dacie0 = 0,
    #[doc = "1: Enabled"]
    Dacie1 = 1,
}
impl From<Dacie> for bool {
    #[inline(always)]
    fn from(variant: Dacie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACIE` reader - SAC DAC interrupt enable"]
pub type DacieR = crate::BitReader<Dacie>;
impl DacieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacie {
        match self.bits {
            false => Dacie::Dacie0,
            true => Dacie::Dacie1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dacie_0(&self) -> bool {
        *self == Dacie::Dacie0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_dacie_1(&self) -> bool {
        *self == Dacie::Dacie1
    }
}
#[doc = "Field `DACIE` writer - SAC DAC interrupt enable"]
pub type DacieW<'a, REG> = crate::BitWriter<'a, REG, Dacie>;
impl<'a, REG> DacieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dacie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacie::Dacie0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dacie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacie::Dacie1)
    }
}
#[doc = "SAC DAC DMA request enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacdmae {
    #[doc = "0: DMA request disabled"]
    Dacdmae0 = 0,
    #[doc = "1: DMA request enabled"]
    Dacdmae1 = 1,
}
impl From<Dacdmae> for bool {
    #[inline(always)]
    fn from(variant: Dacdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACDMAE` reader - SAC DAC DMA request enable"]
pub type DacdmaeR = crate::BitReader<Dacdmae>;
impl DacdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacdmae {
        match self.bits {
            false => Dacdmae::Dacdmae0,
            true => Dacdmae::Dacdmae1,
        }
    }
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn is_dacdmae_0(&self) -> bool {
        *self == Dacdmae::Dacdmae0
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn is_dacdmae_1(&self) -> bool {
        *self == Dacdmae::Dacdmae1
    }
}
#[doc = "Field `DACDMAE` writer - SAC DAC DMA request enable"]
pub type DacdmaeW<'a, REG> = crate::BitWriter<'a, REG, Dacdmae>;
impl<'a, REG> DacdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request disabled"]
    #[inline(always)]
    pub fn dacdmae_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacdmae::Dacdmae0)
    }
    #[doc = "DMA request enabled"]
    #[inline(always)]
    pub fn dacdmae_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacdmae::Dacdmae1)
    }
}
#[doc = "SAC DAC load select. Selects the load trigger for the DAC latch.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Daclsel {
    #[doc = "0: DAC latch loads when DACDAT written"]
    Daclsel0 = 0,
    #[doc = "2: Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    Daclsel2 = 2,
    #[doc = "3: Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    Daclsel3 = 3,
}
impl From<Daclsel> for u8 {
    #[inline(always)]
    fn from(variant: Daclsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Daclsel {
    type Ux = u8;
}
impl crate::IsEnum for Daclsel {}
#[doc = "Field `DACLSEL` reader - SAC DAC load select. Selects the load trigger for the DAC latch."]
pub type DaclselR = crate::FieldReader<Daclsel>;
impl DaclselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Daclsel> {
        match self.bits {
            0 => Some(Daclsel::Daclsel0),
            2 => Some(Daclsel::Daclsel2),
            3 => Some(Daclsel::Daclsel3),
            _ => None,
        }
    }
    #[doc = "DAC latch loads when DACDAT written"]
    #[inline(always)]
    pub fn is_daclsel_0(&self) -> bool {
        *self == Daclsel::Daclsel0
    }
    #[doc = "Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn is_daclsel_2(&self) -> bool {
        *self == Daclsel::Daclsel2
    }
    #[doc = "Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn is_daclsel_3(&self) -> bool {
        *self == Daclsel::Daclsel3
    }
}
#[doc = "Field `DACLSEL` writer - SAC DAC load select. Selects the load trigger for the DAC latch."]
pub type DaclselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Daclsel>;
impl<'a, REG> DaclselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DAC latch loads when DACDAT written"]
    #[inline(always)]
    pub fn daclsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Daclsel::Daclsel0)
    }
    #[doc = "Device specific 0. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn daclsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Daclsel::Daclsel2)
    }
    #[doc = "Device specific 1. DAC always loads data from DACDAT at the positive edge of this signal"]
    #[inline(always)]
    pub fn daclsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Daclsel::Daclsel3)
    }
}
#[doc = "SAC DAC select reference voltage\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacsref {
    #[doc = "0: AVCC"]
    Dacsref0 = 0,
    #[doc = "1: Alternative reference"]
    Dacsref1 = 1,
}
impl From<Dacsref> for bool {
    #[inline(always)]
    fn from(variant: Dacsref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSREF` reader - SAC DAC select reference voltage"]
pub type DacsrefR = crate::BitReader<Dacsref>;
impl DacsrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacsref {
        match self.bits {
            false => Dacsref::Dacsref0,
            true => Dacsref::Dacsref1,
        }
    }
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn is_dacsref_0(&self) -> bool {
        *self == Dacsref::Dacsref0
    }
    #[doc = "Alternative reference"]
    #[inline(always)]
    pub fn is_dacsref_1(&self) -> bool {
        *self == Dacsref::Dacsref1
    }
}
#[doc = "Field `DACSREF` writer - SAC DAC select reference voltage"]
pub type DacsrefW<'a, REG> = crate::BitWriter<'a, REG, Dacsref>;
impl<'a, REG> DacsrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AVCC"]
    #[inline(always)]
    pub fn dacsref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacsref::Dacsref0)
    }
    #[doc = "Alternative reference"]
    #[inline(always)]
    pub fn dacsref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacsref::Dacsref1)
    }
}
impl R {
    #[doc = "Bit 0 - SAC DAC enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SAC DAC interrupt enable"]
    #[inline(always)]
    pub fn dacie(&self) -> DacieR {
        DacieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SAC DAC DMA request enable"]
    #[inline(always)]
    pub fn dacdmae(&self) -> DacdmaeR {
        DacdmaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SAC DAC load select. Selects the load trigger for the DAC latch."]
    #[inline(always)]
    pub fn daclsel(&self) -> DaclselR {
        DaclselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SAC DAC select reference voltage"]
    #[inline(always)]
    pub fn dacsref(&self) -> DacsrefR {
        DacsrefR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SAC DAC enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DacenW<'_, Sac1dacSpec> {
        DacenW::new(self, 0)
    }
    #[doc = "Bit 1 - SAC DAC interrupt enable"]
    #[inline(always)]
    pub fn dacie(&mut self) -> DacieW<'_, Sac1dacSpec> {
        DacieW::new(self, 1)
    }
    #[doc = "Bit 2 - SAC DAC DMA request enable"]
    #[inline(always)]
    pub fn dacdmae(&mut self) -> DacdmaeW<'_, Sac1dacSpec> {
        DacdmaeW::new(self, 2)
    }
    #[doc = "Bits 8:9 - SAC DAC load select. Selects the load trigger for the DAC latch."]
    #[inline(always)]
    pub fn daclsel(&mut self) -> DaclselW<'_, Sac1dacSpec> {
        DaclselW::new(self, 8)
    }
    #[doc = "Bit 12 - SAC DAC select reference voltage"]
    #[inline(always)]
    pub fn dacsref(&mut self) -> DacsrefW<'_, Sac1dacSpec> {
        DacsrefW::new(self, 12)
    }
}
#[doc = "SAC DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac1dacSpec;
impl crate::RegisterSpec for Sac1dacSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac1dac::R`](R) reader structure"]
impl crate::Readable for Sac1dacSpec {}
#[doc = "`write(|w| ..)` method takes [`sac1dac::W`](W) writer structure"]
impl crate::Writable for Sac1dacSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC1DAC to value 0"]
impl crate::Resettable for Sac1dacSpec {}
