#[doc = "Register `SYSCFG3` reader"]
pub struct R(crate::R<SYSCFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG3` writer"]
pub struct W(crate::W<SYSCFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG3_SPEC>;
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
impl From<crate::W<SYSCFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USCIARMP` reader - eUSCIA remapping source selection, please refer to device specific for details"]
pub type USCIARMP_R = crate::BitReader<USCIARMP_A>;
#[doc = "eUSCIA remapping source selection, please refer to device specific for details\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USCIARMP_A {
    #[doc = "0: P1.x is selected, please refer to device specific for details"]
    USCIARMP_0 = 0,
    #[doc = "1: other port is selected, please refer to device specific for details"]
    USCIARMP_1 = 1,
}
impl From<USCIARMP_A> for bool {
    #[inline(always)]
    fn from(variant: USCIARMP_A) -> Self {
        variant as u8 != 0
    }
}
impl USCIARMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USCIARMP_A {
        match self.bits {
            false => USCIARMP_A::USCIARMP_0,
            true => USCIARMP_A::USCIARMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `USCIARMP_0`"]
    #[inline(always)]
    pub fn is_usciarmp_0(&self) -> bool {
        *self == USCIARMP_A::USCIARMP_0
    }
    #[doc = "Checks if the value of the field is `USCIARMP_1`"]
    #[inline(always)]
    pub fn is_usciarmp_1(&self) -> bool {
        *self == USCIARMP_A::USCIARMP_1
    }
}
#[doc = "Field `USCIARMP` writer - eUSCIA remapping source selection, please refer to device specific for details"]
pub type USCIARMP_W<'a, const O: u8> = crate::BitWriter<'a, u16, SYSCFG3_SPEC, USCIARMP_A, O>;
impl<'a, const O: u8> USCIARMP_W<'a, O> {
    #[doc = "P1.x is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp_0(self) -> &'a mut W {
        self.variant(USCIARMP_A::USCIARMP_0)
    }
    #[doc = "other port is selected, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp_1(self) -> &'a mut W {
        self.variant(USCIARMP_A::USCIARMP_1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp(&self) -> USCIARMP_R {
        USCIARMP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCIA remapping source selection, please refer to device specific for details"]
    #[inline(always)]
    pub fn usciarmp(&mut self) -> USCIARMP_W<0> {
        USCIARMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg3](index.html) module"]
pub struct SYSCFG3_SPEC;
impl crate::RegisterSpec for SYSCFG3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syscfg3::R](R) reader structure"]
impl crate::Readable for SYSCFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg3::W](W) writer structure"]
impl crate::Writable for SYSCFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG3 to value 0"]
impl crate::Resettable for SYSCFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
