#[doc = "Register `SYSRSTIV` reader"]
pub type R = crate::R<SysrstivSpec>;
#[doc = "Register `SYSRSTIV` writer"]
pub type W = crate::W<SysrstivSpec>;
#[doc = "Reset interrupt vector\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Sysrstiv {
    #[doc = "2: Brownout (BOR)"]
    Brownout = 2,
    #[doc = "4: RSTIFG RST/NMI (BOR)"]
    ResetPin = 4,
    #[doc = "6: PMMSWBOR software BOR (BOR)"]
    SoftwareBor = 6,
    #[doc = "8: LPMx.5 wakeup (BOR)"]
    Lpmx5WakeUp = 8,
    #[doc = "10: Security violation (BOR)"]
    SecurityViolation = 10,
    #[doc = "14: SVSHIFG SVSH event (BOR)"]
    Svsh = 14,
    #[doc = "20: PMMSWPOR software POR (POR)"]
    SoftwarePor = 20,
    #[doc = "22: WDTIFG watchdog time-out (PUC)"]
    WatchdogTimeout = 22,
    #[doc = "24: WDTPW password violation (PUC)"]
    WatchdogPassword = 24,
    #[doc = "26: FRCTLPW password violation (PUC)"]
    FramPassword = 26,
    #[doc = "28: Uncorrectable FRAM bit error detection"]
    FramBitError = 28,
    #[doc = "30: Peripheral area fetch (PUC)"]
    PeripheralAreaFetch = 30,
    #[doc = "32: PMMPW PMM password violation (PUC)"]
    PmmPassword = 32,
    #[doc = "36: FLL unlock (PUC)"]
    FllUnlock = 36,
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
            2 => Some(Sysrstiv::Brownout),
            4 => Some(Sysrstiv::ResetPin),
            6 => Some(Sysrstiv::SoftwareBor),
            8 => Some(Sysrstiv::Lpmx5WakeUp),
            10 => Some(Sysrstiv::SecurityViolation),
            14 => Some(Sysrstiv::Svsh),
            20 => Some(Sysrstiv::SoftwarePor),
            22 => Some(Sysrstiv::WatchdogTimeout),
            24 => Some(Sysrstiv::WatchdogPassword),
            26 => Some(Sysrstiv::FramPassword),
            28 => Some(Sysrstiv::FramBitError),
            30 => Some(Sysrstiv::PeripheralAreaFetch),
            32 => Some(Sysrstiv::PmmPassword),
            36 => Some(Sysrstiv::FllUnlock),
            _ => None,
        }
    }
    #[doc = "Brownout (BOR)"]
    #[inline(always)]
    pub fn is_brownout(&self) -> bool {
        *self == Sysrstiv::Brownout
    }
    #[doc = "RSTIFG RST/NMI (BOR)"]
    #[inline(always)]
    pub fn is_reset_pin(&self) -> bool {
        *self == Sysrstiv::ResetPin
    }
    #[doc = "PMMSWBOR software BOR (BOR)"]
    #[inline(always)]
    pub fn is_software_bor(&self) -> bool {
        *self == Sysrstiv::SoftwareBor
    }
    #[doc = "LPMx.5 wakeup (BOR)"]
    #[inline(always)]
    pub fn is_lpmx5_wake_up(&self) -> bool {
        *self == Sysrstiv::Lpmx5WakeUp
    }
    #[doc = "Security violation (BOR)"]
    #[inline(always)]
    pub fn is_security_violation(&self) -> bool {
        *self == Sysrstiv::SecurityViolation
    }
    #[doc = "SVSHIFG SVSH event (BOR)"]
    #[inline(always)]
    pub fn is_svsh(&self) -> bool {
        *self == Sysrstiv::Svsh
    }
    #[doc = "PMMSWPOR software POR (POR)"]
    #[inline(always)]
    pub fn is_software_por(&self) -> bool {
        *self == Sysrstiv::SoftwarePor
    }
    #[doc = "WDTIFG watchdog time-out (PUC)"]
    #[inline(always)]
    pub fn is_watchdog_timeout(&self) -> bool {
        *self == Sysrstiv::WatchdogTimeout
    }
    #[doc = "WDTPW password violation (PUC)"]
    #[inline(always)]
    pub fn is_watchdog_password(&self) -> bool {
        *self == Sysrstiv::WatchdogPassword
    }
    #[doc = "FRCTLPW password violation (PUC)"]
    #[inline(always)]
    pub fn is_fram_password(&self) -> bool {
        *self == Sysrstiv::FramPassword
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_fram_bit_error(&self) -> bool {
        *self == Sysrstiv::FramBitError
    }
    #[doc = "Peripheral area fetch (PUC)"]
    #[inline(always)]
    pub fn is_peripheral_area_fetch(&self) -> bool {
        *self == Sysrstiv::PeripheralAreaFetch
    }
    #[doc = "PMMPW PMM password violation (PUC)"]
    #[inline(always)]
    pub fn is_pmm_password(&self) -> bool {
        *self == Sysrstiv::PmmPassword
    }
    #[doc = "FLL unlock (PUC)"]
    #[inline(always)]
    pub fn is_fll_unlock(&self) -> bool {
        *self == Sysrstiv::FllUnlock
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
