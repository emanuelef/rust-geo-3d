use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::position::haversine_distance;
mod point3d;
mod position;

fn main() {
    let p1 = point3d::Point3D { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = point3d::Point3D { x: 4.0, y: 5.0, z: 6.0 };
    let p3 = p1.add(p2);
    println!("p3 = {:?}", p3);

    let position1 = position::Coord3D {
        coord2d: position::Coord2D { lat: 1.0, lon: 2.0 },
        alt: 3.0,
    };

    let position2 = position::Coord3D {
        coord2d: position::Coord2D { lat: 3.0, lon: 7.0 },
        alt: 6.0,
    };

    println!("position1 = {:?}", position1);

    let start_time = Instant::now();

    // Define the number of threads to use
    let num_threads = num_cpus::get();

    // Arc and Mutex for sharing mutable state between threads
    let position1_arc = Arc::new(position1.coord2d);
    let position2_arc = Arc::new(position2.coord2d);

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Use Rayon to easily parallelize the computation
    for _ in 0..num_threads {
        // Clone Arcs for each thread
        let position1_clone = Arc::clone(&position1_arc);
        let position2_clone = Arc::clone(&position2_arc);

        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Perform the computation in each thread
            for _ in 0..250_000_000 {
                let _distance = haversine_distance(&position1_clone, &position2_clone);
            }
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("Total time taken: {:?}", duration);
}
