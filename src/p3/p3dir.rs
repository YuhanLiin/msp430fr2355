#[doc = "Register `P3DIR` reader"]
pub type R = crate::R<P3dirSpec>;
#[doc = "Register `P3DIR` writer"]
pub type W = crate::W<P3dirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3dirSpec;
impl crate::RegisterSpec for P3dirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p3dir::R`](R) reader structure"]
impl crate::Readable for P3dirSpec {}
#[doc = "`write(|w| ..)` method takes [`p3dir::W`](W) writer structure"]
impl crate::Writable for P3dirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3DIR to value 0"]
impl crate::Resettable for P3dirSpec {}
