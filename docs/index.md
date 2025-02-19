---
hide:
  - footer
---


#   
# Librarian

> A tool to predict the sequencing library type from the base composition of a supplied FastQ file.

Reads from high throughput sequencing experiments show base compositions that are characteristic for their library type. For example, data from RNA-seq and WGBS-seq libraries show markedly different distributions of G, A, C and T across the reads. Librarian makes use of different composition signatures to predict the library type of a given test sample by comparing it against previously published data sets
from **mouse** and **human**.

The input to Librarian is just a fastq file, the file which your sequencing provided will have given you.  If you have paired end sequencing where you have two fastq files per sample then you use the read1 file.

To help assess the similarity to published data sets, Librarian produces several plots. The examples shown below are

| sample | library type |
| ----- | ----|
| sample 1 | ATAC-Seq |
| sample 2 | RNA-Seq |
| sample 3 | Bisulfite(BS)-Seq |


## Compositions Map

UMAP representation of compositions of published sequencing data. Different library types are indicated by colours. Compositions of test libraries are projected onto the same manifold and indicated by light green circles.

![Compositions_map-2022-08-15-13-31](https://user-images.githubusercontent.com/51814158/184647396-ed51de1a-29aa-43f8-b013-5d13f6ceb645.svg)

## Probability Maps

 This collection of maps shows the probability of a particular region of the map to correspond to a certain library type. The darker the colour, the more dominated the region is by the indicated library type. The location of test libraries is indicated by a light blue circle.

![Probability_maps-2022-08-15-13-31](https://user-images.githubusercontent.com/51814158/184647578-29cdab87-dc37-45e0-a187-a0c4d8a2d2fa.svg)

## Prediction Plot 

For each projected test library, the location on the Compositions/Probability Map is determined. This plot shows how published library types are represented at the same location.

![Prediction_plot-2022-08-15-13-31](https://user-images.githubusercontent.com/51814158/184647529-8acf7605-eb48-4642-a614-0ae80c803023.svg)

## How to interpret

 Some regions on the map are very specific to a certain library type, others are more mixed. Therefore, for some test libraries the results will be much clearer than for others. The different plots are intended to provide a good overview of how similar the test library is to published data. The cause of any deviations should be inspected; the interpretation will be different depending on how characteristic the composition signature of the library type and how far off the projection of the test sample is.