#[doc = "Register `ICCILSR3` reader"]
pub struct R(crate::R<ICCILSR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCILSR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCILSR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCILSR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCILSR3` writer"]
pub struct W(crate::W<ICCILSR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCILSR3_SPEC>;
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
impl From<crate::W<ICCILSR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCILSR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILSR24` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR24` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR24_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR25` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR25` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR25_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR26` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR26` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR26_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR27` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR27` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR27_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR28` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR28` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR28_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR29` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR29` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR29_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR30` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR30` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR30_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR31` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR31_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR31` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR31_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr24(&self) -> ILSR24_R {
        ILSR24_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr25(&self) -> ILSR25_R {
        ILSR25_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr26(&self) -> ILSR26_R {
        ILSR26_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr27(&self) -> ILSR27_R {
        ILSR27_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr28(&self) -> ILSR28_R {
        ILSR28_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr29(&self) -> ILSR29_R {
        ILSR29_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr30(&self) -> ILSR30_R {
        ILSR30_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr31(&self) -> ILSR31_R {
        ILSR31_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr24(&mut self) -> ILSR24_W<0> {
        ILSR24_W::new(self)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr25(&mut self) -> ILSR25_W<2> {
        ILSR25_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr26(&mut self) -> ILSR26_W<4> {
        ILSR26_W::new(self)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr27(&mut self) -> ILSR27_W<6> {
        ILSR27_W::new(self)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr28(&mut self) -> ILSR28_W<8> {
        ILSR28_W::new(self)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr29(&mut self) -> ILSR29_W<10> {
        ILSR29_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr30(&mut self) -> ILSR30_W<12> {
        ILSR30_W::new(self)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr31(&mut self) -> ILSR31_W<14> {
        ILSR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICCILSR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr3](index.html) module"]
pub struct ICCILSR3_SPEC;
impl crate::RegisterSpec for ICCILSR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iccilsr3::R](R) reader structure"]
impl crate::Readable for ICCILSR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccilsr3::W](W) writer structure"]
impl crate::Writable for ICCILSR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICCILSR3 to value 0"]
impl crate::Resettable for ICCILSR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
