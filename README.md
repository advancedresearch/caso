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
