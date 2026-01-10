#[doc = "Register `P3SELC` reader"]
pub type R = crate::R<P3selcSpec>;
#[doc = "Register `P3SELC` writer"]
pub type W = crate::W<P3selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p3selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3selcSpec;
impl crate::RegisterSpec for P3selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3selc::R`](R) reader structure"]
impl crate::Readable for P3selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p3selc::W`](W) writer structure"]
impl crate::Writable for P3selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3SELC to value 0"]
impl crate::Resettable for P3selcSpec {}
