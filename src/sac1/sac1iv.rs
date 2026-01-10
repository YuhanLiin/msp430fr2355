#[doc = "Register `SAC1IV` reader"]
pub type R = crate::R<Sac1ivSpec>;
#[doc = "Register `SAC1IV` writer"]
pub type W = crate::W<Sac1ivSpec>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Saciv1 {
    #[doc = "0: No interrupt pending"]
    Saciv0 = 0,
    #[doc = "2: S&H completed interrupt flag (Highest priority)"]
    Saciv2 = 2,
    #[doc = "4: DAC channel update interrupt flag"]
    Saciv4 = 4,
}
impl From<Saciv1> for u16 {
    #[inline(always)]
    fn from(variant: Saciv1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Saciv1 {
    type Ux = u16;
}
impl crate::IsEnum for Saciv1 {}
#[doc = "Field `SACIV1` reader - SAC Interrupt Vector Register"]
pub type Saciv1R = crate::FieldReader<Saciv1>;
impl Saciv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Saciv1> {
        match self.bits {
            0 => Some(Saciv1::Saciv0),
            2 => Some(Saciv1::Saciv2),
            4 => Some(Saciv1::Saciv4),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_saciv_0(&self) -> bool {
        *self == Saciv1::Saciv0
    }
    #[doc = "S&H completed interrupt flag (Highest priority)"]
    #[inline(always)]
    pub fn is_saciv_2(&self) -> bool {
        *self == Saciv1::Saciv2
    }
    #[doc = "DAC channel update interrupt flag"]
    #[inline(always)]
    pub fn is_saciv_4(&self) -> bool {
        *self == Saciv1::Saciv4
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv1(&self) -> Saciv1R {
        Saciv1R::new(self.bits)
    }
}
impl W {}
#[doc = "SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac1iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac1iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac1ivSpec;
impl crate::RegisterSpec for Sac1ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac1iv::R`](R) reader structure"]
impl crate::Readable for Sac1ivSpec {}
#[doc = "`write(|w| ..)` method takes [`sac1iv::W`](W) writer structure"]
impl crate::Writable for Sac1ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC1IV to value 0"]
impl crate::Resettable for Sac1ivSpec {}
