#[doc = "Reader of register SYSJMBI1"]
pub type R = crate::R<u16, super::SYSJMBI1>;
#[doc = "Writer for register SYSJMBI1"]
pub type W = crate::W<u16, super::SYSJMBI1>;
#[doc = "Register SYSJMBI1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSJMBI1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "JTAG mailbox incoming message low byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGLO_A {}
impl From<MSGLO_A> for u8 {
    #[inline(always)]
    fn from(variant: MSGLO_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `MSGLO`"]
pub type MSGLO_R = crate::R<u8, MSGLO_A>;
impl MSGLO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSGLO_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `MSGLO`"]
pub struct MSGLO_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGLO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGLO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
#[doc = "JTAG mailbox incoming message high byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSGHI_A {}
impl From<MSGHI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSGHI_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `MSGHI`"]
pub type MSGHI_R = crate::R<u8, MSGHI_A>;
impl MSGHI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSGHI_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `MSGHI`"]
pub struct MSGHI_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGHI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSGHI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - JTAG mailbox incoming message low byte"]
    #[inline(always)]
    pub fn msglo(&self) -> MSGLO_R {
        MSGLO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - JTAG mailbox incoming message high byte"]
    #[inline(always)]
    pub fn msghi(&self) -> MSGHI_R {
        MSGHI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - JTAG mailbox incoming message low byte"]
    #[inline(always)]
    pub fn msglo(&mut self) -> MSGLO_W {
        MSGLO_W { w: self }
    }
    #[doc = "Bits 8:15 - JTAG mailbox incoming message high byte"]
    #[inline(always)]
    pub fn msghi(&mut self) -> MSGHI_W {
        MSGHI_W { w: self }
    }
}
