import pygal
import sys

filename = sys.argv[1]
theFile = open(filename,'r')
testOutput = theFile.read()
inputString = testOutput.split()
inputInt = map(int,inputString)
line_chart = pygal.Line()
line_chart.title = 'Test'
line_chart.add('Test',inputInt)
print inputInt
line_chart.render_to_png('a-test-graph.png')
