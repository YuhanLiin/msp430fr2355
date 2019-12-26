#[doc = "Reader of register CPIV"]
pub type R = crate::R<u16, super::CPIV>;
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CPIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: CPIFG"]
    CPIFG = 2,
    #[doc = "4: CPIIFG"]
    CPIIFG = 4,
}
impl From<CPIV_A> for u16 {
    #[inline(always)]
    fn from(variant: CPIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CPIV`"]
pub type CPIV_R = crate::R<u16, CPIV_A>;
impl CPIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CPIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPIV_A::NONE),
            2 => Val(CPIV_A::CPIFG),
            4 => Val(CPIV_A::CPIIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CPIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `CPIFG`"]
    #[inline(always)]
    pub fn is_cpifg(&self) -> bool {
        *self == CPIV_A::CPIFG
    }
    #[doc = "Checks if the value of the field is `CPIIFG`"]
    #[inline(always)]
    pub fn is_cpiifg(&self) -> bool {
        *self == CPIV_A::CPIIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn cpiv(&self) -> CPIV_R {
        CPIV_R::new((self.bits & 0xffff) as u16)
    }
}
