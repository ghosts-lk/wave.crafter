pub struct Effects {
    pub reverb: f32, // Reverb effect intensity, ranges from 0.0 (no effect) to 1.0 (maximum effect)
    pub delay: f32,  // Delay effect intensity, ranges from 0.0 (no effect) to 1.0 (maximum effect)
}

impl Effects {
    /// Applies the reverb and delay effects to a given audio sample.
    /// 
    /// # Parameters
    /// - `sample`: The input audio sample to which effects will be applied.
    /// 
    /// # Returns
    /// - The modified audio sample after applying the effects.
    pub fn apply(&self, sample: f32) -> f32 {
        // Calculate reverb effect (simplified as a scaled version of the sample)
        let reverb_effect = sample * self.reverb * 0.1;

        // Calculate delay effect (simplified as a scaled version of the sample)
        let delay_effect = sample * self.delay * 0.01;

        // Combine the original sample with the effects
        sample + reverb_effect + delay_effect
    }
}
