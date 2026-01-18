#[doc = "Register `UCB1ADDRX` reader"]
pub type R = crate::R<Ucb1addrxSpec>;
#[doc = "Register `UCB1ADDRX` writer"]
pub type W = crate::W<Ucb1addrxSpec>;
#[doc = "Field `ADDRX` reader - Received Address Register"]
pub type AddrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> AddrxR {
        AddrxR::new(self.bits & 0x03ff)
    }
}
impl W {}
#[doc = "eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1addrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1addrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1addrxSpec;
impl crate::RegisterSpec for Ucb1addrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1addrx::R`](R) reader structure"]
impl crate::Readable for Ucb1addrxSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1addrx::W`](W) writer structure"]
impl crate::Writable for Ucb1addrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1ADDRX to value 0"]
impl crate::Resettable for Ucb1addrxSpec {}
