## Simple_TCP_Check CLI - Multiple Hosts using Rust

<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#build">Build</a></li>
    <li><a href="#using-example">Using Example</a></li>
  </ol>
</details>

## Build
$ git clone https://github.com/andrealvesguimaraes/simple_tcp_check.git

$ cd simple-tcp-check

$ cargo build --release

## Using Example
$ ./target/release/tcp-check --file iplist.txt --port 8080
