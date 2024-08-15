#[doc = "Register `LUT_SEL[%s]` reader"]
pub type R = crate::R<LutSelSpec>;
#[doc = "Register `LUT_SEL[%s]` writer"]
pub type W = crate::W<LutSelSpec>;
#[doc = "Field `LUT_TR0_SEL` reader - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LutTr0SelR = crate::FieldReader;
#[doc = "Field `LUT_TR0_SEL` writer - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LutTr0SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LUT_TR1_SEL` reader - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LutTr1SelR = crate::FieldReader;
#[doc = "Field `LUT_TR1_SEL` writer - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LutTr1SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LUT_TR2_SEL` reader - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
pub type LutTr2SelR = crate::FieldReader;
#[doc = "Field `LUT_TR2_SEL` writer - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
pub type LutTr2SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr0_sel(&self) -> LutTr0SelR {
        LutTr0SelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr1_sel(&self) -> LutTr1SelR {
        LutTr1SelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    pub fn lut_tr2_sel(&self) -> LutTr2SelR {
        LutTr2SelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    #[must_use]
    pub fn lut_tr0_sel(&mut self) -> LutTr0SelW<LutSelSpec> {
        LutTr0SelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    #[must_use]
    pub fn lut_tr1_sel(&mut self) -> LutTr1SelW<LutSelSpec> {
        LutTr1SelW::new(self, 8)
    }
    #[doc = "Bits 16:19 - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn lut_tr2_sel(&mut self) -> LutTr2SelW<LutSelSpec> {
        LutTr2SelW::new(self, 16)
    }
}
#[doc = "LUT component input selection\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutSelSpec;
impl crate::RegisterSpec for LutSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_sel::R`](R) reader structure"]
impl crate::Readable for LutSelSpec {}
#[doc = "`write(|w| ..)` method takes [`lut_sel::W`](W) writer structure"]
impl crate::Writable for LutSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUT_SEL[%s]
to value 0"]
impl crate::Resettable for LutSelSpec {
    const RESET_VALUE: u32 = 0;
}
