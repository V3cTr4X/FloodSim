
# FloodSim (Asynchronous tool for HTTP stress testing)

This program is an asynchronous HTTP stress testing tool written in Rust. It leverages Rust’s powerful async capabilities to efficiently send a high volume of HTTP requests concurrently, allowing users to evaluate the performance, reliability, and scalability of web servers or APIs under load. The tool is designed for flexibility, speed, and safety, making it suitable for developers, testers, and system administrators seeking to identify bottlenecks and optimize their network infrastructure.

# ⚠️ Disclaimer:
This tool is intended solely for ethical testing and benchmarking on systems and servers you own or have explicit permission to test. Unauthorized use against third-party websites or services is strictly prohibited and may violate applicable laws. The developer assumes no responsibility for any misuse or damage caused by this software. Always ensure you comply with legal and ethical guidelines when performing stress testing.


# Installation

Install my-project with npm

```bash
  git clone https://github.com/v3ctr4x/FloodSim
  cd FloodSim
  cargo run --release
  cargo clean
  sudo cp ./target/release/floodsim /usr/bin
```


    
# Dependencies

This project uses the **Rust 2024 edition**, which requires:

- Rust **1.72.0** or later (stable recommended)

You can update your Rust compiler with:

```bash
rustup update stable
rustc --version
```

in Cargo.toml are this dependencies:


| Dependency | Version | Description                                          |
|------------|---------|------------------------------------------------------|
| reqwest    | 0.11    | HTTP client with blocking and json features enabled  |
| tokio      | 1       | Async runtime with full features  |
| colored    | 2       | For colored terminal output                           |
| clap       | 4       | CLI argument parsing with derive macros enabled      |


# Demo

```bash
░▒▓ ~/Escritorio/FloodSim/target/release  master +3  ./floodsim                                                                                                                           ✔ ▓▒░

          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          

         @@                                 @@          
         @    @@@@@@@@@@@@@@@@@@@@@@@@@@@   @@          
         @   .@@@@ @@ @@ @@ @@ @  @@ @@@@   @@           ______   __         ______     ______     _____     ______     __     __    __   
         @   :@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          /\  ___\ /\ \       /\  __ \   /\  __ \   /\  __-.  /\  ___\   /\ \   /\ "-./  \  
         @   =@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          \ \  __\ \ \ \____  \ \ \/\ \  \ \ \/\ \  \ \ \/\ \ \ \___  \  \ \ \  \ \ \-./\ \ 
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@   @@           \ \_\    \ \_____\  \ \_____\  \ \_____\  \ \____-  \/\_____\  \ \_\  \ \_\ \ \_\
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@                \/_/     \/_____/   \/_____/   \/_____/   \/____/   \/_____/   \/_/   \/_/  \_/
         @   #@@@@@@@@@@@@@@ ##  @@@@@@@@    ##                                    v1.0 --->Coded by V3cTr4X<---
         @   #@@@@@@@@@@@@ ######  @@@@@   ######       
         @   #@@@@@@@@@@@@ ########  @@  ########       
         @        @@@@@@@@@  ########  ########         
         @        @@@@@@@@@@@  ##############=          
         @           @@@@@@@@@@  ###########            
         @           @@@@@@@@@@   ########              
         @                      ############            
         @@@@@@@@@@@@@@@@@@@  ################          
                            ########    ########        
                           #######        ########      
                            ####            ####        

Missing arguments. Use --help for more information.

░▒▓ ~/Escritorio/FloodSim/target/release  master +3  ./floodsim --help                                                                                                                    ✔ ▓▒░

          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          

         @@                                 @@          
         @    @@@@@@@@@@@@@@@@@@@@@@@@@@@   @@          
         @   .@@@@ @@ @@ @@ @@ @  @@ @@@@   @@           ______   __         ______     ______     _____     ______     __     __    __   
         @   :@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          /\  ___\ /\ \       /\  __ \   /\  __ \   /\  __-.  /\  ___\   /\ \   /\ "-./  \  
         @   =@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          \ \  __\ \ \ \____  \ \ \/\ \  \ \ \/\ \  \ \ \/\ \ \ \___  \  \ \ \  \ \ \-./\ \ 
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@   @@           \ \_\    \ \_____\  \ \_____\  \ \_____\  \ \____-  \/\_____\  \ \_\  \ \_\ \ \_\
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@                \/_/     \/_____/   \/_____/   \/_____/   \/____/   \/_____/   \/_/   \/_/  \_/
         @   #@@@@@@@@@@@@@@ ##  @@@@@@@@    ##                                    v1.0 --->Coded by V3cTr4X<---
         @   #@@@@@@@@@@@@ ######  @@@@@   ######       
         @   #@@@@@@@@@@@@ ########  @@  ########       
         @        @@@@@@@@@  ########  ########         
         @        @@@@@@@@@@@  ##############=          
         @           @@@@@@@@@@  ###########            
         @           @@@@@@@@@@   ########              
         @                      ############            
         @@@@@@@@@@@@@@@@@@@  ################          
                            ########    ########        
                           #######        ########      
                            ####            ####        

Asynchronous tool for HTTP stress testing

Usage: floodsim [OPTIONS]

Options:
  -u, --url <URL>            URL for testing
  -t, --tasks <TASKS>        Assincronous task
  -d, --duration <DURATION>  Attack duration
  -h, --help                 Print help
  -V, --version              Print version

░▒▓ ~/Escritorio/FloodSim/target/release  master +3  ./floodsim --task 50 --duration 5 --url https://127.0.0.1:8080                                                                 ✔ ▓▒░

          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          

         @@                                 @@          
         @    @@@@@@@@@@@@@@@@@@@@@@@@@@@   @@          
         @   .@@@@ @@ @@ @@ @@ @  @@ @@@@   @@           ______   __         ______     ______     _____     ______     __     __    __   
         @   :@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          /\  ___\ /\ \       /\  __ \   /\  __ \   /\  __-.  /\  ___\   /\ \   /\ "-./  \  
         @   =@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          \ \  __\ \ \ \____  \ \ \/\ \  \ \ \/\ \  \ \ \/\ \ \ \___  \  \ \ \  \ \ \-./\ \ 
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@   @@           \ \_\    \ \_____\  \ \_____\  \ \_____\  \ \____-  \/\_____\  \ \_\  \ \_\ \ \_\
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@                \/_/     \/_____/   \/_____/   \/_____/   \/____/   \/_____/   \/_/   \/_/  \_/
         @   #@@@@@@@@@@@@@@ ##  @@@@@@@@    ##                                    v1.0 --->Coded by V3cTr4X<---
         @   #@@@@@@@@@@@@ ######  @@@@@   ######       
         @   #@@@@@@@@@@@@ ########  @@  ########       
         @        @@@@@@@@@  ########  ########         
         @        @@@@@@@@@@@  ##############=          
         @           @@@@@@@@@@  ###########            
         @           @@@@@@@@@@   ########              
         @                      ############            
         @@@@@@@@@@@@@@@@@@@  ################          
                            ########    ########        
                           #######        ########      
                            ####            ####        

error: unexpected argument '--task' found

  tip: a similar argument exists: '--tasks'

Usage: floodsim --tasks <TASKS>

For more information, try '--help'.

░▒▓ ~/Escritorio/FloodSim/target/release  master +3  ./floodsim --tasks 50 --duration 5 --url https://127.0.0.1:8080                                                            

          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          

         @@                                 @@          
         @    @@@@@@@@@@@@@@@@@@@@@@@@@@@   @@          
         @   .@@@@ @@ @@ @@ @@ @  @@ @@@@   @@           ______   __         ______     ______     _____     ______     __     __    __   
         @   :@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          /\  ___\ /\ \       /\  __ \   /\  __ \   /\  __-.  /\  ___\   /\ \   /\ "-./  \  
         @   =@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          \ \  __\ \ \ \____  \ \ \/\ \  \ \ \/\ \  \ \ \/\ \ \ \___  \  \ \ \  \ \ \-./\ \ 
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@   @@           \ \_\    \ \_____\  \ \_____\  \ \_____\  \ \____-  \/\_____\  \ \_\  \ \_\ \ \_\
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@                \/_/     \/_____/   \/_____/   \/_____/   \/____/   \/_____/   \/_/   \/_/  \_/
         @   #@@@@@@@@@@@@@@ ##  @@@@@@@@    ##                                    v1.0 --->Coded by V3cTr4X<---
         @   #@@@@@@@@@@@@ ######  @@@@@   ######       
         @   #@@@@@@@@@@@@ ########  @@  ########       
         @        @@@@@@@@@  ########  ########         
         @        @@@@@@@@@@@  ##############=          
         @           @@@@@@@@@@  ###########            
         @           @@@@@@@@@@   ########              
         @                      ############            
         @@@@@@@@@@@@@@@@@@@  ################          
                            ########    ########        
                           #######        ########      
                            ####            ####        

Starting the attack for 5 seconds with 50 task to https://127.0.0.1:8080...
Good petitions: 114
Task per seconds: 22.80

```



