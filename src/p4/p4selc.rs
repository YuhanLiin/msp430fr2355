#[doc = "Register `P4SELC` reader"]
pub type R = crate::R<P4selcSpec>;
#[doc = "Register `P4SELC` writer"]
pub type W = crate::W<P4selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 4 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p4selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P4selcSpec;
impl crate::RegisterSpec for P4selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p4selc::R`](R) reader structure"]
impl crate::Readable for P4selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p4selc::W`](W) writer structure"]
impl crate::Writable for P4selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P4SELC to value 0"]
impl crate::Resettable for P4selcSpec {}
