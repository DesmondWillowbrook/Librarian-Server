<center>
<img src="frontend/static/favicon.ico" />

# Librarian 
</center>

> A tool to predict the sequencing library type from the base composition of a supplied FastQ file.

For further information, detailed installation instructions and FAQs visit the [documentation pages](https://kartva.github.io/Librarian/). For an overview of some of the technical work that went into creating Librarian, have a look at [a presentation I made when interviewing at Neuralink](Librarian Technical Presentation.pdf)

Reads from high throughput sequencing experiments show base compositions that are characteristic for their library type. For example, data from ATAC-seq, RNA-seq and WGBS-seq libraries show markedly different distributions of G, A, C and T across the reads. Librarian makes use of different composition signatures for library quality control: Test library compositions are extracted and compared against previously published data sets from **mouse** and **human**.

**Please note that composition signatures from other species may vary significantly due to different overall GC content**.

To that end, Librarian produces several plots to help identify library types. For example, it produces the following given the [ATAC, bisulfite and RNA example files](frontend/example_inputs/):

- Compositions Map: UMAP representation of compositions of published sequencing data. Different library types are indicated by colours. Compositions of test libraries are projected onto the same manifold and indicated by light green circles.

![Compositions_map-2022-08-15-13-31](https://user-images.githubusercontent.com/51814158/184647396-ed51de1a-29aa-43f8-b013-5d13f6ceb645.svg)

- Probability Maps: This collection of maps shows the probability of a particular region of the map to correspond to a certain library type. The darker the colour, the more dominated the region is by the indicated library type. The location of test libraries is indicated by a light blue circle.

![Probability_maps-2022-08-15-13-31](https://user-images.githubusercontent.com/51814158/184647578-29cdab87-dc37-45e0-a187-a0c4d8a2d2fa.svg)

- Prediction Plot: For each projected test library, the location on the Compositions/Probability Map is determined. This plot shows how published library types are represented at the same location.

![Prediction_plot-2022-08-15-13-31](https://user-images.githubusercontent.com/51814158/184647529-8acf7605-eb48-4642-a614-0ae80c803023.svg)

> How to interpret: Some regions on the map are very specific to a certain library type, others are more mixed. Therefore, for some test libraries the results will be much clearer than for others. The different plots are intended to provide a good overview of how similar the test library is to published data. The cause of any deviations should be inspected; the interpretation will be different depending on how characteristic the composition signature of the library type and how far off the projection of the test sample is.

You can try Librarian at the [Babraham Institute website](https://www.bioinformatics.babraham.ac.uk/librarian/), run a [tool to query samples from the command-line, with processing happening either on the server or locally](cli/README.md), or [set up the server yourself](server/README.md).

## Folder Structure
- `frontend` contains code for the website, which consists of the webpage and WebAssembly code responsible for extracting base compositions from given files. Extracted base compositions are sent to the server for plotting.
- `server` contains code for the server, which serves the `frontend` and also responds to plotting requests.
- `cli` is a utility program to send queries to the server from the command line. It can also process queries locally if required.

### Attribution:
- `favicon.ico` sourced from [favicon.io](https://favicon.io/emoji-favicons/books) sourced from [twemoji](https://twemoji.twitter.com/), licensed under [CC BY-4](https://creativecommons.org/licenses/by/4.0/).

### Associated repositories:
- [Library Base Compositions](https://github.com/ChristelKrueger/Library_Base_Compositions) - contains the core library for extracting base compositions from FASTQ files.
- [Web Library Base Compositions](https://github.com/DesmondWillowbrook/Web_Library_Base_Compositions)
