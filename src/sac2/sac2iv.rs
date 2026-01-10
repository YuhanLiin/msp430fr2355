#[doc = "Register `SAC2IV` reader"]
pub type R = crate::R<Sac2ivSpec>;
#[doc = "Register `SAC2IV` writer"]
pub type W = crate::W<Sac2ivSpec>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Saciv2 {
    #[doc = "0: No interrupt pending"]
    Saciv0 = 0,
    #[doc = "2: S&H completed interrupt flag (Highest priority)"]
    Saciv2 = 2,
    #[doc = "4: DAC channel update interrupt flag"]
    Saciv4 = 4,
}
impl From<Saciv2> for u16 {
    #[inline(always)]
    fn from(variant: Saciv2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Saciv2 {
    type Ux = u16;
}
impl crate::IsEnum for Saciv2 {}
#[doc = "Field `SACIV2` reader - SAC Interrupt Vector Register"]
pub type Saciv2R = crate::FieldReader<Saciv2>;
impl Saciv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Saciv2> {
        match self.bits {
            0 => Some(Saciv2::Saciv0),
            2 => Some(Saciv2::Saciv2),
            4 => Some(Saciv2::Saciv4),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_saciv_0(&self) -> bool {
        *self == Saciv2::Saciv0
    }
    #[doc = "S&H completed interrupt flag (Highest priority)"]
    #[inline(always)]
    pub fn is_saciv_2(&self) -> bool {
        *self == Saciv2::Saciv2
    }
    #[doc = "DAC channel update interrupt flag"]
    #[inline(always)]
    pub fn is_saciv_4(&self) -> bool {
        *self == Saciv2::Saciv4
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv2(&self) -> Saciv2R {
        Saciv2R::new(self.bits)
    }
}
impl W {}
#[doc = "SAC Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sac2iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sac2iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sac2ivSpec;
impl crate::RegisterSpec for Sac2ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sac2iv::R`](R) reader structure"]
impl crate::Readable for Sac2ivSpec {}
#[doc = "`write(|w| ..)` method takes [`sac2iv::W`](W) writer structure"]
impl crate::Writable for Sac2ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAC2IV to value 0"]
impl crate::Resettable for Sac2ivSpec {}
