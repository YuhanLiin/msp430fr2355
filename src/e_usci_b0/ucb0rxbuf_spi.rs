#[doc = "Register `UCB0RXBUF_SPI` reader"]
pub type R = crate::R<Ucb0rxbufSpiSpec>;
#[doc = "Register `UCB0RXBUF_SPI` writer"]
pub type W = crate::W<Ucb0rxbufSpiSpec>;
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0rxbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0rxbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0rxbufSpiSpec;
impl crate::RegisterSpec for Ucb0rxbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0rxbuf_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0rxbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0rxbuf_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0rxbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0RXBUF_SPI to value 0"]
impl crate::Resettable for Ucb0rxbufSpiSpec {}
