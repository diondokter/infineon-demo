#[doc = "Register `I2C_CFG` reader"]
pub type R = crate::R<I2cCfgSpec>;
#[doc = "Register `I2C_CFG` writer"]
pub type W = crate::W<I2cCfgSpec>;
#[doc = "Field `SDA_IN_FILT_TRIM` reader - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal SRAM memory. 1: enable clock_scb_en, has no effect on ec_busy_pp 0: disable clock_scb_en, enable ec_busy_pp (grant I2CS_EC or SPIS_EC access)"]
pub type SdaInFiltTrimR = crate::FieldReader;
#[doc = "Field `SDA_IN_FILT_TRIM` writer - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal SRAM memory. 1: enable clock_scb_en, has no effect on ec_busy_pp 0: disable clock_scb_en, enable ec_busy_pp (grant I2CS_EC or SPIS_EC access)"]
pub type SdaInFiltTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_IN_FILT_SEL` reader - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SdaInFiltSelR = crate::BitReader;
#[doc = "Field `SDA_IN_FILT_SEL` writer - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SdaInFiltSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_IN_FILT_TRIM` reader - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SclInFiltTrimR = crate::FieldReader;
#[doc = "Field `SCL_IN_FILT_TRIM` writer - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SclInFiltTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCL_IN_FILT_SEL` reader - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SclInFiltSelR = crate::BitReader;
#[doc = "Field `SCL_IN_FILT_SEL` writer - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
pub type SclInFiltSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDA_OUT_FILT0_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SdaOutFilt0TrimR = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT0_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SdaOutFilt0TrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_OUT_FILT1_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SdaOutFilt1TrimR = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT1_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SdaOutFilt1TrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_OUT_FILT2_TRIM` reader - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SdaOutFilt2TrimR = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT2_TRIM` writer - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
pub type SdaOutFilt2TrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDA_OUT_FILT_SEL` reader - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
pub type SdaOutFiltSelR = crate::FieldReader;
#[doc = "Field `SDA_OUT_FILT_SEL` writer - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
pub type SdaOutFiltSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal SRAM memory. 1: enable clock_scb_en, has no effect on ec_busy_pp 0: disable clock_scb_en, enable ec_busy_pp (grant I2CS_EC or SPIS_EC access)"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SdaInFiltTrimR {
        SdaInFiltTrimR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SdaInFiltSelR {
        SdaInFiltSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SclInFiltTrimR {
        SclInFiltTrimR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SclInFiltSelR {
        SclInFiltSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SdaOutFilt0TrimR {
        SdaOutFilt0TrimR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SdaOutFilt1TrimR {
        SdaOutFilt1TrimR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SdaOutFilt2TrimR {
        SdaOutFilt2TrimR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SdaOutFiltSelR {
        SdaOutFiltSelR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim bits for 'i2c_sda_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values. SDA_IN_FILT_TRIM\\[1\\]
is used to enable I2CS_EC or SPIS_EC access to internal SRAM memory. 1: enable clock_scb_en, has no effect on ec_busy_pp 0: disable clock_scb_en, enable ec_busy_pp (grant I2CS_EC or SPIS_EC access)"]
    #[inline(always)]
    #[must_use]
    pub fn sda_in_filt_trim(&mut self) -> SdaInFiltTrimW<I2cCfgSpec> {
        SdaInFiltTrimW::new(self, 0)
    }
    #[doc = "Bit 4 - Selection of 'i2c_sda_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn sda_in_filt_sel(&mut self) -> SdaInFiltSelW<I2cCfgSpec> {
        SdaInFiltSelW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Trim bits for 'i2c_scl_in' 50 ns filter. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    #[must_use]
    pub fn scl_in_filt_trim(&mut self) -> SclInFiltTrimW<I2cCfgSpec> {
        SclInFiltTrimW::new(self, 8)
    }
    #[doc = "Bit 12 - Selection of 'i2c_scl_in' filter delay: '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn scl_in_filt_sel(&mut self) -> SclInFiltSelW<I2cCfgSpec> {
        SclInFiltSelW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Trim bits for 'i2c_sda_out' 50 ns filter 0. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt0_trim(&mut self) -> SdaOutFilt0TrimW<I2cCfgSpec> {
        SdaOutFilt0TrimW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Trim bits for 'i2c_sda_out' 50 ns filter 1. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt1_trim(&mut self) -> SdaOutFilt1TrimW<I2cCfgSpec> {
        SdaOutFilt1TrimW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Trim bits for 'i2c_sda_out' 50 ns filter 2. See s8i2cs BROS (001-59539) for more details on the trim bit values."]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt2_trim(&mut self) -> SdaOutFilt2TrimW<I2cCfgSpec> {
        SdaOutFilt2TrimW::new(self, 20)
    }
    #[doc = "Bits 28:29 - Selection of cumulative 'i2c_sda_out' filter delay: '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn sda_out_filt_sel(&mut self) -> SdaOutFiltSelW<I2cCfgSpec> {
        SdaOutFiltSelW::new(self, 28)
    }
}
#[doc = "I2C configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCfgSpec;
impl crate::RegisterSpec for I2cCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_cfg::R`](R) reader structure"]
impl crate::Readable for I2cCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_cfg::W`](W) writer structure"]
impl crate::Writable for I2cCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CFG to value 0x002a_1013"]
impl crate::Resettable for I2cCfgSpec {
    const RESET_VALUE: u32 = 0x002a_1013;
}
