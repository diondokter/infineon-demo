#[doc = "Register `BOOKMARK` reader"]
pub type R = crate::R<BookmarkSpec>;
#[doc = "Register `BOOKMARK` writer"]
pub type W = crate::W<BookmarkSpec>;
#[doc = "Field `BOOKMARK` reader - Used by FW. Keeps the Current HV cycle sequence"]
pub type BookmarkR = crate::FieldReader<u32>;
#[doc = "Field `BOOKMARK` writer - Used by FW. Keeps the Current HV cycle sequence"]
pub type BookmarkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    pub fn bookmark(&self) -> BookmarkR {
        BookmarkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Used by FW. Keeps the Current HV cycle sequence"]
    #[inline(always)]
    #[must_use]
    pub fn bookmark(&mut self) -> BookmarkW<BookmarkSpec> {
        BookmarkW::new(self, 0)
    }
}
#[doc = "Bookmark register - keeps the current FW HV seq\n\nYou can [`read`](crate::Reg::read) this register and get [`bookmark::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bookmark::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BookmarkSpec;
impl crate::RegisterSpec for BookmarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bookmark::R`](R) reader structure"]
impl crate::Readable for BookmarkSpec {}
#[doc = "`write(|w| ..)` method takes [`bookmark::W`](W) writer structure"]
impl crate::Writable for BookmarkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOKMARK to value 0"]
impl crate::Resettable for BookmarkSpec {
    const RESET_VALUE: u32 = 0;
}
