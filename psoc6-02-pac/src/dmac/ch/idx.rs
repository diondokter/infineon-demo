#[doc = "Register `IDX` reader"]
pub type R = crate::R<IdxSpec>;
#[doc = "Field `X` reader - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
pub type XR = crate::FieldReader<u16>;
#[doc = "Field `Y` reader - Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
pub type YR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor."]
    #[inline(always)]
    pub fn x(&self) -> XR {
        XR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Specifies the Y loop index, with Y_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it loads a descriptor.."]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel current indices\n\nYou can [`read`](crate::Reg::read) this register and get [`idx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdxSpec;
impl crate::RegisterSpec for IdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idx::R`](R) reader structure"]
impl crate::Readable for IdxSpec {}
#[doc = "`reset()` method sets IDX to value 0"]
impl crate::Resettable for IdxSpec {
    const RESET_VALUE: u32 = 0;
}
