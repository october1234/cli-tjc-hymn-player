# CLI TJC Hymn Player (WIP)
### Description
A CLI hymn player for TJC hymns 1 ~ 469 written in rust.
### How to install
#### Compiling from source
Clone the repo and run cargo build, the executable will be avaliable in the ./target/debug/ directory.
Then run the hymn downloader executable to download the hymns from TJC's website
### Usage
Pass in the hymn number as the first argument or enter it in via stdin without params. <br/>
Once the hymn starts playing, enter commands to change the program's behaviour
#### Commands
1. V : Increases the volume
2. v : Decreases the volume
3. p : Pauses the player, press p again to unpause
4. l : Toggles looping
