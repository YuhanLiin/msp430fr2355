#[doc = "Register `P5SELC` reader"]
pub type R = crate::R<P5selcSpec>;
#[doc = "Register `P5SELC` writer"]
pub type W = crate::W<P5selcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Complement Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p5selc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5selc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5selcSpec;
impl crate::RegisterSpec for P5selcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5selc::R`](R) reader structure"]
impl crate::Readable for P5selcSpec {}
#[doc = "`write(|w| ..)` method takes [`p5selc::W`](W) writer structure"]
impl crate::Writable for P5selcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5SELC to value 0"]
impl crate::Resettable for P5selcSpec {}
