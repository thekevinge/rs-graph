/*
 * Copyright (c) 2018-2021 Frank Fischer <frank-fischer@shadow-soft.de>
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

//! Extend a graph with attributes.
//!
//! Sometimes one needs additional attributes associated with the nodes or edges of a graph.
//! This module provides some traits to access associated node and edge attributes if the graph
//! type supports them.
use crate::traits::Graph;

/// Object with associated node attributes.
pub trait NodeAttributes<G, Attr>
where
    G: Graph,
{
    // Return the attributes associated with a node.
    fn node(&self, u: G::Node<'_>) -> &Attr;

    // Return mutable attributes associated with a node.
    fn node_mut(&mut self, u: G::Node<'_>) -> &mut Attr;
}

/// Object with associated edge attributes.
pub trait EdgeAttributes<G, Attr>
where
    G: Graph,
{
    // Return the attributes associated with an edge.
    fn edge(&self, e: G::Edge<'_>) -> &Attr;

    // Return mutable attributes associated with an node.
    fn edge_mut(&mut self, e: G::Edge<'_>) -> &mut Attr;
}

/// A trait to split the graph and its attributes.
pub trait AttributedGraph {
    type Graph;
    type Attributes<'a>
    where
        Self: 'a;

    fn split(&mut self) -> (&Self::Graph, Self::Attributes<'_>);
}
