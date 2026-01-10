#[doc = "Register `P4DIR` reader"]
pub type R = crate::R<P4dirSpec>;
#[doc = "Register `P4DIR` writer"]
pub type W = crate::W<P4dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p4dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4dirSpec;
impl crate::RegisterSpec for P4dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4dir::R`](R) reader structure"]
impl crate::Readable for P4dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p4dir::W`](W) writer structure"]
impl crate::Writable for P4dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4DIR to value 0"]
impl crate::Resettable for P4dirSpec {}
