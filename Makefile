toc:
	doctoc --maxlevel 2 --notitle README.md
	doctoc --maxlevel 2 --notitle CN.md

link-check:
	# Dep:
	#   npm install -g markdown-link-check
	#   https://github.com/tcort/markdown-link-check
	find . -name \*.md -print0 | xargs -0 -n1 markdown-link-check --alive 200,301,302

# Build each article's images-typst/NAME.typ into images/NAME.png (144 PPI).
# Needs typst >= 0.14, the minimum for CeTZ 0.5.2.
TYPST_ARTICLES := $(patsubst %/images-typst,%,$(wildcard src/list/*/images-typst))
TYP := $(filter-out %/style.typ,$(wildcard src/list/*/images-typst/*.typ))
PNG := $(subst /images-typst/,/images/,$(TYP:.typ=.png))

images: $(PNG)

define article-images
$(1)/images/%.png: $(1)/images-typst/%.typ $(1)/images-typst/style.typ Makefile
	mkdir -p "$$(@D)"
	typst compile --ppi 144 "$$<" "$$@"
endef

$(foreach article,$(TYPST_ARTICLES),$(eval $(call article-images,$(article))))

.PHONY: images
