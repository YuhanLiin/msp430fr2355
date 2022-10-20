#[doc = "Register `ICCILSR1` reader"]
pub struct R(crate::R<ICCILSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCILSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCILSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCILSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCILSR1` writer"]
pub struct W(crate::W<ICCILSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCILSR1_SPEC>;
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
impl From<crate::W<ICCILSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCILSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILSR8` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR8` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR8_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR9` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR9` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR9_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR10` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR10` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR10_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR11` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type ILSR11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR11` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
pub type ILSR11_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR12` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR12` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR12_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR13` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR13` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR13_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR14` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR14` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR14_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR15` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR15` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
pub type ILSR15_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr8(&self) -> ILSR8_R {
        ILSR8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr9(&self) -> ILSR9_R {
        ILSR9_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr10(&self) -> ILSR10_R {
        ILSR10_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr11(&self) -> ILSR11_R {
        ILSR11_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr12(&self) -> ILSR12_R {
        ILSR12_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr13(&self) -> ILSR13_R {
        ILSR13_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr14(&self) -> ILSR14_R {
        ILSR14_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr15(&self) -> ILSR15_R {
        ILSR15_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr8(&mut self) -> ILSR8_W<0> {
        ILSR8_W::new(self)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr9(&mut self) -> ILSR9_W<2> {
        ILSR9_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr10(&mut self) -> ILSR10_W<4> {
        ILSR10_W::new(self)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit"]
    #[inline(always)]
    pub fn ilsr11(&mut self) -> ILSR11_W<6> {
        ILSR11_W::new(self)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr12(&mut self) -> ILSR12_W<8> {
        ILSR12_W::new(self)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr13(&mut self) -> ILSR13_W<10> {
        ILSR13_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr14(&mut self) -> ILSR14_W<12> {
        ILSR14_W::new(self)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRxx bit."]
    #[inline(always)]
    pub fn ilsr15(&mut self) -> ILSR15_W<14> {
        ILSR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICCILSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr1](index.html) module"]
pub struct ICCILSR1_SPEC;
impl crate::RegisterSpec for ICCILSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iccilsr1::R](R) reader structure"]
impl crate::Readable for ICCILSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccilsr1::W](W) writer structure"]
impl crate::Writable for ICCILSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICCILSR1 to value 0"]
impl crate::Resettable for ICCILSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
