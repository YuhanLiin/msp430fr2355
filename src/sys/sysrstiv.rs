#[doc = "Register `SYSRSTIV` reader"]
pub struct R(crate::R<SYSRSTIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSRSTIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSRSTIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSRSTIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSRSTIV` writer"]
pub struct W(crate::W<SYSRSTIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSRSTIV_SPEC>;
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
impl From<crate::W<SYSRSTIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSRSTIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSRSTIV` reader - Reset interrupt vector"]
pub type SYSRSTIV_R = crate::FieldReader<u16, SYSRSTIV_A>;
#[doc = "Reset interrupt vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SYSRSTIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Brownout"]
    BOR = 2,
    #[doc = "4: RSTIFG RST/NMI"]
    RSTNMI = 4,
    #[doc = "6: PMMSWBOR software BOR"]
    PMMSWBOR = 6,
    #[doc = "8: LPMx.5 wakeup"]
    LPM5WU = 8,
    #[doc = "10: Security violation"]
    SECYV = 10,
    #[doc = "12: Reserved"]
    SYSRSTIV_12 = 12,
    #[doc = "14: SVSHIFG SVSH event"]
    SVSHIFG = 14,
    #[doc = "16: Reserved"]
    SYSRSTIV_16 = 16,
    #[doc = "18: Reserved"]
    SYSRSTIV_18 = 18,
    #[doc = "20: PMMSWPOR software POR"]
    PMMSWPOR = 20,
    #[doc = "22: WDTIFG watchdog timeout"]
    WDTIFG = 22,
    #[doc = "24: WDTPW watchdog password violation"]
    WDTPW = 24,
    #[doc = "26: FRCTLPW password violation"]
    FRCTLPW = 26,
    #[doc = "28: Uncorrectable FRAM bit error detection"]
    UBDIFG = 28,
    #[doc = "30: Peripheral area fetch"]
    PERF = 30,
    #[doc = "32: PMM password violation"]
    PMMPW = 32,
    #[doc = "34: Reserved"]
    SYSRSTIV_34 = 34,
    #[doc = "36: FLL unlock (PUC)"]
    FLLUL = 36,
}
impl From<SYSRSTIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSRSTIV_A) -> Self {
        variant as _
    }
}
impl SYSRSTIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSRSTIV_A> {
        match self.bits {
            0 => Some(SYSRSTIV_A::NONE),
            2 => Some(SYSRSTIV_A::BOR),
            4 => Some(SYSRSTIV_A::RSTNMI),
            6 => Some(SYSRSTIV_A::PMMSWBOR),
            8 => Some(SYSRSTIV_A::LPM5WU),
            10 => Some(SYSRSTIV_A::SECYV),
            12 => Some(SYSRSTIV_A::SYSRSTIV_12),
            14 => Some(SYSRSTIV_A::SVSHIFG),
            16 => Some(SYSRSTIV_A::SYSRSTIV_16),
            18 => Some(SYSRSTIV_A::SYSRSTIV_18),
            20 => Some(SYSRSTIV_A::PMMSWPOR),
            22 => Some(SYSRSTIV_A::WDTIFG),
            24 => Some(SYSRSTIV_A::WDTPW),
            26 => Some(SYSRSTIV_A::FRCTLPW),
            28 => Some(SYSRSTIV_A::UBDIFG),
            30 => Some(SYSRSTIV_A::PERF),
            32 => Some(SYSRSTIV_A::PMMPW),
            34 => Some(SYSRSTIV_A::SYSRSTIV_34),
            36 => Some(SYSRSTIV_A::FLLUL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSRSTIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `BOR`"]
    #[inline(always)]
    pub fn is_bor(&self) -> bool {
        *self == SYSRSTIV_A::BOR
    }
    #[doc = "Checks if the value of the field is `RSTNMI`"]
    #[inline(always)]
    pub fn is_rstnmi(&self) -> bool {
        *self == SYSRSTIV_A::RSTNMI
    }
    #[doc = "Checks if the value of the field is `PMMSWBOR`"]
    #[inline(always)]
    pub fn is_pmmswbor(&self) -> bool {
        *self == SYSRSTIV_A::PMMSWBOR
    }
    #[doc = "Checks if the value of the field is `LPM5WU`"]
    #[inline(always)]
    pub fn is_lpm5wu(&self) -> bool {
        *self == SYSRSTIV_A::LPM5WU
    }
    #[doc = "Checks if the value of the field is `SECYV`"]
    #[inline(always)]
    pub fn is_secyv(&self) -> bool {
        *self == SYSRSTIV_A::SECYV
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_12`"]
    #[inline(always)]
    pub fn is_sysrstiv_12(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_12
    }
    #[doc = "Checks if the value of the field is `SVSHIFG`"]
    #[inline(always)]
    pub fn is_svshifg(&self) -> bool {
        *self == SYSRSTIV_A::SVSHIFG
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_16`"]
    #[inline(always)]
    pub fn is_sysrstiv_16(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_16
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_18`"]
    #[inline(always)]
    pub fn is_sysrstiv_18(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_18
    }
    #[doc = "Checks if the value of the field is `PMMSWPOR`"]
    #[inline(always)]
    pub fn is_pmmswpor(&self) -> bool {
        *self == SYSRSTIV_A::PMMSWPOR
    }
    #[doc = "Checks if the value of the field is `WDTIFG`"]
    #[inline(always)]
    pub fn is_wdtifg(&self) -> bool {
        *self == SYSRSTIV_A::WDTIFG
    }
    #[doc = "Checks if the value of the field is `WDTPW`"]
    #[inline(always)]
    pub fn is_wdtpw(&self) -> bool {
        *self == SYSRSTIV_A::WDTPW
    }
    #[doc = "Checks if the value of the field is `FRCTLPW`"]
    #[inline(always)]
    pub fn is_frctlpw(&self) -> bool {
        *self == SYSRSTIV_A::FRCTLPW
    }
    #[doc = "Checks if the value of the field is `UBDIFG`"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == SYSRSTIV_A::UBDIFG
    }
    #[doc = "Checks if the value of the field is `PERF`"]
    #[inline(always)]
    pub fn is_perf(&self) -> bool {
        *self == SYSRSTIV_A::PERF
    }
    #[doc = "Checks if the value of the field is `PMMPW`"]
    #[inline(always)]
    pub fn is_pmmpw(&self) -> bool {
        *self == SYSRSTIV_A::PMMPW
    }
    #[doc = "Checks if the value of the field is `SYSRSTIV_34`"]
    #[inline(always)]
    pub fn is_sysrstiv_34(&self) -> bool {
        *self == SYSRSTIV_A::SYSRSTIV_34
    }
    #[doc = "Checks if the value of the field is `FLLUL`"]
    #[inline(always)]
    pub fn is_fllul(&self) -> bool {
        *self == SYSRSTIV_A::FLLUL
    }
}
impl R {
    #[doc = "Bits 0:15 - Reset interrupt vector"]
    #[inline(always)]
    pub fn sysrstiv(&self) -> SYSRSTIV_R {
        SYSRSTIV_R::new(self.bits)
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
#[doc = "Reset Vector Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysrstiv](index.html) module"]
pub struct SYSRSTIV_SPEC;
impl crate::RegisterSpec for SYSRSTIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sysrstiv::R](R) reader structure"]
impl crate::Readable for SYSRSTIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysrstiv::W](W) writer structure"]
impl crate::Writable for SYSRSTIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSRSTIV to value 0"]
impl crate::Resettable for SYSRSTIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
