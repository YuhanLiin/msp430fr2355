#[doc = "Register `UCA0RXBUF` reader"]
pub type R = crate::R<Uca0rxbufSpec>;
#[doc = "Register `UCA0RXBUF` writer"]
pub type W = crate::W<Uca0rxbufSpec>;
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
#[doc = "eUSCI_Ax Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0rxbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0rxbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0rxbufSpec;
impl crate::RegisterSpec for Uca0rxbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0rxbuf::R`](R) reader structure"]
impl crate::Readable for Uca0rxbufSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0rxbuf::W`](W) writer structure"]
impl crate::Writable for Uca0rxbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0RXBUF to value 0"]
impl crate::Resettable for Uca0rxbufSpec {}
