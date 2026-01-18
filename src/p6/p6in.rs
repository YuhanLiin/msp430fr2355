#[doc = "Register `P6IN` reader"]
pub type R = crate::R<P6inSpec>;
#[doc = "Register `P6IN` writer"]
pub type W = crate::W<P6inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p6in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6inSpec;
impl crate::RegisterSpec for P6inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6in::R`](R) reader structure"]
impl crate::Readable for P6inSpec {}
#[doc = "`write(|w| ..)` method takes [`p6in::W`](W) writer structure"]
impl crate::Writable for P6inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6IN to value 0"]
impl crate::Resettable for P6inSpec {}
