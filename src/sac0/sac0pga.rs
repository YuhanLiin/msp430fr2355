#[doc = "Register `SAC0PGA` reader"]
pub type R = crate::R<Sac0pgaSpec>;
#[doc = "Register `SAC0PGA` writer"]
pub type W = crate::W<Sac0pgaSpec>;
#[doc = "SAC PGA Mode Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msel {
    #[doc = "0: Inverting PGA mode (external pad IN- is selected)"]
    Msel0 = 0,
    #[doc = "1: Buffer mode (floating is selected )"]
    Msel1 = 1,
    #[doc = "2: Non-inverting mode"]
    Msel2 = 2,
    #[doc = "3: Cascade OA Inverting mode"]
    Msel3 = 3,
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(variant: Msel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msel {
    type Ux = u8;
}
impl crate::IsEnum for Msel {}
#[doc = "Field `MSEL` reader - SAC PGA Mode Selection"]
pub type MselR = crate::FieldReader<Msel>;
impl MselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msel {
        match self.bits {
            0 => Msel::Msel0,
            1 => Msel::Msel1,
            2 => Msel::Msel2,
            3 => Msel::Msel3,
            _ => unreachable!(),
        }
    }
    #[doc = "Inverting PGA mode (external pad IN- is selected)"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == Msel::Msel0
    }
    #[doc = "Buffer mode (floating is selected )"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == Msel::Msel1
    }
    #[doc = "Non-inverting mode"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == Msel::Msel2
    }
    #[doc = "Cascade OA Inverting mode"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == Msel::Msel3
    }
}
#[doc = "Field `MSEL` writer - SAC PGA Mode Selection"]
pub type MselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Msel, crate::Safe>;
impl<'a, REG> MselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inverting PGA mode (external pad IN- is selected)"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel0)
    }
    #[doc = "Buffer mode (floating is selected )"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel1)
    }
    #[doc = "Non-inverting mode"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel2)
    }
    #[doc = "Cascade OA Inverting mode"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::Msel3)
    }
}
#[doc = "Field `GAIN` reader - SAC PGA Gain configuration"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - SAC PGA Gain configuration"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - SAC PGA Mode Selection"]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - SAC PGA Gain configuration"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAC PGA Mode Selection"]
    #[inline(always)]
    pub fn msel(&mut self) -> MselW<'_, Sac0pgaSpec> {
        MselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SAC PGA Gain configuration"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<'_, Sac0pgaSpec> {
        GainW::new(self, 4)
    }
}
#[doc = "SAC PGA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0pga::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0pga::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac0pgaSpec;
impl crate::RegisterSpec for Sac0pgaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac0pga::R`](R) reader structure"]
impl crate::Readable for Sac0pgaSpec {}
#[doc = "`write(|w| ..)` method takes [`sac0pga::W`](W) writer structure"]
impl crate::Writable for Sac0pgaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC0PGA to value 0"]
impl crate::Resettable for Sac0pgaSpec {}
