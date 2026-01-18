#[doc = "Register `UCB0BRW` reader"]
pub type R = crate::R<Ucb0brwSpec>;
#[doc = "Register `UCB0BRW` writer"]
pub type W = crate::W<Ucb0brwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0brwSpec;
impl crate::RegisterSpec for Ucb0brwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0brw::R`](R) reader structure"]
impl crate::Readable for Ucb0brwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0brw::W`](W) writer structure"]
impl crate::Writable for Ucb0brwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0BRW to value 0"]
impl crate::Resettable for Ucb0brwSpec {}
