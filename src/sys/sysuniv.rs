#[doc = "Reader of register SYSUNIV"]
pub type R = crate::R<u16, super::SYSUNIV>;
#[doc = "User NMI vector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSUNIV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: NMIIFG NMI pin"]
    NMIIFG,
    #[doc = "4: OFIFG oscillator fault"]
    OFIFG,
}
impl From<SYSUNIV_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSUNIV_A) -> Self {
        match variant {
            SYSUNIV_A::NONE => 0,
            SYSUNIV_A::NMIIFG => 2,
            SYSUNIV_A::OFIFG => 4,
        }
    }
}
#[doc = "Reader of field `SYSUNIV`"]
pub type SYSUNIV_R = crate::R<u16, SYSUNIV_A>;
impl SYSUNIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SYSUNIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYSUNIV_A::NONE),
            2 => Val(SYSUNIV_A::NMIIFG),
            4 => Val(SYSUNIV_A::OFIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SYSUNIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `NMIIFG`"]
    #[inline(always)]
    pub fn is_nmiifg(&self) -> bool {
        *self == SYSUNIV_A::NMIIFG
    }
    #[doc = "Checks if the value of the field is `OFIFG`"]
    #[inline(always)]
    pub fn is_ofifg(&self) -> bool {
        *self == SYSUNIV_A::OFIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - User NMI vector"]
    #[inline(always)]
    pub fn sysuniv(&self) -> SYSUNIV_R {
        SYSUNIV_R::new((self.bits & 0xffff) as u16)
    }
}
