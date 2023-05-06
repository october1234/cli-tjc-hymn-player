# CLI TJC Hymn Player (WIP)
### Description
A CLI hymn player for TJC hymns 1 ~ 469 written in rust.
### How to install
#### Compiling from source
Clone the repo and run cargo build, the executable will be available in the ./target/debug/ directory.
Then run the hymn downloader executable to download the hymns from TJC's website.
Make sure the hymns folder is in the same folder as the executable.
### Usage
Pass in the hymn number as the first argument.<br/>
Once the hymn starts playing, enter commands to change the program's behaviour.
#### Commands
1. V : Increases the volume.
2. v : Decreases the volume.
3. p : Pauses the player, press p again to unpause.
4. l : Toggles looping.
5. j : Skip 5 seconds backwards.
5. k : Skip 5 seconds forwards.
5. q : Quit the player.
