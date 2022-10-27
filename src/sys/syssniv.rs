#[doc = "Register `SYSSNIV` reader"]
pub struct R(crate::R<SYSSNIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSSNIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSSNIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSSNIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSSNIV` writer"]
pub struct W(crate::W<SYSSNIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSSNIV_SPEC>;
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
impl From<crate::W<SYSSNIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSSNIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSSNIV` reader - System NMI vector"]
pub type SYSSNIV_R = crate::FieldReader<u16, SYSSNIV_A>;
#[doc = "System NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SYSSNIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: SVS low-power reset entry"]
    SVSLIFG = 2,
    #[doc = "4: Uncorrectable FRAM bit error detection"]
    UBDIFG = 4,
    #[doc = "6: FRAM Access Time Error"]
    ACCTEIFG = 6,
    #[doc = "8: Reserved"]
    SYSSNIV_8 = 8,
    #[doc = "10: Reserved"]
    SYSSNIV_10 = 10,
    #[doc = "12: Reserved"]
    SYSSNIV_12 = 12,
    #[doc = "14: Reserved"]
    SYSSNIV_14 = 14,
    #[doc = "16: Reserved"]
    SYSSNIV_16 = 16,
    #[doc = "18: VMAIFG Vacant memory access"]
    VMAIFG = 18,
    #[doc = "20: JMBINIFG JTAG mailbox input"]
    JMBINIFG = 20,
    #[doc = "22: JMBOUTIFG JTAG mailbox output"]
    JMBOUTIFG = 22,
    #[doc = "24: Correctable FRAM bit error detection"]
    CBDIFG = 24,
}
impl From<SYSSNIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSSNIV_A) -> Self {
        variant as _
    }
}
impl SYSSNIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSSNIV_A> {
        match self.bits {
            0 => Some(SYSSNIV_A::NONE),
            2 => Some(SYSSNIV_A::SVSLIFG),
            4 => Some(SYSSNIV_A::UBDIFG),
            6 => Some(SYSSNIV_A::ACCTEIFG),
            8 => Some(SYSSNIV_A::SYSSNIV_8),
            10 => Some(SYSSNIV_A::SYSSNIV_10),
            12 => Some(SYSSNIV_A::SYSSNIV_12),
            14 => Some(SYSSNIV_A::SYSSNIV_14),
            16 => Some(SYSSNIV_A::SYSSNIV_16),
            18 => Some(SYSSNIV_A::VMAIFG),
            20 => Some(SYSSNIV_A::JMBINIFG),
            22 => Some(SYSSNIV_A::JMBOUTIFG),
            24 => Some(SYSSNIV_A::CBDIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSSNIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `SVSLIFG`"]
    #[inline(always)]
    pub fn is_svslifg(&self) -> bool {
        *self == SYSSNIV_A::SVSLIFG
    }
    #[doc = "Checks if the value of the field is `UBDIFG`"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == SYSSNIV_A::UBDIFG
    }
    #[doc = "Checks if the value of the field is `ACCTEIFG`"]
    #[inline(always)]
    pub fn is_accteifg(&self) -> bool {
        *self == SYSSNIV_A::ACCTEIFG
    }
    #[doc = "Checks if the value of the field is `SYSSNIV_8`"]
    #[inline(always)]
    pub fn is_syssniv_8(&self) -> bool {
        *self == SYSSNIV_A::SYSSNIV_8
    }
    #[doc = "Checks if the value of the field is `SYSSNIV_10`"]
    #[inline(always)]
    pub fn is_syssniv_10(&self) -> bool {
        *self == SYSSNIV_A::SYSSNIV_10
    }
    #[doc = "Checks if the value of the field is `SYSSNIV_12`"]
    #[inline(always)]
    pub fn is_syssniv_12(&self) -> bool {
        *self == SYSSNIV_A::SYSSNIV_12
    }
    #[doc = "Checks if the value of the field is `SYSSNIV_14`"]
    #[inline(always)]
    pub fn is_syssniv_14(&self) -> bool {
        *self == SYSSNIV_A::SYSSNIV_14
    }
    #[doc = "Checks if the value of the field is `SYSSNIV_16`"]
    #[inline(always)]
    pub fn is_syssniv_16(&self) -> bool {
        *self == SYSSNIV_A::SYSSNIV_16
    }
    #[doc = "Checks if the value of the field is `VMAIFG`"]
    #[inline(always)]
    pub fn is_vmaifg(&self) -> bool {
        *self == SYSSNIV_A::VMAIFG
    }
    #[doc = "Checks if the value of the field is `JMBINIFG`"]
    #[inline(always)]
    pub fn is_jmbinifg(&self) -> bool {
        *self == SYSSNIV_A::JMBINIFG
    }
    #[doc = "Checks if the value of the field is `JMBOUTIFG`"]
    #[inline(always)]
    pub fn is_jmboutifg(&self) -> bool {
        *self == SYSSNIV_A::JMBOUTIFG
    }
    #[doc = "Checks if the value of the field is `CBDIFG`"]
    #[inline(always)]
    pub fn is_cbdifg(&self) -> bool {
        *self == SYSSNIV_A::CBDIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - System NMI vector"]
    #[inline(always)]
    pub fn syssniv(&self) -> SYSSNIV_R {
        SYSSNIV_R::new(self.bits)
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
#[doc = "System NMI Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syssniv](index.html) module"]
pub struct SYSSNIV_SPEC;
impl crate::RegisterSpec for SYSSNIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [syssniv::R](R) reader structure"]
impl crate::Readable for SYSSNIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syssniv::W](W) writer structure"]
impl crate::Writable for SYSSNIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSSNIV to value 0"]
impl crate::Resettable for SYSSNIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
