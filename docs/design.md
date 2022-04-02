Game Design
===

> "As the area of light increases so does the circumference of darkness."
>
> *~ attributed to Albert Einstein*

## 1. Overview
Lights Out is a tile-based game with an infinite grid.
Each tile can be either black, white, or a shade of grey.
White tiles will fade, to grey and then to black, over time.
If all tiles become black, you lose the game.

## 2. Interactions
Clicking on a black or a white tile has no effect.
Clicking on a grey tile will turn it white, and lightens surrounding tiles.
Creating a closed loop of non-black tiles surrounding black tiles will turn all
tiles on and within the loop white.

## 3. Scoring
The score is incremented every tick by an amount determined by the non-black
tiles present in the grid.
This provides an easy way to check the loss condition: if the score does not
increment at all in a tick, this means that all tiles are black, and the game
ends.

The specifics of this increment have not yet been determined and are subject to
balancing, but the potential options are:
* a flat amount for each non-black tile
* an amount determined by each tile's lightness.
