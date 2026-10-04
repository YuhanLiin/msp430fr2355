#[doc = "Register `PM5CTL0` reader"]
pub type R = crate::R<Pm5ctl0Spec>;
#[doc = "Register `PM5CTL0` writer"]
pub type W = crate::W<Pm5ctl0Spec>;
#[doc = "LPMx.5 Lock Bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locklpm5 {
    #[doc = "0: LPMx.5 configuration is not locked and defaults to its reset condition."]
    Locklpm5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    Locklpm5_1 = 1,
}
impl From<Locklpm5> for bool {
    #[inline(always)]
    fn from(variant: Locklpm5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKLPM5` reader - LPMx.5 Lock Bit"]
pub type Locklpm5R = crate::BitReader<Locklpm5>;
impl Locklpm5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locklpm5 {
        match self.bits {
            false => Locklpm5::Locklpm5_0,
            true => Locklpm5::Locklpm5_1,
        }
    }
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == Locklpm5::Locklpm5_0
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == Locklpm5::Locklpm5_1
    }
}
#[doc = "Field `LOCKLPM5` writer - LPMx.5 Lock Bit"]
pub type Locklpm5W<'a, REG> = crate::BitWriter<'a, REG, Locklpm5>;
impl<'a, REG> Locklpm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Locklpm5::Locklpm5_0)
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Locklpm5::Locklpm5_1)
    }
}
impl R {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&self) -> Locklpm5R {
        Locklpm5R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> Locklpm5W<'_, Pm5ctl0Spec> {
        Locklpm5W::new(self, 0)
    }
}
#[doc = "Power mode 5 control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pm5ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm5ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pm5ctl0Spec;
impl crate::RegisterSpec for Pm5ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pm5ctl0::R`](R) reader structure"]
impl crate::Readable for Pm5ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pm5ctl0::W`](W) writer structure"]
impl crate::Writable for Pm5ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PM5CTL0 to value 0"]
impl crate::Resettable for Pm5ctl0Spec {}
