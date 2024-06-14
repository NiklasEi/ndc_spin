#include "file.typ"

a test template
#let input = if sys.inputs.keys().contains("input") {json.decode(sys.inputs.input);} else {json("sample.json");}
#input.test

#set table(
    stroke: none,
    gutter: 0.2em,
    fill: (x, y) =>
      if x == 0 or y == 0 { gray },
    inset: (right: 1.5em),
)

#table(
  columns: 4,
  [], [Exam 1], [Exam 2], [Exam 3],
  ..for (name, one, two, three) in input.items {
    (name, one, two, three)
  }
)

#set text(font: "PT Sans")
This is sans-serif. \
هذا عربي.

#set text(font: (
  "Inria Serif",
  "Noto Sans Arabic",
))

This is Latin. \
هذا عربي.