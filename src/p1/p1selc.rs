#[doc = "Register `P1SELC` reader"]
pub type R = crate::R<P1selcSpec>;
#[doc = "Register `P1SELC` writer"]
pub type W = crate::W<P1selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p1selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1selcSpec;
impl crate::RegisterSpec for P1selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1selc::R`](R) reader structure"]
impl crate::Readable for P1selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p1selc::W`](W) writer structure"]
impl crate::Writable for P1selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1SELC to value 0"]
impl crate::Resettable for P1selcSpec {}
