#[doc = "Register `CSCTL1` reader"]
pub type R = crate::R<Csctl1Spec>;
#[doc = "Register `CSCTL1` writer"]
pub type W = crate::W<Csctl1Spec>;
#[doc = "Modulation. This bit enables/disables the modulation.\n\nValue on reset: 0"]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcorsel {
    #[doc = "0: 1 MHz"]
    Dcorsel0 = 0,
    #[doc = "1: 2 MHz"]
    Dcorsel1 = 1,
    #[doc = "2: 4 MHz"]
    Dcorsel2 = 2,
    #[doc = "3: 8 MHz"]
    Dcorsel3 = 3,
    #[doc = "4: 12 MHz"]
    Dcorsel4 = 4,
    #[doc = "5: 16 MHz"]
    Dcorsel5 = 5,
    #[doc = "6: 20 MHz(Only avaliable in 24MHz clock system)"]
    Dcorsel6 = 6,
    #[doc = "7: 24 MHz(Only avaliable in 24MHz clock system)"]
    Dcorsel7 = 7,
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
            0 => Dcorsel::Dcorsel0,
            1 => Dcorsel::Dcorsel1,
            2 => Dcorsel::Dcorsel2,
            3 => Dcorsel::Dcorsel3,
            4 => Dcorsel::Dcorsel4,
            5 => Dcorsel::Dcorsel5,
            6 => Dcorsel::Dcorsel6,
            7 => Dcorsel::Dcorsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == Dcorsel::Dcorsel0
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == Dcorsel::Dcorsel1
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == Dcorsel::Dcorsel2
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == Dcorsel::Dcorsel3
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == Dcorsel::Dcorsel4
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == Dcorsel::Dcorsel5
    }
    #[doc = "20 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == Dcorsel::Dcorsel6
    }
    #[doc = "24 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == Dcorsel::Dcorsel7
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
    pub fn dcorsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel0)
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel1)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel2)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel3)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel4)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel5)
    }
    #[doc = "20 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel6)
    }
    #[doc = "24 MHz(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Dcorsel::Dcorsel7)
    }
}
#[doc = "Field `DCOFTRIM` reader - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
pub type DcoftrimR = crate::FieldReader;
#[doc = "Field `DCOFTRIM` writer - DCO frequency trim. These bits trims the DCO frequency. By default, it is chipspecific trimmed. These bits can also be trimmed by user code."]
pub type DcoftrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "DCO Frequency Trim Enable. When this bit is set, DCOFTRIM value is selected to set DCO frequency. Otherwise, DCOFTRIM value is bypassed and DCO applies default settings in manufacture.\n\nValue on reset: 0"]
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
