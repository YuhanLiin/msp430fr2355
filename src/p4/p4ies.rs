#[doc = "Register `P4IES` reader"]
pub type R = crate::R<P4iesSpec>;
#[doc = "Register `P4IES` writer"]
pub type W = crate::W<P4iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4iesSpec;
impl crate::RegisterSpec for P4iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ies::R`](R) reader structure"]
impl crate::Readable for P4iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p4ies::W`](W) writer structure"]
impl crate::Writable for P4iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4IES to value 0"]
impl crate::Resettable for P4iesSpec {}
