# Synthesizer
This is a modular synthesizer library for a fun side project -- building a synthesizer controller in real life and connecting it to a raspberry pi to have a (more) affordable synthesizer to mess around with.

# Design
I've designed this with a more top-down approach, unlike most digital synthesizer libraries. Instead of doing something like:
```rust
let osc = OscillatorNode::new();
let gain = GainNode::new();

osc.connect(gain);
gain.connect(audio_out);

fn process_audio() -> f32 {
  gain.receive()
}
```

I designed it more like:
```rust
let osc = OscillatorNode::new();
let gain = GainNode::new();

fn process_audio() -> f32 {
  osc.process();
  gain.receive(osc);
  gain.get_out()
}
```

I hope by designing it this way it will be much more memory efficient, easier to follow, and easier to debug!

### Debugging audio
This command converts the `audio.raw` file into a .wav to be open with any editor: `ffmpeg -f f32le -ar 48000 -i audio.raw -f wav file-out.wav`

### Repositories I consulted
https://github.com/hosackm/BiquadFilter
