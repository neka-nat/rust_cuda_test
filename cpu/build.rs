use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new("../gpu")
        .copy_to("../resources/add.ptx")
        .build()
        .unwrap();
}