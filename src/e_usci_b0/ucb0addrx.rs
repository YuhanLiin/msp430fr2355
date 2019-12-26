#[doc = "Reader of register UCB0ADDRX"]
pub type R = crate::R<u16, super::UCB0ADDRX>;
#[doc = "Reader of field `ADDRX`"]
pub type ADDRX_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new((self.bits & 0x03ff) as u16)
    }
}
