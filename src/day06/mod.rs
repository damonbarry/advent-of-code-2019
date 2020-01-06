#[cfg(test)]
mod tests {
    use petgraph::{
        graph::{Graph, NodeIndex},
        Direction,
    };
    use std::collections::HashMap;

    struct OrbitMap<'a> {
        graph: Graph<&'a str, u32>,
        _com: NodeIndex,
    }

    impl<'a> OrbitMap<'a> {
        pub fn with_orbits(orbits: &[(&'a str, &'a str)]) -> Self {
            let mut com: Option<NodeIndex> = None;
            let mut graph = Graph::<&str, u32>::new();
            let mut object_map = HashMap::<&str, NodeIndex>::new();

            for (obj1, obj2) in orbits {
                if !object_map.contains_key(obj1) {
                    let _ = object_map.insert(obj1, graph.add_node(obj1));
                }

                if !object_map.contains_key(obj2) {
                    let _ = object_map.insert(obj2, graph.add_node(obj2));
                }

                let orbitee = *object_map.get(obj1).unwrap();
                let orbiter = *object_map.get(obj2).unwrap();

                graph.extend_with_edges(&[(orbitee, orbiter)]);

                if *obj1 == "COM" {
                    com = Some(orbitee);
                }
            }

            let com = com.unwrap();

            Self::annotate_with_orbit_depths(com, 1, &mut graph);

            OrbitMap { graph, _com: com }
        }

        fn annotate_with_orbit_depths(
            orbitee: NodeIndex,
            depth: u32,
            graph: &mut Graph<&str, u32>,
        ) {
            let mut orbiters = graph.neighbors(orbitee).detach();
            while let Some(orbiter) = orbiters.next_node(graph) {
                graph.update_edge(orbitee, orbiter, depth);
                Self::annotate_with_orbit_depths(orbiter, depth + 1, graph);
            }
        }

        pub fn count_from(&self, object: &str) -> Result<u32, ()> {
            let object = Self::find(object, &self.graph).ok_or(())?;

            let edge = self.graph.first_edge(object, Direction::Incoming);
            if edge.is_none() {
                return Ok(0);
            }

            let edge = edge.unwrap();
            self.graph.edge_weight(edge).copied().ok_or(())
        }

        fn find(object: &str, graph: &Graph<&str, u32>) -> Option<NodeIndex> {
            graph.node_indices().find(|i| graph[*i] == object)
        }

        pub fn count_all(&self) -> u32 {
            self.graph.edge_indices().map(|i| self.graph[i]).sum()
        }
    }

    #[test]
    fn test_part1_example1() {
        let map = OrbitMap::with_orbits(&[("COM", "B"), ("B", "C"), ("C", "D")]);
        assert_eq!(Ok(3), map.count_from("D"));
    }

    #[test]
    fn test_part1_example2() {
        let map = OrbitMap::with_orbits(&[
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]);
        assert_eq!(Ok(7), map.count_from("L"));
    }

    #[test]
    fn test_part1_example3() {
        let map = OrbitMap::with_orbits(&[("COM", "B")]);
        assert_eq!(Ok(0), map.count_from("COM"));
    }

    #[test]
    fn test_part1_example4() {
        //         G - H       J - K - L
        //        /           /
        // COM - B - C - D - E - F
        //                \
        //                 I
        let map = OrbitMap::with_orbits(&[
            ("COM", "B"),
            ("B", "C"),
            ("B", "G"),
            ("G", "H"),
            ("C", "D"),
            ("D", "E"),
            ("D", "I"),
            ("E", "F"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]);
        assert_eq!(42, map.count_all());
    }

    #[test]
    fn solve_day6_part1() {
        let input = std::fs::read_to_string("src/day06/input.txt").unwrap();
        let orbits: Vec<_> = input
            .lines()
            .map(|l| {
                let objects: Vec<_> = l.split(')').collect();
                (objects[0], objects[1])
            })
            .collect();

        let map = OrbitMap::with_orbits(&orbits[..]);
        assert_eq!(186597, map.count_all())
    }
}
