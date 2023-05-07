# Graph APP
This application was made to study the Rust language and also for the discipline of algorithm design

<div align="center">
  <img src="./app_images/not_loaded.png" alt="login" width="500" height="435" style="margin: 5px;">
</div>
<div align="center">
  <img src="./app_images/loaded.png" alt="void home" width="390" height="418" style="margin: 5px;">
  <img src="./app_images/tree.png" alt="home page" width="390" height="418" style="margin: 5px;">
</div>

## Functionalities
- Tree detection
- Display adjacency between two nodes
- Get all neighbors from selected nodes
- Visit all edges
- Display selected node degree
- Export .dot file to feed graphviz framework
- Export svg graph from .dot file

## Setup Dependencies
Make sure to download graphviz binaries and put on root directory, this is the more important dependency to plot graph.
The especific version used was [8.0.3](https://gitlab.com/api/v4/projects/4207231/packages/generic/graphviz-releases/8.0.3/windows_10_msbuild_Release_graphviz-8.0.3-win32.zip)

- Download Graphviz binaries from [official page](https://graphviz.org/download/)
- Paste zip file content in program root directory
The three folder will be like this:
<pre>
├── /.fingerprint
├── /build
├── /deps
├── /examples
└── /Graphviz
    └── /bin
        └── some_files.some_extension
├── /incremental
├── graph_project.exe
└── some_files.some_extension
</pre>

# How to load your graph file
Fist, your graph need to stay in txt file in respective format:
- First line have "D" to digraph and "ND" to bidirectional graph
- Other line must be tuple of nodes. Brackets, braces, parentheses and letters are optional. Numbers and comma are required.

Important, if your nodes sequence starts with 1 or contains gaps in the count sequence, like [(A1,A4),(A1,A2),(A2,A4)], this nodes will be replaced to [(v0,v2),(v0,v1),(v1,v2)]. Input file examples is above:
```
ND
v0,v3
v1,v4
v3,v1
```
```
ND
node0,node3
node1,node4
node3,node1
```
```
ND
A0,A3
A1,A4
A3,A1
```

## Tech
- [gtk4](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html)
- [Graphviz](https://graphviz.org/)


## Build Configs
### gtk
Maybe you need change gtk from crates.io to compatible verion of your gtk local instalation
