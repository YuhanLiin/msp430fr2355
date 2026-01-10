#[doc = "Register `P3IE` reader"]
pub type R = crate::R<P3ieSpec>;
#[doc = "Register `P3IE` writer"]
pub type W = crate::W<P3ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3ieSpec;
impl crate::RegisterSpec for P3ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ie::R`](R) reader structure"]
impl crate::Readable for P3ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p3ie::W`](W) writer structure"]
impl crate::Writable for P3ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3IE to value 0"]
impl crate::Resettable for P3ieSpec {}
