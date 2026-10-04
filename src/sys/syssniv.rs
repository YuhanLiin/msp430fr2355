#[doc = "Register `SYSSNIV` reader"]
pub type R = crate::R<SyssnivSpec>;
#[doc = "Register `SYSSNIV` writer"]
pub type W = crate::W<SyssnivSpec>;
#[doc = "System NMI vector\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Syssniv {
    #[doc = "2: SVS low-power reset entry"]
    SvsLowPowerResetEntry = 2,
    #[doc = "4: Uncorrectable FRAM bit error detection"]
    FramUncorrectableBitError = 4,
    #[doc = "18: VMAIFG vacant memory access"]
    VacantMemoryAccess = 18,
    #[doc = "20: JMBINIFG JTAG mailbox input"]
    JtagMailboxIn = 20,
    #[doc = "22: JMBOUTIFG JTAG mailbox output"]
    JtagMailboxOut = 22,
    #[doc = "24: Correctable FRAM bit error detection"]
    FramCorrectableBitError = 24,
}
impl From<Syssniv> for u16 {
    #[inline(always)]
    fn from(variant: Syssniv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syssniv {
    type Ux = u16;
}
impl crate::IsEnum for Syssniv {}
#[doc = "Field `SYSSNIV` reader - System NMI vector"]
pub type SyssnivR = crate::FieldReader<Syssniv>;
impl SyssnivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syssniv> {
        match self.bits {
            2 => Some(Syssniv::SvsLowPowerResetEntry),
            4 => Some(Syssniv::FramUncorrectableBitError),
            18 => Some(Syssniv::VacantMemoryAccess),
            20 => Some(Syssniv::JtagMailboxIn),
            22 => Some(Syssniv::JtagMailboxOut),
            24 => Some(Syssniv::FramCorrectableBitError),
            _ => None,
        }
    }
    #[doc = "SVS low-power reset entry"]
    #[inline(always)]
    pub fn is_svs_low_power_reset_entry(&self) -> bool {
        *self == Syssniv::SvsLowPowerResetEntry
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_fram_uncorrectable_bit_error(&self) -> bool {
        *self == Syssniv::FramUncorrectableBitError
    }
    #[doc = "VMAIFG vacant memory access"]
    #[inline(always)]
    pub fn is_vacant_memory_access(&self) -> bool {
        *self == Syssniv::VacantMemoryAccess
    }
    #[doc = "JMBINIFG JTAG mailbox input"]
    #[inline(always)]
    pub fn is_jtag_mailbox_in(&self) -> bool {
        *self == Syssniv::JtagMailboxIn
    }
    #[doc = "JMBOUTIFG JTAG mailbox output"]
    #[inline(always)]
    pub fn is_jtag_mailbox_out(&self) -> bool {
        *self == Syssniv::JtagMailboxOut
    }
    #[doc = "Correctable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_fram_correctable_bit_error(&self) -> bool {
        *self == Syssniv::FramCorrectableBitError
    }
}
impl R {
    #[doc = "Bits 0:15 - System NMI vector"]
    #[inline(always)]
    pub fn syssniv(&self) -> SyssnivR {
        SyssnivR::new(self.bits)
    }
}
impl W {}
#[doc = "System NMI Vector Generator\n\nYou can [`read`](crate::Reg::read) this register and get [`syssniv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syssniv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyssnivSpec;
impl crate::RegisterSpec for SyssnivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syssniv::R`](R) reader structure"]
impl crate::Readable for SyssnivSpec {}
#[doc = "`write(|w| ..)` method takes [`syssniv::W`](W) writer structure"]
impl crate::Writable for SyssnivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSSNIV to value 0"]
impl crate::Resettable for SyssnivSpec {}
