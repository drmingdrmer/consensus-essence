# Distributed Consensus Essence

This repository lists bugs, traps and improvements in Paxos and Raft.
Each idea is one entry, written in English and in Chinese, and `README.md`
links to every entry.

## Files

- `src/list/<slug>/<slug>.md` is an entry's English version, and
  `<slug>.cn.md` is its Chinese version. `raft-read-index-relaxed-order` is
  the one exception: its Chinese file ends in `-cn.md`.
- `src/list/<slug>/images-typst/` holds the Typst source of the entry's
  figures, and `src/list/<slug>/images/` holds the PNGs built from it.
- `README.md` holds the type table and the `## Ideas` list of entries.
- `Makefile`: `make images` builds the figures.
- `.github/workflows/build.yml` checks every link on each push, with the
  settings in `mlc_config.json`.
- `src/scripts/img-add-margin.sh` made the `-margin` copies of some legacy
  images.

## Writing an entry

- The first line of both language files is the title,
  `## <Protocol>: (<Type>): <Title>`. `<Protocol>` is `Paxos`, `Raft` or
  `Paxos/Raft`. The Chinese file usually keeps the English title.
- `<Type>` is one of these:
  - `Bug`: a bug that breaks consensus.
  - `Trap`: not a bug, but misleading, so people may believe it is a bug.
  - `Suboptimal`: a solution that works, but not in the best way.
  - `Optimize`: an improvement to a current design.
  - `Variant`: another way to implement consensus.
  - `Generalize`: a more general protocol that keeps safety and contains
    the original as a special case.
- The first paragraph states the claim. Then a concrete scenario names the
  nodes, the terms or ballots, and the log entries. A `Bug` or `Trap` entry
  shows the execution that fails. Other types state each protocol change and
  argue that safety still holds.
- Sections use `###` and `####`. Sources go in a `**References**:` list at
  the end.
- A link to code is a permalink pinned to a commit SHA, with a line range
  such as `#L284-L307`.
- An entry from a contributor starts with
  `This approach is proposed by [name](url)`.
- Both language files say the same thing. A change to one goes into the
  other in the same commit.

## Adding an entry to README.md

In the same commit as a new entry, add it to the `## Ideas` list in
`README.md`:

```markdown
- [<Protocol>: (<Type>): <Title>](src/list/<slug>/<slug>.md)
  | 🌎 [中文版](src/list/<slug>/<slug>.cn.md)
```

A contributed entry adds a third line, `  | by [name](url)`. Paxos entries
come before Raft entries, and a new entry goes at the end of its protocol's
group.

## Figures

Draw every new or changed figure in Typst, with the CeTZ package. Use
another format only for an image that cannot be drawn, such as a screenshot.

- Each figure is one file, `src/list/<slug>/images-typst/<slug>-<name>.typ`.
  The entry's shared helpers go in `images-typst/style.typ`, which
  `make images` does not compile.
- `make images` compiles each figure into
  `src/list/<slug>/images/<slug>-<name>.png`, at 144 PPI. It needs Typst
  0.14 or later, because CeTZ 0.5.2 requires it.
- The Makefile rule for each PNG depends on the entry's
  `images-typst/style.typ`, so every entry with figures needs that file.
- Typst takes the figure's directory as the project root. So a figure can
  import only files inside its own `images-typst/`, and each entry keeps its
  own `style.typ`. Start it from the generic part of
  `src/list/asymmetric-paxos/images-typst/style.typ`: the palette, `pen`,
  `card`, `panel`, `note`, `node` and `diagram`.
- A figure file imports `style.typ` and draws one
  `diagram(w, h, title, subtitle, body)`, directly or through an entry
  helper such as `rw-figure`.
- The fonts in `style.typ` (Arial, Menlo, Apple Color Emoji) come with
  macOS. On a system without them, Typst falls back to other fonts and the
  PNGs change. So build the figures on macOS.
- Both language files show the same PNG, so the text in a figure is English.
  Both files reference it as `![<name>](images/<slug>-<name>.png)`.
- Open each PNG and check it after `make images`. Commit the `.typ` files
  and the PNGs together: GitHub shows the committed PNGs, and nothing builds
  them.

Six entries still use legacy figures: tldraw JPEGs with a `.tldr` source
beside them, Excalidraw PNGs that embed their scene, and one draw.io SVG.
When a change touches a legacy figure, redraw it in Typst. Then delete the
old image, its `-margin` copy and its source, as commit `0310613` did for
`asymmetric-paxos`.

## Checking links

To check the links in a changed file the way CI does, run
`markdown-link-check -c mlc_config.json <file>`. It needs
`npm install -g markdown-link-check`. `make link-check` checks every file
without `mlc_config.json`, so it also reports the zhihu and Stack Overflow
links that CI skips.

## Commits

The commit scope is the entry slug, for example
`fix: raft-read-index: narrow the livelock claim`. Changes to CI or to
`mlc_config.json` use the scope `ci`.
