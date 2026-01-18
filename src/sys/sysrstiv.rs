#[doc = "Register `SYSRSTIV` reader"]
pub type R = crate::R<SysrstivSpec>;
#[doc = "Register `SYSRSTIV` writer"]
pub type W = crate::W<SysrstivSpec>;
#[doc = "Reset interrupt vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sysrstiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Brownout"]
    Bor = 2,
    #[doc = "4: RSTIFG RST/NMI"]
    Rstnmi = 4,
    #[doc = "6: PMMSWBOR software BOR"]
    Pmmswbor = 6,
    #[doc = "8: LPMx.5 wakeup"]
    Lpm5wu = 8,
    #[doc = "10: Security violation"]
    Secyv = 10,
    #[doc = "12: Reserved"]
    Sysrstiv12 = 12,
    #[doc = "14: SVSHIFG SVSH event"]
    Svshifg = 14,
    #[doc = "16: Reserved"]
    Sysrstiv16 = 16,
    #[doc = "18: Reserved"]
    Sysrstiv18 = 18,
    #[doc = "20: PMMSWPOR software POR"]
    Pmmswpor = 20,
    #[doc = "22: WDTIFG watchdog timeout"]
    Wdtifg = 22,
    #[doc = "24: WDTPW watchdog password violation"]
    Wdtpw = 24,
    #[doc = "26: FRCTLPW password violation"]
    Frctlpw = 26,
    #[doc = "28: Uncorrectable FRAM bit error detection"]
    Ubdifg = 28,
    #[doc = "30: Peripheral area fetch"]
    Perf = 30,
    #[doc = "32: PMM password violation"]
    Pmmpw = 32,
    #[doc = "34: Reserved"]
    Sysrstiv34 = 34,
    #[doc = "36: FLL unlock (PUC)"]
    Fllul = 36,
}
impl From<Sysrstiv> for u16 {
    #[inline(always)]
    fn from(variant: Sysrstiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysrstiv {
    type Ux = u16;
}
impl crate::IsEnum for Sysrstiv {}
#[doc = "Field `SYSRSTIV` reader - Reset interrupt vector"]
pub type SysrstivR = crate::FieldReader<Sysrstiv>;
impl SysrstivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysrstiv> {
        match self.bits {
            0 => Some(Sysrstiv::None),
            2 => Some(Sysrstiv::Bor),
            4 => Some(Sysrstiv::Rstnmi),
            6 => Some(Sysrstiv::Pmmswbor),
            8 => Some(Sysrstiv::Lpm5wu),
            10 => Some(Sysrstiv::Secyv),
            12 => Some(Sysrstiv::Sysrstiv12),
            14 => Some(Sysrstiv::Svshifg),
            16 => Some(Sysrstiv::Sysrstiv16),
            18 => Some(Sysrstiv::Sysrstiv18),
            20 => Some(Sysrstiv::Pmmswpor),
            22 => Some(Sysrstiv::Wdtifg),
            24 => Some(Sysrstiv::Wdtpw),
            26 => Some(Sysrstiv::Frctlpw),
            28 => Some(Sysrstiv::Ubdifg),
            30 => Some(Sysrstiv::Perf),
            32 => Some(Sysrstiv::Pmmpw),
            34 => Some(Sysrstiv::Sysrstiv34),
            36 => Some(Sysrstiv::Fllul),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sysrstiv::None
    }
    #[doc = "Brownout"]
    #[inline(always)]
    pub fn is_bor(&self) -> bool {
        *self == Sysrstiv::Bor
    }
    #[doc = "RSTIFG RST/NMI"]
    #[inline(always)]
    pub fn is_rstnmi(&self) -> bool {
        *self == Sysrstiv::Rstnmi
    }
    #[doc = "PMMSWBOR software BOR"]
    #[inline(always)]
    pub fn is_pmmswbor(&self) -> bool {
        *self == Sysrstiv::Pmmswbor
    }
    #[doc = "LPMx.5 wakeup"]
    #[inline(always)]
    pub fn is_lpm5wu(&self) -> bool {
        *self == Sysrstiv::Lpm5wu
    }
    #[doc = "Security violation"]
    #[inline(always)]
    pub fn is_secyv(&self) -> bool {
        *self == Sysrstiv::Secyv
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_sysrstiv_12(&self) -> bool {
        *self == Sysrstiv::Sysrstiv12
    }
    #[doc = "SVSHIFG SVSH event"]
    #[inline(always)]
    pub fn is_svshifg(&self) -> bool {
        *self == Sysrstiv::Svshifg
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_sysrstiv_16(&self) -> bool {
        *self == Sysrstiv::Sysrstiv16
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_sysrstiv_18(&self) -> bool {
        *self == Sysrstiv::Sysrstiv18
    }
    #[doc = "PMMSWPOR software POR"]
    #[inline(always)]
    pub fn is_pmmswpor(&self) -> bool {
        *self == Sysrstiv::Pmmswpor
    }
    #[doc = "WDTIFG watchdog timeout"]
    #[inline(always)]
    pub fn is_wdtifg(&self) -> bool {
        *self == Sysrstiv::Wdtifg
    }
    #[doc = "WDTPW watchdog password violation"]
    #[inline(always)]
    pub fn is_wdtpw(&self) -> bool {
        *self == Sysrstiv::Wdtpw
    }
    #[doc = "FRCTLPW password violation"]
    #[inline(always)]
    pub fn is_frctlpw(&self) -> bool {
        *self == Sysrstiv::Frctlpw
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == Sysrstiv::Ubdifg
    }
    #[doc = "Peripheral area fetch"]
    #[inline(always)]
    pub fn is_perf(&self) -> bool {
        *self == Sysrstiv::Perf
    }
    #[doc = "PMM password violation"]
    #[inline(always)]
    pub fn is_pmmpw(&self) -> bool {
        *self == Sysrstiv::Pmmpw
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_sysrstiv_34(&self) -> bool {
        *self == Sysrstiv::Sysrstiv34
    }
    #[doc = "FLL unlock (PUC)"]
    #[inline(always)]
    pub fn is_fllul(&self) -> bool {
        *self == Sysrstiv::Fllul
    }
}
impl R {
    #[doc = "Bits 0:15 - Reset interrupt vector"]
    #[inline(always)]
    pub fn sysrstiv(&self) -> SysrstivR {
        SysrstivR::new(self.bits)
    }
}
impl W {}
#[doc = "Reset Vector Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrstiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrstiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysrstivSpec;
impl crate::RegisterSpec for SysrstivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sysrstiv::R`](R) reader structure"]
impl crate::Readable for SysrstivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysrstiv::W`](W) writer structure"]
impl crate::Writable for SysrstivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSRSTIV to value 0"]
impl crate::Resettable for SysrstivSpec {}
