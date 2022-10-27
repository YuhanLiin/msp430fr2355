#[doc = "Register `ICCMVS` reader"]
pub struct R(crate::R<ICCMVS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCMVS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCMVS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCMVS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCMVS` writer"]
pub struct W(crate::W<ICCMVS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCMVS_SPEC>;
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
impl From<crate::W<ICCMVS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCMVS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICM0` reader - Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0."]
pub type ICM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICM1` reader - Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1."]
pub type ICM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICM2` reader - Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2."]
pub type ICM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICM3` reader - Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3."]
pub type ICM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MVSSP` reader - MVS stack pointer indicate register"]
pub type MVSSP_R = crate::FieldReader<u8, MVSSP_A>;
#[doc = "MVS stack pointer indicate register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MVSSP_A {
    #[doc = "0: 000b = Stack empty"]
    MVSSP_0 = 0,
    #[doc = "1: 001b = ICM0 affected"]
    MVSSP_1 = 1,
    #[doc = "2: 010b = ICM0 and ICM1 affected"]
    MVSSP_2 = 2,
    #[doc = "3: 011b = ICM0, ICM1, and ICM2 affected"]
    MVSSP_3 = 3,
    #[doc = "4: 100b = ICM0, ICM1, ICM2, and ICM3 affected. Also means the stack is full."]
    MVSSP_4 = 4,
}
impl From<MVSSP_A> for u8 {
    #[inline(always)]
    fn from(variant: MVSSP_A) -> Self {
        variant as _
    }
}
impl MVSSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MVSSP_A> {
        match self.bits {
            0 => Some(MVSSP_A::MVSSP_0),
            1 => Some(MVSSP_A::MVSSP_1),
            2 => Some(MVSSP_A::MVSSP_2),
            3 => Some(MVSSP_A::MVSSP_3),
            4 => Some(MVSSP_A::MVSSP_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MVSSP_0`"]
    #[inline(always)]
    pub fn is_mvssp_0(&self) -> bool {
        *self == MVSSP_A::MVSSP_0
    }
    #[doc = "Checks if the value of the field is `MVSSP_1`"]
    #[inline(always)]
    pub fn is_mvssp_1(&self) -> bool {
        *self == MVSSP_A::MVSSP_1
    }
    #[doc = "Checks if the value of the field is `MVSSP_2`"]
    #[inline(always)]
    pub fn is_mvssp_2(&self) -> bool {
        *self == MVSSP_A::MVSSP_2
    }
    #[doc = "Checks if the value of the field is `MVSSP_3`"]
    #[inline(always)]
    pub fn is_mvssp_3(&self) -> bool {
        *self == MVSSP_A::MVSSP_3
    }
    #[doc = "Checks if the value of the field is `MVSSP_4`"]
    #[inline(always)]
    pub fn is_mvssp_4(&self) -> bool {
        *self == MVSSP_A::MVSSP_4
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0."]
    #[inline(always)]
    pub fn icm0(&self) -> ICM0_R {
        ICM0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1."]
    #[inline(always)]
    pub fn icm1(&self) -> ICM1_R {
        ICM1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2."]
    #[inline(always)]
    pub fn icm2(&self) -> ICM2_R {
        ICM2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3."]
    #[inline(always)]
    pub fn icm3(&self) -> ICM3_R {
        ICM3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - MVS stack pointer indicate register"]
    #[inline(always)]
    pub fn mvssp(&self) -> MVSSP_R {
        MVSSP_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICCMVS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccmvs](index.html) module"]
pub struct ICCMVS_SPEC;
impl crate::RegisterSpec for ICCMVS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iccmvs::R](R) reader structure"]
impl crate::Readable for ICCMVS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccmvs::W](W) writer structure"]
impl crate::Writable for ICCMVS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICCMVS to value 0"]
impl crate::Resettable for ICCMVS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
