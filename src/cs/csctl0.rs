#[doc = "Reader of register CSCTL0"]
pub type R = crate::R<u16, super::CSCTL0>;
#[doc = "Writer for register CSCTL0"]
pub type W = crate::W<u16, super::CSCTL0>;
#[doc = "Register CSCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSCTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCO_A {}
impl From<DCO_A> for u16 {
    #[inline(always)]
    fn from(variant: DCO_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `DCO`"]
pub type DCO_R = crate::R<u16, DCO_A>;
impl DCO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, DCO_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `DCO`"]
pub struct DCO_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u16) & 0x01ff);
        self.w
    }
}
#[doc = "Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOD_A {}
impl From<MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `MOD`"]
pub type MOD_R = crate::R<u8, MOD_A>;
impl MOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOD_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `MOD`"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | (((value as u16) & 0x1f) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
    #[inline(always)]
    pub fn dco(&self) -> DCO_R {
        DCO_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:13 - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DCO tap selection. These bits select the DCO tap and are modified automatically during FLL operation."]
    #[inline(always)]
    pub fn dco(&mut self) -> DCO_W {
        DCO_W { w: self }
    }
    #[doc = "Bits 9:13 - Modulation bit counter. These bits select the modulation pattern. All MOD bits are modified automatically during FLL operation. The DCO register value is incremented when the modulation bit counter rolls over from 31 to 0. If the modulation bit counter decrements from 0 to the maximum count, the DCO register value is also decreased."]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
}
