# Thrill Digger solver (Expert)
The online version can be found on [GitHub Pages](https://lepelog.github.io/thrill-digger/). It uses 100MB of RAM (to store all possible boards) and it takes a few seconds to initialize
## Why expert?
Expert has the most entropy for generating boards, the amount of possible seeds is
significantly lower that the amount of valid expert boards, making it possible to predict bomb positions more accurately
## Will this always work?
No lol. The first dig is completely random, there is a chance you get rekt right there. And then, depending on the found rupees the
chances of finding a bomb in the first few digs is really high. However, the more spots are dug up, the more accurate the prediction gets.
Another case where this doesn't work is directly after booting up the game. This solver relies on a quirk of the Skyward-Sword RNG, where the RNG
eventually converges to a few loops with a total of around 2000000 possible values. At the start of the game, you might not be in the loops yet. Just run around Eldin or something, it advances the RNG thousands of times per frame.
## Why do you waste time with this minigame
The music is good
## Is there a way to read more about the technical explanaitions behind this?
not yet, although hopefully soon™
