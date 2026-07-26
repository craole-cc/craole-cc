#!/usr/bin/env python3
"""Pre-generate natural-sounding MP3 narration for published Markdown posts.

No network request is made with --dry-run. Credentials are read only from the
process environment and are never written to disk.
"""
from __future__ import annotations

import argparse
import html
import os
import re
import subprocess
import sys
import tempfile
import urllib.error
import urllib.request
from pathlib import Path

CODE_SENTENCE = "Code snippet omitted; see the article."


def sanitize_markdown(markdown: str) -> str:
    """Turn technical Markdown into narration-friendly plain text."""
    text = markdown.replace("\r\n", "\n")
    text = re.sub(r"(?ms)^\s*```[^\n]*\n.*?^\s*```\s*$", f" {CODE_SENTENCE} ", text)
    text = re.sub(r"(?m)^(?:    |\t).+$", f" {CODE_SENTENCE} ", text)
    text = re.sub(r"!\[([^]]*)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"\[([^]]+)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"<[^>]+>", " ", text)
    text = re.sub(r"`([^`]+)`", lambda m: _spoken_inline_code(m.group(1)), text)
    text = re.sub(r"^\s{0,3}#{1,6}\s*", "", text, flags=re.MULTILINE)
    text = re.sub(r"^\s*>\s?", "", text, flags=re.MULTILINE)
    text = re.sub(r"[*_~]", "", text)
    text = html.unescape(text)
    text = re.sub(r"&mut\b", "mutable reference", text)
    text = re.sub(r"\bfn\b", "function", text)
    text = re.sub(r"\bimpl\b", "implementation", text)
    text = re.sub(r"\bserde\b", "ser-dee", text, flags=re.IGNORECASE)
    text = text.replace("::", " double colon ")
    text = text.replace("->", " returns ").replace("=>", " produces ")
    text = re.sub(r"\s+", " ", text)
    return text.strip()


def _spoken_inline_code(value: str) -> str:
    value = re.sub(r"&mut\b", "mutable reference", value)
    value = re.sub(r"\bfn\b", "function", value)
    value = re.sub(r"\bimpl\b", "implementation", value)
    value = re.sub(r"\bserde\b", "ser-dee", value, flags=re.IGNORECASE)
    return value.replace("::", " double colon ").replace("->", " returns ")


def parse_frontmatter(path: Path) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---"):
        return {}
    end = text.find("\n---", 3)
    if end < 0:
        return {}
    result: dict[str, str] = {}
    for line in text[4:end].splitlines():
        key, separator, value = line.partition(":")
        if separator:
            result[key.strip()] = value.strip().strip('"\'')
    return result


def post_text(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    if text.startswith("---"):
        end = text.find("\n---", 3)
        if end >= 0:
            text = text[end + 4 :]
    return sanitize_markdown(text)


def request_piper_audio(text: str, model: str) -> bytes:
    """Run local Piper and convert its WAV output to MP3 with ffmpeg."""
    piper = os.environ.get("PIPER_BIN", "piper")
    ffmpeg = os.environ.get("FFMPEG_BIN", "ffmpeg")
    with tempfile.TemporaryDirectory(prefix="craole-tts-") as directory:
        wav = Path(directory) / "speech.wav"
        mp3 = Path(directory) / "speech.mp3"
        try:
            subprocess.run(
                [piper, "-m", model, "-f", str(wav)],
                input=text.encode("utf-8"),
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            subprocess.run(
                [ffmpeg, "-hide_banner", "-loglevel", "error", "-y", "-i", str(wav), "-codec:a", "libmp3lame", "-q:a", "4", str(mp3)],
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
        except FileNotFoundError as error:
            raise RuntimeError(f"local TTS tool is missing: {error.filename}; install piper-tts and ffmpeg") from error
        except subprocess.CalledProcessError as error:
            detail = error.stderr.decode("utf-8", errors="replace")[-500:]
            raise RuntimeError(f"local Piper generation failed: {detail}") from error
        return mp3.read_bytes()


def request_audio(provider: str, text: str, voice: str) -> bytes:
    if provider == "piper":
        return request_piper_audio(text, voice)
    if provider == "elevenlabs":
        key = os.environ.get("ELEVENLABS_API_KEY")
        if not key:
            raise RuntimeError("ELEVENLABS_API_KEY is required")
        url = f"https://api.elevenlabs.io/v1/text-to-speech/{voice}"
        payload = '{"text":' + _json_string(text) + ',"model_id":"eleven_multilingual_v2","output_format":"mp3_44100_128"}'
        headers = {"xi-api-key": key, "Accept": "audio/mpeg", "Content-Type": "application/json"}
    elif provider == "openai":
        key = os.environ.get("OPENAI_API_KEY")
        if not key:
            raise RuntimeError("OPENAI_API_KEY is required")
        url = "https://api.openai.com/v1/audio/speech"
        payload = '{"model":"tts-1-hd","voice":' + _json_string(voice) + ',"input":' + _json_string(text) + ',"response_format":"mp3"}'
        headers = {"Authorization": f"Bearer {key}", "Content-Type": "application/json"}
    else:
        raise ValueError(f"unsupported provider: {provider}")
    request = urllib.request.Request(url, data=payload.encode(), headers=headers, method="POST")
    try:
        with urllib.request.urlopen(request, timeout=180) as response:
            return response.read()
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", errors="replace")[:500]
        raise RuntimeError(f"{provider} API returned HTTP {error.code}: {detail}") from error


def _json_string(value: str) -> str:
    import json
    return json.dumps(value, ensure_ascii=False)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--output", type=Path, default=None, help="default: ROOT/assets/audio")
    parser.add_argument("--provider", choices=("piper", "elevenlabs", "openai"), default=os.getenv("TTS_PROVIDER", "piper"))
    parser.add_argument("--voice", default=None, help="ElevenLabs voice ID or OpenAI voice name")
    parser.add_argument("--dry-run", action="store_true", help="print sanitized text; never call an API")
    parser.add_argument("posts", nargs="*", type=Path, help="specific Markdown files; default: all published posts")
    args = parser.parse_args()

    root = args.root.resolve()
    output = (args.output or root / "public" / "audio").resolve()
    paths = args.posts or sorted((root / "content" / "posts").glob("*.md"))
    if not paths:
        print("No Markdown posts found", file=sys.stderr)
        return 1
    if args.provider == "piper":
        default_voice = os.getenv("PIPER_MODEL", "")
    else:
        default_voice = os.getenv("ELEVENLABS_VOICE_ID" if args.provider == "elevenlabs" else "OPENAI_TTS_VOICE", "")
    voice = args.voice or default_voice
    if not args.dry_run and not voice:
        print("A voice is required: --voice or the provider voice environment variable", file=sys.stderr)
        return 2

    for path in paths:
        path = path if path.is_absolute() else root / path
        metadata = parse_frontmatter(path)
        if metadata.get("published", "true").lower() not in ("true", "1", "yes"):
            continue
        slug = metadata.get("slug") or path.stem
        text = post_text(path)
        if not text:
            print(f"skip {slug}: no narration text")
            continue
        if args.dry_run:
            print(f"--- {slug} ---\n{text}\n")
            continue
        output.mkdir(parents=True, exist_ok=True)
        destination = output / f"{slug}.mp3"
        destination.write_bytes(request_audio(args.provider, text, voice))
        print(f"generated {destination} ({destination.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
