#[doc = "Reader of register RTCIV"]
pub type R = crate::R<u16, super::RTCIV>;
#[doc = "Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RTCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: upt Source: RTC Counter Overflow; Interrupt Flag: RTCIFG"]
    RTCIFG = 2,
}
impl From<RTCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: RTCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCIV`"]
pub type RTCIV_R = crate::R<u16, RTCIV_A>;
impl RTCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, RTCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTCIV_A::NONE),
            2 => Val(RTCIV_A::RTCIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RTCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `RTCIFG`"]
    #[inline(always)]
    pub fn is_rtcifg(&self) -> bool {
        *self == RTCIV_A::RTCIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RTCIV_R {
        RTCIV_R::new((self.bits & 0xffff) as u16)
    }
}
