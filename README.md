# NandScript2
My rewrite of NandScript. This one is more organized and has an interactive shell.

Only NAND is defined at first. Arguments are unsigned 8 bit integers (unsigned bytes).

To define a new Chip called `MYCHIP`, with args `a`, `b`, and `c`, do:

```
MYCHIP(a, b, c) = OTHERCHIP1(a, OTHERCHIP2(b, c))
```

where `OTHERCHIP1` and `OTHERCHIP2` were previously defined chips.

See below screenshot for example usage:

![image](https://user-images.githubusercontent.com/18058977/231328971-ac9ba902-c0af-4928-b076-e803562cbd75.png)
