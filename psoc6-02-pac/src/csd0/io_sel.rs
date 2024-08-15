#[doc = "Register `IO_SEL` reader"]
pub type R = crate::R<IoSelSpec>;
#[doc = "Register `IO_SEL` writer"]
pub type W = crate::W<IoSelSpec>;
#[doc = "Field `CSD_TX_OUT` reader - Select waveform for csd_tx_out output signal"]
pub type CsdTxOutR = crate::FieldReader;
#[doc = "Field `CSD_TX_OUT` writer - Select waveform for csd_tx_out output signal"]
pub type CsdTxOutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_OUT_EN` reader - Select waveform for csd_tx_out_en output signal"]
pub type CsdTxOutEnR = crate::FieldReader;
#[doc = "Field `CSD_TX_OUT_EN` writer - Select waveform for csd_tx_out_en output signal"]
pub type CsdTxOutEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_AMUXB_EN` reader - Select waveform for csd_tx_amuxb_en output signal"]
pub type CsdTxAmuxbEnR = crate::FieldReader;
#[doc = "Field `CSD_TX_AMUXB_EN` writer - Select waveform for csd_tx_amuxb_en output signal"]
pub type CsdTxAmuxbEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_N_OUT` reader - Select waveform for csd_tx_n_out output signal"]
pub type CsdTxNOutR = crate::FieldReader;
#[doc = "Field `CSD_TX_N_OUT` writer - Select waveform for csd_tx_n_out output signal"]
pub type CsdTxNOutW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_N_OUT_EN` reader - Select waveform for csd_tx_n_out_en output signal"]
pub type CsdTxNOutEnR = crate::FieldReader;
#[doc = "Field `CSD_TX_N_OUT_EN` writer - Select waveform for csd_tx_n_out_en output signal"]
pub type CsdTxNOutEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSD_TX_N_AMUXA_EN` reader - Select waveform for csd_tx_n_amuxa_en output signal"]
pub type CsdTxNAmuxaEnR = crate::FieldReader;
#[doc = "Field `CSD_TX_N_AMUXA_EN` writer - Select waveform for csd_tx_n_amuxa_en output signal"]
pub type CsdTxNAmuxaEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    pub fn csd_tx_out(&self) -> CsdTxOutR {
        CsdTxOutR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_out_en(&self) -> CsdTxOutEnR {
        CsdTxOutEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    pub fn csd_tx_amuxb_en(&self) -> CsdTxAmuxbEnR {
        CsdTxAmuxbEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out(&self) -> CsdTxNOutR {
        CsdTxNOutR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_out_en(&self) -> CsdTxNOutEnR {
        CsdTxNOutEnR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    pub fn csd_tx_n_amuxa_en(&self) -> CsdTxNAmuxaEnR {
        CsdTxNAmuxaEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select waveform for csd_tx_out output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_out(&mut self) -> CsdTxOutW<IoSelSpec> {
        CsdTxOutW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select waveform for csd_tx_out_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_out_en(&mut self) -> CsdTxOutEnW<IoSelSpec> {
        CsdTxOutEnW::new(self, 4)
    }
    #[doc = "Bits 12:15 - Select waveform for csd_tx_amuxb_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_amuxb_en(&mut self) -> CsdTxAmuxbEnW<IoSelSpec> {
        CsdTxAmuxbEnW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Select waveform for csd_tx_n_out output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_n_out(&mut self) -> CsdTxNOutW<IoSelSpec> {
        CsdTxNOutW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Select waveform for csd_tx_n_out_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_n_out_en(&mut self) -> CsdTxNOutEnW<IoSelSpec> {
        CsdTxNOutEnW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Select waveform for csd_tx_n_amuxa_en output signal"]
    #[inline(always)]
    #[must_use]
    pub fn csd_tx_n_amuxa_en(&mut self) -> CsdTxNAmuxaEnW<IoSelSpec> {
        CsdTxNAmuxaEnW::new(self, 24)
    }
}
#[doc = "IO output control Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`io_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoSelSpec;
impl crate::RegisterSpec for IoSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_sel::R`](R) reader structure"]
impl crate::Readable for IoSelSpec {}
#[doc = "`write(|w| ..)` method takes [`io_sel::W`](W) writer structure"]
impl crate::Writable for IoSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IO_SEL to value 0"]
impl crate::Resettable for IoSelSpec {
    const RESET_VALUE: u32 = 0;
}
