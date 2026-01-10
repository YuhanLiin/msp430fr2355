#[doc = "Register `SAC3IV` reader"]
pub type R = crate::R<Sac3ivSpec>;
#[doc = "Register `SAC3IV` writer"]
pub type W = crate::W<Sac3ivSpec>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Saciv3 {
    #[doc = "0: No interrupt pending"]
    Saciv0 = 0,
    #[doc = "2: S&H completed interrupt flag (Highest priority)"]
    Saciv2 = 2,
    #[doc = "4: DAC channel update interrupt flag"]
    Saciv4 = 4,
}
impl From<Saciv3> for u16 {
    #[inline(always)]
    fn from(variant: Saciv3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Saciv3 {
    type Ux = u16;
}
impl crate::IsEnum for Saciv3 {}
#[doc = "Field `SACIV3` reader - SAC Interrupt Vector Register"]
pub type Saciv3R = crate::FieldReader<Saciv3>;
impl Saciv3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Saciv3> {
        match self.bits {
            0 => Some(Saciv3::Saciv0),
            2 => Some(Saciv3::Saciv2),
            4 => Some(Saciv3::Saciv4),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_saciv_0(&self) -> bool {
        *self == Saciv3::Saciv0
    }
    #[doc = "S&H completed interrupt flag (Highest priority)"]
    #[inline(always)]
    pub fn is_saciv_2(&self) -> bool {
        *self == Saciv3::Saciv2
    }
    #[doc = "DAC channel update interrupt flag"]
    #[inline(always)]
    pub fn is_saciv_4(&self) -> bool {
        *self == Saciv3::Saciv4
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv3(&self) -> Saciv3R {
        Saciv3R::new(self.bits)
    }
}
impl W {}
#[doc = "SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac3iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac3iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac3ivSpec;
impl crate::RegisterSpec for Sac3ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac3iv::R`](R) reader structure"]
impl crate::Readable for Sac3ivSpec {}
#[doc = "`write(|w| ..)` method takes [`sac3iv::W`](W) writer structure"]
impl crate::Writable for Sac3ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC3IV to value 0"]
impl crate::Resettable for Sac3ivSpec {}
