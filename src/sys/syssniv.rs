#[doc = "Reader of register SYSSNIV"]
pub type R = crate::R<u16, super::SYSSNIV>;
#[doc = "System NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `SYSSNIV`"]
pub type SYSSNIV_R = crate::R<u16, SYSSNIV_A>;
impl SYSSNIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSSNIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSSNIV_A::NONE),
            2 => Val(SYSSNIV_A::SVSLIFG),
            4 => Val(SYSSNIV_A::UBDIFG),
            6 => Val(SYSSNIV_A::ACCTEIFG),
            8 => Val(SYSSNIV_A::SYSSNIV_8),
            10 => Val(SYSSNIV_A::SYSSNIV_10),
            12 => Val(SYSSNIV_A::SYSSNIV_12),
            14 => Val(SYSSNIV_A::SYSSNIV_14),
            16 => Val(SYSSNIV_A::SYSSNIV_16),
            18 => Val(SYSSNIV_A::VMAIFG),
            20 => Val(SYSSNIV_A::JMBINIFG),
            22 => Val(SYSSNIV_A::JMBOUTIFG),
            24 => Val(SYSSNIV_A::CBDIFG),
            i => Res(i),
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
        SYSSNIV_R::new((self.bits & 0xffff) as u16)
    }
}
