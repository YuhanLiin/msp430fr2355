#[doc = "Register `UCB1BRW_SPI` reader"]
pub type R = crate::R<Ucb1brwSpiSpec>;
#[doc = "Register `UCB1BRW_SPI` writer"]
pub type W = crate::W<Ucb1brwSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Bx Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1brw_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1brw_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1brwSpiSpec;
impl crate::RegisterSpec for Ucb1brwSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1brw_spi::R`](R) reader structure"]
impl crate::Readable for Ucb1brwSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb1brw_spi::W`](W) writer structure"]
impl crate::Writable for Ucb1brwSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1BRW_SPI to value 0"]
impl crate::Resettable for Ucb1brwSpiSpec {}
