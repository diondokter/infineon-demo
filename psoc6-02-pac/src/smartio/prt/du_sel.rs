#[doc = "Register `DU_SEL` reader"]
pub type R = crate::R<DuSelSpec>;
#[doc = "Register `DU_SEL` writer"]
pub type W = crate::W<DuSelSpec>;
#[doc = "Field `DU_TR0_SEL` reader - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
pub type DuTr0SelR = crate::FieldReader;
#[doc = "Field `DU_TR0_SEL` writer - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
pub type DuTr0SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DU_TR1_SEL` reader - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DuTr1SelR = crate::FieldReader;
#[doc = "Field `DU_TR1_SEL` writer - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DuTr1SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DU_TR2_SEL` reader - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DuTr2SelR = crate::FieldReader;
#[doc = "Field `DU_TR2_SEL` writer - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DuTr2SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DU_DATA0_SEL` reader - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
pub type DuData0SelR = crate::FieldReader;
#[doc = "Field `DU_DATA0_SEL` writer - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
pub type DuData0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DU_DATA1_SEL` reader - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
pub type DuData1SelR = crate::FieldReader;
#[doc = "Field `DU_DATA1_SEL` writer - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
pub type DuData1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_tr0_sel(&self) -> DuTr0SelR {
        DuTr0SelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr1_sel(&self) -> DuTr1SelR {
        DuTr1SelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr2_sel(&self) -> DuTr2SelR {
        DuTr2SelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub fn du_data0_sel(&self) -> DuData0SelR {
        DuData0SelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn du_data1_sel(&self) -> DuData1SelR {
        DuData1SelR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    #[must_use]
    pub fn du_tr0_sel(&mut self) -> DuTr0SelW<DuSelSpec> {
        DuTr0SelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn du_tr1_sel(&mut self) -> DuTr1SelW<DuSelSpec> {
        DuTr1SelW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn du_tr2_sel(&mut self) -> DuTr2SelW<DuSelSpec> {
        DuTr2SelW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    #[must_use]
    pub fn du_data0_sel(&mut self) -> DuData0SelW<DuSelSpec> {
        DuData0SelW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn du_data1_sel(&mut self) -> DuData1SelW<DuSelSpec> {
        DuData1SelW::new(self, 28)
    }
}
#[doc = "Data unit component input selection\n\nYou can [`read`](crate::Reg::read) this register and get [`du_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`du_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DuSelSpec;
impl crate::RegisterSpec for DuSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`du_sel::R`](R) reader structure"]
impl crate::Readable for DuSelSpec {}
#[doc = "`write(|w| ..)` method takes [`du_sel::W`](W) writer structure"]
impl crate::Writable for DuSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DU_SEL to value 0"]
impl crate::Resettable for DuSelSpec {
    const RESET_VALUE: u32 = 0;
}
