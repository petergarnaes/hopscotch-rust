import pygal
import sys

filename = sys.argv[1]
theFile = open(filename,'r')
testOutput = theFile.readlines()
theTitle = testOutput[0]
xAxisTitle = testOutput[1]
yAxisTitle = testOutput[2]
inputX = testOutput[3].split()
inputY = testOutput[4].split()
line_chart = pygal.Line(title=theTitle,x_title=xAxisTitle,y_title=yAxisTitle)
line_chart.x_labels = inputX
line_chart.add('Test',map(int,inputY))
#Partitions filename to name the graph
(directory,seperator,theFile) = filename.rpartition("/")
line_chart.render_to_png('graph-pictures/'+theFile+'-graph.png')
