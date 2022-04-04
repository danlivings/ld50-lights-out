Lights Out
===

A game for [Ludum Dare 50](https://ldjam.com/events/ludum-dare/50).

Made using the [Bevy](https://bevyengine.org/) game engine for Rust.

---

> "As the area of light increases so does the circumference of darkness."
>
> _~ attributed to Albert Einstein_

*Lights Out* is a tile-based game.
Each tile can be either black, white, or a shade of grey.
White tiles will fade, to grey and then to black, over time.
If all tiles become black, you lose the game.

Clicking on a black or a white tile has no effect.
Clicking on a grey tile will turn it white, and lightens surrounding tiles.

Each tick, the tiles will fade, and your score will be updated.
Lighter tiles provide a higher score than dark ones.