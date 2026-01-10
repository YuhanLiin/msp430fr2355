#[doc = "Register `P1OUT` reader"]
pub type R = crate::R<P1outSpec>;
#[doc = "Register `P1OUT` writer"]
pub type W = crate::W<P1outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p1out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1outSpec;
impl crate::RegisterSpec for P1outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1out::R`](R) reader structure"]
impl crate::Readable for P1outSpec {}
#[doc = "`write(|w| ..)` method takes [`p1out::W`](W) writer structure"]
impl crate::Writable for P1outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1OUT to value 0"]
impl crate::Resettable for P1outSpec {}
