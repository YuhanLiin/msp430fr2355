#[doc = "Register `UCB0STATW` reader"]
pub type R = crate::R<Ucb0statwSpec>;
#[doc = "Register `UCB0STATW` writer"]
pub type W = crate::W<Ucb0statwSpec>;
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbbusy {
    #[doc = "0: Bus inactive"]
    Idle = 0,
    #[doc = "1: Bus busy"]
    Busy = 1,
}
impl From<Ucbbusy> for bool {
    #[inline(always)]
    fn from(variant: Ucbbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBBUSY` reader - Bus busy"]
pub type UcbbusyR = crate::BitReader<Ucbbusy>;
impl UcbbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbbusy {
        match self.bits {
            false => Ucbbusy::Idle,
            true => Ucbbusy::Busy,
        }
    }
    #[doc = "Bus inactive"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Ucbbusy::Idle
    }
    #[doc = "Bus busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ucbbusy::Busy
    }
}
#[doc = "General call address received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucgc {
    #[doc = "0: No general call address received"]
    Ucgc0 = 0,
    #[doc = "1: General call address received"]
    Ucgc1 = 1,
}
impl From<Ucgc> for bool {
    #[inline(always)]
    fn from(variant: Ucgc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCGC` reader - General call address received"]
pub type UcgcR = crate::BitReader<Ucgc>;
impl UcgcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucgc {
        match self.bits {
            false => Ucgc::Ucgc0,
            true => Ucgc::Ucgc1,
        }
    }
    #[doc = "No general call address received"]
    #[inline(always)]
    pub fn is_ucgc_0(&self) -> bool {
        *self == Ucgc::Ucgc0
    }
    #[doc = "General call address received"]
    #[inline(always)]
    pub fn is_ucgc_1(&self) -> bool {
        *self == Ucgc::Ucgc1
    }
}
#[doc = "SCL low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucscllow {
    #[doc = "0: SCL is not held low"]
    Ucscllow0 = 0,
    #[doc = "1: SCL is held low"]
    Ucscllow1 = 1,
}
impl From<Ucscllow> for bool {
    #[inline(always)]
    fn from(variant: Ucscllow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UcscllowR = crate::BitReader<Ucscllow>;
impl UcscllowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucscllow {
        match self.bits {
            false => Ucscllow::Ucscllow0,
            true => Ucscllow::Ucscllow1,
        }
    }
    #[doc = "SCL is not held low"]
    #[inline(always)]
    pub fn is_ucscllow_0(&self) -> bool {
        *self == Ucscllow::Ucscllow0
    }
    #[doc = "SCL is held low"]
    #[inline(always)]
    pub fn is_ucscllow_1(&self) -> bool {
        *self == Ucscllow::Ucscllow1
    }
}
#[doc = "Field `UCBCNT` reader - Hardware byte counter value"]
pub type UcbcntR = crate::FieldReader;
impl R {
    #[doc = "Bit 4 - Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UcbbusyR {
        UcbbusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General call address received"]
    #[inline(always)]
    pub fn ucgc(&self) -> UcgcR {
        UcgcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UcscllowR {
        UcscllowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&self) -> UcbcntR {
        UcbcntR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0statw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0statw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0statwSpec;
impl crate::RegisterSpec for Ucb0statwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0statw::R`](R) reader structure"]
impl crate::Readable for Ucb0statwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0statw::W`](W) writer structure"]
impl crate::Writable for Ucb0statwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0STATW to value 0"]
impl crate::Resettable for Ucb0statwSpec {}
