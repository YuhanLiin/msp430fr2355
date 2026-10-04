#[doc = "Register `DIE_Y_POSITION` reader"]
pub type R = crate::R<DieYPositionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Die Y position\n\nYou can [`read`](crate::Reg::read) this register and get [`die_y_position::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DieYPositionSpec;
impl crate::RegisterSpec for DieYPositionSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`die_y_position::R`](R) reader structure"]
impl crate::Readable for DieYPositionSpec {}
#[doc = "`reset()` method sets DIE_Y_POSITION to value 0"]
impl crate::Resettable for DieYPositionSpec {}
