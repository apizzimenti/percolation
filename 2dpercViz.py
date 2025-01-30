import re
from PIL import Image

#Dimensions of Main graph
n = 50
m = 52

#Essentially just get the edges in (Vertex1, Vertex2) form and put them in a list
dataFiltered = []
final = []
frames = []

dataFile = open("message.txt", "r")
data = dataFile.read()
dataFile.close()
dataFiltered = re.findall("[0-9]+", data)
for i in range(0, len(dataFiltered), 2):
    s = (int(dataFiltered[i]), int(dataFiltered[i+1]))
    final.append(s)
    
#Create empty image of size (2n-1, 2m-1)
image = Image.new(mode='RGB', size=((2*n)-1,(2*m)-1), color=(255,255,255))
for i in range (0, (2*m)-1, 2):
    for j in range(0, (2*n)-1, 2):
        #Place a black pixel on each vertex of the primal
        image.putpixel(xy=(j,i), value=(0,0,0))

#add empty graph as a frame to gif
im = image.copy()#.resize((10*((2*n)-1), 10*((2*m)-1)))
frames.append(im)

for edge in final:
    #for each edge, get the coordinates from the vertex indecies
    v1x = 2 * (edge[0] % n)
    v1y = 2 * (edge[0] // n)
    v2x = 2 * (edge[1] % n)
    v2y = 2 * (edge[1] // n)

    #get final coordinate
    ex = (v1x + v2x) // 2
    ey = (v1y + v2y) // 2

    #place black square on edge @ coordinate
    image.putpixel(xy=(ex, ey), value=(0,0,0))

    #add new edge as frame
    im = image.copy()#.resize((10*((2*n)-1), 10*((2*m)-1)))
    frames.append(im)

#save and exit
frames[0].save('FINAL.gif', save_all = True, append_images = frames[1:], optimize = False, duration = 0.25)
