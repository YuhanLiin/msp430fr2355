#[doc = "Register `P3IES` reader"]
pub type R = crate::R<P3iesSpec>;
#[doc = "Register `P3IES` writer"]
pub type W = crate::W<P3iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3iesSpec;
impl crate::RegisterSpec for P3iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ies::R`](R) reader structure"]
impl crate::Readable for P3iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p3ies::W`](W) writer structure"]
impl crate::Writable for P3iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3IES to value 0"]
impl crate::Resettable for P3iesSpec {}
