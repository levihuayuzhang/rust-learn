#!/usr/bin/Rscript

t <- read.table('values.data')
library(ggplot2)
ggplot(t, aes(n, comparisons, colour = algorithm)) + geom_point() + geom_smooth
