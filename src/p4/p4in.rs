#[doc = "Register `P4IN` reader"]
pub type R = crate::R<P4inSpec>;
#[doc = "Register `P4IN` writer"]
pub type W = crate::W<P4inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p4in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4inSpec;
impl crate::RegisterSpec for P4inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4in::R`](R) reader structure"]
impl crate::Readable for P4inSpec {}
#[doc = "`write(|w| ..)` method takes [`p4in::W`](W) writer structure"]
impl crate::Writable for P4inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4IN to value 0"]
impl crate::Resettable for P4inSpec {}
