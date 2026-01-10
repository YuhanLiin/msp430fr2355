#[doc = "Register `P5OUT` reader"]
pub type R = crate::R<P5outSpec>;
#[doc = "Register `P5OUT` writer"]
pub type W = crate::W<P5outSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p5out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5outSpec;
impl crate::RegisterSpec for P5outSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5out::R`](R) reader structure"]
impl crate::Readable for P5outSpec {}
#[doc = "`write(|w| ..)` method takes [`p5out::W`](W) writer structure"]
impl crate::Writable for P5outSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5OUT to value 0"]
impl crate::Resettable for P5outSpec {}
