df = read.delim('testData/PCS_data.csv', row.names=1, sep=",")
head(df)

cfiles = system('ls testData/TestClustering/*.tsv  -rt' ,intern=T)
cfiles

grouping = read.delim(cfiles[length(cfiles)], sep="\t", row.names=1)
head(grouping)

df_scale = t(apply( df, 1, function(x) { (x-min(x))/ ( min(x) -max(x) ) } ))


for (group in sort(unique(grouping[,1]))){
	png( file=paste(sep="", "testData/TestClustering/cluster_", group,".png"), width=800, height=800)
	ok = rownames(grouping)[which(grouping[,1] == group )]
	plot(df_scale[ok[1],], type='l', main=paste(sep="_","cluster", group) )
	for (name in ok[-1] ){
		lines( df_scale[name,] )
	}
	dev.off()
}