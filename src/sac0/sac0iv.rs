#[doc = "Register `SAC0IV` reader"]
pub type R = crate::R<Sac0ivSpec>;
#[doc = "Register `SAC0IV` writer"]
pub type W = crate::W<Sac0ivSpec>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Saciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "4: DAC channel update interrupt flag"]
    Dacifg = 4,
}
impl From<Saciv> for u16 {
    #[inline(always)]
    fn from(variant: Saciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Saciv {
    type Ux = u16;
}
impl crate::IsEnum for Saciv {}
#[doc = "Field `SACIV` reader - SAC Interrupt Vector Register"]
pub type SacivR = crate::FieldReader<Saciv>;
impl SacivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Saciv> {
        match self.bits {
            0 => Some(Saciv::None),
            4 => Some(Saciv::Dacifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Saciv::None
    }
    #[doc = "DAC channel update interrupt flag"]
    #[inline(always)]
    pub fn is_dacifg(&self) -> bool {
        *self == Saciv::Dacifg
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv(&self) -> SacivR {
        SacivR::new(self.bits)
    }
}
impl W {}
#[doc = "SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac0iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac0iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac0ivSpec;
impl crate::RegisterSpec for Sac0ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac0iv::R`](R) reader structure"]
impl crate::Readable for Sac0ivSpec {}
#[doc = "`write(|w| ..)` method takes [`sac0iv::W`](W) writer structure"]
impl crate::Writable for Sac0ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC0IV to value 0"]
impl crate::Resettable for Sac0ivSpec {}
