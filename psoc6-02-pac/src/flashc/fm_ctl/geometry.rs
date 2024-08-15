#[doc = "Register `GEOMETRY` reader"]
pub type R = crate::R<GeometrySpec>;
#[doc = "Field `ROW_COUNT` reader - Number of rows (minus 1): 0: 1 row 1: 2 rows 2: 3 rows ... '65535': 65536 rows"]
pub type RowCountR = crate::FieldReader<u16>;
#[doc = "Field `BANK_COUNT` reader - Number of banks (minus 1): 0: 1 bank 1: 2 banks ... '255': 256 banks"]
pub type BankCountR = crate::FieldReader;
#[doc = "Field `WORD_SIZE_LOG2` reader - Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 3: 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
pub type WordSizeLog2R = crate::FieldReader;
#[doc = "Field `PAGE_SIZE_LOG2` reader - Number of Bytes per page (log 2): 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 15: 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
pub type PageSizeLog2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Number of rows (minus 1): 0: 1 row 1: 2 rows 2: 3 rows ... '65535': 65536 rows"]
    #[inline(always)]
    pub fn row_count(&self) -> RowCountR {
        RowCountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of banks (minus 1): 0: 1 bank 1: 2 banks ... '255': 256 banks"]
    #[inline(always)]
    pub fn bank_count(&self) -> BankCountR {
        BankCountR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Number of Bytes per word (log 2). A word is defined as the data that is read from the flash macro over the R interface with a single read access: 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 3: 128 Bytes The currently planned flash macros have a word size of either 32-bit, 64-bit or 128-bit, resulting in WORD_SIZE_LOG2 settings of 2, 3 and 4 respectively."]
    #[inline(always)]
    pub fn word_size_log2(&self) -> WordSizeLog2R {
        WordSizeLog2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Number of Bytes per page (log 2): 0: 1 Byte 1: 2 Bytes 2: 4 Bytes ... 15: 32768 Bytes The currently planned flash macros have a page size of either 256 Byte or 512 Byte, resulting in PAGE_SIZE_LOG2 settings of 8 and 9 respectively."]
    #[inline(always)]
    pub fn page_size_log2(&self) -> PageSizeLog2R {
        PageSizeLog2R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Regular flash geometry\n\nYou can [`read`](crate::Reg::read) this register and get [`geometry::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GeometrySpec;
impl crate::RegisterSpec for GeometrySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`geometry::R`](R) reader structure"]
impl crate::Readable for GeometrySpec {}
#[doc = "`reset()` method sets GEOMETRY to value 0"]
impl crate::Resettable for GeometrySpec {
    const RESET_VALUE: u32 = 0;
}
