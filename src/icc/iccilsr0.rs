#[doc = "Register `ICCILSR0` reader"]
pub struct R(crate::R<ICCILSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCILSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCILSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCILSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCILSR0` writer"]
pub struct W(crate::W<ICCILSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCILSR0_SPEC>;
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
impl From<crate::W<ICCILSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCILSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ILSR0` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR0` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR1` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR1` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR2` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR2` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR2_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR3` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR3` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR3_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR4` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR4` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR4_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR5` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR5` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR5_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR6` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR6` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR6_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ILSR7` reader - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ILSR7` writer - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
pub type ILSR7_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ICCILSR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr0(&self) -> ILSR0_R {
        ILSR0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr1(&self) -> ILSR1_R {
        ILSR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr2(&self) -> ILSR2_R {
        ILSR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr3(&self) -> ILSR3_R {
        ILSR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr4(&self) -> ILSR4_R {
        ILSR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr5(&self) -> ILSR5_R {
        ILSR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr6(&self) -> ILSR6_R {
        ILSR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr7(&self) -> ILSR7_R {
        ILSR7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr0(&mut self) -> ILSR0_W<0> {
        ILSR0_W::new(self)
    }
    #[doc = "Bits 2:3 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr1(&mut self) -> ILSR1_W<2> {
        ILSR1_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr2(&mut self) -> ILSR2_W<4> {
        ILSR2_W::new(self)
    }
    #[doc = "Bits 6:7 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr3(&mut self) -> ILSR3_W<6> {
        ILSR3_W::new(self)
    }
    #[doc = "Bits 8:9 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr4(&mut self) -> ILSR4_W<8> {
        ILSR4_W::new(self)
    }
    #[doc = "Bits 10:11 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr5(&mut self) -> ILSR5_W<10> {
        ILSR5_W::new(self)
    }
    #[doc = "Bits 12:13 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr6(&mut self) -> ILSR6_W<12> {
        ILSR6_W::new(self)
    }
    #[doc = "Bits 14:15 - Sets the interrupt level for this interrupt source. Maskable interrupt sources only. See the device-specific data sheet to determine the interrupt source for each ILSRx bit."]
    #[inline(always)]
    pub fn ilsr7(&mut self) -> ILSR7_W<14> {
        ILSR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICCILSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccilsr0](index.html) module"]
pub struct ICCILSR0_SPEC;
impl crate::RegisterSpec for ICCILSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iccilsr0::R](R) reader structure"]
impl crate::Readable for ICCILSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccilsr0::W](W) writer structure"]
impl crate::Writable for ICCILSR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICCILSR0 to value 0"]
impl crate::Resettable for ICCILSR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
