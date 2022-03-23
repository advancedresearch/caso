# Caso

Category Theory Solver for Commutative Diagrams.


```text
=== Caso 0.1 ===
Type `help` for more information.
> (A <-> B)[(A <-> C) -> (B <-> D)] <=> (C -> D)
(A <-> B)[(A <-> C) -> (B <-> D)] <=> (C <-> D)
```

To run Case from your Terminal, type:

`cargo install --example caso caso`

Then, to run:

`caso`

### Syntax

A commuative diagram in Caso is written in the following grammar:

`<left>[<top> -> <bottom>] <=> <right>`

This syntax is based on the notation for [Path Semantics](https://github.com/advancedresearch/path_semantics).

Caso automatically corrects directional errors,
e.g. when you type `(a -> b)[(c -> a) -> ...] <=> ...`.
the `c` is wrong relative to `a`,
so, Caso corrects this to `(a -> b)[(a <- c) -> ...] <=> ...`.

Higher morpisms are supported by counting `-` (1) and `=` (2) in the arrow.
For example, `<->` is a 1-isomorphism and `<=>` is a 2-isomorphism.

| Morphism | Notation |
| --- | --- |
| Directional | `->` |
| Reverse Directional | `<-` |
| Epi | `->>` |
| Reverse Epi | `<<-` |
| Mono | `!->` |
| Reverse Mono | `<-!` |
| Left Inverse | `<!->` |
| Reverse Left Inverse | `<-!>` |
| Right Inverse | `<->>` |
| Reverse Right Inverse | `<<->` |
| Epi-Mono | `!->>` |
| Reverse Epi-Mono | `<<-!` |
| Iso | `<->` |
| Zero | `<>` |
