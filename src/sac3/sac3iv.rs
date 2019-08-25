#[doc = "Reader of register SAC3IV"]
pub type R = crate::R<u16, super::SAC3IV>;
#[doc = "SAC Interrupt Vector Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACIV3_A {
    #[doc = "0: No interrupt pending"]
    SACIV_0,
    #[doc = "2: S&H completed interrupt flag (Highest priority)"]
    SACIV_2,
    #[doc = "4: DAC channel update interrupt flag"]
    SACIV_4,
}
impl From<SACIV3_A> for u16 {
    #[inline(always)]
    fn from(variant: SACIV3_A) -> Self {
        match variant {
            SACIV3_A::SACIV_0 => 0,
            SACIV3_A::SACIV_2 => 2,
            SACIV3_A::SACIV_4 => 4,
        }
    }
}
#[doc = "Reader of field `SACIV3`"]
pub type SACIV3_R = crate::R<u16, SACIV3_A>;
impl SACIV3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SACIV3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SACIV3_A::SACIV_0),
            2 => Val(SACIV3_A::SACIV_2),
            4 => Val(SACIV3_A::SACIV_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SACIV_0`"]
    #[inline(always)]
    pub fn is_saciv_0(&self) -> bool {
        *self == SACIV3_A::SACIV_0
    }
    #[doc = "Checks if the value of the field is `SACIV_2`"]
    #[inline(always)]
    pub fn is_saciv_2(&self) -> bool {
        *self == SACIV3_A::SACIV_2
    }
    #[doc = "Checks if the value of the field is `SACIV_4`"]
    #[inline(always)]
    pub fn is_saciv_4(&self) -> bool {
        *self == SACIV3_A::SACIV_4
    }
}
impl R {
    #[doc = "Bits 0:15 - SAC Interrupt Vector Register"]
    #[inline(always)]
    pub fn saciv3(&self) -> SACIV3_R {
        SACIV3_R::new((self.bits & 0xffff) as u16)
    }
}
