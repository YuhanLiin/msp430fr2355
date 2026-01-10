#[doc = "Register `UCB1TXBUF` reader"]
pub type R = crate::R<Ucb1txbufSpec>;
#[doc = "Register `UCB1TXBUF` writer"]
pub type W = crate::W<Ucb1txbufSpec>;
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub type UctxbufR = crate::FieldReader;
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub type UctxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UctxbufR {
        UctxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UctxbufW<'_, Ucb1txbufSpec> {
        UctxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1txbufSpec;
impl crate::RegisterSpec for Ucb1txbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1txbuf::R`](R) reader structure"]
impl crate::Readable for Ucb1txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1txbuf::W`](W) writer structure"]
impl crate::Writable for Ucb1txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1TXBUF to value 0"]
impl crate::Resettable for Ucb1txbufSpec {}
