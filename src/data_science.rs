use ndarray::OwnedRepr;
use ndarray::prelude::*;
use ndarray_rand::{RandomExt, SamplingStrategy};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::rand as rand;
use rand::seq::IteratorRandom;

pub fn test_array() -> () {
    let arr1: ArrayBase<OwnedRepr<f64>, Ix1> = array![1., 2., 3., 4., 5., 6.];
    let arr2: ArrayBase<OwnedRepr<f64>, Ix1> = array![1., 2.2, 3.3, 4., 5., 6.];
    let arr3 = arr1 + arr2;
    println!("1D array: \t{}", arr3);
    let arr4 = array![[1., 2., 3.], [ 4., 5., 6.]];
    let arr5 = Array::from_elem((2, 1), 1.);
    let arr6 = arr4 + arr5;
    let arr7 = Array::<f64, _>::zeros(arr6.raw_dim());
    let arr8 = arr6 * arr7;
    println!("\n{}", arr8);
    println!("Shape : {:?}", arr8.raw_dim());

    let identity: &Array2<f64> = &Array::eye(3);
    let arr9 = array![[1., 2., 3.], [ 4., 5., 6.], [7., 8., 9.]];
    let arr10 = &arr9 * identity;
    println!("\n{}", identity);
    println!("\n{}", arr10);

    let arr11 = arr9.dot(identity);
    println!("\n{}", arr11);

    let arr12 = Array::<i8, _>::ones((2, 3, 4, 3));
    println!("\nMULTIDIMENSIONAL\n{}", arr12);
}

pub fn test_ndarray_rand() -> () {
    let arr13 = Array::random(
        (2,5), Uniform::new(0., 10.0)
    );
    println!("{:5.2}", arr13);

    let arr14 = array![1., 2., 3., 4., 5., 6.];
    let arr15 = arr14.sample_axis(Axis(0), 2,
                                  SamplingStrategy::WithoutReplacement);
    println!("\nSampling from:\t{}\nTwo elements:\t{}", arr14, arr15);

    let mut rng = rand::thread_rng();
    let faces = "123456";
    let arr16 = Array::from_shape_vec(
        (2,2), faces.chars().choose_multiple(&mut rng, 4)
    ).unwrap();
    println!("\nSampling from:\t{}", faces);
    println!("Elements:\n{}", arr16);
}

use ndarray_rand::rand_distr::StandardNormal;
use ndarray_stats::HistogramExt;
use ndarray_stats::histogram::{strategies::Sqrt, GridBuilder};
use noisy_float::types::{N64, n64};

pub fn test_visualize() -> () {
    let array17 = Array::<f64, _>::random_using(
        (10000,2),
        StandardNormal,
        &mut rand::thread_rng()
    );
    let data = array17.mapv(|e| n64(e));
    let grid =
        GridBuilder::<Sqrt<N64>>::from_array(&data).unwrap()
            .build();
    let histogram = data.histogram(grid);
    let histogram_matrix = histogram.counts();
    let data = histogram_matrix.sum_axis(Axis(0));
    let his_data:Vec<(f32, f32)> =
        data.iter().enumerate().map(|(e,i)| (e as f32, *i as f32) ).collect();

    let file = std::fs::File::create("standard_normal_hist.svg").unwrap();
    let mut graph = poloto::plot("Histogram", "x", "y");
    graph.histogram("Stand.Norm.Dist.", his_data).xmarker(0).ymarker(0);
    graph.simple_theme(poloto::upgrade_write(file));


    let arr18 = Array::<f64, _>::random_using((300, 2), StandardNormal, &mut rand::thread_rng());

    let data:  Vec<(f64, f64)> = arr18.axis_iter(Axis(0)).map(|e| {
        let v = e.to_vec();
        (v[0], v[1])
    }).collect();

    let x_line = [[-3,0], [3, 0]];
    let y_line = [[0,-3], [0,3]];

    let file = std::fs::File::create("standard_normal_scatter.svg").unwrap();
    let mut graph = poloto::plot("Scatter Plot", "x", "y");
    graph.line("", &x_line);
    graph.line("", &y_line);
    graph.scatter("Stand.Norm.Dist", data).ymarker(0);
    graph.simple_theme(poloto::upgrade_write(file));
}