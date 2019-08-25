#[doc = "Reader of register SYSRSTIV"]
pub type R = crate::R<u16, super::SYSRSTIV>;
#[doc = "Reset interrupt vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRSTIV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Brownout"]
    BOR,
    #[doc = "4: RSTIFG RST/NMI"]
    RSTNMI,
    #[doc = "6: PMMSWBOR software BOR"]
    PMMSWBOR,
    #[doc = "8: LPMx.5 wakeup"]
    LPM5WU,
    #[doc = "10: Security violation"]
    SECYV,
    #[doc = "12: Reserved"]
    SYSRSTIV_12,
    #[doc = "14: SVSHIFG SVSH event"]
    SVSHIFG,
    #[doc = "16: Reserved"]
    SYSRSTIV_16,
    #[doc = "18: Reserved"]
    SYSRSTIV_18,
    #[doc = "20: PMMSWPOR software POR"]
    PMMSWPOR,
    #[doc = "22: WDTIFG watchdog timeout"]
    WDTIFG,
    #[doc = "24: WDTPW watchdog password violation"]
    WDTPW,
    #[doc = "26: FRCTLPW password violation"]
    FRCTLPW,
    #[doc = "28: Uncorrectable FRAM bit error detection"]
    UBDIFG,
    #[doc = "30: Peripheral area fetch"]
    PERF,
    #[doc = "32: PMM password violation"]
    PMMPW,
    #[doc = "34: Reserved"]
    SYSRSTIV_34,
    #[doc = "36: FLL unlock (PUC)"]
    FLLUL,
}
impl From<SYSRSTIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSRSTIV_A) -> Self {
        match variant {
            SYSRSTIV_A::NONE => 0,
            SYSRSTIV_A::BOR => 2,
            SYSRSTIV_A::RSTNMI => 4,
            SYSRSTIV_A::PMMSWBOR => 6,
            SYSRSTIV_A::LPM5WU => 8,
            SYSRSTIV_A::SECYV => 10,
            SYSRSTIV_A::SYSRSTIV_12 => 12,
            SYSRSTIV_A::SVSHIFG => 14,
            SYSRSTIV_A::SYSRSTIV_16 => 16,
            SYSRSTIV_A::SYSRSTIV_18 => 18,
            SYSRSTIV_A::PMMSWPOR => 20,
            SYSRSTIV_A::WDTIFG => 22,
            SYSRSTIV_A::WDTPW => 24,
            SYSRSTIV_A::FRCTLPW => 26,
            SYSRSTIV_A::UBDIFG => 28,
            SYSRSTIV_A::PERF => 30,
            SYSRSTIV_A::PMMPW => 32,
            SYSRSTIV_A::SYSRSTIV_34 => 34,
            SYSRSTIV_A::FLLUL => 36,
        }
    }
}
#[doc = "Reader of field `SYSRSTIV`"]
pub type SYSRSTIV_R = crate::R<u16, SYSRSTIV_A>;
impl SYSRSTIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSRSTIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSRSTIV_A::NONE),
            2 => Val(SYSRSTIV_A::BOR),
            4 => Val(SYSRSTIV_A::RSTNMI),
            6 => Val(SYSRSTIV_A::PMMSWBOR),
            8 => Val(SYSRSTIV_A::LPM5WU),
            10 => Val(SYSRSTIV_A::SECYV),
            12 => Val(SYSRSTIV_A::SYSRSTIV_12),
            14 => Val(SYSRSTIV_A::SVSHIFG),
            16 => Val(SYSRSTIV_A::SYSRSTIV_16),
            18 => Val(SYSRSTIV_A::SYSRSTIV_18),
            20 => Val(SYSRSTIV_A::PMMSWPOR),
            22 => Val(SYSRSTIV_A::WDTIFG),
            24 => Val(SYSRSTIV_A::WDTPW),
            26 => Val(SYSRSTIV_A::FRCTLPW),
            28 => Val(SYSRSTIV_A::UBDIFG),
            30 => Val(SYSRSTIV_A::PERF),
            32 => Val(SYSRSTIV_A::PMMPW),
            34 => Val(SYSRSTIV_A::SYSRSTIV_34),
            36 => Val(SYSRSTIV_A::FLLUL),
            i => Res(i),
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
        SYSRSTIV_R::new((self.bits & 0xffff) as u16)
    }
}
