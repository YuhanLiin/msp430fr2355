#[doc = "Reader of register SYSJMBO0"]
pub type R = crate::R<u16, super::SYSJMBO0>;
#[doc = "Writer for register SYSJMBO0"]
pub type W = crate::W<u16, super::SYSJMBO0>;
#[doc = "Register SYSJMBO0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSJMBO0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSGLO`"]
pub type MSGLO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSGLO`"]
pub struct MSGLO_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `MSGHI`"]
pub type MSGHI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MSGHI`"]
pub struct MSGHI_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - JTAG mailbox outgoing message low byte"]
    #[inline(always)]
    pub fn msglo(&self) -> MSGLO_R {
        MSGLO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - JTAG mailbox outgoing message high byte"]
    #[inline(always)]
    pub fn msghi(&self) -> MSGHI_R {
        MSGHI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - JTAG mailbox outgoing message low byte"]
    #[inline(always)]
    pub fn msglo(&mut self) -> MSGLO_W {
        MSGLO_W { w: self }
    }
    #[doc = "Bits 8:15 - JTAG mailbox outgoing message high byte"]
    #[inline(always)]
    pub fn msghi(&mut self) -> MSGHI_W {
        MSGHI_W { w: self }
    }
}
