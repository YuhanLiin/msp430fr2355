#[doc = "Reader of register SAC0IV"]
pub type R = crate::R<u16, super::SAC0IV>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SACIV0_A {
    #[doc = "0: No interrupt pending"]
    SACIV_0 = 0,
    #[doc = "2: S&H completed interrupt flag (Highest priority)"]
    SACIV_2 = 2,
    #[doc = "4: DAC channel update interrupt flag"]
    SACIV_4 = 4,
}
impl From<SACIV0_A> for u16 {
    #[inline(always)]
    fn from(variant: SACIV0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SACIV0`"]
pub type SACIV0_R = crate::R<u16, SACIV0_A>;
impl SACIV0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SACIV0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SACIV0_A::SACIV_0),
            2 => Val(SACIV0_A::SACIV_2),
            4 => Val(SACIV0_A::SACIV_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SACIV_0`"]
    #[inline(always)]
    pub fn is_saciv_0(&self) -> bool {
        *self == SACIV0_A::SACIV_0
    }
    #[doc = "Checks if the value of the field is `SACIV_2`"]
    #[inline(always)]
    pub fn is_saciv_2(&self) -> bool {
        *self == SACIV0_A::SACIV_2
    }
    #[doc = "Checks if the value of the field is `SACIV_4`"]
    #[inline(always)]
    pub fn is_saciv_4(&self) -> bool {
        *self == SACIV0_A::SACIV_4
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv0(&self) -> SACIV0_R {
        SACIV0_R::new((self.bits & 0xffff) as u16)
    }
}
