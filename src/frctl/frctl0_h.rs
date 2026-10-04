#[doc = "Register `FRCTL0_H` reader"]
pub type R = crate::R<Frctl0HSpec>;
#[doc = "Register `FRCTL0_H` writer"]
pub type W = crate::W<Frctl0HSpec>;
#[doc = "FRAM controller password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frctlpwr {
    #[doc = "150: Value always read from the FRCTL0 password"]
    Password = 150,
}
impl From<Frctlpwr> for u8 {
    #[inline(always)]
    fn from(variant: Frctlpwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frctlpwr {
    type Ux = u8;
}
impl crate::IsEnum for Frctlpwr {}
#[doc = "Field `FRCTLPW` reader - FRAM controller password"]
pub type FrctlpwR = crate::FieldReader<Frctlpwr>;
impl FrctlpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frctlpwr> {
        match self.bits {
            150 => Some(Frctlpwr::Password),
            _ => None,
        }
    }
    #[doc = "Value always read from the FRCTL0 password"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Frctlpwr::Password
    }
}
#[doc = "FRAM controller password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FrctlpwwWO {
    #[doc = "0: Locks the FRAM controller registers again"]
    Lock = 0,
    #[doc = "165: Unlocks the FRAM controller registers"]
    Password = 165,
}
impl From<FrctlpwwWO> for u8 {
    #[inline(always)]
    fn from(variant: FrctlpwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FrctlpwwWO {
    type Ux = u8;
}
impl crate::IsEnum for FrctlpwwWO {}
#[doc = "Field `FRCTLPW` writer - FRAM controller password"]
pub type FrctlpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, FrctlpwwWO>;
impl<'a, REG> FrctlpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Locks the FRAM controller registers again"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(FrctlpwwWO::Lock)
    }
    #[doc = "Unlocks the FRAM controller registers"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FrctlpwwWO::Password)
    }
}
impl R {
    #[doc = "Bits 0:7 - FRAM controller password"]
    #[inline(always)]
    pub fn frctlpw(&self) -> FrctlpwR {
        FrctlpwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - FRAM controller password"]
    #[inline(always)]
    pub fn frctlpw(&mut self) -> FrctlpwW<'_, Frctl0HSpec> {
        FrctlpwW::new(self, 0)
    }
}
#[doc = "FRAM Controller Control Register 0, upper byte\n\nYou can [`read`](crate::Reg::read) this register and get [`frctl0_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frctl0_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frctl0HSpec;
impl crate::RegisterSpec for Frctl0HSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`frctl0_h::R`](R) reader structure"]
impl crate::Readable for Frctl0HSpec {}
#[doc = "`write(|w| ..)` method takes [`frctl0_h::W`](W) writer structure"]
impl crate::Writable for Frctl0HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRCTL0_H to value 0"]
impl crate::Resettable for Frctl0HSpec {}
