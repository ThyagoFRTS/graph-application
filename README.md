## Tech
- [gtk4](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html)
- [Graphviz](https://graphviz.org/)

## Setup
Make sure to download graphviz binaries and put on root directory.
- Download Graphviz binaries from [official page](https://graphviz.org/download/)
The especific version used was [8.0.3](https://gitlab.com/api/v4/projects/4207231/packages/generic/graphviz-releases/8.0.3/windows_10_msbuild_Release_graphviz-8.0.3-win32.zip)
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



## Build Configs
### gtk
Maybe you need change gtk from crates.io to compatible verion of your gtk local instalation
