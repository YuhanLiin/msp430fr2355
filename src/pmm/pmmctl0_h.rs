#[doc = "Register `PMMCTL0_H` reader"]
pub type R = crate::R<Pmmctl0HSpec>;
#[doc = "Register `PMMCTL0_H` writer"]
pub type W = crate::W<Pmmctl0HSpec>;
#[doc = "PMM password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmmpwr {
    #[doc = "150: Value always read from the PMM password"]
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
#[doc = "Field `PMMPW` reader - PMM password"]
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
    #[doc = "Value always read from the PMM password"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Pmmpwr::Password
    }
}
#[doc = "PMM password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmmpwwWO {
    #[doc = "0: Locks the PMM registers again"]
    Lock = 0,
    #[doc = "165: Unlocks the PMM registers"]
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
#[doc = "Field `PMMPW` writer - PMM password"]
pub type PmmpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, PmmpwwWO>;
impl<'a, REG> PmmpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Locks the PMM registers again"]
    #[inline(always)]
    pub fn lock(self) -> &'a mut crate::W<REG> {
        self.variant(PmmpwwWO::Lock)
    }
    #[doc = "Unlocks the PMM registers"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(PmmpwwWO::Password)
    }
}
impl R {
    #[doc = "Bits 0:7 - PMM password"]
    #[inline(always)]
    pub fn pmmpw(&self) -> PmmpwR {
        PmmpwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - PMM password"]
    #[inline(always)]
    pub fn pmmpw(&mut self) -> PmmpwW<'_, Pmmctl0HSpec> {
        PmmpwW::new(self, 0)
    }
}
#[doc = "Power Management Module Control Register 0, upper byte\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmctl0_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmctl0_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmmctl0HSpec;
impl crate::RegisterSpec for Pmmctl0HSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmmctl0_h::R`](R) reader structure"]
impl crate::Readable for Pmmctl0HSpec {}
#[doc = "`write(|w| ..)` method takes [`pmmctl0_h::W`](W) writer structure"]
impl crate::Writable for Pmmctl0HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMCTL0_H to value 0"]
impl crate::Resettable for Pmmctl0HSpec {}
