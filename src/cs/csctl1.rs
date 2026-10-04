#[doc = "Register `CSCTL1` reader"]
pub type R = crate::R<Csctl1Spec>;
#[doc = "Register `CSCTL1` writer"]
pub type W = crate::W<Csctl1Spec>;
#[doc = "Modulation. This bit enables/disables the modulation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dismod {
    #[doc = "0: Modulation enabled"]
    Dismod0 = 0,
    #[doc = "1: Modulation disabled"]
    Dismod1 = 1,
}
impl From<Dismod> for bool {
    #[inline(always)]
    fn from(variant: Dismod) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISMOD` reader - Modulation. This bit enables/disables the modulation."]
pub type DismodR = crate::BitReader<Dismod>;
impl DismodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dismod {
        match self.bits {
            false => Dismod::Dismod0,
            true => Dismod::Dismod1,
        }
    }
    #[doc = "Modulation enabled"]
    #[inline(always)]
    pub fn is_dismod_0(&self) -> bool {
        *self == Dismod::Dismod0
    }
    #[doc = "Modulation disabled"]
    #[inline(always)]
    pub fn is_dismod_1(&self) -> bool {
        *self == Dismod::Dismod1
    }
}
#[doc = "Field `DISMOD` writer - Modulation. This bit enables/disables the modulation."]
pub type DismodW<'a, REG> = crate::BitWriter<'a, REG, Dismod>;
impl<'a, REG> DismodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Modulation enabled"]
    #[inline(always)]
    pub fn dismod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dismod::Dismod0)
    }
    #[doc = "Modulation disabled"]
    #[inline(always)]
    pub fn dismod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dismod::Dismod1)
    }
}
#[doc = "DCO Range Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcorsel {
    #[doc = "0: 1 MHz"]
    Range1mhz = 0,
    #[doc = "1: 2 MHz"]
    Range2mhz = 1,
    #[doc = "2: 4 MHz"]
    Range4mhz = 2,
    #[doc = "3: 8 MHz"]
    Range8mhz = 3,
    #[doc = "4: 12 MHz"]
    Range12mhz = 4,
    #[doc = "5: 16 MHz"]
    Range16mhz = 5,
    #[doc = "6: 20 MHz"]
    Range20mhz = 6,
    #[doc = "7: 24 MHz"]
    Range24mhz = 7,
}
impl From<Dcorsel> for u8 {
    #[inline(always)]
    fn from(variant: Dcorsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcorsel {
    type Ux = u8;
}
impl crate::IsEnum for Dcorsel {}
#[doc = "Field `DCORSEL` reader - DCO Range Select"]
pub type DcorselR = crate::FieldReader<Dcorsel>;
impl DcorselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcorsel {
        match self.bits {
            0 => Dcorsel::Range1mhz,
            1 => Dcorsel::Range2mhz,
            2 => Dcorsel::Range4mhz,
            3 => Dcorsel::Range8mhz,
            4 => Dcorsel::Range12mhz,
            5 => Dcorsel::Range16mhz,
            6 => Dcorsel::Range20mhz,
            7 => Dcorsel::Range24mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn is_range_1mhz(&self) -> bool {
        *self == Dcorsel::Range1mhz
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn is_range_2mhz(&self) -> bool {
        *self == Dcorsel::Range2mhz
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn is_range_4mhz(&self) -> bool {
        *self == Dcorsel::Range4mhz
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn is_range_8mhz(&self) -> bool {
        *self == Dcorsel::Range8mhz
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_range_12mhz(&self) -> bool {
        *self == Dcorsel::Range12mhz
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn is_range_16mhz(&self) -> bool {
        *self == Dcorsel::Range16mhz
    }
    #[doc = "20 MHz"]
    #[inline(always)]
    pub fn is_range_20mhz(&self) -> bool {
        *self == Dcorsel::Range20mhz
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_range_24mhz(&self) -> bool {
        *self == Dcorsel::Range24mhz
    }
}
#[doc = "Field `DCORSEL` writer - DCO Range Select"]
pub type DcorselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dcorsel, crate::Safe>;
impl<'a, REG> DcorselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn range_1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range1mhz)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn range_2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range2mhz)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn range_4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range4mhz)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn range_8mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range8mhz)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn range_12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range12mhz)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn range_16mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range16mhz)
    }
    #[doc = "20 MHz"]
    #[inline(always)]
    pub fn range_20mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range20mhz)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn range_24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Range24mhz)
    }
}
#[doc = "Field `DCOFTRIM` reader - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
pub type DcoftrimR = crate::FieldReader;
#[doc = "Field `DCOFTRIM` writer - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
pub type DcoftrimW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcoftrimen {
    #[doc = "0: Disable frequency trim"]
    Dcoftrimen0 = 0,
    #[doc = "1: Enable frequency trim"]
    Dcoftrimen1 = 1,
}
impl From<Dcoftrimen> for bool {
    #[inline(always)]
    fn from(variant: Dcoftrimen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCOFTRIMEN` reader - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
pub type DcoftrimenR = crate::BitReader<Dcoftrimen>;
impl DcoftrimenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcoftrimen {
        match self.bits {
            false => Dcoftrimen::Dcoftrimen0,
            true => Dcoftrimen::Dcoftrimen1,
        }
    }
    #[doc = "Disable frequency trim"]
    #[inline(always)]
    pub fn is_dcoftrimen_0(&self) -> bool {
        *self == Dcoftrimen::Dcoftrimen0
    }
    #[doc = "Enable frequency trim"]
    #[inline(always)]
    pub fn is_dcoftrimen_1(&self) -> bool {
        *self == Dcoftrimen::Dcoftrimen1
    }
}
#[doc = "Field `DCOFTRIMEN` writer - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
pub type DcoftrimenW<'a, REG> = crate::BitWriter<'a, REG, Dcoftrimen>;
impl<'a, REG> DcoftrimenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable frequency trim"]
    #[inline(always)]
    pub fn dcoftrimen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrimen::Dcoftrimen0)
    }
    #[doc = "Enable frequency trim"]
    #[inline(always)]
    pub fn dcoftrimen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcoftrimen::Dcoftrimen1)
    }
}
impl R {
    #[doc = "Bit 0 - Modulation. This bit enables/disables the modulation."]
    #[inline(always)]
    pub fn dismod(&self) -> DismodR {
        DismodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DCO Range Select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DcorselR {
        DcorselR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
    #[inline(always)]
    pub fn dcoftrim(&self) -> DcoftrimR {
        DcoftrimR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
    #[inline(always)]
    pub fn dcoftrimen(&self) -> DcoftrimenR {
        DcoftrimenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulation. This bit enables/disables the modulation."]
    #[inline(always)]
    pub fn dismod(&mut self) -> DismodW<'_, Csctl1Spec> {
        DismodW::new(self, 0)
    }
    #[doc = "Bits 1:3 - DCO Range Select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DcorselW<'_, Csctl1Spec> {
        DcorselW::new(self, 1)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
    #[inline(always)]
    pub fn dcoftrim(&mut self) -> DcoftrimW<'_, Csctl1Spec> {
        DcoftrimW::new(self, 4)
    }
    #[doc = "Bit 7 - DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture."]
    #[inline(always)]
    pub fn dcoftrimen(&mut self) -> DcoftrimenW<'_, Csctl1Spec> {
        DcoftrimenW::new(self, 7)
    }
}
#[doc = "Clock System Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl1Spec;
impl crate::RegisterSpec for Csctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl1::R`](R) reader structure"]
impl crate::Readable for Csctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl1::W`](W) writer structure"]
impl crate::Writable for Csctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL1 to value 0"]
impl crate::Resettable for Csctl1Spec {}
