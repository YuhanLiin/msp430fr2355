#[doc = "Register `P6REN` reader"]
pub struct R(crate::R<P6REN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P6REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P6REN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P6REN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P6REN` writer"]
pub struct W(crate::W<P6REN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P6REN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<P6REN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P6REN_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 6 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p6ren](index.html) module"]
pub struct P6REN_SPEC;
impl crate::RegisterSpec for P6REN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p6ren::R](R) reader structure"]
impl crate::Readable for P6REN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p6ren::W](W) writer structure"]
impl crate::Writable for P6REN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P6REN to value 0"]
impl crate::Resettable for P6REN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
