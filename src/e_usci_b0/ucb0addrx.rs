#[doc = "Reader of register UCB0ADDRX"]
pub type R = crate::R<u16, super::UCB0ADDRX>;
#[doc = "Received Address Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRX_A {}
impl From<ADDRX_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDRX_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ADDRX`"]
pub type ADDRX_R = crate::R<u16, ADDRX_A>;
impl ADDRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADDRX_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new((self.bits & 0x03ff) as u16)
    }
}
