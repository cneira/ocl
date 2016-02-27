//!
//! [UNIMPLEMENTED][WORK IN PROGRESS]
//!

#![allow(unused_imports, unused_variables, dead_code)]

extern crate ocl;
use ocl::{SimpleDims, Context, DeviceSpecifier, Image};

static KERNEL_SRC: &'static str = r#"
        __kernel void multiply_by_scalar(
                    __global float const* const src,
                    __private float const coeff,
                    __global float* const res)
        {
            uint const idx = get_global_id(0);

            res[idx] = src[idx] * coeff;
        }
    "#;

#[allow(unused_variables)]
fn main() {
    let image_dims = SimpleDims::Two(200, 100);

    // Create a context with the first available platform and default device type:
    // let context = Context::new_by_index_and_type(None, None).unwrap();

    let context = Context::builder().device_spec(DeviceSpecifier::Index(0)).build().unwrap();

    let img_formats = Image::supported_formats(&context, ocl::MEM_READ_WRITE, 
        ocl::MemObjectType::Image2d).unwrap();

    println!("Image Formats Avaliable: {}.", img_formats.len());

    let data: Vec<u8> = Vec::with_capacity(100000);

    let image = Image::builder().flags(ocl::MEM_READ_WRITE | ocl::MEM_COPY_HOST_PTR)
        .build_with_data(&context, &data).unwrap();

    println!("{:#}", image);

    // pub enum ImageInfo {
    //     Format = cl_h::CL_IMAGE_FORMAT as isize,
    //     ElementSize = cl_h::CL_IMAGE_ELEMENT_SIZE as isize,
    //     RowPitch = cl_h::CL_IMAGE_ROW_PITCH as isize,
    //     SlicePitch = cl_h::CL_IMAGE_SLICE_PITCH as isize,
    //     Width = cl_h::CL_IMAGE_WIDTH as isize,
    //     Height = cl_h::CL_IMAGE_HEIGHT as isize,
    //     Depth = cl_h::CL_IMAGE_DEPTH as isize,
    //     ArraySize = cl_h::CL_IMAGE_ARRAY_SIZE as isize,
    //     Buffer = cl_h::CL_IMAGE_BUFFER as isize,
    //     NumMipLevels = cl_h::CL_IMAGE_NUM_MIP_LEVELS as isize,
    //     NumSamples = cl_h::CL_IMAGE_NUM_SAMPLES as isize,
    // }


    // // Create a program/queue with the first available device: 
    // let ocl_pq = ProQue::builder().src(KERNEL_SRC).build().expect("ProQue build");

    // // Set up our work dimensions / data set size:
    // let dims = SimpleDims::One(data_set_size);

    // // Create a 'Buffer' (a local vector + a remote buffer) as a data source:
    // let source_buffer: Buffer<f32> = 
    //  Buffer::with_vec_scrambled(0.0f32, 20.0f32, &dims, &ocl_pq.queue());

    // // Create another empty buffer for results:
    // let mut result_buffer: Buffer<f32> = Buffer::with_vec(&dims, &ocl_pq.queue());

    // // Create a kernel with three arguments corresponding to those in the kernel:
    // let kernel = ocl_pq.create_kernel_with_dims("multiply_by_scalar", dims.clone())
    //  .arg_buf(&source_buffer)
    //  .arg_buf(&mut result_buffer)
    // ;

    // // Enqueue kernel depending on and creating no events:
    // kernel.enqueue();

    // // Read results from the device into the buffer's vector:
    // result_buffer.fill_vec();

    // // Check results and print the first 20:
    // for idx in 0..data_set_size {
    //  // Check:
    //  assert_eq!(result_buffer[idx], source_buffer[idx] * coeff);

    //  // Print:
    //  if idx < RESULTS_TO_PRINT { 
    //      println!("source_buffer[idx]: {}, coeff: {}, result_buffer[idx]: {}",
    //      source_buffer[idx], coeff, result_buffer[idx]); 
    //  }
    // }
}