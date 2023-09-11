
every ply is a frame

render render the point cloud to mp4 or png

vv read ./Ply +output=plys
- probably read the type plys and output to plys

- metric: not really sure what the metric is doing

- how upsample works? if there is a hole in the point cloud, how do you fill up the hole and predict what kind of point should be inside

- didn't test dash




for meeting:
```bash
vv read -n 60 ./pcd +output=pcd \
    render ./mp4 \
    +input=f --format mp4 --fps 20
```

- is there any possible error for this command +input=f part?
- try with pcd

```bash
vv read ./Ply +output=ply \
       upsample --factor 2 +input=ply +output=ply_up \
       write ./ply_up \
             +input=ply_up \
             --storage-type binary \
             --output-format ply
Killed
```
- skip this part
- when this is ran, it is killed. why?
- also killed with pcd_b dir input

- typo at: An example of netowrk setting file is provided in ./test_files/dash/sim_nw_avg_14050.txt


reading the ply file again and again
players code
figure out and speed it up

buffer, read ahead

view point doesn't matter, clockwise nad counterclock wise
what to store:
- ascii ply file

- pcd read the whole file: no proocessing involve
- ply_ascii: text to byte (need improve) (the problem is this part), need preload, put in bufer
    - read and understand the player
    - try to implement the buffer system

master and spring cleanning

try with main branch and vvplay

- render display point cloud: 

- vvtk/src/bin/vvplay.rs: familiarised with the codebase, and see how it works, and speed it up
- try with the one with higher resolution

question:
- what is metrics? does it matter? and what is metric renderer
- another ply_ascii also not really usable
- if there is only one ply file, means that there is only one frame?

5/8/2023:
- it is predicted that it is either go left or go right
- take a look at renderer.rs 


progress:
29/ aug 30 mins: still cannot find the main function that cause the problem
30 aug 20 mins: reviewed renderer.rs and ply.rs, suspect that the render function might be related to read_ply, trace which called that

3 september 2023:
- quantitative performance analysis can use perf to track how much time per function
https://stackoverflow.com/questions/64683236/using-vscode-debug-console-with-rust
- try vscode but build failed, try this infuture
```rust
    let render = builder.add_window(Renderer::new(
        reader,
        args.fps,
        camera,
        (args.width, args.height),
        metrics,
        args.bg_color.to_str().unwrap(),
    ));
    ```
    - can trace from this line in src/bin/vvplay.rs
- main function that being called by vvplay.rs is in ~/Documents/vvtk/src/render/wgpu
- can try t trace performance next time
```rust
impl From<VelodynPoint> for pointxyzrgba::PointXyzRgba {
    fn from(value: VelodynPoint) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            r: (value.intensity * 255.0) as u8,
            g: (value.intensity * 255.0) as u8,
            b: (value.intensity * 255.0) as u8,
            a: 255,
        }
    }
}
````
- will this be something related? in ~/Documents/vvtk/src/formats

notes:
- clap crate: parsing command line argument 
in src/util.rs: pointXyzRgba is the type of the point cloud, just need to trace which one rendered the whole frame
```rust
    #[test]
    fn test_read_ply() {
        let ply_ascii_path = PathBuf::from("./test_files/ply_ascii/longdress_vox10_1213_short.ply");
        let pc = read_ply(&ply_ascii_path).unwrap();
        assert_eq!(pc.number_of_points, 20);
        assert_eq!(
            pc.points[0],
            PointXyzRgba {
                x: 171.0,
                y: 63.0,
                z: 255.0,
                r: 183,
                g: 165,
                b: 155,
                a: 255
            }
        );
        ```
- cfg flag: for configuration
```rust
#[cfg(target_os = "linux")]
fn linux_specific_function() {
    // This code is only compiled on Linux
}
```
```bash
cargo run --bin vvplay /home/byt/Documents/vvtk/test_files/longdress/longdress/test
```
- running this work
```bash
gdb --args executablename arg1 arg2 arg3
```
- provide argument for rust gdb

4/9/2023
- ask what is going on in src/render/wgpu/builder.rs
    0 what is window
- try add a few more frame to see what is triggered
- try perf and flame to analyse, write benchmark later
https://blog.anp.lol/rust/2016/07/24/profiling-rust-perf-flamegraph/

5/9/2023
- guide for flamegraph, not in path yet https://nanxiao.me/en/use-perf-and-flamegraph-to-profile-program-on-linux/
- change config.toml according to this: https://rustc-dev-guide.rust-lang.org/profiling/with_perf.html
    - not done yet

6/9/2023
run sudo sh -c 'echo 1 >/proc/sys/kernel/perf_event_paranoid': https://superuser.com/questions/980632/run-perf-without-root-rights
in flamegraph/vvtk result, run perf script | ../stackcollapse-perf.pl | ../flamegraph.pl > perf.svg
use perf report for tui interface
the unknown should be caused by the restriction listed, it is ok in the new one: https://users.rust-lang.org/t/flamegraph-shows-every-caller-is-unknown/52408
add .cargo and the thing inside
run perf record -g ./vvplay ../../test_files/longdress/longdress/test/, in vvtk/target/debug
need to guess and write something smaller

8/9/2023: meeting
- learn more about the codebase?? with chatgpt
- when go from ply_ascii to pointcloud to render crate, when you load back and forth, does it start from the ply_ascii or it start from point cloud to rendered
- ^ this determine what to implement
- read more about the wgpu crate, and see which part is called to render to code

11/9/2023: familiarise code base
- write test for any functions if needed
- ply.rs is basically doing the ply reading and converting job  





utils.rs/test/test_pcd_to_ply: run the test again
