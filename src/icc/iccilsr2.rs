#[doc = "Register `ICCILSR2` reader"]
pub struct R(crate::R<ICCILSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCILSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCILSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCILSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCILSR2` writer"]
pub struct W(crate::W<ICCILSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCILSR2_SPEC>;
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
impl From<crate::W<ICCILSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCILSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILSR16` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR16` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR16_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR17` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type ILSR17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR17` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type ILSR17_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR18` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR18` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR18_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR19` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR19` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR19_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR20` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR20` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR20_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR21` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR21` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR21_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR22` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type ILSR22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR22` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type ILSR22_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR23` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type ILSR23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR23` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
pub type ILSR23_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr16(&self) -> ILSR16_R {
        ILSR16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr17(&self) -> ILSR17_R {
        ILSR17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr18(&self) -> ILSR18_R {
        ILSR18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr19(&self) -> ILSR19_R {
        ILSR19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr20(&self) -> ILSR20_R {
        ILSR20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr21(&self) -> ILSR21_R {
        ILSR21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr22(&self) -> ILSR22_R {
        ILSR22_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr23(&self) -> ILSR23_R {
        ILSR23_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr16(&mut self) -> ILSR16_W<0> {
        ILSR16_W::new(self)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr17(&mut self) -> ILSR17_W<2> {
        ILSR17_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr18(&mut self) -> ILSR18_W<4> {
        ILSR18_W::new(self)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr19(&mut self) -> ILSR19_W<6> {
        ILSR19_W::new(self)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr20(&mut self) -> ILSR20_W<8> {
        ILSR20_W::new(self)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr21(&mut self) -> ILSR21_W<10> {
        ILSR21_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr22(&mut self) -> ILSR22_W<12> {
        ILSR22_W::new(self)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each"]
    #[inline(always)]
    pub fn ilsr23(&mut self) -> ILSR23_W<14> {
        ILSR23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICCILSR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr2](index.html) module"]
pub struct ICCILSR2_SPEC;
impl crate::RegisterSpec for ICCILSR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iccilsr2::R](R) reader structure"]
impl crate::Readable for ICCILSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccilsr2::W](W) writer structure"]
impl crate::Writable for ICCILSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICCILSR2 to value 0"]
impl crate::Resettable for ICCILSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
