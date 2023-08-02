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
$ Cargo add clap

$ Cargo build --release

## Using Example
$ ./tcp-check --file iplist.txt --port 8080
