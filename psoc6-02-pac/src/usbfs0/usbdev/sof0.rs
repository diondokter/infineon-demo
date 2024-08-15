#[doc = "Register `SOF0` reader"]
pub type R = crate::R<Sof0Spec>;
#[doc = "Field `FRAME_NUMBER` reader - It has the lower 8 bits \\[7:0\\]
of the SOF frame number."]
pub type FrameNumberR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - It has the lower 8 bits \\[7:0\\]
of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number(&self) -> FrameNumberR {
        FrameNumberR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Start Of Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sof0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sof0Spec;
impl crate::RegisterSpec for Sof0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof0::R`](R) reader structure"]
impl crate::Readable for Sof0Spec {}
#[doc = "`reset()` method sets SOF0 to value 0"]
impl crate::Resettable for Sof0Spec {
    const RESET_VALUE: u32 = 0;
}
