#[doc = "Register `P6OUT` reader"]
pub type R = crate::R<P6outSpec>;
#[doc = "Register `P6OUT` writer"]
pub type W = crate::W<P6outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p6out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6outSpec;
impl crate::RegisterSpec for P6outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6out::R`](R) reader structure"]
impl crate::Readable for P6outSpec {}
#[doc = "`write(|w| ..)` method takes [`p6out::W`](W) writer structure"]
impl crate::Writable for P6outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6OUT to value 0"]
impl crate::Resettable for P6outSpec {}
