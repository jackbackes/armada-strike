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

def generate_place_sound():
    """Generate a satisfying 'thunk' sound for placing ships"""
    sample_rate = 22050
    duration = 0.15
    samples = int(sample_rate * duration)
    
    t = np.linspace(0, duration, samples)
    
    # Create a low frequency thump
    thump_freq = 100
    thump = np.sin(2 * np.pi * thump_freq * t)
    
    # Add a higher frequency click at the beginning
    click_freq = 800
    click_duration = 0.02
    click_samples = int(click_duration * sample_rate)
    click = np.zeros(samples)
    click[:click_samples] = np.sin(2 * np.pi * click_freq * t[:click_samples])
    
    # Combine with different envelopes
    thump_envelope = np.exp(-t * 12)
    click_envelope = np.zeros(samples)
    click_envelope[:click_samples] = np.exp(-t[:click_samples] * 80)
    
    # Mix the sounds
    sound = thump * thump_envelope * 0.7 + click * click_envelope * 0.3
    
    # Add a subtle square wave for 8-bit feel
    square = np.sign(np.sin(2 * np.pi * thump_freq * t)) * 0.1
    sound = sound + square * thump_envelope
    
    # Normalize and convert to 16-bit
    sound = np.clip(sound, -1, 1)
    sound = (sound * 32767 * 0.8).astype(np.int16)
    
    # Write WAV file
    with wave.open('assets/sounds/place.wav', 'wb') as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(sound.tobytes())
    
    print("Generated place.wav (ship placement sound)")

def generate_sink_sound():
    """Generate a dramatic sinking sound with bubbles and descent"""
    sample_rate = 22050
    duration = 1.0  # Longer for dramatic effect
    samples = int(sample_rate * duration)
    
    t = np.linspace(0, duration, samples)
    
    # Descending tone (like something sinking)
    start_freq = 400
    end_freq = 50
    freq_sweep = np.linspace(start_freq, end_freq, samples)
    sinking_tone = np.sin(2 * np.pi * freq_sweep * t)
    
    # Add bubble sounds (random bursts)
    bubbles = np.zeros(samples)
    num_bubbles = 8
    for i in range(num_bubbles):
        bubble_start = int(np.random.uniform(0.1, 0.7) * samples)
        bubble_duration = int(0.05 * sample_rate)
        if bubble_start + bubble_duration < samples:
            bubble_freq = np.random.uniform(600, 1200)
            bubble_t = np.linspace(0, 0.05, bubble_duration)
            bubble_sound = np.sin(2 * np.pi * bubble_freq * bubble_t) * np.exp(-bubble_t * 30)
            bubbles[bubble_start:bubble_start + bubble_duration] += bubble_sound * 0.3
    
    # Water splash at the beginning
    splash_duration = 0.15
    splash_samples = int(splash_duration * sample_rate)
    splash = np.random.normal(0, 1, splash_samples) * 0.5
    # Low-pass filter effect
    for _ in range(3):
        splash = np.convolve(splash, np.ones(3)/3, mode='same')
    
    # Combine everything
    sound = np.zeros(samples)
    sound[:splash_samples] += splash
    sound += sinking_tone * 0.6
    sound += bubbles
    
    # Apply envelope
    envelope = np.ones(samples)
    envelope[:int(0.1 * sample_rate)] = np.linspace(0, 1, int(0.1 * sample_rate))  # Fade in
    envelope[int(0.8 * sample_rate):] = np.linspace(1, 0, samples - int(0.8 * sample_rate))  # Fade out
    sound = sound * envelope
    
    # Add 8-bit quantization
    levels = 64
    sound = np.round(sound * levels) / levels
    
    # Normalize and convert to 16-bit
    sound = np.clip(sound, -1, 1)
    sound = (sound * 32767 * 0.7).astype(np.int16)
    
    # Write WAV file
    with wave.open('assets/sounds/sink.wav', 'wb') as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(sound.tobytes())
    
    print("Generated sink.wav (ship sinking sound)")

if __name__ == "__main__":
    generate_pew_sound()
    generate_boom_sound()
    generate_place_sound()
    generate_sink_sound()
    print("Sound effects generated successfully!")