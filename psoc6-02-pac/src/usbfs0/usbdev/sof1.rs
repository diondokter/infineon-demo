#[doc = "Register `SOF1` reader"]
pub type R = crate::R<Sof1Spec>;
#[doc = "Field `FRAME_NUMBER_MSB` reader - It has the upper 3 bits \\[10:8\\]
of the SOF frame number."]
pub type FrameNumberMsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - It has the upper 3 bits \\[10:8\\]
of the SOF frame number."]
    #[inline(always)]
    pub fn frame_number_msb(&self) -> FrameNumberMsbR {
        FrameNumberMsbR::new((self.bits & 7) as u8)
    }
}
#[doc = "Start Of Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sof1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sof1Spec;
impl crate::RegisterSpec for Sof1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof1::R`](R) reader structure"]
impl crate::Readable for Sof1Spec {}
#[doc = "`reset()` method sets SOF1 to value 0"]
impl crate::Resettable for Sof1Spec {
    const RESET_VALUE: u32 = 0;
}
