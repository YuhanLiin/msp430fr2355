#[doc = "Register `P3IFG` reader"]
pub type R = crate::R<P3ifgSpec>;
#[doc = "Register `P3IFG` writer"]
pub type W = crate::W<P3ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3ifgSpec;
impl crate::RegisterSpec for P3ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3ifg::R`](R) reader structure"]
impl crate::Readable for P3ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p3ifg::W`](W) writer structure"]
impl crate::Writable for P3ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3IFG to value 0"]
impl crate::Resettable for P3ifgSpec {}
