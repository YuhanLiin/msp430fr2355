#[doc = "Register `P3OUT` reader"]
pub type R = crate::R<P3outSpec>;
#[doc = "Register `P3OUT` writer"]
pub type W = crate::W<P3outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3outSpec;
impl crate::RegisterSpec for P3outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3out::R`](R) reader structure"]
impl crate::Readable for P3outSpec {}
#[doc = "`write(|w| ..)` method takes [`p3out::W`](W) writer structure"]
impl crate::Writable for P3outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3OUT to value 0"]
impl crate::Resettable for P3outSpec {}
