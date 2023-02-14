# Vicda's dice
Dice game I saw a bunch of nerds playing at school.  
I have out-nerded them by coding their silly game in rust.

The game goes as follows, the player throws six dice from which they will try to get some points  
The goal is to reach 4000 points in the fewest number of turns  
Points are awarded when the dice match a certain pattern e.g 2,2,2 5,5,5,5 1,2,3,4,5 etc  
If the player chooses a pattern those dice are 'consumed' and they will continue to throw with the remaining dice  
This process repeats until the either 'banks' the points they have accumulated during their turn or they go 'bust' by throwing  
and having no scoring options available  
Their turn ends after this, if they banked the points it goes towards their score but if they went bust they get nothing  
If all dice are consumed they player gets six more dice  
The player must deiced to either peruse more points by throwing and risk losing them or bank their points, potentiality  
missing out on more  

## Running
clone the repo and run with cargo
```fish
git clone https://github.com/ElSargo/Vicdas-dice 
cd Vicdas-dice 
cargo run --release
```

