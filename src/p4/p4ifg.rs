#[doc = "Register `P4IFG` reader"]
pub type R = crate::R<P4ifgSpec>;
#[doc = "Register `P4IFG` writer"]
pub type W = crate::W<P4ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4ifgSpec;
impl crate::RegisterSpec for P4ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4ifg::R`](R) reader structure"]
impl crate::Readable for P4ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p4ifg::W`](W) writer structure"]
impl crate::Writable for P4ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4IFG to value 0"]
impl crate::Resettable for P4ifgSpec {}
