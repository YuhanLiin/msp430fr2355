#[doc = "Register `P1DIR` reader"]
pub type R = crate::R<P1dirSpec>;
#[doc = "Register `P1DIR` writer"]
pub type W = crate::W<P1dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p1dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1dirSpec;
impl crate::RegisterSpec for P1dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1dir::R`](R) reader structure"]
impl crate::Readable for P1dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p1dir::W`](W) writer structure"]
impl crate::Writable for P1dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1DIR to value 0"]
impl crate::Resettable for P1dirSpec {}
