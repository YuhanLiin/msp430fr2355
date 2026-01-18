#[doc = "Register `P6SELC` reader"]
pub type R = crate::R<P6selcSpec>;
#[doc = "Register `P6SELC` writer"]
pub type W = crate::W<P6selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 6 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p6selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P6selcSpec;
impl crate::RegisterSpec for P6selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p6selc::R`](R) reader structure"]
impl crate::Readable for P6selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p6selc::W`](W) writer structure"]
impl crate::Writable for P6selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P6SELC to value 0"]
impl crate::Resettable for P6selcSpec {}
