#[doc = "Register `UCB1TBCNT` reader"]
pub type R = crate::R<Ucb1tbcntSpec>;
#[doc = "Register `UCB1TBCNT` writer"]
pub type W = crate::W<Ucb1tbcntSpec>;
#[doc = "Field `UCTBCNT` reader - Byte counter threshold value"]
pub type UctbcntR = crate::FieldReader;
#[doc = "Field `UCTBCNT` writer - Byte counter threshold value"]
pub type UctbcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UctbcntR {
        UctbcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UctbcntW<'_, Ucb1tbcntSpec> {
        UctbcntW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1tbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1tbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1tbcntSpec;
impl crate::RegisterSpec for Ucb1tbcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1tbcnt::R`](R) reader structure"]
impl crate::Readable for Ucb1tbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1tbcnt::W`](W) writer structure"]
impl crate::Writable for Ucb1tbcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1TBCNT to value 0"]
impl crate::Resettable for Ucb1tbcntSpec {}
