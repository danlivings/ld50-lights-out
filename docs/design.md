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

The calculation for each tile's score is:

<img src="https://render.githubusercontent.com/render/math?math=\lfloor%20log_2{(lightness%20%2b%201)}\rfloor">

Which gives the following table:

| Tile lightness | Score given |
|---------------:|------------:|
| 0              | 0           |
| 1              | 1           |
| 2              | 1           |
| 3              | 2           |
| 4              | 2           |
| ...            | ...         |
| 7              | 3           |
| 8              | 3           |
| ...            | ...         |
| 15             | 4           |
| 16             | 4           |
| ...            | ...         |
| 31             | 5           |
| 32             | 5           |
| ...            | ...         |
| 63             | 6           |
| 64             | 6           |
| ...            | ...         |
| 127            | 7           |
| 128            | 7           |
| ...            | ...         |
| 255            | 8           |