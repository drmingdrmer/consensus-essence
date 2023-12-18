toc:
	doctoc --maxlevel 2 --notitle README.md
	doctoc --maxlevel 2 --notitle CN.md

link-check:
	# Dep:
	#   npm install -g markdown-link-check
	#   https://github.com/tcort/markdown-link-check
	find . -name \*.md -print0 | xargs -0 -n1 markdown-link-check
