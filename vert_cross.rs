use std::collections::HashSet;

use union_find::{QuickUnionUf, Union, UnionFind, UnionResult};

//Function which takes edge indicies to pairs of vertex indicies
fn endpoints(n: usize, i: usize) -> (usize, usize) {
    if i < (n+1)*n {
        (i, i+n)
    } else {
        let i = i - (n+1)*n;
        let x = i % (n - 1);
        let y = 1 + (i / (n - 1));
        (y*n+x, y*n+x+1)
    }
}

//Structure which holds the metadata about each connected compoent: It's rank and whether it's connected to the top and/or bottom
#[derive(Copy, Clone, Debug, Default)]
struct ComponentData {
    rank: u8,
    bottom: bool,
    top: bool,
}

//Implentation for how to combine the data attached to two connected components, the top and/or bottom flags are marked
//as true for the new combined component iff the respective flag is true for at least one of the two partial components.
impl Union for ComponentData {
    fn union(left: ComponentData, right: ComponentData) -> UnionResult<ComponentData> {
        let rank = if left.rank == right.rank {left.rank + 1} else {std::cmp::max(left.rank, right.rank)};

        let (bottom, top) = (left.bottom || right.bottom, left.top || right.top);

        if left.rank < right.rank {
            UnionResult::Right(ComponentData{rank, bottom, top})
        } else {
            UnionResult::Left(ComponentData{rank, bottom, top})
        }
    }
}


fn main(){
    //Grid Size
    let n = 4000;

    //Initialize Union-Find strcture on verticies
    let mut uf = QuickUnionUf::<ComponentData>::new((n+2)*n);

    //Initalize edge hashset (not used for algorithim but needed to recover the graph)
    let mut edges = HashSet::<usize>::new();

    //Mark the top and bottom row of verticies as connected to the top and bottom respectivly (default for those flags is false)
    (0..n).for_each(|i| {
        uf.get_mut(i).bottom = true; 
        uf.get_mut((n+1)*n+i).top = true; 
    });

    //Initalize some varibles, 'i' to hold the edge index and 'a' and 'b' to hold the vertex indicies of the endpoints
    let (mut i, mut a, mut b); (a, i) = (0, 0);

    while !(uf.get(a).top && uf.get(a).bottom) {
        //Generate a random edge index.
        i = rand::random::<usize>() % ((n+1)*n + n*(n-1));

        //Insert the edge index into the HashSet
        edges.insert(i);

        //Determine the two endpoint indicies
        (a, b) = endpoints(n, i);
        
        //Combine the two components that the endpoints lie within
        uf.union(a, b);
    }
    
    //Remove the most recent edge added
    edges.remove(&i);

    //Display the number of edges
    println!("{:?}", edges.len());
}
