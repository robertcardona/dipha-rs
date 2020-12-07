mod dipha;

fn main() {
    println!("Hello, world!");

    let files = vec![
    "../dipha/test_data/dual_explicit.complex",
    "../dipha/test_data/explicit.diagram",
    "../dipha/test_data/noise_3_16.complex",
    "../dipha/test_data/noise_3_16_def.diagram",
    "../dipha/test_data/noise_4_8.complex",
    "../dipha/test_data/noise_4_8_def.diagram",
    "../dipha/test_data/primal_explicit.complex",
    "../dipha/test_data/ramp_3_16.complex",
    "../dipha/test_data/ramp_3_16_def.diagram",
    "../dipha/test_data/ramp_4_8.complex",
    "../dipha/test_data/ramp_4_8_def.diagram",
    "../dipha/test_data/smooth_16.complex",
    "../dipha/test_data/smooth_2.diagram",
    "../dipha/test_data/smooth_def.diagram",
    "../dipha/test_data/sphere_1_0.3.diagram",
    "../dipha/test_data/sphere_1.diagram",
    "../dipha/test_data/sphere_2_0.5.diagram",
    "../dipha/test_data/sphere_2.diagram",
    "../dipha/test_data/sphere_3_0.8.diagram",
    "../dipha/test_data/sphere_3_2.diagram",
    "../dipha/test_data/sphere_3_32_0.3.complex",
    "../dipha/test_data/sphere_3_32_0.5.complex",
    "../dipha/test_data/sphere_3_32_0.8.complex",
    "../dipha/test_data/sphere_3_32_2.complex",
    "../dipha/test_data/sphere_3_32.complex",
    "../dipha/test_data/sphere_3.diagram",
    "../ripser/examples/projective_plane.dipha",
    "../../cubical-ripser/CubicalRipser_2dim/dat/data-1.complex",
    "../../cubical-ripser/CubicalRipser_2dim/dat/data-1.diagram",
    // "../../cubical-ripser/CubicalRipser_2dim/dat/noise_2_1000.complex",
    "../../cubical-ripser/CubicalRipser_2dim/dat/test_2_100.complex",
    "../../cubical-ripser/CubicalRipser_2dim/dat/test_2_100.csv",
    "../../cubical-ripser/CubicalRipser_2dim/dat/test_2_100.diagram"
    ];

    for file in files{
        dipha::read_binary_dipha(&file);
    }

    // dipha::read_binary_dipha("../dipha/test_data/sphere_3_32.complex");
    // let raw_dipha_file = dipha::get_file_as_byte_vec("../dipha/test_data/dual_explicit.complex");


    // for byte in raw_dipha_file {
    //     println!("{}", byte);
    //
    //     int64 = int64 << 8 | (byte as i64);
    //     println!("{}", int64);
    //
    //     if counter != 0 && counter % 8 == 0 {
    //         counter = 0;
    //         break;
    //     }
    //
    //     counter += 1;
    // }

}
