#[doc = "Register `SYSCFG3` reader"]
pub type R = crate::R<Syscfg3Spec>;
#[doc = "Register `SYSCFG3` writer"]
pub type W = crate::W<Syscfg3Spec>;
#[doc = "eUSCIA remapping source selection, please refer to device specific for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usciarmp {
    #[doc = "0: P1.x is selected, please refer to device specific for details"]
    Usciarmp0 = 0,
    #[doc = "1: other port is selected, please refer to device specific for details"]
    Usciarmp1 = 1,
}
impl From<Usciarmp> for bool {
    #[inline(always)]
    fn from(variant: Usciarmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USCIARMP` reader - eUSCIA remapping source selection, please refer to device specific for details"]
pub type UsciarmpR = crate::BitReader<Usciarmp>;
impl UsciarmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usciarmp {
        match self.bits {
            false => Usciarmp::Usciarmp0,
            true => Usciarmp::Usciarmp1,
        }
    }
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn is_usciarmp_0(&self) -> bool {
        *self == Usciarmp::Usciarmp0
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn is_usciarmp_1(&self) -> bool {
        *self == Usciarmp::Usciarmp1
    }
}
#[doc = "Field `USCIARMP` writer - eUSCIA remapping source selection, please refer to device specific for details"]
pub type UsciarmpW<'a, REG> = crate::BitWriter<'a, REG, Usciarmp>;
impl<'a, REG> UsciarmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usciarmp::Usciarmp0)
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usciarmp::Usciarmp1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp(&self) -> UsciarmpR {
        UsciarmpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp(&mut self) -> UsciarmpW<'_, Syscfg3Spec> {
        UsciarmpW::new(self, 0)
    }
}
#[doc = "System Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`syscfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg3Spec;
impl crate::RegisterSpec for Syscfg3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syscfg3::R`](R) reader structure"]
impl crate::Readable for Syscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg3::W`](W) writer structure"]
impl crate::Writable for Syscfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCFG3 to value 0"]
impl crate::Resettable for Syscfg3Spec {}
