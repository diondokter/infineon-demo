#[doc = "Register `GEOMETRY_SUPERVISORY` reader"]
pub type R = crate::R<GeometrySupervisorySpec>;
#[doc = "Field `ROW_COUNT` reader - Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
pub type RowCountR = crate::FieldReader<u16>;
#[doc = "Field `BANK_COUNT` reader - Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
pub type BankCountR = crate::FieldReader;
#[doc = "Field `WORD_SIZE_LOG2` reader - Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
pub type WordSizeLog2R = crate::FieldReader;
#[doc = "Field `PAGE_SIZE_LOG2` reader - Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
pub type PageSizeLog2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Number of rows (minus 1). ROW_COUNT is typically less than GEOMETRY.ROW_COUNT"]
    #[inline(always)]
    pub fn row_count(&self) -> RowCountR {
        RowCountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of banks (minus 1). BANK_COUNT is less or equal to GEOMETRY.BANK_COUNT."]
    #[inline(always)]
    pub fn bank_count(&self) -> BankCountR {
        BankCountR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Number of Bytes per word (log 2). See GEOMETRY.WORD_SIZE_LOG2. Typically, WORD_SIZE_LOG2 equals GEOMETRY.WORD_SIZE_LOG2."]
    #[inline(always)]
    pub fn word_size_log2(&self) -> WordSizeLog2R {
        WordSizeLog2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Number of Bytes per page (log 2). See GEOMETRY.PAGE_SIZE_LOG2. Typically, PAGE_SIZE_LOG2 equals GEOMETRY.PAGE_SIZE_LOG2."]
    #[inline(always)]
    pub fn page_size_log2(&self) -> PageSizeLog2R {
        PageSizeLog2R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Supervisory flash geometry\n\nYou can [`read`](crate::Reg::read) this register and get [`geometry_supervisory::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GeometrySupervisorySpec;
impl crate::RegisterSpec for GeometrySupervisorySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`geometry_supervisory::R`](R) reader structure"]
impl crate::Readable for GeometrySupervisorySpec {}
#[doc = "`reset()` method sets GEOMETRY_SUPERVISORY to value 0"]
impl crate::Resettable for GeometrySupervisorySpec {
    const RESET_VALUE: u32 = 0;
}
