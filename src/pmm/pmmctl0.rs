#[doc = "Register `PMMCTL0` reader"]
pub type R = crate::R<Pmmctl0Spec>;
#[doc = "Register `PMMCTL0` writer"]
pub type W = crate::W<Pmmctl0Spec>;
#[doc = "Software brownout reset.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmswbor {
    #[doc = "0: Normal operation"]
    Pmmswbor0 = 0,
    #[doc = "1: Set to 1 to trigger a BOR"]
    Pmmswbor1 = 1,
}
impl From<Pmmswbor> for bool {
    #[inline(always)]
    fn from(variant: Pmmswbor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMSWBOR` reader - Software brownout reset."]
pub type PmmswborR = crate::BitReader<Pmmswbor>;
impl PmmswborR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmswbor {
        match self.bits {
            false => Pmmswbor::Pmmswbor0,
            true => Pmmswbor::Pmmswbor1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_pmmswbor_0(&self) -> bool {
        *self == Pmmswbor::Pmmswbor0
    }
    #[doc = "Set to 1 to trigger a BOR"]
    #[inline(always)]
    pub fn is_pmmswbor_1(&self) -> bool {
        *self == Pmmswbor::Pmmswbor1
    }
}
#[doc = "Field `PMMSWBOR` writer - Software brownout reset."]
pub type PmmswborW<'a, REG> = crate::BitWriter<'a, REG, Pmmswbor>;
impl<'a, REG> PmmswborW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pmmswbor_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmswbor::Pmmswbor0)
    }
    #[doc = "Set to 1 to trigger a BOR"]
    #[inline(always)]
    pub fn pmmswbor_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmswbor::Pmmswbor1)
    }
}
#[doc = "Software POR.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmswpor {
    #[doc = "0: Normal operation"]
    Pmmswpor0 = 0,
    #[doc = "1: Set to 1 to trigger a POR"]
    Pmmswpor1 = 1,
}
impl From<Pmmswpor> for bool {
    #[inline(always)]
    fn from(variant: Pmmswpor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMSWPOR` reader - Software POR."]
pub type PmmswporR = crate::BitReader<Pmmswpor>;
impl PmmswporR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmswpor {
        match self.bits {
            false => Pmmswpor::Pmmswpor0,
            true => Pmmswpor::Pmmswpor1,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_pmmswpor_0(&self) -> bool {
        *self == Pmmswpor::Pmmswpor0
    }
    #[doc = "Set to 1 to trigger a POR"]
    #[inline(always)]
    pub fn is_pmmswpor_1(&self) -> bool {
        *self == Pmmswpor::Pmmswpor1
    }
}
#[doc = "Field `PMMSWPOR` writer - Software POR."]
pub type PmmswporW<'a, REG> = crate::BitWriter<'a, REG, Pmmswpor>;
impl<'a, REG> PmmswporW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn pmmswpor_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmswpor::Pmmswpor0)
    }
    #[doc = "Set to 1 to trigger a POR"]
    #[inline(always)]
    pub fn pmmswpor_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmswpor::Pmmswpor1)
    }
}
#[doc = "Regulator off\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmmregoff {
    #[doc = "0: Regulator remains on when going into LPM3 or LPM4"]
    Pmmregoff0 = 0,
    #[doc = "1: Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    Pmmregoff1 = 1,
}
impl From<Pmmregoff> for bool {
    #[inline(always)]
    fn from(variant: Pmmregoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMMREGOFF` reader - Regulator off"]
pub type PmmregoffR = crate::BitReader<Pmmregoff>;
impl PmmregoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmmregoff {
        match self.bits {
            false => Pmmregoff::Pmmregoff0,
            true => Pmmregoff::Pmmregoff1,
        }
    }
    #[doc = "Regulator remains on when going into LPM3 or LPM4"]
    #[inline(always)]
    pub fn is_pmmregoff_0(&self) -> bool {
        *self == Pmmregoff::Pmmregoff0
    }
    #[doc = "Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    #[inline(always)]
    pub fn is_pmmregoff_1(&self) -> bool {
        *self == Pmmregoff::Pmmregoff1
    }
}
#[doc = "Field `PMMREGOFF` writer - Regulator off"]
pub type PmmregoffW<'a, REG> = crate::BitWriter<'a, REG, Pmmregoff>;
impl<'a, REG> PmmregoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regulator remains on when going into LPM3 or LPM4"]
    #[inline(always)]
    pub fn pmmregoff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmregoff::Pmmregoff0)
    }
    #[doc = "Regulator is turned off when going to LPM3 or LPM4. System enters LPM3.5 or LPM4.5, respectively."]
    #[inline(always)]
    pub fn pmmregoff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmmregoff::Pmmregoff1)
    }
}
#[doc = "High-side SVS enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svshe {
    #[doc = "0: High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    Disabled = 0,
    #[doc = "1: SVSH is always enabled."]
    Enabled = 1,
}
impl From<Svshe> for bool {
    #[inline(always)]
    fn from(variant: Svshe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSHE` reader - High-side SVS enable."]
pub type SvsheR = crate::BitReader<Svshe>;
impl SvsheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svshe {
        match self.bits {
            false => Svshe::Disabled,
            true => Svshe::Enabled,
        }
    }
    #[doc = "High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Svshe::Disabled
    }
    #[doc = "SVSH is always enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Svshe::Enabled
    }
}
#[doc = "Field `SVSHE` writer - High-side SVS enable."]
pub type SvsheW<'a, REG> = crate::BitWriter<'a, REG, Svshe>;
impl<'a, REG> SvsheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High-side SVS (SVSH) is disabled in LPM2, LPM3, LPM4, LPM3.5, and LPM4.5. SVSH is always enabled in active mode, LPM0, and LPM1."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Svshe::Disabled)
    }
    #[doc = "SVSH is always enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Svshe::Enabled)
    }
}
#[doc = "PMM password.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmmpwr {
    #[doc = "150: Values always reads from the PMMCTL0 register"]
    Password = 150,
}
impl From<Pmmpwr> for u8 {
    #[inline(always)]
    fn from(variant: Pmmpwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmmpwr {
    type Ux = u8;
}
impl crate::IsEnum for Pmmpwr {}
#[doc = "Field `PMMPW` reader - PMM password."]
pub type PmmpwR = crate::FieldReader<Pmmpwr>;
impl PmmpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmmpwr> {
        match self.bits {
            150 => Some(Pmmpwr::Password),
            _ => None,
        }
    }
    #[doc = "Values always reads from the PMMCTL0 register"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Pmmpwr::Password
    }
}
#[doc = "PMM password.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmmpwwWO {
    #[doc = "165: Values which must be written to the PMMCTL0 register"]
    Password = 165,
}
impl From<PmmpwwWO> for u8 {
    #[inline(always)]
    fn from(variant: PmmpwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmmpwwWO {
    type Ux = u8;
}
impl crate::IsEnum for PmmpwwWO {}
#[doc = "Field `PMMPW` writer - PMM password."]
pub type PmmpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, PmmpwwWO>;
impl<'a, REG> PmmpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Values which must be written to the PMMCTL0 register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(PmmpwwWO::Password)
    }
}
impl R {
    #[doc = "Bit 2 - Software brownout reset."]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PmmswborR {
        PmmswborR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software POR."]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PmmswporR {
        PmmswporR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PmmregoffR {
        PmmregoffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - High-side SVS enable."]
    #[inline(always)]
    pub fn svshe(&self) -> SvsheR {
        SvsheR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PMM password."]
    #[inline(always)]
    pub fn pmmpw(&self) -> PmmpwR {
        PmmpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Software brownout reset."]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PmmswborW<'_, Pmmctl0Spec> {
        PmmswborW::new(self, 2)
    }
    #[doc = "Bit 3 - Software POR."]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PmmswporW<'_, Pmmctl0Spec> {
        PmmswporW::new(self, 3)
    }
    #[doc = "Bit 4 - Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PmmregoffW<'_, Pmmctl0Spec> {
        PmmregoffW::new(self, 4)
    }
    #[doc = "Bit 6 - High-side SVS enable."]
    #[inline(always)]
    pub fn svshe(&mut self) -> SvsheW<'_, Pmmctl0Spec> {
        SvsheW::new(self, 6)
    }
    #[doc = "Bits 8:15 - PMM password."]
    #[inline(always)]
    pub fn pmmpw(&mut self) -> PmmpwW<'_, Pmmctl0Spec> {
        PmmpwW::new(self, 8)
    }
}
#[doc = "Power Management Module control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl0Spec;
impl crate::RegisterSpec for Pmmctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmmctl0::R`](R) reader structure"]
impl crate::Readable for Pmmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl0::W`](W) writer structure"]
impl crate::Writable for Pmmctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for Pmmctl0Spec {}
