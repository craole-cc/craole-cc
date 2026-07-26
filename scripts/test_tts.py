#!/usr/bin/env python3
import unittest

from generate_tts import CODE_SENTENCE, sanitize_markdown


class SanitizerTests(unittest.TestCase):
    def test_technical_markdown_becomes_speech(self):
        source = """# Deploying Rust\n\nUse [`&mut`] with [Leptos](https://leptos.dev).\n\n```rust\nfn main() { println!(\"hello\"); }\n```\n\nThe `serde::Serialize` implementation returns a value."""
        result = sanitize_markdown(source)
        self.assertNotIn("```", result)
        self.assertNotIn("https://", result)
        self.assertIn("mutable reference", result)
        self.assertIn(CODE_SENTENCE, result)
        self.assertIn("ser-dee double colon Serialize", result)
        self.assertIn("implementation", result)

    def test_html_and_formatting_are_removed(self):
        result = sanitize_markdown("<p>**Hello** <em>world</em></p>\n## Next")
        self.assertEqual(result, "Hello world Next")


if __name__ == "__main__":
    unittest.main()
