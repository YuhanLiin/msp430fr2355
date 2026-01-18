#[doc = "Register `TB0CCTL0` reader"]
pub type R = crate::R<Tb0cctl0Spec>;
#[doc = "Register `TB0CCTL0` writer"]
pub type W = crate::W<Tb0cctl0Spec>;
#[doc = "Capture/compare interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccifg {
    #[doc = "0: No interrupt pending"]
    Ccifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Ccifg1 = 1,
}
impl From<Ccifg> for bool {
    #[inline(always)]
    fn from(variant: Ccifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIFG` reader - Capture/compare interrupt flag"]
pub type CcifgR = crate::BitReader<Ccifg>;
impl CcifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccifg {
        match self.bits {
            false => Ccifg::Ccifg0,
            true => Ccifg::Ccifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_ccifg_0(&self) -> bool {
        *self == Ccifg::Ccifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_ccifg_1(&self) -> bool {
        *self == Ccifg::Ccifg1
    }
}
#[doc = "Field `CCIFG` writer - Capture/compare interrupt flag"]
pub type CcifgW<'a, REG> = crate::BitWriter<'a, REG, Ccifg>;
impl<'a, REG> CcifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ccifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccifg::Ccifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ccifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccifg::Ccifg1)
    }
}
#[doc = "Capture overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cov {
    #[doc = "0: No capture overflow occurred"]
    Cov0 = 0,
    #[doc = "1: Capture overflow occurred"]
    Cov1 = 1,
}
impl From<Cov> for bool {
    #[inline(always)]
    fn from(variant: Cov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COV` reader - Capture overflow"]
pub type CovR = crate::BitReader<Cov>;
impl CovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cov {
        match self.bits {
            false => Cov::Cov0,
            true => Cov::Cov1,
        }
    }
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn is_cov_0(&self) -> bool {
        *self == Cov::Cov0
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn is_cov_1(&self) -> bool {
        *self == Cov::Cov1
    }
}
#[doc = "Field `COV` writer - Capture overflow"]
pub type CovW<'a, REG> = crate::BitWriter<'a, REG, Cov>;
impl<'a, REG> CovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No capture overflow occurred"]
    #[inline(always)]
    pub fn cov_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cov::Cov0)
    }
    #[doc = "Capture overflow occurred"]
    #[inline(always)]
    pub fn cov_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cov::Cov1)
    }
}
#[doc = "Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    #[doc = "0: Output low"]
    Low = 0,
    #[doc = "1: Output high"]
    High = 1,
}
impl From<Out> for bool {
    #[inline(always)]
    fn from(variant: Out) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT` reader - Output"]
pub type OutR = crate::BitReader<Out>;
impl OutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Out {
        match self.bits {
            false => Out::Low,
            true => Out::High,
        }
    }
    #[doc = "Output low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Out::Low
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Out::High
    }
}
#[doc = "Field `OUT` writer - Output"]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG, Out>;
impl<'a, REG> OutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Out::Low)
    }
    #[doc = "Output high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Out::High)
    }
}
#[doc = "Field `CCI` reader - Capture/compare input"]
pub type CciR = crate::BitReader;
#[doc = "Capture/compare interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccie {
    #[doc = "0: Interrupt disabled"]
    Ccie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Ccie1 = 1,
}
impl From<Ccie> for bool {
    #[inline(always)]
    fn from(variant: Ccie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Capture/compare interrupt enable"]
pub type CcieR = crate::BitReader<Ccie>;
impl CcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccie {
        match self.bits {
            false => Ccie::Ccie0,
            true => Ccie::Ccie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_ccie_0(&self) -> bool {
        *self == Ccie::Ccie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_ccie_1(&self) -> bool {
        *self == Ccie::Ccie1
    }
}
#[doc = "Field `CCIE` writer - Capture/compare interrupt enable"]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG, Ccie>;
impl<'a, REG> CcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ccie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccie::Ccie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ccie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccie::Ccie1)
    }
}
#[doc = "Output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outmod {
    #[doc = "0: OUT bit value"]
    Outmod0 = 0,
    #[doc = "1: Set"]
    Outmod1 = 1,
    #[doc = "2: Toggle/reset"]
    Outmod2 = 2,
    #[doc = "3: Set/reset"]
    Outmod3 = 3,
    #[doc = "4: Toggle"]
    Outmod4 = 4,
    #[doc = "5: Reset"]
    Outmod5 = 5,
    #[doc = "6: Toggle/set"]
    Outmod6 = 6,
    #[doc = "7: Reset/set"]
    Outmod7 = 7,
}
impl From<Outmod> for u8 {
    #[inline(always)]
    fn from(variant: Outmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outmod {
    type Ux = u8;
}
impl crate::IsEnum for Outmod {}
#[doc = "Field `OUTMOD` reader - Output mode"]
pub type OutmodR = crate::FieldReader<Outmod>;
impl OutmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outmod {
        match self.bits {
            0 => Outmod::Outmod0,
            1 => Outmod::Outmod1,
            2 => Outmod::Outmod2,
            3 => Outmod::Outmod3,
            4 => Outmod::Outmod4,
            5 => Outmod::Outmod5,
            6 => Outmod::Outmod6,
            7 => Outmod::Outmod7,
            _ => unreachable!(),
        }
    }
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn is_outmod_0(&self) -> bool {
        *self == Outmod::Outmod0
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_outmod_1(&self) -> bool {
        *self == Outmod::Outmod1
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn is_outmod_2(&self) -> bool {
        *self == Outmod::Outmod2
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn is_outmod_3(&self) -> bool {
        *self == Outmod::Outmod3
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn is_outmod_4(&self) -> bool {
        *self == Outmod::Outmod4
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_outmod_5(&self) -> bool {
        *self == Outmod::Outmod5
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn is_outmod_6(&self) -> bool {
        *self == Outmod::Outmod6
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn is_outmod_7(&self) -> bool {
        *self == Outmod::Outmod7
    }
}
#[doc = "Field `OUTMOD` writer - Output mode"]
pub type OutmodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Outmod, crate::Safe>;
impl<'a, REG> OutmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OUT bit value"]
    #[inline(always)]
    pub fn outmod_0(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod0)
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn outmod_1(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod1)
    }
    #[doc = "Toggle/reset"]
    #[inline(always)]
    pub fn outmod_2(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod2)
    }
    #[doc = "Set/reset"]
    #[inline(always)]
    pub fn outmod_3(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod3)
    }
    #[doc = "Toggle"]
    #[inline(always)]
    pub fn outmod_4(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod4)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn outmod_5(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod5)
    }
    #[doc = "Toggle/set"]
    #[inline(always)]
    pub fn outmod_6(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod6)
    }
    #[doc = "Reset/set"]
    #[inline(always)]
    pub fn outmod_7(self) -> &'a mut crate::W<REG> {
        self.variant(Outmod::Outmod7)
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    #[doc = "0: Compare mode"]
    Compare = 0,
    #[doc = "1: Capture mode"]
    Capture = 1,
}
impl From<Cap> for bool {
    #[inline(always)]
    fn from(variant: Cap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP` reader - Capture mode"]
pub type CapR = crate::BitReader<Cap>;
impl CapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap {
        match self.bits {
            false => Cap::Compare,
            true => Cap::Capture,
        }
    }
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == Cap::Compare
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Cap::Capture
    }
}
#[doc = "Field `CAP` writer - Capture mode"]
pub type CapW<'a, REG> = crate::BitWriter<'a, REG, Cap>;
impl<'a, REG> CapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(Cap::Compare)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Cap::Capture)
    }
}
#[doc = "Compare latch load\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clld {
    #[doc = "0: TBxCLn loads on write to TBxCCRn"]
    Clld0 = 0,
    #[doc = "1: TBxCLn loads when TBxR counts to 0"]
    Clld1 = 1,
    #[doc = "2: TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode)."]
    Clld2 = 2,
    #[doc = "3: TBxCLn loads when TBxR counts to TBxCLn"]
    Clld3 = 3,
}
impl From<Clld> for u8 {
    #[inline(always)]
    fn from(variant: Clld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clld {
    type Ux = u8;
}
impl crate::IsEnum for Clld {}
#[doc = "Field `CLLD` reader - Compare latch load"]
pub type ClldR = crate::FieldReader<Clld>;
impl ClldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clld {
        match self.bits {
            0 => Clld::Clld0,
            1 => Clld::Clld1,
            2 => Clld::Clld2,
            3 => Clld::Clld3,
            _ => unreachable!(),
        }
    }
    #[doc = "TBxCLn loads on write to TBxCCRn"]
    #[inline(always)]
    pub fn is_clld_0(&self) -> bool {
        *self == Clld::Clld0
    }
    #[doc = "TBxCLn loads when TBxR counts to 0"]
    #[inline(always)]
    pub fn is_clld_1(&self) -> bool {
        *self == Clld::Clld1
    }
    #[doc = "TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode)."]
    #[inline(always)]
    pub fn is_clld_2(&self) -> bool {
        *self == Clld::Clld2
    }
    #[doc = "TBxCLn loads when TBxR counts to TBxCLn"]
    #[inline(always)]
    pub fn is_clld_3(&self) -> bool {
        *self == Clld::Clld3
    }
}
#[doc = "Field `CLLD` writer - Compare latch load"]
pub type ClldW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clld, crate::Safe>;
impl<'a, REG> ClldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TBxCLn loads on write to TBxCCRn"]
    #[inline(always)]
    pub fn clld_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clld::Clld0)
    }
    #[doc = "TBxCLn loads when TBxR counts to 0"]
    #[inline(always)]
    pub fn clld_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clld::Clld1)
    }
    #[doc = "TBxCLn loads when TBxR counts to 0 (up or continuous mode). TBxCLn loads when TBxR counts to TBxCL0 or to 0 (up/down mode)."]
    #[inline(always)]
    pub fn clld_2(self) -> &'a mut crate::W<REG> {
        self.variant(Clld::Clld2)
    }
    #[doc = "TBxCLn loads when TBxR counts to TBxCLn"]
    #[inline(always)]
    pub fn clld_3(self) -> &'a mut crate::W<REG> {
        self.variant(Clld::Clld3)
    }
}
#[doc = "Synchronize capture source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scs {
    #[doc = "0: Asynchronous capture"]
    Async = 0,
    #[doc = "1: Synchronous capture"]
    Sync = 1,
}
impl From<Scs> for bool {
    #[inline(always)]
    fn from(variant: Scs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCS` reader - Synchronize capture source"]
pub type ScsR = crate::BitReader<Scs>;
impl ScsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scs {
        match self.bits {
            false => Scs::Async,
            true => Scs::Sync,
        }
    }
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Scs::Async
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Scs::Sync
    }
}
#[doc = "Field `SCS` writer - Synchronize capture source"]
pub type ScsW<'a, REG> = crate::BitWriter<'a, REG, Scs>;
impl<'a, REG> ScsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous capture"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Async)
    }
    #[doc = "Synchronous capture"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Scs::Sync)
    }
}
#[doc = "Capture/compare input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccis {
    #[doc = "0: CCIxA"]
    Ccia = 0,
    #[doc = "1: CCIxB"]
    Ccib = 1,
    #[doc = "2: GND"]
    Gnd = 2,
    #[doc = "3: VCC"]
    Vcc = 3,
}
impl From<Ccis> for u8 {
    #[inline(always)]
    fn from(variant: Ccis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccis {
    type Ux = u8;
}
impl crate::IsEnum for Ccis {}
#[doc = "Field `CCIS` reader - Capture/compare input select"]
pub type CcisR = crate::FieldReader<Ccis>;
impl CcisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccis {
        match self.bits {
            0 => Ccis::Ccia,
            1 => Ccis::Ccib,
            2 => Ccis::Gnd,
            3 => Ccis::Vcc,
            _ => unreachable!(),
        }
    }
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn is_ccia(&self) -> bool {
        *self == Ccis::Ccia
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn is_ccib(&self) -> bool {
        *self == Ccis::Ccib
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Ccis::Gnd
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == Ccis::Vcc
    }
}
#[doc = "Field `CCIS` writer - Capture/compare input select"]
pub type CcisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccis, crate::Safe>;
impl<'a, REG> CcisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCIxA"]
    #[inline(always)]
    pub fn ccia(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccia)
    }
    #[doc = "CCIxB"]
    #[inline(always)]
    pub fn ccib(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Ccib)
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Gnd)
    }
    #[doc = "VCC"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut crate::W<REG> {
        self.variant(Ccis::Vcc)
    }
}
#[doc = "Capture mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cm {
    #[doc = "0: No capture"]
    None = 0,
    #[doc = "1: Capture on rising edge"]
    Rising = 1,
    #[doc = "2: Capture on falling edge"]
    Falling = 2,
    #[doc = "3: Capture on both rising and falling edges"]
    Both = 3,
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cm {
    type Ux = u8;
}
impl crate::IsEnum for Cm {}
#[doc = "Field `CM` reader - Capture mode"]
pub type CmR = crate::FieldReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            0 => Cm::None,
            1 => Cm::Rising,
            2 => Cm::Falling,
            3 => Cm::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "No capture"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cm::None
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Cm::Rising
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Cm::Falling
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Cm::Both
    }
}
#[doc = "Field `CM` writer - Capture mode"]
pub type CmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cm, crate::Safe>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No capture"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::None)
    }
    #[doc = "Capture on rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Rising)
    }
    #[doc = "Capture on falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Falling)
    }
    #[doc = "Capture on both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Both)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&self) -> CcifgR {
        CcifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&self) -> CovR {
        CovR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare input"]
    #[inline(always)]
    pub fn cci(&self) -> CciR {
        CciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&self) -> OutmodR {
        OutmodR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Compare latch load"]
    #[inline(always)]
    pub fn clld(&self) -> ClldR {
        ClldR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&self) -> ScsR {
        ScsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&self) -> CcisR {
        CcisR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline(always)]
    pub fn ccifg(&mut self) -> CcifgW<'_, Tb0cctl0Spec> {
        CcifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture overflow"]
    #[inline(always)]
    pub fn cov(&mut self) -> CovW<'_, Tb0cctl0Spec> {
        CovW::new(self, 1)
    }
    #[doc = "Bit 2 - Output"]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<'_, Tb0cctl0Spec> {
        OutW::new(self, 2)
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CcieW<'_, Tb0cctl0Spec> {
        CcieW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Output mode"]
    #[inline(always)]
    pub fn outmod(&mut self) -> OutmodW<'_, Tb0cctl0Spec> {
        OutmodW::new(self, 5)
    }
    #[doc = "Bit 8 - Capture mode"]
    #[inline(always)]
    pub fn cap(&mut self) -> CapW<'_, Tb0cctl0Spec> {
        CapW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Compare latch load"]
    #[inline(always)]
    pub fn clld(&mut self) -> ClldW<'_, Tb0cctl0Spec> {
        ClldW::new(self, 9)
    }
    #[doc = "Bit 11 - Synchronize capture source"]
    #[inline(always)]
    pub fn scs(&mut self) -> ScsW<'_, Tb0cctl0Spec> {
        ScsW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Capture/compare input select"]
    #[inline(always)]
    pub fn ccis(&mut self) -> CcisW<'_, Tb0cctl0Spec> {
        CcisW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Capture mode"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<'_, Tb0cctl0Spec> {
        CmW::new(self, 14)
    }
}
#[doc = "Timer_B Capture/Compare Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0cctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0cctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb0cctl0Spec;
impl crate::RegisterSpec for Tb0cctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0cctl0::R`](R) reader structure"]
impl crate::Readable for Tb0cctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tb0cctl0::W`](W) writer structure"]
impl crate::Writable for Tb0cctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB0CCTL0 to value 0"]
impl crate::Resettable for Tb0cctl0Spec {}
