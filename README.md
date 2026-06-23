# typst2docx

> The missing Word bridge for [Typst](https://typst.app) — convert `.typ` to `.docx`
> with equations as **native, editable Word math**, fully offline, in a single binary.

<!-- Replace YOUR_USERNAME before pushing -->
[![CI](https://github.com/YOUR_USERNAME/typst2docx/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/typst2docx/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/typst2docx.svg)](https://crates.io/crates/typst2docx)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
![Status](https://img.shields.io/badge/status-alpha-orange.svg)

Typst is a joy to write in. But the moment you need to **submit to a journal, send a
draft to a co-author, or answer "can you send the Word version?"**, you hit a wall:
the rest of academia runs on `.docx`.

Today your options are bad:

- **Pandoc's Typst reader** re-implements its own parser and *does not evaluate* Typst.
  Anything that relies on packages, scripting, or `#import` degrades or breaks.
- **typ2docx** routes your document through **Adobe's paid PDF Services** — it uploads
  your file to the cloud, needs Adobe credentials, and only recovers equations.

`typst2docx` takes a third path: it drives the **official `typst` compiler** to emit
HTML + MathML, then translates that into clean OOXML. Your equations land in Word as
**real, clickable, editable math** — not flattened bitmaps — and nothing ever leaves
your machine.

---

## How it differs

| | **typst2docx** | Pandoc (`--from typst`) | typ2docx (PyPI) |
|---|---|---|---|
| Typst language fidelity (packages, scripting, `#import`) | **Full** — uses the real compiler | Partial — custom reader, no evaluation | Full — via Typst's own PDF |
| Equations in the output | **Native editable OMML** | Native OMML, for what the reader parsed | Native OMML (math only) |
| Runs 100% locally | **Yes** | Yes | **No** — uploads to Adobe |
| Third-party services | **None** | None | Adobe PDF Services (paid) |
| Cost | **Free** | Free | Paid Adobe tier |
| Distribution | **Single Rust binary** | Single binary | Python + Adobe creds |

The structural edge is the input side: because we let Typst itself evaluate the
document, we inherit *everything* Typst can do for free — and we never depend on a
cloud service to do it.

---

## Status — alpha

This is early. It already produces valid Word documents that open cleanly in Word,
LibreOffice, and Google Docs, but coverage is a work in progress.

**Works today**
- Headings (H1–H6), paragraphs, bold / italic / inline code
- Ordered and unordered lists, including nesting
- Tables, including horizontally merged cells (`colspan` → `w:gridSpan`)
- Inline and display equations → OMML: fractions, super/subscripts, roots,
  identifiers, numbers, operators

**Not yet (see roadmap)**
- Matrices and big operators with limits (`mtable`, `munder`/`mover`)
- Vertically merged table cells (`w:vMerge`)
- Real hyperlinks and embedded images
- `--reference-doc` journal templates
- Live cross-references and citation fields

If a construct isn't handled yet, its text content is preserved rather than dropped.

---

## Install

From source (requires a recent stable Rust):

```sh
cargo install --git https://github.com/YOUR_USERNAME/typst2docx
```

To convert `.typ` files directly you also need the [`typst`](https://github.com/typst/typst)
CLI on your `PATH` (v0.14+, for HTML export). You can skip it if you feed pre-rendered
HTML instead (see below).

---

## Usage

```sh
# Straight from Typst (shells out to `typst` for HTML export):
typst2docx paper.typ -o paper.docx

# Or convert HTML you produced yourself:
typst compile --features html --format html paper.typ paper.html
typst2docx paper.html -o paper.docx

# Format is inferred from the extension; override with --from if needed:
typst2docx weird-name --from typst
```

---

## How it works

```
  paper.typ
     │
     │  ① typst compile --features html --format html
     ▼
  HTML + MathML            ← Typst evaluates the whole document for us
     │
     │  ② parse → intermediate representation
     ▼
  IR (headings / lists / tables / math / inlines)
     │
     │  ③ lower each node to OOXML   (MathML → OMML is the heart of it)
     ▼
  word/document.xml + styles + numbering
     │
     │  ④ zip the OOXML parts
     ▼
  paper.docx   ← equations are native, editable Word math
```

Building on Typst's official HTML export is a deliberate choice: it means we ride
Typst's own evaluation and semantic structuring instead of fighting its unstable
internal model, and our effort concentrates on the one hard, valuable problem —
faithful **HTML + MathML → OOXML + OMML** translation for academic documents.

---

## Roadmap

- [ ] MathML coverage: matrices (`mtable`), sums/integrals with limits
      (`munder`/`mover`/`munderover`), stretchy fences, accents
- [ ] Vertically merged cells (`w:vMerge`)
- [ ] Embedded images and figure captions
- [ ] Real `<w:hyperlink>` relationships
- [ ] `--reference-doc` templates for journal-specific styling
- [ ] Bookmarks + `REF` fields for live cross-references; citation fields
- [ ] In-process Typst (`typst` + `typst-kit` + `typst-html`) for a true single binary
- [ ] Publish prebuilt binaries and a `crates.io` release

Contributions very welcome — the math mapping in particular is easy to extend one
MathML element at a time.

---

## Contributing

Issues and PRs are welcome. Good first contributions: add a MathML element to the
`mathml → OMML` mapping, or drop in a `.typ` file that converts poorly so we can turn
it into a fixture. Run `cargo test` and `cargo fmt` before opening a PR.

---

## License

Licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option. Typst itself is Apache-2.0, so this stays compatible.
