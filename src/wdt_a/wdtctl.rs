#[doc = "Register `WDTCTL` reader"]
pub type R = crate::R<WdtctlSpec>;
#[doc = "Register `WDTCTL` writer"]
pub type W = crate::W<WdtctlSpec>;
#[doc = "Watchdog timer interval select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtis {
    #[doc = "0: Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    _2g = 0,
    #[doc = "1: Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    _128m = 1,
    #[doc = "2: Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    _8192k = 2,
    #[doc = "3: Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    _512k = 3,
    #[doc = "4: Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    _32k = 4,
    #[doc = "5: Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    _8192 = 5,
    #[doc = "6: Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    _512 = 6,
    #[doc = "7: Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    _64 = 7,
}
impl From<Wdtis> for u8 {
    #[inline(always)]
    fn from(variant: Wdtis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtis {
    type Ux = u8;
}
impl crate::IsEnum for Wdtis {}
#[doc = "Field `WDTIS` reader - Watchdog timer interval select"]
pub type WdtisR = crate::FieldReader<Wdtis>;
impl WdtisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtis {
        match self.bits {
            0 => Wdtis::_2g,
            1 => Wdtis::_128m,
            2 => Wdtis::_8192k,
            3 => Wdtis::_512k,
            4 => Wdtis::_32k,
            5 => Wdtis::_8192,
            6 => Wdtis::_512,
            7 => Wdtis::_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_2g(&self) -> bool {
        *self == Wdtis::_2g
    }
    #[doc = "Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_128m(&self) -> bool {
        *self == Wdtis::_128m
    }
    #[doc = "Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_8192k(&self) -> bool {
        *self == Wdtis::_8192k
    }
    #[doc = "Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == Wdtis::_512k
    }
    #[doc = "Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == Wdtis::_32k
    }
    #[doc = "Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_8192(&self) -> bool {
        *self == Wdtis::_8192
    }
    #[doc = "Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Wdtis::_512
    }
    #[doc = "Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Wdtis::_64
    }
}
#[doc = "Field `WDTIS` writer - Watchdog timer interval select"]
pub type WdtisW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdtis, crate::Safe>;
impl<'a, REG> WdtisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _2g(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_2g)
    }
    #[doc = "Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _128m(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_128m)
    }
    #[doc = "Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _8192k(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_8192k)
    }
    #[doc = "Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_512k)
    }
    #[doc = "Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_32k)
    }
    #[doc = "Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _8192(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_8192)
    }
    #[doc = "Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_512)
    }
    #[doc = "Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtis::_64)
    }
}
#[doc = "Watchdog timer counter clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcntcl {
    #[doc = "0: No action"]
    Wdtcntcl0 = 0,
    #[doc = "1: WDTCNT = 0000h"]
    Wdtcntcl1 = 1,
}
impl From<Wdtcntcl> for bool {
    #[inline(always)]
    fn from(variant: Wdtcntcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCNTCL` reader - Watchdog timer counter clear"]
pub type WdtcntclR = crate::BitReader<Wdtcntcl>;
impl WdtcntclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtcntcl {
        match self.bits {
            false => Wdtcntcl::Wdtcntcl0,
            true => Wdtcntcl::Wdtcntcl1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_wdtcntcl_0(&self) -> bool {
        *self == Wdtcntcl::Wdtcntcl0
    }
    #[doc = "WDTCNT = 0000h"]
    #[inline(always)]
    pub fn is_wdtcntcl_1(&self) -> bool {
        *self == Wdtcntcl::Wdtcntcl1
    }
}
#[doc = "Field `WDTCNTCL` writer - Watchdog timer counter clear"]
pub type WdtcntclW<'a, REG> = crate::BitWriter<'a, REG, Wdtcntcl>;
impl<'a, REG> WdtcntclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn wdtcntcl_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcntcl::Wdtcntcl0)
    }
    #[doc = "WDTCNT = 0000h"]
    #[inline(always)]
    pub fn wdtcntcl_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcntcl::Wdtcntcl1)
    }
}
#[doc = "Watchdog timer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdttmsel {
    #[doc = "0: Watchdog mode"]
    Wdttmsel0 = 0,
    #[doc = "1: Interval timer mode"]
    Wdttmsel1 = 1,
}
impl From<Wdttmsel> for bool {
    #[inline(always)]
    fn from(variant: Wdttmsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTTMSEL` reader - Watchdog timer mode select"]
pub type WdttmselR = crate::BitReader<Wdttmsel>;
impl WdttmselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdttmsel {
        match self.bits {
            false => Wdttmsel::Wdttmsel0,
            true => Wdttmsel::Wdttmsel1,
        }
    }
    #[doc = "Watchdog mode"]
    #[inline(always)]
    pub fn is_wdttmsel_0(&self) -> bool {
        *self == Wdttmsel::Wdttmsel0
    }
    #[doc = "Interval timer mode"]
    #[inline(always)]
    pub fn is_wdttmsel_1(&self) -> bool {
        *self == Wdttmsel::Wdttmsel1
    }
}
#[doc = "Field `WDTTMSEL` writer - Watchdog timer mode select"]
pub type WdttmselW<'a, REG> = crate::BitWriter<'a, REG, Wdttmsel>;
impl<'a, REG> WdttmselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog mode"]
    #[inline(always)]
    pub fn wdttmsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wdttmsel::Wdttmsel0)
    }
    #[doc = "Interval timer mode"]
    #[inline(always)]
    pub fn wdttmsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdttmsel::Wdttmsel1)
    }
}
#[doc = "Watchdog timer clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdtssel {
    #[doc = "0: SMCLK"]
    Smclk = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: VLOCLK"]
    Vloclk = 2,
    #[doc = "3: BCLK"]
    Bclk = 3,
}
impl From<Wdtssel> for u8 {
    #[inline(always)]
    fn from(variant: Wdtssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdtssel {
    type Ux = u8;
}
impl crate::IsEnum for Wdtssel {}
#[doc = "Field `WDTSSEL` reader - Watchdog timer clock source select"]
pub type WdtsselR = crate::FieldReader<Wdtssel>;
impl WdtsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtssel {
        match self.bits {
            0 => Wdtssel::Smclk,
            1 => Wdtssel::Aclk,
            2 => Wdtssel::Vloclk,
            3 => Wdtssel::Bclk,
            _ => unreachable!(),
        }
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Wdtssel::Smclk
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Wdtssel::Aclk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Wdtssel::Vloclk
    }
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn is_bclk(&self) -> bool {
        *self == Wdtssel::Bclk
    }
}
#[doc = "Field `WDTSSEL` writer - Watchdog timer clock source select"]
pub type WdtsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wdtssel, crate::Safe>;
impl<'a, REG> WdtsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Smclk)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Aclk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Vloclk)
    }
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn bclk(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtssel::Bclk)
    }
}
#[doc = "Watchdog timer hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdthold {
    #[doc = "0: Watchdog timer is not stopped"]
    Unhold = 0,
    #[doc = "1: Watchdog timer is stopped"]
    Hold = 1,
}
impl From<Wdthold> for bool {
    #[inline(always)]
    fn from(variant: Wdthold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTHOLD` reader - Watchdog timer hold"]
pub type WdtholdR = crate::BitReader<Wdthold>;
impl WdtholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdthold {
        match self.bits {
            false => Wdthold::Unhold,
            true => Wdthold::Hold,
        }
    }
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn is_unhold(&self) -> bool {
        *self == Wdthold::Unhold
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == Wdthold::Hold
    }
}
#[doc = "Field `WDTHOLD` writer - Watchdog timer hold"]
pub type WdtholdW<'a, REG> = crate::BitWriter<'a, REG, Wdthold>;
impl<'a, REG> WdtholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn unhold(self) -> &'a mut crate::W<REG> {
        self.variant(Wdthold::Unhold)
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(Wdthold::Hold)
    }
}
#[doc = "Field `WDTPW` reader - Watchdog timer password"]
pub type WdtpwR = crate::FieldReader;
#[doc = "Field `WDTPW` writer - Watchdog timer password"]
pub type WdtpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&self) -> WdtisR {
        WdtisR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WdtcntclR {
        WdtcntclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WdttmselR {
        WdttmselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WdtsselR {
        WdtsselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WdtholdR {
        WdtholdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WdtpwR {
        WdtpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WdtisW<'_, WdtctlSpec> {
        WdtisW::new(self, 0)
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WdtcntclW<'_, WdtctlSpec> {
        WdtcntclW::new(self, 3)
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WdttmselW<'_, WdtctlSpec> {
        WdttmselW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WdtsselW<'_, WdtctlSpec> {
        WdtsselW::new(self, 5)
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WdtholdW<'_, WdtctlSpec> {
        WdtholdW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WdtpwW<'_, WdtctlSpec> {
        WdtpwW::new(self, 8)
    }
}
#[doc = "Watchdog Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtctlSpec;
impl crate::RegisterSpec for WdtctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdtctl::R`](R) reader structure"]
impl crate::Readable for WdtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtctl::W`](W) writer structure"]
impl crate::Writable for WdtctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WdtctlSpec {}
