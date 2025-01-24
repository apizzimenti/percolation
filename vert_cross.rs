//Subsitutes all occurences of one element within a vector with another element
fn subst<T: Eq + Clone>(v: &mut Vec<T>, old: T, new: T) {
    v.iter_mut().for_each(|x| if *x == old {*x = new.clone()});
}

//Given the index of an edge, returns the indicies of it's corresponding endpoint vertices
fn endpoints(r: usize, c: usize, i : usize) -> (usize, usize) {
    if i < (r + 1) * c {
        ((i / c) * r + (i % c), 
         (i / c) * r + (i % c) + r)
    } else {
        let i = i - ((r + 1) * c);
        ((i / (c - 1)) * r + (i % (c - 1)),
         (i / (c - 1)) * r + (i % (c - 1)) + 1)
    }
}

fn main() {
    //The number of rows and columns (not counting top and bottom vertical edges)
    let (r, c) = (100, 100);

    //The total number of vertices (including top and bottom "halfway" verticies)
    let num_vertices = (r + 2) * c;
    //Initalize an array that keeps track of connected components of verticies. Initalize with every vertex having it's own ID (0, 1, 2...).
    let mut vertices = (0..num_vertices).map(|i| i).collect::<Vec<_>>();
    
    //The total number of edges
    let num_edges = (r + 1) * c + r * (c - 1);
    //Initalize an array full of 'false' to correspond with an empty graph
    let mut edges = (0..num_edges).map(|_| false).collect::<Vec<_>>();
    
    //Maximum number of edges to try
    let max_edges = 100000;

    for n in 0..max_edges {
        //Pick a random edge index
        let mut i = rand::random::<usize>() % num_edges;

        //Check if edge is already present, if so, select new edge index
        while edges[i] {
            i = rand::random::<usize>() % num_edges;
        }

        //Add edge to graph
        edges[i] = true;

        //Determine connected component IDs of endpoints
        let (a, b) = endpoints(r, c, i);
        let min: usize = std::cmp::min(vertices[a], vertices[b]);
        let max: usize = std::cmp::max(vertices[a], vertices[b]);

        //Check if this edge will create a vertical crossing
        let mut completer = false;
        for j in vertices.len()-c..vertices.len() {
            if (vertices[j] < c) || ((min < c) && vertices[j] == max) {
                completer = true;
            }
        }
        if completer {
            //If so, break out of the loop
            println!("Vertical crossing created after {} edges added", n);
            break;
        } else {
            //If not, subsittue all occurences of the max ID with the min ID
            subst(&mut vertices, max, min);
        }
        //Panic if about to run out of edges
        if n == max_edges - 1 {
            panic!("Added {} edges, did not create vertical crossing", n);
        }
    }
}