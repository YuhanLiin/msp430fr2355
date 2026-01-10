#[doc = "Register `SYSSNIV` reader"]
pub type R = crate::R<SyssnivSpec>;
#[doc = "Register `SYSSNIV` writer"]
pub type W = crate::W<SyssnivSpec>;
#[doc = "System NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Syssniv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: SVS low-power reset entry"]
    Svslifg = 2,
    #[doc = "4: Uncorrectable FRAM bit error detection"]
    Ubdifg = 4,
    #[doc = "6: FRAM Access Time Error"]
    Accteifg = 6,
    #[doc = "8: Reserved"]
    Syssniv8 = 8,
    #[doc = "10: Reserved"]
    Syssniv10 = 10,
    #[doc = "12: Reserved"]
    Syssniv12 = 12,
    #[doc = "14: Reserved"]
    Syssniv14 = 14,
    #[doc = "16: Reserved"]
    Syssniv16 = 16,
    #[doc = "18: VMAIFG Vacant memory access"]
    Vmaifg = 18,
    #[doc = "20: JMBINIFG JTAG mailbox input"]
    Jmbinifg = 20,
    #[doc = "22: JMBOUTIFG JTAG mailbox output"]
    Jmboutifg = 22,
    #[doc = "24: Correctable FRAM bit error detection"]
    Cbdifg = 24,
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
            0 => Some(Syssniv::None),
            2 => Some(Syssniv::Svslifg),
            4 => Some(Syssniv::Ubdifg),
            6 => Some(Syssniv::Accteifg),
            8 => Some(Syssniv::Syssniv8),
            10 => Some(Syssniv::Syssniv10),
            12 => Some(Syssniv::Syssniv12),
            14 => Some(Syssniv::Syssniv14),
            16 => Some(Syssniv::Syssniv16),
            18 => Some(Syssniv::Vmaifg),
            20 => Some(Syssniv::Jmbinifg),
            22 => Some(Syssniv::Jmboutifg),
            24 => Some(Syssniv::Cbdifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Syssniv::None
    }
    #[doc = "SVS low-power reset entry"]
    #[inline(always)]
    pub fn is_svslifg(&self) -> bool {
        *self == Syssniv::Svslifg
    }
    #[doc = "Uncorrectable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_ubdifg(&self) -> bool {
        *self == Syssniv::Ubdifg
    }
    #[doc = "FRAM Access Time Error"]
    #[inline(always)]
    pub fn is_accteifg(&self) -> bool {
        *self == Syssniv::Accteifg
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syssniv_8(&self) -> bool {
        *self == Syssniv::Syssniv8
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syssniv_10(&self) -> bool {
        *self == Syssniv::Syssniv10
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syssniv_12(&self) -> bool {
        *self == Syssniv::Syssniv12
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syssniv_14(&self) -> bool {
        *self == Syssniv::Syssniv14
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_syssniv_16(&self) -> bool {
        *self == Syssniv::Syssniv16
    }
    #[doc = "VMAIFG Vacant memory access"]
    #[inline(always)]
    pub fn is_vmaifg(&self) -> bool {
        *self == Syssniv::Vmaifg
    }
    #[doc = "JMBINIFG JTAG mailbox input"]
    #[inline(always)]
    pub fn is_jmbinifg(&self) -> bool {
        *self == Syssniv::Jmbinifg
    }
    #[doc = "JMBOUTIFG JTAG mailbox output"]
    #[inline(always)]
    pub fn is_jmboutifg(&self) -> bool {
        *self == Syssniv::Jmboutifg
    }
    #[doc = "Correctable FRAM bit error detection"]
    #[inline(always)]
    pub fn is_cbdifg(&self) -> bool {
        *self == Syssniv::Cbdifg
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
