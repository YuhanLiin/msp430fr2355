#[doc = "Register `RTCIV` reader"]
pub type R = crate::R<RtcivSpec>;
#[doc = "Register `RTCIV` writer"]
pub type W = crate::W<RtcivSpec>;
#[doc = "Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Rtciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: upt Source: RTC Counter Overflow; Interrupt Flag: RTCIFG"]
    Rtcifg = 2,
}
impl From<Rtciv> for u16 {
    #[inline(always)]
    fn from(variant: Rtciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtciv {
    type Ux = u16;
}
impl crate::IsEnum for Rtciv {}
#[doc = "Field `RTCIV` reader - Real-time clock interrupt vector value"]
pub type RtcivR = crate::FieldReader<Rtciv>;
impl RtcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtciv> {
        match self.bits {
            0 => Some(Rtciv::None),
            2 => Some(Rtciv::Rtcifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Rtciv::None
    }
    #[doc = "upt Source: RTC Counter Overflow; Interrupt Flag: RTCIFG"]
    #[inline(always)]
    pub fn is_rtcifg(&self) -> bool {
        *self == Rtciv::Rtcifg
    }
}
impl R {
    #[doc = "Bits 0:15 - Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RtcivR {
        RtcivR::new(self.bits)
    }
}
impl W {}
#[doc = "Real-Time Clock Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcivSpec;
impl crate::RegisterSpec for RtcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtciv::R`](R) reader structure"]
impl crate::Readable for RtcivSpec {}
#[doc = "`write(|w| ..)` method takes [`rtciv::W`](W) writer structure"]
impl crate::Writable for RtcivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCIV to value 0"]
impl crate::Resettable for RtcivSpec {}
