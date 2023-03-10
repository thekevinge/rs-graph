/*
 * Copyright (c) 2017-2022 Frank Fischer <frank-fischer@shadow-soft.de>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see  <http://www.gnu.org/licenses/>
 */

//! Reverse the direction of the edges of a digraph.

use crate::traits::refs::{DirectedRef, FiniteDigraphRef, FiniteGraphRef, GraphTypeRef, IndexGraphRef, UndirectedRef};
use crate::traits::{
    Directed, DirectedEdge, FiniteDigraph, FiniteGraph, GraphIterator, GraphType, IndexGraph, Undirected,
};

/// A digraph wrapping an existing graph with edges in opposite
/// directions.
///
/// The sets of outgoing and incoming edges handled by the methods of
/// `Digraph` and `Network` are swapped, so incoming edges becoming
/// outgoing edges and vice versa.
///
/// # Example
///
/// ```
/// use rs_graph::LinkedListGraph;
/// use rs_graph::traits::*;
/// use rs_graph::reverse;
/// use rs_graph::classes::star;
///
/// let g = star::<LinkedListGraph>(42);
/// assert_eq!(g.num_nodes(), 43);
/// assert_eq!(g.num_edges(), 42);
/// assert!(g.edges().all(|e| g.node_id(g.src(e)) == 0 && g.node_id(g.snk(e)) > 0));
/// assert!(g.outedges(g.id2node(0)).all(|(_,v)| g.node_id(v) > 0));
/// assert!(g.inedges(g.id2node(0)).all(|(_,v)| g.node_id(v) == 0));
/// assert_eq!(g.outedges(g.id2node(0)).count(), 42);
/// assert_eq!(g.inedges(g.id2node(0)).count(), 0);
///
/// // Can be used by wrapping a reference.
/// {
///     let g = reverse(&g);
///     assert_eq!(g.num_nodes(), 43);
///     assert_eq!(g.num_edges(), 42);
/// }
///
/// // Or by conversion.
/// let g = reverse(&g);
/// assert_eq!(g.num_nodes(), 43);
/// assert_eq!(g.num_edges(), 42);
/// assert!(g.edges().all(|e| g.node_id(g.snk(e)) == 0 && g.node_id(g.src(e)) > 0));
/// assert!(g.outedges(g.id2node(0)).all(|(_,v)| g.node_id(v) == 0));
/// assert!(g.inedges(g.id2node(0)).all(|(_,v)| g.node_id(v) > 0));
/// assert_eq!(g.outedges(g.id2node(0)).count(), 0);
/// assert_eq!(g.inedges(g.id2node(0)).count(), 42);
///
/// ```
#[derive(Clone, Copy)]
pub struct ReverseDigraph<'a, G>(&'a G);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ReverseDirectedEdge<E>(E);

#[derive(Clone)]
pub struct ReverseWrapIt<I>(I);

impl<'a, G, I> GraphIterator<ReverseDigraph<'a, G>> for ReverseWrapIt<I>
where
    I: GraphIterator<G>,
{
    type Item = I::Item;

    fn next(&mut self, g: &ReverseDigraph<'a, G>) -> Option<I::Item> {
        self.0.next(g.0)
    }

    fn size_hint(&self, g: &ReverseDigraph<'a, G>) -> (usize, Option<usize>) {
        self.0.size_hint(g.0)
    }

    fn count(self, g: &ReverseDigraph<'a, G>) -> usize {
        self.0.count(g.0)
    }
}

impl<E> DirectedEdge for ReverseDirectedEdge<E>
where
    E: DirectedEdge,
{
    type Edge = E::Edge;

    fn is_incoming(&self) -> bool {
        self.0.is_outgoing()
    }

    fn is_outgoing(&self) -> bool {
        self.0.is_incoming()
    }

    fn edge(&self) -> Self::Edge {
        self.0.edge()
    }
}

impl<'g, G> GraphType for ReverseDigraph<'g, G>
where
    G: GraphType,
{
    type Node<'a> = G::Node<'a>;

    type Edge<'a> = G::Edge<'a>;
}

impl<'g, G> FiniteGraph for ReverseDigraph<'g, G>
where
    G: FiniteGraph,
{
    type NodeIt<'a> = ReverseWrapIt<G::NodeIt<'a>>
    where
        G: 'a,
        'g: 'a;

    type EdgeIt<'a> = ReverseWrapIt<G::EdgeIt<'a>>
    where
        G: 'a,
        'g: 'a;

    fn num_nodes(&self) -> usize {
        self.0.num_nodes()
    }

    fn num_edges(&self) -> usize {
        self.0.num_edges()
    }

    fn nodes_iter(&self) -> Self::NodeIt<'_> {
        ReverseWrapIt(self.0.nodes_iter())
    }

    fn edges_iter(&self) -> Self::EdgeIt<'_> {
        ReverseWrapIt(self.0.edges_iter())
    }

    fn enodes(&self, e: Self::Edge<'_>) -> (Self::Node<'_>, Self::Node<'_>) {
        self.0.enodes(e)
    }
}

impl<'g, G> FiniteDigraph for ReverseDigraph<'g, G>
where
    G: FiniteDigraph,
{
    fn src(&self, e: Self::Edge<'_>) -> Self::Node<'_> {
        self.0.snk(e)
    }

    fn snk(&self, e: Self::Edge<'_>) -> Self::Node<'_> {
        self.0.src(e)
    }
}

impl<'g, G> Undirected for ReverseDigraph<'g, G>
where
    G: Undirected,
{
    type NeighIt<'a> = ReverseWrapIt<G::NeighIt<'a>>
    where
        G: 'a,
        'g: 'a;

    fn neigh_iter(&self, u: Self::Node<'_>) -> Self::NeighIt<'_> {
        ReverseWrapIt(self.0.neigh_iter(u))
    }
}

impl<'g, G> IndexGraph for ReverseDigraph<'g, G>
where
    G: IndexGraph,
{
    fn node_id(&self, u: Self::Node<'_>) -> usize {
        self.0.node_id(u)
    }

    fn id2node(&self, id: usize) -> Self::Node<'_> {
        self.0.id2node(id)
    }

    fn edge_id(&self, e: Self::Edge<'_>) -> usize {
        self.0.edge_id(e)
    }

    fn id2edge(&self, id: usize) -> Self::Edge<'_> {
        self.0.id2edge(id)
    }
}

#[derive(Clone)]
pub struct ReverseIncidentIt<I>(I);

impl<'a, G, I, N, D> GraphIterator<ReverseDigraph<'a, G>> for ReverseIncidentIt<I>
where
    I: GraphIterator<G, Item = (D, N)>,
    D: DirectedEdge,
{
    type Item = (ReverseDirectedEdge<D>, N);

    fn next(&mut self, g: &ReverseDigraph<G>) -> Option<Self::Item> {
        self.0.next(g.0).map(|(e, v)| (ReverseDirectedEdge(e), v))
    }

    fn size_hint(&self, g: &ReverseDigraph<G>) -> (usize, Option<usize>) {
        self.0.size_hint(g.0)
    }

    fn count(self, g: &ReverseDigraph<G>) -> usize {
        self.0.count(g.0)
    }
}

impl<'g, G> Directed for ReverseDigraph<'g, G>
where
    G: Directed,
{
    type OutIt<'a> = ReverseWrapIt<G::InIt<'a>>
    where
        G: 'a,
        'g: 'a;

    type InIt<'a> = ReverseWrapIt<G::OutIt<'a>>
    where
        G: 'a,
        'g: 'a;

    type IncidentIt<'a> = ReverseIncidentIt<G::IncidentIt<'a>>
    where
        G: 'a,
        'g: 'a,;

    type DirectedEdge<'a> = ReverseDirectedEdge<G::DirectedEdge<'a>>
    where
        Self: 'a;

    fn out_iter(&self, u: Self::Node<'_>) -> Self::OutIt<'_> {
        ReverseWrapIt(self.0.in_iter(u))
    }

    fn in_iter(&self, u: Self::Node<'_>) -> Self::InIt<'_> {
        ReverseWrapIt(self.0.out_iter(u))
    }

    fn incident_iter(&self, u: Self::Node<'_>) -> Self::IncidentIt<'_> {
        ReverseIncidentIt(self.0.incident_iter(u))
    }
}

pub fn reverse<G: Directed>(g: &G) -> ReverseDigraph<G> {
    ReverseDigraph(g)
}

impl<'a, G> GraphTypeRef<'a> for ReverseDigraph<'a, G>
where
    G: GraphTypeRef<'a>,
{
    type Node = G::Node;
    type Edge = G::Edge;
}

impl<'a, G> FiniteGraphRef<'a> for ReverseDigraph<'a, G>
where
    G: FiniteGraphRef<'a>,
{
    type NodeIt = ReverseWrapIt<G::NodeIt>;

    type EdgeIt = ReverseWrapIt<G::EdgeIt>;

    fn num_nodes(&self) -> usize {
        self.0.num_nodes()
    }

    fn num_edges(&self) -> usize {
        self.0.num_edges()
    }

    fn nodes_iter(&self) -> Self::NodeIt {
        ReverseWrapIt(self.0.nodes_iter())
    }

    fn edges_iter(&self) -> Self::EdgeIt {
        ReverseWrapIt(self.0.edges_iter())
    }

    fn enodes(&self, e: Self::Edge) -> (Self::Node, Self::Node) {
        self.0.enodes(e)
    }
}

impl<'a, G> FiniteDigraphRef<'a> for ReverseDigraph<'a, G>
where
    G: FiniteDigraphRef<'a>,
{
    fn src(&self, e: Self::Edge) -> Self::Node {
        self.0.snk(e)
    }

    fn snk(&self, e: Self::Edge) -> Self::Node {
        self.0.src(e)
    }
}

impl<'a, G> UndirectedRef<'a> for ReverseDigraph<'a, G>
where
    G: UndirectedRef<'a>,
{
    type NeighIt = ReverseWrapIt<G::NeighIt>;

    fn neigh_iter(&self, u: Self::Node) -> Self::NeighIt {
        ReverseWrapIt(UndirectedRef::neigh_iter(self.0, u))
    }
}

impl<'a, G> DirectedRef<'a> for ReverseDigraph<'a, G>
where
    G: DirectedRef<'a>,
{
    type OutIt = ReverseWrapIt<G::InIt>;

    type InIt = ReverseWrapIt<G::OutIt>;

    type IncidentIt = ReverseIncidentIt<G::IncidentIt>;

    type DirectedEdge = ReverseDirectedEdge<G::DirectedEdge>;

    fn out_iter(&self, u: Self::Node) -> Self::OutIt {
        ReverseWrapIt(DirectedRef::in_iter(self.0, u))
    }

    fn in_iter(&self, u: Self::Node) -> Self::InIt {
        ReverseWrapIt(DirectedRef::out_iter(self.0, u))
    }

    fn incident_iter(&self, u: Self::Node) -> Self::IncidentIt {
        ReverseIncidentIt(self.0.incident_iter(u))
    }
}

impl<'a, G> IndexGraphRef<'a> for ReverseDigraph<'a, G>
where
    G: IndexGraphRef<'a>,
{
    fn node_id(&self, u: Self::Node) -> usize {
        self.0.node_id(u)
    }

    fn edge_id(&self, e: Self::Edge) -> usize {
        self.0.edge_id(e)
    }

    fn id2node(&self, id: usize) -> Self::Node {
        self.0.id2node(id)
    }

    fn id2edge(&self, id: usize) -> Self::Edge {
        self.0.id2edge(id)
    }
}
