#[doc = "Register `DIE_X_POSITION` reader"]
pub type R = crate::R<DieXPositionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die X position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_x_position::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieXPositionSpec;
impl crate::RegisterSpec for DieXPositionSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`die_x_position::R`](R) reader structure"]
impl crate::Readable for DieXPositionSpec {}
#[doc = "`reset()` method sets DIE_X_POSITION to value 0"]
impl crate::Resettable for DieXPositionSpec {}
