# Pre-generated article audio

The blog's reader uses static MP3 files generated at build/deploy time. The browser does not call a TTS provider and does not synthesize speech locally.

## Generate locally

From the repository root:

```bash
python3 scripts/generate_tts.py --dry-run
```

The default provider is **Piper**, an open-source local neural TTS engine. It makes no network request and has no per-character charge.

Install it temporarily with Nix:

```bash
nix shell nixpkgs#piper-tts nixpkgs#ffmpeg
```

Download one free Piper voice model, then point the generator at the `.onnx` file and its companion `.onnx.json` file:

```bash
export PIPER_MODEL="$HOME/.local/share/piper/en_US-lessac-medium.onnx"
export TTS_PROVIDER=piper
python3 scripts/generate_tts.py
```

Piper models are open voice files; choose a voice whose license suits your use. The model stays local. `ffmpeg` converts Piper's WAV output to MP3.

Kokoro 82M was also evaluated as a possible higher-quality free backend. The current Nix wrapper on this ARM host fails on first run because its spaCy English model is not packaged into the runtime and it attempts a dynamic package installation. Piper is therefore the reproducible production default for now; Kokoro can be revisited when its model/runtime is packaged cleanly.

For optional hosted providers, ElevenLabs and OpenAI remain available, but they are not required:

```bash
export ELEVENLABS_API_KEY='[REDACTED]'
export ELEVENLABS_VOICE_ID='your-voice-id'
python3 scripts/generate_tts.py --provider elevenlabs
```

## Pipeline policy

Audio generation is opt-in in CI/deployment so builds remain fast. With the default Piper provider, enabling it uses local CPU time but no API credits. A gated pipeline step can run:

```bash
GENERATE_TTS=1 TTS_PROVIDER=piper PIPER_MODEL=/path/to/voice.onnx ./scripts/ci.sh
```

Run generation before the static site export so `assets/audio/` is copied into the Leptos site bundle. Do not commit API keys or generated audio unless explicitly desired; generated files are deployment artifacts.

## Sanitization

The generator:

- replaces fenced and indented code with “Code snippet omitted; see the article”;
- keeps Markdown link text while dropping URLs;
- removes HTML, headings, emphasis, blockquote markers, and image URLs;
- expands common Rust syntax such as `&mut`, `fn`, `impl`, `serde`, `::`, `->`, and `=>` into more speakable phrases;
- strips frontmatter before sending text to the provider.

## Cost estimate

With Piper, the API cost is **$0**. The only cost is local CPU time and disk space for the voice model and generated MP3s. Hosted providers remain optional and use their current published pricing if explicitly selected.

## Player

Each post renders:

```html
<audio controls preload="metadata">
  <source src="/audio/my-post-slug.mp3" type="audio/mpeg">
</audio>
```

The component also provides a download link and a text fallback. Static MP3 generation is required for the player to have audio.
