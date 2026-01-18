#[doc = "Register `MPY32CTL0` reader"]
pub type R = crate::R<Mpy32ctl0Spec>;
#[doc = "Register `MPY32CTL0` writer"]
pub type W = crate::W<Mpy32ctl0Spec>;
#[doc = "Carry of the multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpyc {
    #[doc = "0: No carry for result."]
    Mpyc0 = 0,
    #[doc = "1: Result has a carry."]
    Mpyc1 = 1,
}
impl From<Mpyc> for bool {
    #[inline(always)]
    fn from(variant: Mpyc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYC` reader - Carry of the multiplier"]
pub type MpycR = crate::BitReader<Mpyc>;
impl MpycR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpyc {
        match self.bits {
            false => Mpyc::Mpyc0,
            true => Mpyc::Mpyc1,
        }
    }
    #[doc = "No carry for result."]
    #[inline(always)]
    pub fn is_mpyc_0(&self) -> bool {
        *self == Mpyc::Mpyc0
    }
    #[doc = "Result has a carry."]
    #[inline(always)]
    pub fn is_mpyc_1(&self) -> bool {
        *self == Mpyc::Mpyc1
    }
}
#[doc = "Field `MPYC` writer - Carry of the multiplier"]
pub type MpycW<'a, REG> = crate::BitWriter<'a, REG, Mpyc>;
impl<'a, REG> MpycW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No carry for result."]
    #[inline(always)]
    pub fn mpyc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyc::Mpyc0)
    }
    #[doc = "Result has a carry."]
    #[inline(always)]
    pub fn mpyc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyc::Mpyc1)
    }
}
#[doc = "Fractional mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpyfrac {
    #[doc = "0: Fractional mode disabled."]
    Disable = 0,
    #[doc = "1: Fractional mode enabled."]
    Enable = 1,
}
impl From<Mpyfrac> for bool {
    #[inline(always)]
    fn from(variant: Mpyfrac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYFRAC` reader - Fractional mode."]
pub type MpyfracR = crate::BitReader<Mpyfrac>;
impl MpyfracR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpyfrac {
        match self.bits {
            false => Mpyfrac::Disable,
            true => Mpyfrac::Enable,
        }
    }
    #[doc = "Fractional mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpyfrac::Disable
    }
    #[doc = "Fractional mode enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpyfrac::Enable
    }
}
#[doc = "Field `MPYFRAC` writer - Fractional mode."]
pub type MpyfracW<'a, REG> = crate::BitWriter<'a, REG, Mpyfrac>;
impl<'a, REG> MpyfracW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fractional mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyfrac::Disable)
    }
    #[doc = "Fractional mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyfrac::Enable)
    }
}
#[doc = "Saturation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpysat {
    #[doc = "0: Saturation mode disabled."]
    Disable = 0,
    #[doc = "1: Saturation mode enabled."]
    Enable = 1,
}
impl From<Mpysat> for bool {
    #[inline(always)]
    fn from(variant: Mpysat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYSAT` reader - Saturation mode"]
pub type MpysatR = crate::BitReader<Mpysat>;
impl MpysatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpysat {
        match self.bits {
            false => Mpysat::Disable,
            true => Mpysat::Enable,
        }
    }
    #[doc = "Saturation mode disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mpysat::Disable
    }
    #[doc = "Saturation mode enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Mpysat::Enable
    }
}
#[doc = "Field `MPYSAT` writer - Saturation mode"]
pub type MpysatW<'a, REG> = crate::BitWriter<'a, REG, Mpysat>;
impl<'a, REG> MpysatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Saturation mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpysat::Disable)
    }
    #[doc = "Saturation mode enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Mpysat::Enable)
    }
}
#[doc = "Multiplier mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mpym {
    #[doc = "0: MPY Multiply"]
    Mpy = 0,
    #[doc = "1: MPYS Signed multiply"]
    Mpys = 1,
    #[doc = "2: MAC Multiply accumulate"]
    Mac = 2,
    #[doc = "3: MACS Signed multiply accumulate"]
    Macs = 3,
}
impl From<Mpym> for u8 {
    #[inline(always)]
    fn from(variant: Mpym) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mpym {
    type Ux = u8;
}
impl crate::IsEnum for Mpym {}
#[doc = "Field `MPYM` reader - Multiplier mode"]
pub type MpymR = crate::FieldReader<Mpym>;
impl MpymR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpym {
        match self.bits {
            0 => Mpym::Mpy,
            1 => Mpym::Mpys,
            2 => Mpym::Mac,
            3 => Mpym::Macs,
            _ => unreachable!(),
        }
    }
    #[doc = "MPY Multiply"]
    #[inline(always)]
    pub fn is_mpy(&self) -> bool {
        *self == Mpym::Mpy
    }
    #[doc = "MPYS Signed multiply"]
    #[inline(always)]
    pub fn is_mpys(&self) -> bool {
        *self == Mpym::Mpys
    }
    #[doc = "MAC Multiply accumulate"]
    #[inline(always)]
    pub fn is_mac(&self) -> bool {
        *self == Mpym::Mac
    }
    #[doc = "MACS Signed multiply accumulate"]
    #[inline(always)]
    pub fn is_macs(&self) -> bool {
        *self == Mpym::Macs
    }
}
#[doc = "Field `MPYM` writer - Multiplier mode"]
pub type MpymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mpym, crate::Safe>;
impl<'a, REG> MpymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MPY Multiply"]
    #[inline(always)]
    pub fn mpy(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpy)
    }
    #[doc = "MPYS Signed multiply"]
    #[inline(always)]
    pub fn mpys(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mpys)
    }
    #[doc = "MAC Multiply accumulate"]
    #[inline(always)]
    pub fn mac(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Mac)
    }
    #[doc = "MACS Signed multiply accumulate"]
    #[inline(always)]
    pub fn macs(self) -> &'a mut crate::W<REG> {
        self.variant(Mpym::Macs)
    }
}
#[doc = "Multiplier bit width of operand 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpyop1_32 {
    #[doc = "0: 16 bits."]
    _16 = 0,
    #[doc = "1: 32 bits."]
    _32 = 1,
}
impl From<Mpyop1_32> for bool {
    #[inline(always)]
    fn from(variant: Mpyop1_32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYOP1_32` reader - Multiplier bit width of operand 1"]
pub type Mpyop1_32R = crate::BitReader<Mpyop1_32>;
impl Mpyop1_32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpyop1_32 {
        match self.bits {
            false => Mpyop1_32::_16,
            true => Mpyop1_32::_32,
        }
    }
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Mpyop1_32::_16
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Mpyop1_32::_32
    }
}
#[doc = "Field `MPYOP1_32` writer - Multiplier bit width of operand 1"]
pub type Mpyop1_32W<'a, REG> = crate::BitWriter<'a, REG, Mpyop1_32>;
impl<'a, REG> Mpyop1_32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyop1_32::_16)
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyop1_32::_32)
    }
}
#[doc = "Multiplier bit width of operand 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpyop2_32 {
    #[doc = "0: 16 bits."]
    _16 = 0,
    #[doc = "1: 32 bits."]
    _32 = 1,
}
impl From<Mpyop2_32> for bool {
    #[inline(always)]
    fn from(variant: Mpyop2_32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYOP2_32` reader - Multiplier bit width of operand 2"]
pub type Mpyop2_32R = crate::BitReader<Mpyop2_32>;
impl Mpyop2_32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpyop2_32 {
        match self.bits {
            false => Mpyop2_32::_16,
            true => Mpyop2_32::_32,
        }
    }
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Mpyop2_32::_16
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Mpyop2_32::_32
    }
}
#[doc = "Field `MPYOP2_32` writer - Multiplier bit width of operand 2"]
pub type Mpyop2_32W<'a, REG> = crate::BitWriter<'a, REG, Mpyop2_32>;
impl<'a, REG> Mpyop2_32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16 bits."]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyop2_32::_16)
    }
    #[doc = "32 bits."]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Mpyop2_32::_32)
    }
}
#[doc = "Delayed write enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpydlywrten {
    #[doc = "0: Writes are not delayed."]
    Mpydlywrten0 = 0,
    #[doc = "1: Writes are delayed."]
    Mpydlywrten1 = 1,
}
impl From<Mpydlywrten> for bool {
    #[inline(always)]
    fn from(variant: Mpydlywrten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYDLYWRTEN` reader - Delayed write enable."]
pub type MpydlywrtenR = crate::BitReader<Mpydlywrten>;
impl MpydlywrtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpydlywrten {
        match self.bits {
            false => Mpydlywrten::Mpydlywrten0,
            true => Mpydlywrten::Mpydlywrten1,
        }
    }
    #[doc = "Writes are not delayed."]
    #[inline(always)]
    pub fn is_mpydlywrten_0(&self) -> bool {
        *self == Mpydlywrten::Mpydlywrten0
    }
    #[doc = "Writes are delayed."]
    #[inline(always)]
    pub fn is_mpydlywrten_1(&self) -> bool {
        *self == Mpydlywrten::Mpydlywrten1
    }
}
#[doc = "Field `MPYDLYWRTEN` writer - Delayed write enable."]
pub type MpydlywrtenW<'a, REG> = crate::BitWriter<'a, REG, Mpydlywrten>;
impl<'a, REG> MpydlywrtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes are not delayed."]
    #[inline(always)]
    pub fn mpydlywrten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpydlywrten::Mpydlywrten0)
    }
    #[doc = "Writes are delayed."]
    #[inline(always)]
    pub fn mpydlywrten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpydlywrten::Mpydlywrten1)
    }
}
#[doc = "Delayed write mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mpydly32 {
    #[doc = "0: Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    Mpydly32_0 = 0,
    #[doc = "1: Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    Mpydly32_1 = 1,
}
impl From<Mpydly32> for bool {
    #[inline(always)]
    fn from(variant: Mpydly32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPYDLY32` reader - Delayed write mode."]
pub type Mpydly32R = crate::BitReader<Mpydly32>;
impl Mpydly32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mpydly32 {
        match self.bits {
            false => Mpydly32::Mpydly32_0,
            true => Mpydly32::Mpydly32_1,
        }
    }
    #[doc = "Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    #[inline(always)]
    pub fn is_mpydly32_0(&self) -> bool {
        *self == Mpydly32::Mpydly32_0
    }
    #[doc = "Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    #[inline(always)]
    pub fn is_mpydly32_1(&self) -> bool {
        *self == Mpydly32::Mpydly32_1
    }
}
#[doc = "Field `MPYDLY32` writer - Delayed write mode."]
pub type Mpydly32W<'a, REG> = crate::BitWriter<'a, REG, Mpydly32>;
impl<'a, REG> Mpydly32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes are delayed until 64-bit result (RES0 to RES3) is available."]
    #[inline(always)]
    pub fn mpydly32_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mpydly32::Mpydly32_0)
    }
    #[doc = "Writes are delayed until 32-bit result (RES0 to RES1) is available. 8 MPYDLYWRTEN"]
    #[inline(always)]
    pub fn mpydly32_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mpydly32::Mpydly32_1)
    }
}
impl R {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&self) -> MpycR {
        MpycR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fractional mode."]
    #[inline(always)]
    pub fn mpyfrac(&self) -> MpyfracR {
        MpyfracR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&self) -> MpysatR {
        MpysatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Multiplier mode"]
    #[inline(always)]
    pub fn mpym(&self) -> MpymR {
        MpymR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Multiplier bit width of operand 1"]
    #[inline(always)]
    pub fn mpyop1_32(&self) -> Mpyop1_32R {
        Mpyop1_32R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multiplier bit width of operand 2"]
    #[inline(always)]
    pub fn mpyop2_32(&self) -> Mpyop2_32R {
        Mpyop2_32R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Delayed write enable."]
    #[inline(always)]
    pub fn mpydlywrten(&self) -> MpydlywrtenR {
        MpydlywrtenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed write mode."]
    #[inline(always)]
    pub fn mpydly32(&self) -> Mpydly32R {
        Mpydly32R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline(always)]
    pub fn mpyc(&mut self) -> MpycW<'_, Mpy32ctl0Spec> {
        MpycW::new(self, 0)
    }
    #[doc = "Bit 2 - Fractional mode."]
    #[inline(always)]
    pub fn mpyfrac(&mut self) -> MpyfracW<'_, Mpy32ctl0Spec> {
        MpyfracW::new(self, 2)
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline(always)]
    pub fn mpysat(&mut self) -> MpysatW<'_, Mpy32ctl0Spec> {
        MpysatW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Multiplier mode"]
    #[inline(always)]
    pub fn mpym(&mut self) -> MpymW<'_, Mpy32ctl0Spec> {
        MpymW::new(self, 4)
    }
    #[doc = "Bit 6 - Multiplier bit width of operand 1"]
    #[inline(always)]
    pub fn mpyop1_32(&mut self) -> Mpyop1_32W<'_, Mpy32ctl0Spec> {
        Mpyop1_32W::new(self, 6)
    }
    #[doc = "Bit 7 - Multiplier bit width of operand 2"]
    #[inline(always)]
    pub fn mpyop2_32(&mut self) -> Mpyop2_32W<'_, Mpy32ctl0Spec> {
        Mpyop2_32W::new(self, 7)
    }
    #[doc = "Bit 8 - Delayed write enable."]
    #[inline(always)]
    pub fn mpydlywrten(&mut self) -> MpydlywrtenW<'_, Mpy32ctl0Spec> {
        MpydlywrtenW::new(self, 8)
    }
    #[doc = "Bit 9 - Delayed write mode."]
    #[inline(always)]
    pub fn mpydly32(&mut self) -> Mpydly32W<'_, Mpy32ctl0Spec> {
        Mpydly32W::new(self, 9)
    }
}
#[doc = "MPY32 control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mpy32ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpy32ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpy32ctl0Spec;
impl crate::RegisterSpec for Mpy32ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mpy32ctl0::R`](R) reader structure"]
impl crate::Readable for Mpy32ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`mpy32ctl0::W`](W) writer structure"]
impl crate::Writable for Mpy32ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPY32CTL0 to value 0"]
impl crate::Resettable for Mpy32ctl0Spec {}
