fn main()
{
    cc::Build::new()
        .include("libsquish")
        .file("src/wrapper.cpp")
        .file("libsquish/alpha.cpp")
        .file("libsquish/clusterfit.cpp")
        .file("libsquish/colourblock.cpp")
        .file("libsquish/colourfit.cpp")
        .file("libsquish/colourset.cpp")
        .file("libsquish/maths.cpp")
        .file("libsquish/rangefit.cpp")
        .file("libsquish/singlecolourfit.cpp")
        .file("libsquish/squish.cpp")
        .cpp(true)
        .compile("libsquish");
}

