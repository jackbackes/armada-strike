#!/usr/bin/env python3
import numpy as np
import wave
import struct

def generate_pew_sound():
    """Generate a retro 'pew' sound for misses - like a laser shot"""
    sample_rate = 22050
    duration = 0.15
    samples = int(sample_rate * duration)
    
    # Create a descending frequency sweep (laser sound)
    t = np.linspace(0, duration, samples)
    
    # Start at 800Hz, sweep down to 200Hz
    start_freq = 800
    end_freq = 200
    freq = np.linspace(start_freq, end_freq, samples)
    
    # Generate sine wave with frequency sweep
    wave_data = np.sin(2 * np.pi * freq * t)
    
    # Apply envelope (quick attack, quick decay)
    envelope = np.exp(-t * 15)  # Exponential decay
    wave_data = wave_data * envelope
    
    # Add some square wave harmonics for 8-bit feel
    square_wave = np.sign(np.sin(2 * np.pi * freq * t * 0.5)) * 0.2
    wave_data = wave_data * 0.7 + square_wave * envelope * 0.3
    
    # Normalize and convert to 16-bit
    wave_data = np.clip(wave_data, -1, 1)
    wave_data = (wave_data * 32767).astype(np.int16)
    
    # Write WAV file
    with wave.open('assets/sounds/miss.wav', 'wb') as wav_file:
        wav_file.setnchannels(1)  # Mono
        wav_file.setsampwidth(2)  # 16-bit
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(wave_data.tobytes())
    
    print("Generated miss.wav (pew sound)")

def generate_boom_sound():
    """Generate a retro 'boom' explosion sound for hits"""
    sample_rate = 22050
    duration = 0.3
    samples = int(sample_rate * duration)
    
    t = np.linspace(0, duration, samples)
    
    # Create white noise base
    noise = np.random.normal(0, 1, samples)
    
    # Low-pass filter simulation (crude but effective for 8-bit style)
    # Apply multiple passes for stronger effect
    filtered = noise.copy()
    for _ in range(3):
        filtered = np.convolve(filtered, np.ones(5)/5, mode='same')
    
    # Add some low frequency rumble
    rumble_freq = 40
    rumble = np.sin(2 * np.pi * rumble_freq * t) * 0.5
    
    # Combine noise and rumble
    explosion = filtered * 0.6 + rumble
    
    # Apply envelope (sharp attack, gradual decay)
    attack_time = 0.01
    attack_samples = int(attack_time * sample_rate)
    envelope = np.ones(samples)
    envelope[:attack_samples] = np.linspace(0, 1, attack_samples)
    envelope[attack_samples:] = np.exp(-t[attack_samples:] * 8)
    
    explosion = explosion * envelope
    
    # Add initial "punch" with a short sine burst
    punch_freq = 80
    punch_duration = 0.02
    punch_samples = int(punch_duration * sample_rate)
    punch = np.zeros(samples)
    punch[:punch_samples] = np.sin(2 * np.pi * punch_freq * t[:punch_samples]) * 0.8
    punch[:punch_samples] *= np.exp(-t[:punch_samples] * 50)  # Quick decay
    
    # Combine everything
    final_sound = explosion + punch
    
    # Normalize and add slight clipping for 8-bit harshness
    final_sound = np.clip(final_sound * 1.2, -1, 1)
    
    # Quantize to fewer levels for 8-bit feel
    levels = 32
    final_sound = np.round(final_sound * levels) / levels
    
    # Convert to 16-bit
    final_sound = (final_sound * 32767 * 0.7).astype(np.int16)  # Slightly quieter
    
    # Write WAV file
    with wave.open('assets/sounds/hit.wav', 'wb') as wav_file:
        wav_file.setnchannels(1)  # Mono
        wav_file.setsampwidth(2)  # 16-bit
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(final_sound.tobytes())
    
    print("Generated hit.wav (boom sound)")

if __name__ == "__main__":
    generate_pew_sound()
    generate_boom_sound()
    print("Sound effects generated successfully!")