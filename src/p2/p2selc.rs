#[doc = "Register `P2SELC` reader"]
pub type R = crate::R<P2selcSpec>;
#[doc = "Register `P2SELC` writer"]
pub type W = crate::W<P2selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 2 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p2selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2selcSpec;
impl crate::RegisterSpec for P2selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p2selc::R`](R) reader structure"]
impl crate::Readable for P2selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p2selc::W`](W) writer structure"]
impl crate::Writable for P2selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2SELC to value 0"]
impl crate::Resettable for P2selcSpec {}
