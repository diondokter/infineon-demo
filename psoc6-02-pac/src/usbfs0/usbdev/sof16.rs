#[doc = "Register `SOF16` reader"]
pub type R = crate::R<Sof16Spec>;
#[doc = "Field `FRAME_NUMBER16` reader - The frame number (11b)"]
pub type FrameNumber16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - The frame number (11b)"]
    #[inline(always)]
    pub fn frame_number16(&self) -> FrameNumber16R {
        FrameNumber16R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "Start Of Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sof16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sof16Spec;
impl crate::RegisterSpec for Sof16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof16::R`](R) reader structure"]
impl crate::Readable for Sof16Spec {}
#[doc = "`reset()` method sets SOF16 to value 0"]
impl crate::Resettable for Sof16Spec {
    const RESET_VALUE: u32 = 0;
}
