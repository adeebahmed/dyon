use super::*;

/// Normalize directed acyclic graph such that all children are sorted in memory,
/// and no child is stored before its parent.
pub fn fix(nodes: &mut [Node]) {
    sort(nodes, |n| &mut n.parent, |n| &mut n.children)
}

/// Performs in-memory topological sort on a directed acyclic graph where
/// order is determined by every child being greater than their parent,
/// and every sibling being greater than previous siblings.
pub fn sort<T, P, C>(nodes: &mut [T], parent: P, children: C)
    where P: Fn(&mut T) -> &mut Option<usize>,
          C: Fn(&mut T) -> &mut [usize]
{
    // This problem can be solving efficiently using Group Theory.
    // This avoid the need for cloning nodes into a new array,
    // while performing the minimum work to get a normalized graph.
    //
    // Create a group generator that is modified by swapping to find a solution.
    // The group generator keeps track of indices, such that child-parent relations
    // do not have to change until later.
    //
    // Use the order in the generator to detect whether a swap has been performed.
    // The condition for swapping `a, b` is `gen[a] > gen[b]`.
    let mut gen: Vec<usize> = (0..nodes.len()).collect();
    loop {
        let mut changed = false;
        for i in 0..nodes.len() {
            let children = children(&mut nodes[i]);
            for j in 0..children.len() {
                let a = children[j];
                // Store child after its parent.
                if gen[i] > gen[a] {
                    gen.swap(i, a);
                    changed = true;
                }
                // Check all pairs of children.
                for k in j+1..children.len() {
                    let b = children[k];

                    // Store children in sorted order.
                    if gen[a] > gen[b] {
                        gen.swap(a, b);
                        changed = true;
                    }
                }
            }
        }
        if !changed {break}
    }

    // Update the graph data with the new indices from the generator.
    // Do this before performing the actual swapping,
    // since the generator maps from old indices to new indices.
    for i in 0..nodes.len() {
        let p = parent(&mut nodes[i]);
        *p = p.map(|p| gen[p]);
        for ch in children(&mut nodes[i]) {*ch = gen[*ch]}
    }

    // Swap nodes using the group generator as guide.
    // When swapping has been performed, update the generator to keep track of state.
    // This is because multiple swaps sharing elements might require multiple steps.
    //
    // The order which swaps are retraced might be different than the solving phase:
    //
    // `a, b, c` => `a, (c, b)` => `(c, a), b` => `c, a, b` (solving phase)
    // `c, a, b` => `(b), a, (c)` => `(a, b), c` => `a, b, c` (retrace phase)
    //
    // However, since the generator solution is produced by swapping operations,
    // it is guaranteed to be restorable to the identity generator when retracing.
    //
    // There is no need to loop more than once because each index is stored uniquely by lookup,
    // such that if `g[i] = i` then there exists no `j != i` such that `g[j] = i`.
    for i in 0..nodes.len() {
        while gen[i] != i {
            let j = gen[i];
            nodes.swap(i, j);
            gen.swap(i, j);
        }
    }
}
