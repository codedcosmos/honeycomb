<img align="right" src="https://raw.githubusercontent.com/codedcosmos/honeycomb/main/res/logo.png" height="200" width="200">

# Honeycomb
### By codedcosmos and Ironfist95

Honeycomb is a tool that automatically types teleport commands to pre-generate a minecraft world.

### Why pregenerate?
Generating chunks fresh from the world takes up additional cpu resources. 
The server has to first generate the chunk and then send it to the client.
If chunks are pregenerated the server simply needs to load them from disk and send them to the client, which is a lot faster.

This is why big servers like [Hermitcraft Season 7](https://www.youtube.com/watch?v=eA35S2GW-jI) pregenerate their chunks.

### Usage

Windows should work just fine, but if on linux you also need to install xdotool.

Ubuntu/Debian: `apt install libxdo-dev`

Arch: `pacman -S xdotool`

### Running
Along with some run arguments (listed below), you simply open up a terminal or command prompt and type:

`./honeycomb`

If on windows you should be able to just doubleclick on honeycomb.exe and it will launch command prompt for you.

It will wait for 5 seconds by default, and then start sending commands. 
After executing you should quickly switch to minecraft to prevent typing commands all over the place.

<details>
<summary>Run arguments</summary>

    Honeycomb - Chunk Pregenerator 1.0
    IronFist95 & codedcosmos (codedcosmos.mail@gmail.com)
    Automatically types teleport commands to pre-generate a minecraft world.

    USAGE:
    honeycomb [FLAGS] [OPTIONS]

    FLAGS:
    -r, --auto_return       Automatically returns to spawn
    -c, --creative_mode     Starts the script by setting the players gamemode to creative
    -s, --spectator_mode    Starts the script by setting the players gamemode to spectator
    -h, --help              Prints help information
    -V, --version           Prints version information

    OPTIONS:
    -t, --teleport_delay <Delay Between Teleports>
    Sets the delay between teleports (milliseconds) [default: 12000]

    -d, --input_delay <Input Delay>                      Sets the delay between keypresses (milliseconds) [default: 120]
    -p, --pregenerate_distance <Pregenerate Distance>
            Used to set how far to pregenerate (blocks). If set to 0, continues until manually stopped. [default: 1024]

    -e, --start_delay <Start Delay time>                 Sets the delay before starting (seconds) [default: 5]
    -x, --start_x <Start X>                              Sets the starting location on the x axis [default: 0]
    -y, --start_y <Start Y>                              Sets the starting location on the y axis [default: 0]
    -b, --view_buffer <View Buffer>
            Safety Buffer distance, to help make sure all chunks are loaded [default: 0.9]

    -v, --view_distance <View Distance>                  Used to calculate how far to teleport the player [default: 12]

</details>  

### Why won't it work?
Though this may not be the issue, it's likely you need to set your open chat key to t. 
You might have rebound it to a different key.

If not feel free to post an github issue :) 

### Why hexagon?
Well apart from [hexagons being the bestagons](https://www.youtube.com/watch?v=thOifuHs6eY).

Hexagons are the shape with the highest, command to area ratio, when tiling. 
Additionally, they are much closer to a circle than a rectangle, this helps make every pregenerated chunk be equidistant from spawn.

A pregenerated world can be seed below:

<img src="https://raw.githubusercontent.com/codedcosmos/honeycomb/main/res/hexagon_screenshot.png" height="400" width="400">

### Building
When building on Ubuntu/Debian make sure to install mingw: `apt install mingw-w64`

Either build directly with `cargo build --release`
or run `sh build.sh` if on linux.