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
    ///
    /// # Notes
    /// - The reverb effect is simplified as a scaled version of the sample. In a real-world implementation, the reverb effect would be more complex, e.g. using a digital signal processing algorithm.
    /// - The delay effect is also simplified as a scaled version of the sample. In a real-world implementation, the delay effect would use a circular buffer to store the audio samples and then read them back with a delay.
    pub fn apply(&self, sample: f32) -> f32 {
        // Apply the reverb effect (simplified as a scaled version of the sample)
        let reverb_effect = sample * self.reverb * 0.1;

        // Apply the delay effect (simplified as a scaled version of the sample)
        let delay_effect = sample * self.delay * 0.01;

        // Combine the original sample with the effects
        sample + reverb_effect + delay_effect
    }
}
