# nuttchess
*Ethan Thummel / UraniumNutt 2025*

For a long time, I have wanted to get *really good* at rust. 🦀
So I set out to make a somewhat big project: A chess engine!

This engine is a work in progress, but so far it has:
- UCI
- Bitboard representation 
- *Magic* bitboards for 'ray' like pieces
- Simple piece square tables
- Basic time managment
- Quiescence search
- Iterative deepening
- Negamax / Alpha Beta pruning
- Primitive move ordering

I would also like to add:
- Transposition tables
- Nullmove pruning
- MVV-LVA move ordering
- Late move reduction
- More robust UCI
- And more 😀

I think it would be nifty to achieve a rating of 2500 - 3000 elo! 🤓
Speaking of elo, you can play with the engine here! https://lichess.org/@/nuttchess_bot

I found online information to be really valuable in developing nuttchess, especially:
- The BBC chess engine and the associated videos by Code Monkey King (https://github.com/maksimKorzh/bbc)
- The Chess Programming Wiki (https://www.chessprogramming.org/Main_Page)

I hope by making nuttchess open source it could become a resource for others who are interested
in chess programming. Perhaps one day I will even make my own set of videos explaining the nuttchess engine 😏

* License 📜
This project is MIT licensed. See LICENSE.txt for more details.
