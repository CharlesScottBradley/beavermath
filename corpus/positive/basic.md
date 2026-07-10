# Positive corpus (normative)

| id | input | expect_spans | notes |
|----|-------|--------------|-------|
| p01 | `The answer is $x^2+y^2=z^2$.` | 1 | inline dollar |
| p02 | `Display $$\int_0^1 x\,dx$$` | 1 | display dollar |
| p03 | `Inline \(a+b\) ok` | 1 | paren |
| p04 | `Block \[c=d\]` | 1 | bracket |
| p05 | fence math `x+1` | 1 | see fence_math.txt |
| p06 | `mix $a$ and $$b$$` | 2 | mixed |
